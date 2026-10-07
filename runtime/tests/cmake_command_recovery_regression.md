# CMake command recovery verification (bucket `6c68c51b`)

## Status

The assigned bucket is already fixed on the merged baseline `1a3242b`. Fresh
verification passes all seven reported inputs; no additional runtime/scanner
change or duplicate regression test is needed. This record consolidates the
previous revalidations and records checks rerun on this baseline.

## Root cause and existing correction

The seven reported fresh-CMake inputs had an `ERROR` child of `source_file` in
C, but a `normal_command` in the port. The smallest was
`Tests/RunCMake/Syntax/UnterminatedBrace1.cmake` (`set(var "${")`, followed by a
newline).

The current reference `sources/grammars/cmake/src/scanner.c` initializes scanner
state with `ts_calloc` and resets both `level` and `token` to zero when
deserializing an empty or incorrectly sized snapshot. Zero is
`BRACKET_ARGUMENT_OPEN`, so recovery, which enables all external symbols, can
emit bracket-argument content even without a preceding opener. The former
inert initial token and token-preserving reset prevented this and changed the
resulting recovery trees.

The merged Rust implementation uses zero-valued `Default` and restores that
whole state on invalid snapshot lengths, matching C. Existing scanner tests
cover fresh/reset recovery, zero-width content at EOF, trailing newlines,
delimiter levels, callback order, and native-endian snapshot round trips.

## Executable regression coverage

`cmake_command_recovery.rs` covers all five cases listed in the assignment:
`UnterminatedBrace1`, `NameWithTabs`, `NameWithSpaces`, `CommandError0`, and
`ParenInVarName0`. Its two integration tests assert top-level ERROR grouping,
flags, child counts, exact byte/point ranges, leading whitespace exclusion from
recovery content, preservation of an earlier valid command, and zero
progress-callback calls. Each input runs with fresh/reused scanners and
whole-source/one-byte input chunks.

## Verification at `1a3242b`

- `run_oracle(inputs = "bucket:6c68c51b")`: **7/7 pass**, with query checks.
- `run_oracle(languages = "cmake", inputs = "all")`:
  - **270/270 gate inputs pass**, with seven incremental checks and queries.
  - **9878/9878 fresh inputs pass**, with queries enabled.
- `cargo check --workspace --all-targets`: passes. The existing unused-assignment
  warning in host-owned `grammars/yaml/src/lex.rs:20` is unrelated and unchanged.
- `cargo test -p ts_port -p ts_port_cmake`: **212 runtime unit tests**, all
  integration/doc tests (including both command-recovery regressions), and
  **24 CMake tests** pass.
- `cargo clippy -p ts_port -p ts_port_cmake --all-targets -- -D warnings`: passes.

Nothing remains to fix in the assigned bucket. This verification introduces no
runtime changes, deviations from C, unsafe code, or generated-file changes.
