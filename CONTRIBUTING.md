# Contributing

Thanks for considering a contribution!

## Getting started

1. Fork the repo and clone your fork.
2. Install the prerequisites from the
   [Getting started](README.md#getting-started) section (Node 20+, pnpm, a
   Rust toolchain per the Tauri prerequisites).
3. Install dependencies and run the app:

   ```bash
   pnpm install
   pnpm tauri dev
   ```

   The first dev build compiles the whole Rust backend — allow 5–10 minutes.
   After that, frontend changes hot-reload instantly.

## Before you open a PR

- `pnpm build` passes (TypeScript + Vite production build).
- `cargo check --manifest-path src-tauri/Cargo.toml` passes.
- Keep changes focused — one topic per pull request.
- Match the existing code style; see [ARCHITECTURE.md](ARCHITECTURE.md) for
  the module map before touching the backend.

## Pull requests

PRs target `main`. The branch is protected: a pull request needs one
approving review (from the code owner) and is merged by squash or rebase to
keep the history linear. For bigger features, please open an issue first so
the approach can be discussed before you invest time in code.

## Reporting bugs

Open an issue with your OS, app version (Settings → Updates shows it) and
steps to reproduce. Don't use issues for security vulnerabilities — see
[SECURITY.md](SECURITY.md).
