# Vendoring crates to ./vendor so builds work offline.
# Run from repo root on a good connection.
$ErrorActionPreference = "Stop"
Write-Host "Vendoring crates to ./vendor ..."
cargo vendor --versioned-dirs --respect-source-config vendor
Write-Host "Enable vendored (directory) source by appending to .cargo/config.toml:"
Write-Host "[source.crates-io]"
Write-Host "replace-with = 'vendored-sources'"
Write-Host ""
Write-Host "[source.vendored-sources]"
Write-Host "directory = 'vendor'"
