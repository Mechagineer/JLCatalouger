# Guardrails — Dev & Build Rules (Windows, Offline)

## Tauri ⇄ Vite
- Vite dev port is **1420** (`vite.config.ts`).
- Tauri `build.devPath` points to `http://localhost:1420` (`src-tauri/tauri.conf.json`).
- Tauri runs Vite via `build.beforeDevCommand = "pnpm run dev"`.

If Tauri shows “waiting for frontend dev server”, start `pnpm run dev` separately and confirm it prints `http://localhost:1420/`.

## Cargo on corporate networks
- `.cargo/config.toml` sets:
  - `[http] check-revoke=false, multiplexing=false`
  - `[net] retry=5, git-fetch-with-cli=true`
- If downloads remain flaky, run vendoring *once* on a good connection:
  - `cargo install cargo-vendor`
  - `cargo vendor --versioned-dirs --respect-source-config --locked vendor`
  - Then flip `.cargo/config.toml`:
    ```
    [source.crates-io]
    replace-with = "vendored-sparse"

    [source.vendored-sparse]
    registry = "sparse+file://vendor/index"
    ```
  - Commit `vendor/` (or store internally). After that, builds are offline.

## Runtime DLLs
- Place platform DLLs in `src-tauri/bin/`:
  - PDFium, Tesseract (optional), ONNX Runtime (optional), OpenCV (optional).
- They ship via `bundle.resources`.
- Don’t commit DLLs to git unless you intend to version them explicitly.

## Quick first run
1. `pnpm install`
2. `pnpm approve-builds` (approve esbuild + @esbuild/win32-x64)
3. `pnpm tauri dev`
