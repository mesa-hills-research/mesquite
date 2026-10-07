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

## Executable command-recovery regression

`cmake_command_recovery.rs` now checks the three smallest malformed variable
references (`UnterminatedBrace1`, `NameWithTabs`, and `NameWithSpaces`) through
the public parser API. It asserts the top-level `ERROR` kind and flags, child
counts, and the recovery content's byte/point ranges through the final newline.
Each input is tested on a fresh parser and after bracket-argument and
bracket-comment parses to exercise scanner reset as well as creation.

Reverification: the bucket remains **7/7**, the CMake gate **270/270**, and fresh
CMake **9878/9878**. The new integration test and its `cargo clippy` target pass
with `-D warnings`.

The follow-up at `a90c17b` adds `CommandError0` and `ParenInVarName0` to the
executable coverage. These cases assert multiline byte/point boundaries,
leading whitespace exclusion from recovery content, preservation of an earlier
valid command, and zero progress-callback calls. They exercise fresh and reused
scanners. All 7 bucket, 270 gate, and 9878 fresh inputs still pass; both integration
tests, all 24 CMake crate tests, workspace checking, and targeted clippy pass
(the same host-owned YAML warning remains unchanged).

## Revalidation at `69f98e0`

The assigned bucket already passes on this merged baseline. Comparing the Rust
scanner with the current C source confirms the zero-valued initialization and
whole-state reset described above are present and correct. Existing integration
coverage already checks the five smallest reported inputs with fresh and reused
scanners, including exact recovery ranges and progress-callback counts; no
additional implementation change or duplicate test is warranted.

Checks rerun on this revision:

- Bucket `6c68c51b`: **7/7 pass**, with query checks.
- All CMake inputs: **270/270 gate** (incremental and query checks enabled) and
  **9878/9878 fresh** pass.
- `cargo check --workspace --all-targets`: passes; the pre-existing warning in
  host-owned `grammars/yaml/src/lex.rs:20` is unchanged.
- `cargo test -p ts_port_cmake`: **24 tests pass**.
- `cargo test -p ts_port --test cmake_command_recovery`: **2 tests pass**.
- `cargo clippy -p ts_port_cmake -p ts_port --all-targets -- -D warnings`: passes.

This follow-up only records verification. It introduces no runtime changes,
C deviations, unsafe code, new warnings, or host-owned generated-file changes.
