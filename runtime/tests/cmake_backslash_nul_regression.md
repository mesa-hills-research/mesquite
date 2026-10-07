# CMake backslash/NUL recovery (bucket `9a8ee9c6`)

Reverified at starting revision `0dbe312` (after the latest main merge).
The reported 113-byte input,
`Tests/RunCMake/Syntax/NullAfterBackslash.cmake`, already passes with the merged
CMake scanner correction; no additional runtime or scanner change is needed.

## Root cause and existing fix

The current reference scanner creates its state with `ts_calloc` and resets
both `level` and `token` to zero on empty or wrong-length deserialization.
Zero is `BRACKET_ARGUMENT_OPEN`, which allows recovery to emit bracket content
without a preceding opener. The previous inert initial token and preserved
reset token prevented this recovery path. The Rust scanner now uses a zeroed
`Default` and resets both fields, matching C.

Bracket-content scanning uses `eof()`, not a zero lookahead, to stop. Therefore
the embedded NUL after the backslash remains part of the content. The corrected
parse has one ERROR with three children: identifier `0..1`, `(` at `1..2`, and
`bracket_argument_content` at `54..113`, ending at point `(2, 0)`. It invokes the
progress callback once.

## Regression coverage

- `runtime/tests/cmake_backslash_nul.rs` checks this exact source, tree shape,
  node flags, byte/point ranges, and progress-callback count with both whole-file
  and one-byte input chunks. It also checks symbol IDs and chunk
  boundaries immediately before/after the backslash, NUL, and newline (chunk
  sizes 54, 55, 56, and 57). It reuses the parser after bracket arguments and
  bracket comments to exercise scanner reset.
- Scanner test `backslash_nul_recovery_consumes_and_marks_every_byte` checks
  ordinary lexing rejection followed by recovery acceptance, including each
  advance/mark callback across the embedded NUL, before and after reset.

## Verification

All checks below were rerun at this revision. The bucket passed before any
changes, and both the existing scanner tests and parser regression pass.

- Bucket oracle: **1/1 passes** (queries enabled).
- All CMake oracle inputs: **270/270 gate** (incremental and query checks),
  **9878/9878 fresh** (query checks).
- `cargo check --workspace --all-targets`: passes. The existing unused-assignment
  warning in host-owned `grammars/yaml/src/lex.rs:20` remains unchanged.
- `cargo test -p ts_port_cmake`: **24 tests pass**.
- `cargo test -p ts_port --test cmake_backslash_nul`: **1 test passes**.
- `cargo clippy -p ts_port_cmake -p ts_port --all-targets -- -D warnings`: passes.

The assigned `bucket:9a8ee9c6` passed on the first oracle run at this starting
revision, and the full CMake rerun reported no remaining divergence to fix. This
follow-up only refreshes the verification record: no production behavior changes,
C deviations, new unsafe code, or changes to host-owned generated files. The
existing regression already covers this bucket and scanner reset across repeated
parses, so no duplicate test was added.
