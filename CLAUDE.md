# sw-cor24-x-tinyc - C compiler for COR24 (24-bit RISC) ISA

Forked from sw-vibe-coding/tc24r into sw-embed/sw-cor24-x-tinyc.

## CRITICAL: AgentRail Session Protocol (MUST follow exactly)

This project uses AgentRail. Every session follows this exact sequence:

### 1. START (do this FIRST, before anything else)
```bash
agentrail next
```
Read the output carefully. It tells you your current step, prompt,
skill docs, and past trajectories.

### 2. BEGIN (immediately after reading the next output)
```bash
agentrail begin
```

### 3. WORK (do what the step prompt says)
Do NOT ask the user "want me to proceed?" or "shall I start?". The
step prompt IS your instruction. Execute it.

### 4. COMMIT (after the work is done)
Commit your code changes with git. Use `/mw-cp` for the checkpoint
process (pre-commit checks, docs, detailed commit, push).
**Run `/mw-cp` in every repo that was modified during the step.**

### 5. COMPLETE (LAST thing, after committing)
```bash
agentrail complete --summary "what you accomplished" \
  --reward 1 \
  --actions "tools and approach used" \
  --next-slug "next-step-slug" \
  --next-prompt "what the next step should do" \
  --next-task-type "task-type"
```
If the step failed: `--reward -1 --failure-mode "what went wrong"`
If the saga is finished: add `--done`

### 6. STOP (after complete, DO NOT continue working)
Do NOT make any further code changes after running agentrail complete.
Any changes after complete are untracked and invisible to the next
session. If you see more work to do, it belongs in the NEXT step.

Do NOT skip any of these steps. The next session depends on your
trajectory recording.

### Useful Commands
```bash
agentrail status          # Current saga state
agentrail history         # All completed steps
agentrail plan            # View the plan
agentrail next            # Current step + context
```

## Build & Test

- No workspace-level Cargo.toml; build individual components:
  `cargo build --manifest-path components/cli/Cargo.toml --release`
- Build all: `scripts/build.sh`
- Run reg-rs regression tests: `reg-rs run -p tc24r`
- Run a single demo: `bash demos/run-demo.sh`
- Preprocess only: `tc24r-pp <input.c> [-o output] [-I dir]`

### Test Suites
```bash
scripts/run-subset-tests.sh      # chibicc-subset (5 curated tests)
scripts/run-chibicc-tests.sh     # full chibicc test suite (41 tests)
scripts/run-beej-tests.sh        # beej-c-guide examples (11 tests)
```

## Dependencies

- `../sw-cor24-emulator/isa` — COR24 ISA definitions (path dep from codegen-emit)

## Key Rules

- **Never use `sed` in test scripts or demo scripts.** Use `awk`, `grep`, and `tr` instead. BSD sed (macOS) and GNU sed (Linux) have incompatible extensions (`\s`, `-E` vs `-r`, etc.) that break cross-platform testing.
- reg-rs baselines (`.out` files) contain absolute paths. After switching machines, run `reg-rs rebase -p tc24r` to update baselines.
- Edition 2024. Never suppress warnings.

## Project Structure

- `components/` - Rust crates organized by compiler phase (core, frontend, backend, codegen-*, cli, testing)
- `demos/` - 55 end-to-end demo scripts (run-demo*.sh) with C source files
- `work/reg-rs/` - reg-rs regression test definitions (.rgt) and baselines (.out)
- `docs/` - Design docs, testing strategy, known issues
- `.agentrail/` - Saga plan and session tracking

## Available Task Types

`rust-project-init`, `rust-clippy-fix`, `pre-commit`
