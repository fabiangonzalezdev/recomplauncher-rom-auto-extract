//! Local ROM archives. Only a checked, single-file game is published beside its archive.

use crate::{gamebanana, profile};
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

pub const PERMISSION: &str = "rom.unpack";

pub fn prepare(need: &profile::GameCopy, archive: &Path, report: &dyn Fn(&str)) -> Result<PathBuf, String> {
    let parent = archive.parent().ok_or("The archive has no parent folder.")?;
    let work = tempfile::Builder::new().prefix(".rl-rom-").tempdir_in(parent).map_err(|e| format!("Couldn't extract beside your ROM: {e}"))?;
    report("extracting");
    gamebanana::extract(archive, work.path()).map_err(|e| e.to_string())?;
    report("selecting");
    let mut candidates = Vec::new();
    collect(need, work.path(), &mut candidates, 0)?;
    let source = match candidates.as_slice() {
        [one] => one,
        [] => return Err("The archive has no single-file ROM that passes this game's checks.".into()),
        _ => return Err("The archive contains several compatible ROMs. Choose an archive containing just the copy you want to use.".into()),
    };
    publish(source, parent)
}

fn collect(need: &profile::GameCopy, dir: &Path, out: &mut Vec<PathBuf>, depth: usize) -> Result<(), String> {
    if depth > 32 { return Err("The archive's folders are nested too deeply.".into()); }
    for entry in fs::read_dir(dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let kind = entry.file_type().map_err(|e| e.to_string())?;
        let path = entry.path();
        // Never follow archive links or select a program, readme, playlist or a disc track.
        if kind.is_dir() {
            collect(need, &path, out, depth + 1)?;
        } else if kind.is_file() && single_rom(&path) && profile::check_copy(need, &path).is_ok() {
            out.push(path);
        }
    }
    Ok(())
}

fn single_rom(path: &Path) -> bool {
    let ext = path.extension().and_then(|x| x.to_str()).unwrap_or("").to_ascii_lowercase();
    matches!(ext.as_str(), "z64" | "n64" | "v64" | "nes" | "sfc" | "smc" | "gba" | "gb" | "gbc" | "nds" | "3ds" | "md" | "gen" | "sms" | "gg" | "pce" | "a26" | "a78" | "nup" | "nsp" | "xci" | "iso")
}

fn same_bytes(a: &Path, b: &Path) -> std::io::Result<bool> {
    if fs::metadata(a)?.len() != fs::metadata(b)?.len() { return Ok(false); }
    let (mut a, mut b) = (fs::File::open(a)?, fs::File::open(b)?);
    let (mut left, mut right) = ([0u8; 65536], [0u8; 65536]);
    loop {
        let n = a.read(&mut left)?;
        if n == 0 { return Ok(true); }
        b.read_exact(&mut right[..n])?;
        if left[..n] != right[..n] { return Ok(false); }
    }
}

fn publish(source: &Path, parent: &Path) -> Result<PathBuf, String> {
    let name = source.file_name().ok_or("The ROM has no file name.")?;
    let dest = parent.join(name);
    // A completed ROM appears atomically. An interrupted extraction leaves no partial ROM
    // for the next library scan, and an existing copy is never overwritten.
    let mut output = tempfile::NamedTempFile::new_in(parent).map_err(|e| e.to_string())?;
    let mut input = fs::File::open(source).map_err(|e| e.to_string())?;
    std::io::copy(&mut input, &mut output).map_err(|e| e.to_string())?;
    output.flush().map_err(|e| e.to_string())?;
    match output.persist_noclobber(&dest) {
        Ok(_) => Ok(dest),
        Err(e) if e.error.kind() == std::io::ErrorKind::AlreadyExists => {
            let regular = fs::symlink_metadata(&dest).map(|m| m.file_type().is_file()).unwrap_or(false);
            if regular && same_bytes(source, &dest).map_err(|e| e.to_string())? { return Ok(dest); }
            Err(format!("{} already exists with different contents. Rename it before extracting this copy.", dest.display()))
        }
        Err(e) => Err(format!("Couldn't save the extracted ROM: {}", e.error)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn need() -> profile::GameCopy {
        serde_json::from_value(serde_json::json!({"kind":"iso", "name":"game.z64", "label":"ROM"})).unwrap()
    }

    fn rom() -> Vec<u8> {
        let mut bytes = vec![0; 128];
        bytes[..4].copy_from_slice(&[0x80, 0x37, 0x12, 0x40]);
        bytes
    }

    fn zip(dir: &Path, entries: &[(&str, Vec<u8>)]) -> PathBuf {
        let file = dir.join("Game.ZIP");
        let mut z = zip::ZipWriter::new(fs::File::create(&file).unwrap());
        for (name, bytes) in entries {
            z.start_file(*name, zip::write::SimpleFileOptions::default()).unwrap();
            z.write_all(bytes).unwrap();
        }
        z.finish().unwrap();
        file
    }

    #[test]
    fn extracts_nested_rom_beside_archive_and_reports_selection() {
        let dir = tempfile::tempdir().unwrap();
        let archive = zip(dir.path(), &[("nested/Game.z64", rom()), ("readme.txt", b"notes".to_vec())]);
        let phases = std::cell::RefCell::new(Vec::new());
        let picked = prepare(&need(), &archive, &|p| phases.borrow_mut().push(p.to_string())).unwrap();
        assert_eq!(picked, dir.path().join("Game.z64"));
        assert_eq!(fs::read(picked).unwrap(), rom());
        assert!(archive.is_file());
        assert_eq!(*phases.borrow(), ["extracting", "selecting"]);
        assert_eq!(fs::read_dir(dir.path()).unwrap().count(), 2);
    }

    #[test]
    fn reuses_only_an_identical_existing_rom() {
        let dir = tempfile::tempdir().unwrap();
        let archive = zip(dir.path(), &[("Game.z64", rom())]);
        let picked = prepare(&need(), &archive, &|_| {}).unwrap();
        assert_eq!(prepare(&need(), &archive, &|_| {}).unwrap(), picked);
        fs::write(&picked, b"keep my file").unwrap();
        assert!(prepare(&need(), &archive, &|_| {}).unwrap_err().contains("different contents"));
        assert_eq!(fs::read(picked).unwrap(), b"keep my file");
    }

    #[test]
    fn refuses_ambiguous_and_invalid_copies_without_publishing() {
        let dir = tempfile::tempdir().unwrap();
        let archive = zip(dir.path(), &[("one.z64", rom()), ("two.z64", rom())]);
        assert!(prepare(&need(), &archive, &|_| {}).unwrap_err().contains("several"));
        let archive = zip(dir.path(), &[("fake.z64", b"not a ROM".to_vec())]);
        assert!(prepare(&need(), &archive, &|_| {}).is_err());
        assert_eq!(fs::read_dir(dir.path()).unwrap().count(), 1);
    }

    #[test]
    fn extracts_7z() {
        let dir = tempfile::tempdir().unwrap();
        let input = tempfile::tempdir().unwrap();
        fs::write(input.path().join("Game.z64"), rom()).unwrap();
        let archive = dir.path().join("Game.7z");
        sevenz_rust::compress_to_path(input.path(), &archive).unwrap();
        assert_eq!(fs::read(prepare(&need(), &archive, &|_| {}).unwrap()).unwrap(), rom());
    }

    #[test]
    fn extracts_rar() {
        let dir = tempfile::tempdir().unwrap();
        let archive = dir.path().join("Game.rar");
        fs::write(&archive, include_bytes!("../../tests/fixtures/rom-archive/Game.rar")).unwrap();
        assert_eq!(fs::read(prepare(&need(), &archive, &|_| {}).unwrap()).unwrap(), rom());
        assert!(crate::roms::identify(&archive, fs::metadata(&archive).unwrap().len()).is_some());
    }
}
