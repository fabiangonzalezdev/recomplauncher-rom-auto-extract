$ErrorActionPreference = 'Stop'
$repo = Split-Path -Parent $PSScriptRoot
$output = Join-Path $repo 'dist/rom-auto-extract'
$plugin = Join-Path $repo 'docs/examples/rom-auto-extract'
New-Item -ItemType Directory -Force $output | Out-Null

Push-Location $repo
try {
    cargo test --manifest-path src-tauri/Cargo.toml rom_archive --lib
    if ($LASTEXITCODE -ne 0) { throw 'Fallaron las pruebas de ROM Auto Extract.' }
    cargo build --manifest-path src-tauri/Cargo.toml --bin recomplauncher
    if ($LASTEXITCODE -ne 0) { throw 'No se pudo compilar el lanzador.' }
    Copy-Item -LiteralPath (Join-Path $repo 'src-tauri/target/debug/recomplauncher.exe') -Destination $output
    $archive = Join-Path $output 'rom-auto-extract.zip'
    Compress-Archive -Path (Join-Path $plugin '*') -DestinationPath $archive -Force
    Move-Item -LiteralPath $archive -Destination (Join-Path $output 'rom-auto-extract.rlplugin') -Force
    Copy-Item -LiteralPath (Join-Path $repo 'LICENSE') -Destination $output
    Write-Output "Archivos generados en $output"
} finally {
    Pop-Location
}
