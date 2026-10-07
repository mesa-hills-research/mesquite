# CMake command recovery verification (bucket `6c68c51b`)

The seven reported fresh-CMake inputs had an `ERROR` child of `source_file` in
C, but a `normal_command` in the port. The smallest was
`Tests/RunCMake/Syntax/UnterminatedBrace1.cmake` (`set(var "${")`, followed by a
newline). The bucket already passes at this worktree's starting revision,
`8fbaaf9`; no additional runtime or scanner behavior change is needed.

## Root cause and existing correction

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

## Verification

- `run_oracle(inputs = "bucket:6c68c51b")`: **7/7 pass**, with queries enabled.
- `run_oracle(languages = "cmake", inputs = "all")`:
  - **270/270 gate inputs pass**, with seven incremental checks and queries.
  - **9878/9878 fresh inputs pass**, with queries enabled.
- `cargo check --workspace --all-targets`: passes. The existing unused-assignment
  warning in host-owned `grammars/yaml/src/lex.rs:20` is unrelated and unchanged.
- `cargo test -p ts_port_cmake`: **20 tests pass**.
- `cargo clippy -p ts_port_cmake --all-targets -- -D warnings`: passes.

This verification adds no runtime changes, deviations from C, or unsafe code.
