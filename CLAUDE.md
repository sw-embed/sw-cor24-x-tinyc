# tc24r - C compiler for COR24 (24-bit RISC) ISA

## Build & Test

- No workspace-level Cargo.toml; build individual components:
  `cargo build --manifest-path components/cli/Cargo.toml --release`
- Run reg-rs regression tests: `reg-rs run -p tc24r`
- Run a single demo: `bash demos/run-demo.sh`

## Key Rules

- **Never use `sed` in test scripts or demo scripts.** Use `awk`, `grep`, and `tr` instead. BSD sed (macOS) and GNU sed (Linux) have incompatible extensions (`\s`, `-E` vs `-r`, etc.) that break cross-platform testing. All demo scripts have been ported to use `awk` for field extraction.
- reg-rs baselines (`.out` files) contain absolute paths. After switching machines, run `reg-rs rebase -p tc24r` to update baselines.

## Project Structure

- `components/` - Rust crates organized by compiler phase (core, frontend, backend, codegen-*, cli, testing)
- `demos/` - 50 end-to-end demo scripts (run-demo*.sh) with C source files
- `work/reg-rs/` - reg-rs regression test definitions (.rgt) and baselines (.out)
- `docs/` - Design docs, testing strategy, known issues
