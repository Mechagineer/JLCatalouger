# Runs vendoring to make builds offline.
# Use on a good connection; afterwards the repo builds without network.
# Steps:
# 1) cargo install cargo-vendor
# 2) cargo vendor --versioned-dirs --respect-source-config --locked vendor
# 3) Edit .cargo/config.toml: set replace-with = "vendored-sparse" and uncomment the vendored-sparse block.

Write-Output "Vendoring crates to ./vendor ..."
$ErrorActionPreference = "Stop"
cargo vendor --versioned-dirs --respect-source-config --locked vendor
Write-Output "Done. Now edit .cargo/config.toml to replace-with = 'vendored-sparse' and commit vendor/."
