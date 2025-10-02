# Guardrails — Dev & Build Rules (Windows, Offline)

This file captures rules for contributors and for Cursor to follow. It prevents repeat issues and keeps the project deterministic.

## 1) Local dev (Windows only)
- Use **PowerShell** (Windows Terminal).  
- Ensure **x64** toolchain: MSVC (cl.exe), CMake, Ninja on PATH.
- Run `pnpm approve-builds` once after cloning to allow esbuild and related scripts.

## 2) Tauri ⇄ Vite wiring
- **Vite dev port** is **1420**. Do not change without also updating `src-tauri/tauri.conf.json: tauri.devPath`.
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
- If a port conflict occurs: pick a new port; update **both** `vite.config.ts` (server.port) **and** `tauri.conf.json` (tauri.devPath) in one commit.

## 7) Cursor-specific instructions
- Do not rename or relocate `src-tauri/`, `brand_packs/`, `schema/`, `qa/`, or `tests/`.
- When editing config files, **replace the entire file content** to avoid partial merges.
- Keep the project **brand-agnostic**. No vendor names in code or docs.

## 8) Common pitfalls
- **cl.exe not found**: add Hostx64\x64 to PATH, restart terminal.
- **Tauri waits for dev server**: make sure Vite binds to `http://localhost:1420`.
- **DLL missing at runtime**: ensure DLLs live in `src-tauri/bin/` and are listed in `bundle.resources`.
- **Permission denied writing packs**: use a user folder (e.g., `Documents`).
