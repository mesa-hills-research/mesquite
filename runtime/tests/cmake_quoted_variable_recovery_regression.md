# CMake quoted-variable recovery verification (bucket `1b5a6fb7`)

The four reported inputs already pass at the starting revision `7fe8128`:

- `Tests/RunCMake/Syntax/NameWithTabsQuoted.cmake`
- `Tests/RunCMake/Syntax/NameWithSpacesQuoted.cmake`
- `Tests/RunCMake/cmake_language/defer_call_syntax_error.cmake`
- `Tests/RunCMake/CompileFeatures/CMP0128Common.cmake`

The reported signature was a nested `ERROR` where C exposed the opening quote
as a direct child of the enclosing `ERROR`. No further parser or scanner
behavior change is needed after the merged scanner correction.

## Root cause and existing fix

The current C reference scanner initializes its state with `ts_calloc` and
resets both `level` and `token` to zero on empty or incorrectly sized snapshots.
Zero is `BRACKET_ARGUMENT_OPEN`, which allows bracket-argument content during
error recovery even without a preceding bracket opener. The older port used
an inert initial token and retained the previous token on empty reset, changing
recovery tokens and error-node grouping.

The merged Rust scanner uses zero-valued `Default` and restores that whole state
on invalid snapshot lengths. Its existing
`recovery_in_quoted_variable_names_skips_only_leading_whitespace` regression
covers the two smallest inputs: recovery skips the first space/tab at byte 14,
then consumes the remaining suffix, including later whitespace, quotes, and the
final newline. It verifies scanner callback order both initially and after an
empty-snapshot reset.

## Verification

Checks rerun at `7fe8128`:

- `run_oracle(inputs = "bucket:1b5a6fb7")`: **4/4 pass**, queries enabled.
- `run_oracle(inputs = "all", languages = "cmake")`:
  - **270/270 gate inputs pass**, with incremental checks (`incremental=7`) and
    queries enabled.
  - **9878/9878 fresh inputs pass**, queries enabled.
- `cargo check --workspace --all-targets`: passes. The existing unused-assignment
  warning in host-owned `grammars/yaml/src/lex.rs:20` remains unchanged.
- `cargo test -p ts_port_cmake`: **22 tests pass**.
- `cargo clippy -p ts_port_cmake --all-targets -- -D warnings`: passes.

That follow-up recorded verification only, without runtime changes, deviations
from C, unsafe code, or clippy warnings.

## End-to-end regression

A subsequent check at `80c0a0a` still passed all four bucket inputs and all
270 gate / 9878 fresh CMake inputs. `cmake_quoted_variable_recovery.rs` now
also verifies the two smallest inputs through the public parser API. It checks
that the quote, dollar sign, and opening brace are direct children of the single
`ERROR`, alongside the identifier, parenthesis, and recovery content. Assertions
cover symbol IDs, named/extra/error/missing flags, byte/point ranges, child counts,
and the zero progress-callback count. Each input is parsed both with a fresh
scanner and after prior bracket argument/comment parses, exercising scanner reset.

`cargo test -p ts_port --test cmake_quoted_variable_recovery`,
`cargo check --workspace --all-targets`, and
`cargo clippy -p ts_port --test cmake_quoted_variable_recovery -- -D warnings`
pass; only the same pre-existing host-owned YAML warning remains in the workspace
check. No additional runtime correction was necessary.

## Revalidation at `9d32b3d`

The bucket remains resolved after the latest merge; no further behavior change
is needed. Rechecked the Rust scanner against the current C `ts_calloc` and
whole-state deserialization reset, and reran:

- Bucket oracle: **4/4 pass**.
- Full CMake oracle: **270/270 gate** (incremental and query checks) and
  **9878/9878 fresh** (query checks) pass.
- `cargo check --workspace --all-targets`: passes, with only the existing
  host-owned YAML lexer unused-assignment warning noted above.
- `cargo test -p ts_port_cmake`: **24 tests pass**.
- `cargo test -p ts_port --test cmake_quoted_variable_recovery`: passes.
- `cargo clippy -p ts_port_cmake -p ts_port --all-targets -- -D warnings`: passes.

This revalidation changes documentation only; it introduces no C deviations.
