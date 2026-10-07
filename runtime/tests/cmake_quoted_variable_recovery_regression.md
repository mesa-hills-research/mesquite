# CMake quoted-variable recovery verification (bucket `1b5a6fb7`)

All four reported inputs already pass at the starting revision `eb946ad`
(superseding the previous verification at `d14c3c2`):

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
on invalid snapshot lengths. Its
`recovery_in_quoted_variable_names_skips_only_leading_whitespace` regression
covers the two smallest inputs: recovery skips the first space/tab at byte 14,
then consumes the remaining suffix, including later whitespace, quotes, and the
final newline. It verifies scanner callback order both initially and after an
empty-snapshot reset.

## End-to-end regression coverage

`cmake_quoted_variable_recovery.rs` covers all four fixtures through the public
parser API, independent of the external fixture checkout:

- The two malformed variable names retain the quote, dollar sign, and opening
  brace as direct children of the enclosing `ERROR`, with no nested error.
- The larger deferred-call and compile-features fixtures retain valid arguments
  preceding the malformed quote as direct children of the same `ERROR`.
- Checks cover symbol IDs, named/extra/error/missing flags, byte/point ranges,
  child counts, and named-child navigation. Progress-callback counts match C:
  zero for the small fixtures, one for deferred-call, two for compile-features.
- Every fixture is parsed with whole-input and one-byte chunks, initially and
  after bracket-argument/comment parses to exercise scanner reset.
- Two incremental repair/restore cycles for each malformed variable name reuse
  edited trees and verify the restored flat error children and ranges.

## Latest verification

Rechecked the Rust scanner against the current C source and reran at `eb946ad`:

- `run_oracle(inputs = "bucket:1b5a6fb7")`: **4/4 pass**, queries enabled.
- `run_oracle(inputs = "all", languages = "cmake")`:
  - **270/270 gate inputs pass**, with incremental checks (`incremental=7`) and
    queries enabled.
  - **9878/9878 fresh inputs pass**, queries enabled.
- `cargo check --workspace --all-targets`: passes. The existing unused-assignment
  warning in host-owned `grammars/yaml/src/lex.rs:20` remains unchanged.
- `cargo test -p ts_port_cmake -p ts_port`: **24 CMake scanner unit tests**,
  **212 runtime unit tests**, and **20 integration tests** pass, including all
  **3 quoted-variable recovery tests**; doc tests pass as well.
- `cargo clippy -p ts_port -p ts_port_cmake --all-targets -- -D warnings`: passes.

This assignment records a fresh verification of the merged baseline only: the
existing fix and regressions already resolve the bucket. No runtime changes,
deviations from C, unsafe code, or new clippy warnings were introduced, and no
bucket failures remain.

## Registry-query bucket `4d6cc948`

Reverified at `e5772de` (current assigned baseline, superseding `b34de06`): the assigned
`Tests/RunCMake/find_package/Registry-query.cmake` already passes on the starting
checkout. Its malformed `${CMAKE_ CURRENT_SOURCE_DIR}/${FILE_DIR}` reference
exercised the same incorrect scanner initialization/reset described above:
retaining an inert or previous token prevented recovery content and changed
nested `ERROR` grouping. The current Rust scanner matches the current C source's
zero initialization and reset of both fields, so no further behavior change is
needed.

Existing coverage includes that exact reference in both the scanner callback-order
test and the `SOURCES` cases in `cmake_quoted_variable_recovery.rs`. The parser
regressions check fresh/reused scanners, whole-input and one-byte chunks, two
incremental repair/restore cycles, tree ranges/flags, and progress counts. No
duplicate test was added.

Checks rerun for this assignment:

- `run_oracle(inputs = "bucket:4d6cc948")`: **1/1 pass**, queries enabled.
- `run_oracle(inputs = "all", languages = "cmake")`: **270/270 gate** inputs
  (including incremental and query checks) and **9878/9878 fresh** inputs pass.
- `cargo check --workspace --all-targets`: passes with the pre-existing
  unused-assignment warning in host-owned `grammars/yaml/src/lex.rs:20` unchanged.
- `cargo test -p ts_port_cmake -p ts_port`: **212 runtime unit tests**,
  **24 CMake unit tests**, and all integration and doc tests pass, including
  the three quoted-variable tests.
- `cargo clippy -p ts_port -p ts_port_cmake --all-targets -- -D warnings`: passes.

This follow-up records verification only. No scanner/runtime behavior changes,
C deviations, unsafe code, generated-file edits, or new warnings are introduced.
