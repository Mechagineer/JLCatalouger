# Guardrails — Dev & Build Rules (Windows, Offline)

This file captures rules for contributors and for Cursor to follow. It prevents repeat issues and keeps the project deterministic.

## 1) Local dev (Windows only)
- Use **PowerShell** (Windows Terminal).  
- Ensure **x64** toolchain: MSVC (cl.exe), CMake, Ninja on PATH.
- Run `pnpm approve-builds` once after cloning to allow esbuild and related scripts.

## 2) Tauri ⇄ Vite wiring
- **Vite dev port** is **1420**. Do not change without also updating `src-tauri/tauri.conf.json: build.devPath`.
- Tauri launches Vite via `beforeDevCommand: "pnpm run dev"`; Vite must bind to port **1420** (see `vite.config.ts`).
- If dev window "waits for frontend dev server", start `pnpm run dev` in another terminal; confirm it prints `http://localhost:1420`.

## 3) Bundled runtime DLLs
- Put platform DLLs in `src-tauri/bin/`:
  - PDFium, Tesseract (if used), ONNX Runtime (+DirectML), OpenCV (if used).
- Do **not** commit DLLs; they're shipped via Tauri bundler (see `tauri.conf.json > bundle.resources`).
- Keep a short README in `src-tauri/bin/` if needed to document versions.

## 4) Paths & environment
- Ensure MSVC `cl.exe` is globally available:
  - Typical path: `C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC\<ver>\bin\Hostx64\x64`
  - Add to **System PATH** so cargo builds work inside Cursor without Developer Prompt.
- Add CMake and Ninja to PATH:
  - `C:\Program Files\CMake\bin`, `C:\Program Files\Ninja`

## 5) Packs & output
- Use a **user-writable** path for `/packs/` (avoid Program Files).
- Packs are deterministic (sorted CSVs, canonical numbers). Do not hand-edit built packs.

## 6) Approval & first run checklist
- After clone:
  1. `pnpm install`
  2. `pnpm approve-builds` (approve **esbuild** + **@esbuild/win32-x64**)
  3. `pnpm tauri dev` (or run `pnpm run dev` in one terminal and `pnpm tauri dev` in another)
- If a port conflict occurs: pick a new port; update **both** `vite.config.ts` (server.port) **and** `tauri.conf.json` (build.devPath) in one commit.

## 7) Cursor-specific instructions
- Do not rename or relocate `src-tauri/`, `brand_packs/`, `schema/`, `qa/`, or `tests/`.
- When editing config files, **replace the entire file content** to avoid partial merges.
- Keep the project **brand-agnostic**. No vendor names in code or docs.

## 8) Common pitfalls
- **cl.exe not found**: add Hostx64\x64 to PATH, restart terminal.
- **Tauri waits for dev server**: make sure Vite binds to `http://localhost:1420`.
- **DLL missing at runtime**: ensure DLLs live in `src-tauri/bin/` and are listed in `bundle.resources`.
- **Permission denied writing packs**: use a user folder (e.g., `Documents`).

## Tauri config schema guardrails (v1)
- We are on **Tauri v1.x** (see package.json and Cargo.toml). Use the **v1 schema**.
- **Do NOT** place `devPath` or `distDir` under `"tauri"`.  
  They must be under the top-level `"build"` object:
  - `"build.devPath": "http://localhost:1420"`
  - `"build.distDir": "../dist"`
- Keep these in sync with Vite:
  - Vite dev port is **1420** in `vite.config.ts` (`server.port = 1420`, `strictPort = true`).
  - `tauri.conf.json -> build.devPath` must point to the same port.

### Quick self-check (run after editing config)
1) `pnpm approve-builds` (approve **esbuild** + **@esbuild/win32-x64** if prompted).
2) `pnpm install`
3) `pnpm tauri dev`
   - If you see **"Additional properties are not allowed ('devPath', 'distDir')"**: you put these under the wrong key. Move them to `"build"`.
   - If you see **"Waiting for your frontend dev server"**: verify Vite is on port **1420** and matches `build.devPath`.

### Don't change without updating both places
- If you need a different port, update **both**:
  - `vite.config.ts` → `server.port`
  - `src-tauri/tauri.conf.json` → `build.devPath`
- Make the change in a single commit titled:  
  `"Dev port change: Vite + Tauri devPath sync"`

## Cargo TLS (Windows schannel) guardrail
If `pnpm tauri dev` (or any `cargo` build) fails with:
- `CRYPT_E_NO_REVOCATION_CHECK` or
- `The revocation function was unable to check revocation for the certificate`

it is a known Windows schannel issue on some networks. We pin a repo-local fix:
- `.cargo/config.toml` contains:
  - `[http] check-revoke = false`
  - `[net] git-fetch-with-cli = true`

This disables only the revocation **check** for Cargo **inside this repo** and lets builds proceed offline-first.  
If you later build on a different network (without filtering), you may remove the workaround.

### Steps after seeing the error
1. Confirm `.cargo/config.toml` exists as above.
2. Re-run: `pnpm tauri dev` (or `cargo build -p jlcataloguer`).
3. If still blocked:
   - Check corporate proxy settings (`HTTPS_PROXY`, `HTTP_PROXY`) if required in your environment.
   - Try again from a non-filtered network or VPN.

## Cargo downloads on corporate networks
If you see partial downloads or TLS errors (e.g., `Transferred a partial file`, `CRYPT_E_NO_REVOCATION_CHECK`):
- We harden Cargo via `.cargo/config.toml`:
  - `[http] check-revoke=false`, `multiplexing=false`
  - `[net] retry=5`, `git-fetch-with-cli=true`
- Re-run the build: `pnpm tauri dev` (or `cargo build`).

### Make builds fully offline (recommended)
Run vendoring **once on a good connection**:
1. Install `cargo-vendor` (temporarily online): `cargo install cargo-vendor`
2. From repo root: `cargo vendor --versioned-dirs --respect-source-config --locked vendor`
   - This creates a `vendor/` folder with all crates and an `index/` for sparse registry
3. Edit `.cargo/config.toml`:
   - Set `[source.crates-io].replace-with = "vendored-sparse"`
   - Uncomment the `[source.vendored-sparse]` block pointing to `sparse+file://vendor/index`
4. Commit `vendor/` (or keep internal-only if repo size is a concern)

After vendoring, builds do not need the internet.

### Quick checklist
- If downloads fail: try again (we set `retry=5`)
- Still failing: run from a non-filtered network once, **vendor**, then you're offline forever
- Keep `vendor/` updated when you update dependencies (rerun `cargo vendor`)
