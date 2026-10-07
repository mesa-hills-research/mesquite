# CMake recovery verification (bucket `4d6cc948`)

Input: `sources/fresh/cmake/Tests/RunCMake/find_package/Registry-query.cmake`
(8100 bytes). The reported divergence was an `ERROR` child in the C tree versus
an exposed `(` in the port, around byte 3335 in a malformed quoted variable
reference.

The bucket already passed at revision `9b5a105` and was reverified after the
merged fixes through `d490ab6`. The scanner initialization/reset correction is
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

All checks below were rerun at `d490ab6`.

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
- `cargo test -p ts_port --test cmake_command_recovery`: 1 test passes,
  covering malformed variable references with fresh and reused scanners.
- `cargo test -p ts_port`: all 212 unit tests and all integration tests pass,
  including `cmake_quoted_variable_recovery`.
- `cargo clippy -p ts_port_cmake -p ts_port --all-targets -- -D warnings`: passes.

This follow-up changes only this verification note; it introduces no deviation
from C and no new unsafe code.

## ERROR range bucket `e0b0bff8`

Reverified at `c373f7e`: all five reported inputs already pass with the merged
scanner initialization/reset correction above. In particular, recovery from
byte 5 of `E_sleep-no-args-stderr.cmake` emits bracket content through byte 72,
including the final newline. The enclosing ERROR therefore reaches EOF, rather
than ending at byte 71 with only the initial identifier as a visible child.
`PropertiesSources-stdout.cmake` likewise emits one content token through EOF
instead of splitting the ERROR. No further implementation change is needed.

The scanner test `recovery_content_keeps_multiline_error_ranges_through_eof`
checks both recovery starting after an identifier and recovery starting at byte
zero, before and after an empty-snapshot reset. The differential oracle checks
the complete resulting trees and progress callbacks for all five bucket inputs.

Checks rerun for this bucket:

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

Revalidation at `c373f7e`: bucket **5/5**, CMake gate **270/270**, and fresh
**9878/9878** pass. `cargo check --workspace --all-targets`,
`cargo test -p ts_port`, and `cargo test -p ts_port_cmake` pass (24
scanner/grammar tests). The two `cmake_error_ranges` parser tests also pass
when run separately. Strict clippy for both packages and all their targets
passes. Workspace check still reports only the existing host-owned generated
YAML lexer warning described above. The bucket was already resolved on this
checkout; this revalidation changes no scanner or runtime behavior.

## Empty recovery-content bucket `f5e2762e`

Reverified at `d835a69`: all 16 reported inputs already pass. The merged scanner
initialization/reset correction above allows `bracket_argument_content` at EOF
without an opener, including after `a` and after skipping the newline in `if(\n`.
This zero-width token is a real scanner token, not an inserted missing node.
No additional scanner or runtime behavior change is necessary.

The existing `cmake_empty_recovery_content` integration tests assert the complete
small recovery trees, content ranges and flags, and zero progress-callback calls,
both with fresh scanners and after parsing bracket arguments/comments. Scanner
unit tests additionally check the EOF callback order and snapshot reset behavior.
The parser-level whitespace cases now also cover CRLF, lone CR, spaces/tabs,
and multiple newlines, asserting exact EOF byte and point ranges after skipping
that whitespace with both fresh and reused scanners.

Checks rerun for this bucket:

- `run_oracle(inputs = "bucket:f5e2762e")`: 16/16 pass.
- `run_oracle(languages = "cmake", inputs = "all")`: 270/270 gate inputs
  (including incremental and query checks), and 9878/9878 fresh inputs pass.
- `cargo check --workspace --all-targets`: passes with the pre-existing warning
  in host-owned `grammars/yaml/src/lex.rs:20` noted above.
- `cargo test -p ts_port_cmake`: 24 tests pass.
- `cargo test -p ts_port --test cmake_empty_recovery_content`: 2 tests pass.
- `cargo clippy -p ts_port_cmake -p ts_port --all-targets -- -D warnings`: passes.

This follow-up expands regression coverage without changing scanner/runtime
behavior; it adds no C deviations, unsafe code, or generated-file changes.
