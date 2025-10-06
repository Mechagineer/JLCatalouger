# Setup & Onboarding (Windows, Offline)

1) Install toolchains: MSVC (Build Tools), Rust (MSVC), Node LTS + pnpm, CMake, Ninja, Git.
2) Verify:
   - `cl.exe`, `rustc --version`, `cargo --version`, `node -v`, `pnpm -v`, `cmake --version`, `ninja --version`, `git --version`.
3) Repo prep:
   - `pnpm install`
   - `pnpm approve-builds` (approve esbuild + @esbuild/win32-x64)
4) Dev run:
   - `pnpm tauri dev` (Vite at http://localhost:1420, Tauri launches window)
5) (If Cargo downloads are flaky) follow vendoring steps in `guardrails.md`.
6) Create `src-tauri/bin/` and drop runtime DLLs there when ready. They are bundled automatically.
