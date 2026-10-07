# CMake truncated BOM recovery verification (bucket `381f9bd4`)

The reported inputs are `Tests/RunCMake/Syntax/Broken-BOM-UTF-32-BE.cmake`
(`00 00 FE`) and `Broken-BOM-UTF-32-LE.cmake` (`FF FE 00`) in the fresh
CMake repository. The expected tree is `source_file > ERROR >
bracket_argument_content`, with every node spanning bytes `0..3` and points
`0:0..0:3`. The outer `ERROR` is extra; its content child is neither extra nor
an error. Both parses make zero progress-callback calls.

## Root cause and existing correction

The C scanner uses `ts_calloc` to create zeroed state and resets both `level`
and `token` to zero on empty or wrong-length deserialization. Zero denotes
`BRACKET_ARGUMENT_OPEN`, so recovery, which enables all external symbols, can
accept bracket content without an opening delimiter. The former Rust inert
initial token and preserved token on reset prevented this recovery path.

The scanner in the starting worktree already matches this C behavior, and both
bucket inputs pass without further changes. Its content loop tests `lexer.eof()`
rather than zero lookahead: embedded NUL bytes and invalid UTF-8 lookahead (`-1`)
are consumed as content, not mistaken for EOF.

Existing regression coverage is sufficient and was rerun rather than duplicated:

- `grammars/cmake/src/scanner.rs`:
  `broken_utf32_boms_are_bracket_content_during_recovery` checks scanner recovery.
- `runtime/tests/cmake_broken_bom.rs`:
  `truncated_utf32_boms_recover_as_one_bracket_content_token` checks both exact
  inputs, chunk sizes 1 through 3, fresh parsers and reuse after bracket arguments
  and comments, node kinds/ids, ranges, flags, child counts, and progress calls.

## Verification on baseline `699a8fb`

Reassigned bucket `381f9bd4` already passes on merged baseline `699a8fb`.
Rechecked the current C source and reran the checks below on this baseline;
all results remain unchanged from the previous verification (`9142da1`).
The existing tests cover both exact inputs, so no additional behavior change
or duplicate regression test was needed.

- `run_oracle(inputs = "bucket:381f9bd4")`: 2/2 pass, query checks enabled.
- `run_oracle(inputs = "all", languages = "cmake")`: gate 270/270 pass
  (incremental=7, queries enabled), fresh repository 9,878/9,878 pass.
- `cargo check --workspace --all-targets`: passes. The existing unused-assignment
  warning in host-owned `grammars/yaml/src/lex.rs:20` remains unchanged.
- `cargo test -p ts_port --test cmake_broken_bom`: passes.
- `cargo test -p ts_port_cmake`: all 24 tests pass.
- `cargo clippy -p ts_port -p ts_port_cmake --all-targets -- -D warnings`: passes.

This follow-up records verification of the already committed correction; no
additional runtime changes, deviations from C, unsafe code, or generated-file
changes are needed.
