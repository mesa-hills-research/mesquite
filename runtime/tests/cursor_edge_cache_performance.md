# Cursor edge-metadata cache (worker-10, stopped by operator)

## Candidate

Base: `ac3a698` (main was checked again after the implementation).
Implementation commits: `f639043`, `3aecd02`.

The cursor used to resolve aliases repeatedly for visibility, current_node,
and ancestor field lookup. Cursor entries now retain alias, visibility, and
resolved field id. An invisible parent's resolved field propagates to its
non-extra children unless their own field map supplies a non-inherited field;
visible parents and extras terminate inheritance, matching the C stack scan.
Root cursors remain visible, have no field, and retain the root Node's alias
except for extras. Entry storage grows from 32 to 40 bytes on 64-bit systems.
The changed-range walker uses the entry structure only as its own traversal
stack; it does not consume these new public-cursor metadata members.

Forward sibling traversal specializes its direction and skips the existing
edge using its cached visibility and original subtree summaries. It no longer
resolves that edge's alias/field map merely to discard the resulting entry.
Reverse traversal retains C's int8 sentinel, structural-index behavior, and
zero descendant indices. Query current_status and parent_node still use their
original distinct metadata rules.

All changes are safe Rust, scoped to runtime/. No dependencies or public API
changes. The C traversal order, ranges, and parser progress calls are unchanged.

## Validation

- Final gate: **11,816 / 11,816** pass, zero failures/skips, incremental=7,
  queries=on (after both implementation commits).
- Runtime suite before the final dedicated test: 212 unit tests and both
  integration tests passed. The six final cursor tests pass, including new
  cache precedence/alias/extra checks.
- `cargo clippy -p ts_port --all-targets -- -D warnings`: clean.

## Benchmark evidence (not acceptance proof)

The initial cache alone measured overall port/C 0.795 vs main 0.813. Walks
improved in every language in that run, generally 4–10%, ~20% for Markdown.

With specialized forward sibling skipping, the single final benchmark measured
**0.769 vs main 0.820** overall (~6.2% lower), but reported **8.3% main noise**.
Typical walks improved ~20–25% (C 1.27 -> 0.98, Rust 1.33 -> 1.04,
JavaScript 1.28 -> 1.00). Markdown walks were 1.48 -> 1.02. HTML regressed
1.10 -> 1.32 in that run; main's TSX had a conspicuous timing outlier
(total 1.16, vs ~0.81 in earlier runs). Parse code was not changed.

The run was stopped by the operator before repeat measurements could establish
whether the HTML regression is real. **Do not claim this clears the performance
gate.** Resume by merging main and repeating the alternating benchmark (median
of >=3), then investigate HTML if the walk regression repeats. Exhaustive kernel
and fresh-repository validation was not run locally and is still required.
