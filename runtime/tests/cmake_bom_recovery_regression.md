# CMake BOM recovery verification (bucket `548436bf`)

The four reported inputs are `Tests/RunCMake/Syntax/BOM-UTF-16-BE.cmake`,
`BOM-UTF-16-LE.cmake`, `BOM-UTF-32-BE.cmake`, and `BOM-UTF-32-LE.cmake`
in the fresh CMake repository. The reported divergence was a root `ERROR`
instead of `source_file` containing an extra `ERROR` with a single
`bracket_argument_content` child.

## Root cause and merged correction

The reference CMake scanner creates zeroed state with `ts_calloc` and resets
both `level` and `token` to zero on empty or wrong-length deserialization.
Token zero is `BRACKET_ARGUMENT_OPEN`. When error recovery enables all external
tokens, this state accepts bracket content even without an opening delimiter.
The former Rust inert initial token and preserved token on reset prevented that
recovery path.

The merged Rust scanner already matches C's initialization/reset behavior.
Bracket content tests `lexer.eof()`, not whether lookahead is zero, so the
embedded NULs in these UTF-16/32 files are consumed along with invalid UTF-8
bytes. The existing unit test
`recovery_content_consumes_non_utf8_boms_and_embedded_nuls` covers all four
encoding layouts, both on creation and after empty-state deserialization.
The parser-level regression in `cmake_bom_recovery.rs` also checks all four
complete byte inputs against the reference tree shape, flags, byte/point ranges,
and progress-callback count. Each is parsed with a fresh scanner and after
parsing valid bracket arguments and comments to exercise scanner reset.
No additional scanner or runtime behavior change is required for this bucket.

## Verification

All checks were rerun for this task with the correction already merged:

- `run_oracle(inputs = "bucket:548436bf")`: all 4 inputs pass, queries enabled.
- `run_oracle(inputs = "all", languages = "cmake")`:
  - Gate: 270/270 pass, including incremental (`incremental=7`) and query checks.
  - Fresh repository: 9,878/9,878 pass, queries enabled.
- `cargo check --workspace --all-targets`: passes; the existing unused-assignment
  warning in host-owned `grammars/yaml/src/lex.rs:20` is unrelated and unchanged.
- `cargo test -p ts_port_cmake`: all 24 tests pass.
- `cargo test -p ts_port --test cmake_bom_recovery`: passes (all four encodings
  with fresh and reused parsers).
- `cargo clippy -p ts_port_cmake --all-targets -- -D warnings`: passes.
- `cargo clippy -p ts_port --test cmake_bom_recovery -- -D warnings`: passes.

This follow-up adds parser-level regression coverage: no deviations from C, no
unsafe code, and no host-owned generated files changed.

## Reverification after merges `6a25441` and `2d1d5ac`

On reassignment of bucket `548436bf`, its four inputs already passed before any
changes. Rechecked the current read-only C scanner against Rust: zero-initialized
creation and resetting both fields on invalid-length deserialization still match.
Reran every verification command above on both merged baselines: bucket 4/4,
CMake gate 270/270, fresh CMake 9,878/9,878, all 24 scanner tests and the parser BOM
regression pass. Workspace all-target checking and both targeted strict clippy
commands also pass, with only the unchanged generated YAML warning during the
workspace check. No further behavior change or duplicate regression is needed;
this reassignment records verification of the already committed correction.
