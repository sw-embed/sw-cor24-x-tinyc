# Known Issues

## reg-rs Cross-Platform Output Differences

**Status:** Open -- blocking cross-platform CI

The demo run scripts use `sed` and `grep` with patterns that behave
differently on macOS (BSD sed) vs Linux (GNU sed). This causes reg-rs
`.out` baselines captured on one platform to fail on the other, even
when the compiler output is identical.

### Affected

All 14 `demos/run-demo*.sh` scripts use `sed` to extract r0 value,
halted status, LED state, and UART output from cor24-run dump output.

### Examples of Differences

- `sed 's/.*(\s*//'` -- `\s` is not portable (GNU extension)
- `grep -P` -- Perl regex not available on macOS default grep
- Trailing whitespace in cor24-run output may vary

### Possible Solutions

1. **Use `awk` instead of `sed`** -- more portable field extraction
2. **Use a dedicated validation tool** -- write a small Rust binary
   that parses cor24-run output and checks assertions
3. **Configure reg-rs with flexible diff** -- if reg-rs supports
   custom diff commands or ignore-whitespace options, use those
4. **Normalize output** -- pipe through a normalizer before capture
5. **Platform-specific baselines** -- .out.linux / .out.macos
   (last resort, doubles maintenance)

### Impact

- Developers on macOS cannot run `reg-rs run` against Linux baselines
- CI would need platform-specific baseline sets
- Does not affect `cargo test` (those are platform-independent)

### Recommendation

Write a small `cc24-validate` tool in Rust that replaces the shell
script validation. It would parse cor24-run output, extract register
values, check assertions, and output a consistent PASS/FAIL format.
This eliminates all sed/grep portability issues.
