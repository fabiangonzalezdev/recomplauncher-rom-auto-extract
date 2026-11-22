# ROM Auto Extract for RecompLauncher

Experimental, unofficial plugin **and modified Windows launcher** for preparing your own compressed game copies.

[Download v1.0.0](https://github.com/fabiangonzalezdev/recomplauncher-rom-auto-extract/releases/tag/v1.0.0) · [Instrucciones en español](LEEME-ES.md)

## Important compatibility information

The stock launcher plugin API at the base commit does not expose local archive extraction or setup-input replacement. **Installing the `.rlplugin` alone in the official launcher is insufficient.** Use the compatible `recomplauncher.exe` supplied in this project's release. This is a development build, not an official RecompLauncher release.

## What it does

- Extracts local ZIP, RAR and 7z archives while preparing a game, including Big Picture setup.
- Shows “Extrayendo…” and “Seleccionando…”, then selects the extracted ROM next to the archive.
- Keeps the archive and never replaces a different existing file. Identical extracted files can be reused.
- Requires the plugin's `rom.unpack` permission and respects individual and global disable switches.
- Does not download or include ROMs, BIOS files, keys or user data.

One compatible single-file ROM must be identifiable in the archive. Password-protected/multipart archives and multi-file disc sets need manual preparation. Extraction does not fix an unsupported region/revision; the launcher may suggest a region the game cannot use.

## Install on Windows x64

1. Download and extract `ROM-Auto-Extract-1.0.0-Windows.zip` from Releases.
2. Back up your launcher data and fully exit the existing launcher, including the tray instance.
3. Run the package's `recomplauncher.exe`. It uses the normal launcher data folder; it does not replace the existing executable.
4. Open Settings → Plugins → Install from file and select the supplied `rom-auto-extract.rlplugin`. Enable it.
5. Prepare a game using a ZIP/RAR/7z copy. The plugin page does not need to stay open.

Requires the normal Windows x64 WebView2 and Visual C++ runtimes. A stock launcher update may remove this custom integration.

## Validation

- Real ZIP setup: Majora's Mask / 2 Ship 2 Harkinian started and rendered its introduction with the USA copy. The European copy extracted but was rejected by 2 Ship.
- Super Mario 64 Co-op DX and Ship of Harkinian were configured with extracted ZIP copies; the user confirmed both work.
- 7 new native tests passed, including synthetic ZIP/7z/RAR, candidate validation, conflicts/reuse and permission gating.
- UI suite: 720 passed, 11 skipped, 0 failed.
- Native full suite: 904 passed, 1 skipped, 1 failed (`pads::tests::through_sdl`: detected an additional PlayStation controller).
- RAR and 7z have automated fixture coverage; they have not yet been tested with real games.

## Source and attribution

Based on [elripalda/RecompLauncher](https://github.com/elripalda/RecompLauncher), commit `95ca2128b3088c634e771232a9ef8d911cfa9d87` (beta.68).

- `plugin/`: plugin manifest, page and assets.
- `launcher-source/`: complete corresponding launcher source, native integration, tests and build script.
- `LEEME-ES.md`: detailed usage, limitations and distribution notes.

Build from `launcher-source` with Rust and the Windows C++ build tools:

```powershell
powershell -ExecutionPolicy Bypass -File tools/build-rom-auto-extract.ps1
```

The original project retains its authorship. This community modification is distributed under **GPL-3.0-or-later**; preserve the license, credits and corresponding source when redistributing. See [LICENSE](LICENSE) and [CREDITS.md](CREDITS.md).
