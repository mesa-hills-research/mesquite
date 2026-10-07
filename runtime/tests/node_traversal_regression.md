# Large C-header oracle timeout (bucket `b2e0e4ac`)

Inputs:

- `sources/fresh/cpython/Modules/_ssl_data_300.h` (281854 bytes)
- `sources/fresh/cpython/Modules/_ssl_data_31.h` (282765 bytes)

The original oracle limit of 20 seconds per file produced timeouts for both
inputs on this worktree. The operator identified the cause as quadratic dumper
navigation checks on a node with roughly 60,000 children, rather than a parser
failure: parsing itself took roughly 0.25 seconds and produced identical trees.
The operator raised the host's per-file limit to 120 seconds and instructed that
no runtime fix be made for this bucket.

After the timeout update, `run_oracle(inputs = "bucket:b2e0e4ac")` reports:

- Checks: `incremental=off queries=on`.
- Both files pass; zero failures, crashes, timeouts, or skips.
- Total reported check time: 41 seconds.

`cargo check --workspace --all-targets` also passes. No runtime code was changed
for this resolution. The verification above covers bucket mode, not a separate
incremental explicit-path run.
