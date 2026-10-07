# CMake recovery verification (bucket `4d6cc948`)

Input: `sources/fresh/cmake/Tests/RunCMake/find_package/Registry-query.cmake`
(8100 bytes). The reported divergence was an `ERROR` child in the C tree versus
an exposed `(` in the port, around byte 3335 in a malformed quoted variable
reference.

The bucket already passed at revision `9b5a105` and was reverified after the
merged fixes through `41bd48a`. The scanner initialization/reset correction is
already merged; no additional runtime or scanner behavior change is needed.

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
The parser-level `cmake_quoted_variable_recovery` test additionally checks flat
ERROR children, exact ranges, and progress callbacks with fresh and reused
scanners.

## Verification

All checks below were rerun at `41bd48a`; the assigned input passed before
any changes. Existing scanner and parser regressions already cover the reduced
Registry-query expression, including single-byte input chunks and incremental
repair/restoration, so no duplicate test or further behavior change was needed.

- `run_oracle(inputs = "bucket:4d6cc948")`: 1/1 passes; incremental off,
  queries on.
- `run_oracle(languages = "cmake", inputs = "all")`:
  - Gate: 270/270 passes, including incremental checks (`incremental=7`) and
    query checks.
  - Fresh repository: 9878/9878 passes; incremental off, queries on.
- `cargo check --workspace --all-targets`: passes. It reports an existing
  unused-assignment warning in host-owned `grammars/yaml/src/lex.rs:20`, which
  is outside this bucket and was not modified.
- `cargo test -p ts_port_cmake`: 24 tests pass.
- `cargo test -p ts_port --test cmake_malformed_closer`: 1 test passes,
  covering recovery tree structure with fresh and reused scanners.
- `cargo test -p ts_port --test cmake_command_recovery`: 2 tests pass,
  covering malformed variable references with fresh and reused scanners.
- `cargo test -p ts_port`: all 212 unit tests and all integration tests pass,
  including all three `cmake_quoted_variable_recovery` tests, which cover fresh/reused
  scanners, single-byte input chunks, and incremental repair/restoration.
- `cargo clippy -p ts_port_cmake -p ts_port --all-targets -- -D warnings`: passes.

This follow-up changes only this verification note; it introduces no deviation
from C and no new unsafe code.


### Revalidation of `4d6cc948` at `a0b7071`

The assigned Registry-query input already passes on this starting revision
(previously verified at `8f652b4`). All checks below were rerun rather than
relying on that earlier verification. Direct comparison with the current C
scanner confirms that the merged zero-valued creation/reset behavior above is
the required correction; no
additional runtime change is warranted. All checks below were rerun at this
revision, including the parser regression with fresh and reused scanner state,
single-byte input chunks, and incremental repair/restoration. The reduced
Registry-query expression is already present in both scanner and parser tests;
no duplicate regression was added.

- Bucket: **1/1 passes**, with query checks.
- All CMake inputs: **270/270 gate** (including incremental/query checks) and
  **9878/9878 fresh** pass.
- `cargo check --workspace --all-targets`: passes; only the pre-existing
  host-owned YAML lexer unused-assignment warning remains.
- `cargo test -p ts_port_cmake -p ts_port`: all unit, integration, and doc tests
  pass, including **24** CMake tests and **3** quoted-variable parser regressions.
  These cover fresh/reused scanners, chunked input, incremental repair/restoration,
  and malformed quotes after valid arguments.
- `cargo clippy -p ts_port_cmake -p ts_port --all-targets -- -D warnings`: passes.

This revalidation only updates this note. No behavior changes, C deviations,
new warnings, unsafe code, or generated-file changes were introduced.

### Registry-query parser regression (`538cd80`)

Bucket `4d6cc948` already passes at this starting revision. The C scanner still
uses zero-initialization and whole-state reset, matching the merged correction;
no additional runtime change is needed. Extended
`cmake_quoted_variable_recovery.rs` with the reduced Registry-query expression
`${CMAKE_ CURRENT_SOURCE_DIR}/${FILE_DIR}`. It checks that the second variable
reference remains within the recovery content, using the same exact tree/range/
flag assertions as the existing whitespace cases. The new case runs with fresh
and reused scanners and through two incremental repair/restoration cycles.

Validation: bucket **1/1**, CMake gate **270/270** (incremental/query checks), and
fresh CMake **9878/9878** (query checks) pass. Both parser regressions and all
24 CMake scanner/grammar tests pass. Workspace all-targets checking passes with
the pre-existing host-owned YAML warning above; strict all-targets clippy for
`ts_port` and `ts_port_cmake` passes. No new runtime behavior, C deviations,
unsafe code, or generated-file changes are introduced.

## ERROR range bucket `e0b0bff8`

Reverified at `3cb8c98`: all five reported inputs already pass on the starting
checkout with the merged scanner initialization/reset correction above. Direct
comparison with the current C scanner confirms that no additional implementation
change is needed: creation uses `ts_calloc`, and invalid-length deserialization
resets both `level` and `token` to zero. Token zero is `BRACKET_ARGUMENT_OPEN`,
not an inert sentinel, so recovery can emit bracket content without an opener.
All checks listed below were rerun, including the existing parser regressions
with chunked input, reused scanners, and incremental EOF edits. This follow-up
only refreshes the verification record; it does not change runtime behavior or
duplicate existing tests. Recovery from
byte 5 of `E_sleep-no-args-stderr.cmake` emits bracket content through byte 72,
including the final newline. The enclosing ERROR therefore reaches EOF, rather
than ending at byte 71 with only the initial identifier as a visible child.
`PropertiesSources-stdout.cmake` likewise emits one content token through EOF
instead of splitting the ERROR. No further implementation change is needed.

The scanner test `recovery_content_keeps_multiline_error_ranges_through_eof`
checks both recovery starting after an identifier and recovery starting at byte
zero, before and after an empty-snapshot reset. The differential oracle checks
the complete resulting trees and progress callbacks for all five bucket inputs.

Checks rerun for this bucket at `3cb8c98` (before this documentation-only update):

- `run_oracle(inputs = "bucket:e0b0bff8")`: 5/5 pass.
- `run_oracle(languages = "cmake", inputs = "all")`: 270/270 gate inputs
  (including incremental and query checks), and 9878/9878 fresh inputs pass.
- `cargo check --workspace --all-targets`: passes with only the pre-existing
  host-owned generated YAML lexer warning noted above.
- `cargo test -p ts_port_cmake`: 24 tests pass.
- `cargo test -p ts_port --test cmake_error_ranges`: both parser regressions pass.
- `cargo test -p ts_port`: all unit, integration, and doc tests pass.
- `cargo clippy -p ts_port_cmake -p ts_port --all-targets -- -D warnings`: passes.

This verification adds no C deviations, unsafe code, or generated-file changes.

### Parser-level range regression coverage

At starting revision `7fcdd89`, this bucket still passes with no further
implementation changes. `cmake_error_ranges.rs` now parses both smallest bucket
inputs and checks the single ERROR, its content/identifier children, exact byte
and point ranges through the final newline, and zero progress callbacks. Each
case also runs after bracket arguments and bracket comments on the same parser
to exercise scanner reset through the public API, not just scanner unit tests.

Revalidation at `adc3fd3`: bucket **5/5**, CMake gate **270/270**, and fresh
**9878/9878** pass. `cargo check --workspace --all-targets`,
`cargo test -p ts_port`, and `cargo test -p ts_port_cmake` pass (24
scanner/grammar tests). Both `cmake_error_ranges` parser tests pass as part of
the runtime suite, with whole-source, seven-byte, and byte-at-a-time input
callbacks. Strict clippy for both packages and all their targets passes.
Workspace check still reports only the existing host-owned generated YAML
lexer warning described above. Direct comparison with the current C scanner
confirms the zero-initialized creation/reset semantics; the bucket and its
regression tests were already resolved on this checkout. This revalidation
changes only the verification record, not scanner or runtime behavior.

### Incremental EOF-range coverage (`45f70f2`)

The assigned five inputs already pass at this starting revision. The current C
scanner still zero-initializes creation and resets both fields on invalid-length
snapshots, matching the merged correction; no further behavior change is needed.
Extended `cmake_error_ranges.rs` to delete and restore the final newline twice
using edited old trees. Both smallest bucket inputs retain the exact ERROR and
content children, updated byte/point ranges, and zero progress callbacks. These
checks run after fresh creation and bracket-argument/comment parses, with whole,
seven-byte, and single-byte input chunks.

Validation: bucket **5/5**, CMake gate **270/270** (incremental/query checks), and
fresh CMake **9878/9878** (query checks) pass. All runtime and CMake scanner tests
pass, as do strict all-targets clippy for both packages and workspace all-targets
checking. The latter still reports the pre-existing host-owned generated YAML
lexer warning noted above. This change only strengthens regression coverage;
there are no runtime changes, C deviations, new unsafe code, or generated-file
changes.

### ERROR-range revalidation at `50d1277`

Rechecked the previously resolved bucket at this starting revision; all checks
below were rerun, rather than relying on the earlier verification at `1c3d310`.
Bucket `e0b0bff8` already passes on this starting checkout. Comparing the current
C scanner with Rust confirms the merged zero-valued creation/reset fix remains
correct; no new implementation change or duplicate regression is needed.
Existing `cmake_error_ranges` tests cover both smallest inputs, fresh/reused
scanners, whole/seven-byte/single-byte chunks, incremental EOF deletion and
restoration, exact tree ranges/flags, boundary navigation, and zero progress calls.

Checks rerun at this revision:

- Assigned bucket: **5/5 pass** (query checks enabled).
- All CMake inputs: **270/270 gate** (incremental/query checks) and
  **9878/9878 fresh** (query checks) pass.
- `cargo check --workspace --all-targets`: passes; the pre-existing warning in
  host-owned `grammars/yaml/src/lex.rs:20` remains unchanged.
- `cargo test -p ts_port_cmake -p ts_port`: passes, including 212 runtime unit
  tests, all integration/doc tests, both ERROR-range tests, and 24 CMake tests.
- `cargo clippy -p ts_port_cmake -p ts_port --all-targets -- -D warnings`: passes.
- Prior `cargo clippy --workspace --all-targets -- -D warnings` validation was
  blocked by that same pre-existing unused-assignment warning in host-owned
  `grammars/yaml/src/lex.rs:20`. The generated lexer was not modified.

This follow-up only records verification: no behavior changes, C deviations,
new unsafe code, warnings, or generated-file modifications are introduced.

## Empty recovery-content bucket `f5e2762e`

Reverified at `db032fc` (previously at `329753e`):
all 16 reported inputs already pass on the starting checkout. The merged scanner
initialization/reset correction above allows `bracket_argument_content` at EOF
without an opener, including after `a` and after skipping the newline in `if(\n`.
This zero-width token is a real scanner token, not an inserted missing node.
No additional scanner or runtime behavior change is necessary.

The existing `cmake_empty_recovery_content` integration tests assert the complete
small recovery trees, content ranges and flags, and zero progress-callback calls,
both with fresh scanners and after parsing bracket arguments/comments. Scanner
unit tests additionally check the EOF callback order and snapshot reset behavior.
The existing parser-level whitespace cases also cover CRLF, lone CR, spaces/tabs,
and multiple newlines, asserting exact EOF byte and point ranges after skipping
that whitespace with both fresh and reused scanners. The three fresh-parse tests
run with whole-source and byte-at-a-time input callbacks, covering whitespace and
CRLF split across chunks without changing the zero-width token's EOF position.
The block/while cases additionally verify that a complete header remains a sibling
of the empty recovery content when its terminating command is missing. Tree
cursor assertions confirm that the empty token remains traversable at EOF. A fourth
integration test repairs each incomplete command/block by inserting at EOF, then
undoes the edit twice using the previous tree. It verifies that recovery recreates
the empty external token with the original kind, flags, and byte/point ranges.

The current C source was compared directly with the Rust scanner: creation uses
`ts_calloc`, and empty/wrong-length deserialization clears both state fields.
The matching implementation and all regression cases were already present at
this starting revision, so no duplicate test or behavior change was warranted.

Checks rerun for this bucket at `db032fc`:

- `run_oracle(inputs = "bucket:f5e2762e")`: 16/16 pass.
- `run_oracle(languages = "cmake", inputs = "all")`: 270/270 gate inputs
  (including incremental and query checks), and 9878/9878 fresh inputs pass.
- `cargo check --workspace --all-targets`: passes with the pre-existing warning
  in host-owned `grammars/yaml/src/lex.rs:20` noted above.
- `cargo test -p ts_port_cmake -p ts_port`: all 24 CMake tests, 212 runtime
  unit tests, and all integration/doc tests pass, including the four
  `cmake_empty_recovery_content` tests.
- `cargo clippy -p ts_port_cmake -p ts_port --all-targets -- -D warnings`: passes.
- `cargo clippy --workspace --all-targets -- -D warnings`: rerun and blocked
  by the same pre-existing unused-assignment warning in host-owned
  `grammars/yaml/src/lex.rs:20`. That generated file was left unchanged.

This follow-up only updates the verification record: the implementation and
regression coverage were already present at the starting revision. No further
scanner/runtime fix is needed, and no C deviations, unsafe code, or generated-file
changes are introduced.

## Unterminated-call bucket `7f8795fb`

Reverified at `ef7c321` (previously at `610f58b`): all three
reported inputs already pass on the starting checkout with the merged scanner
initialization/reset correction above. The current C scanner uses
`ts_calloc` and clears both fields on empty or invalid-length snapshots, matching
the Rust implementation. This allows recovery content without an opener:
zero-width content at EOF in `UnterminatedCall1.cmake` and
`UnterminatedCall2.cmake`, and content through the embedded NUL and remaining
input in `NullTerminatedArgument.cmake`. An inert initial token instead produced
`source_file` roots with missing delimiters rather than the reference `ERROR`.

Existing `cmake_unterminated_call_recovery` integration tests cover all three
inputs, asserting root/content kinds, flags, byte/point ranges, and exact progress
callback counts: one for each unterminated-call input and zero for the embedded-NUL
input. They exercise fresh and reused scanners with both whole-source and one-byte
input chunks. Each test also repairs the malformed source and undoes the edit
twice with an edited old tree, checking restoration of the ERROR and recovery
content (including its zero-width EOF range). The earlier verification note
incorrectly described all three callback counts as zero; the existing tests
already assert the correct counts. No additional behavior change or duplicate
test is needed.

Validation rerun for this bucket at `ef7c321`:

- `run_oracle(inputs = "bucket:7f8795fb")`: 3/3 pass.
- `run_oracle(languages = "cmake", inputs = "all")`: 270/270 gate inputs
  (including incremental and query checks) and 9878/9878 fresh inputs pass.
- `cargo check --workspace --all-targets`: passes, with only the existing warning
  in host-owned `grammars/yaml/src/lex.rs:20` noted above.
- `cargo test -p ts_port_cmake -p ts_port`: all 24 CMake tests, 212 runtime
  unit tests, and all integration/doc tests pass. This includes all three
  `cmake_unterminated_call_recovery` tests, asserting each direct ERROR child's
  kind, symbol id, range, flags, child count, and exact progress-callback counts.
- `cargo clippy -p ts_port -p ts_port_cmake --all-targets -- -D warnings`: passes.
- Prior workspace-wide clippy validation was blocked by the
  pre-existing unused-assignment warning in host-owned `grammars/yaml/src/lex.rs:20`.
  That generated file is outside this bucket and was left unchanged.

The bucket and its regression coverage were already resolved at this revision,
so this follow-up only records verification; no C deviations, unsafe code, or
host-owned generated-file changes were introduced.
