# CMake recovery verification (bucket `4d6cc948`)

Input: `sources/fresh/cmake/Tests/RunCMake/find_package/Registry-query.cmake`
(8100 bytes). The reported divergence was an `ERROR` child in the C tree versus
an exposed `(` in the port, around byte 3335 in a malformed quoted variable
reference.

The bucket already passed at revision `9b5a105` and was reverified after the
merged fixes at `c99a0a2`. The scanner initialization/reset correction is already
merged; no additional runtime or scanner behavior change is needed.

## Root cause and existing correction

The current reference `sources/grammars/cmake/src/scanner.c` creates its payload
with `ts_calloc` and resets both `level` and `token` to zero when deserializing an
empty or wrong-length snapshot. Token zero is `BRACKET_ARGUMENT_OPEN`. During
error recovery, when all external symbols are enabled, that state deliberately
permits bracket-argument content without a preceding opener.

The earlier translation used an inert initial token and preserved the previous
token on empty-state reset. That changed recovery tokens and consequently error
node grouping. The current Rust scanner uses zero-valued `Default` and resets
the whole state on invalid snapshot lengths, matching the C reference. Existing
scanner unit tests cover fresh/reset recovery, delimiter-level reset, empty
content at EOF, invalid snapshot lengths, and native-endian state round trips.
They also cover leading whitespace inside malformed quoted variable names,
the same recovery scenario as `${CMAKE_ CURRENT_SOURCE_DIR}` in this bucket.

## Verification

All checks below were rerun at `c99a0a2`.

- `run_oracle(inputs = "bucket:4d6cc948")`: 1/1 passes; incremental off,
  queries on.
- `run_oracle(languages = "cmake", inputs = "all")`:
  - Gate: 270/270 passes, including incremental checks (`incremental=7`) and
    query checks.
  - Fresh repository: 9878/9878 passes; incremental off, queries on.
- `cargo check --workspace --all-targets`: passes. It reports an existing
  unused-assignment warning in host-owned `grammars/yaml/src/lex.rs:20`, which
  is outside this bucket and was not modified.
- `cargo test -p ts_port_cmake`: 22 tests pass.
- `cargo test -p ts_port --test cmake_malformed_closer`: 1 test passes,
  covering recovery tree structure with fresh and reused scanners.
- `cargo clippy -p ts_port_cmake --all-targets -- -D warnings`: passes.

This follow-up changes only this verification note; it introduces no deviation
from C and no new unsafe code.
