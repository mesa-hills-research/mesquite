# Cursor edge-metadata cache and iterator optimization

## Implementation

Base: `ac3a698` (main was rechecked before final benchmarking).
Production commits: `f639043`, `3aecd02`, `a5fc59c`.

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

Child iterators borrow the child slice once instead of repeatedly decoding
the parent's Subtree enum. Profiling the parse-and-walk benchmark after ordinary
inline hints still showed 4.4% self time in child_iterator_next and 4.2% in
iterate_children_at. Forcing those internal C-static-inline helpers into their
callers eliminates large iterator/entry argument and return copies and allows
unused state updates to disappear. This was important: ordinary `#[inline]`
left them out of line in the release binary.

All changes are safe Rust, scoped to runtime/. No dependencies or public API
changes. The C traversal order, ranges, costs, limits, and parser progress calls
are unchanged. The added fields cache immutable results rather than altering
field or alias semantics.

## Validation

- **11,816 / 11,816** gate inputs pass: zero failures/skips, incremental=7,
  queries=on.
- **163,507 / 163,507** kernel and fresh-repository inputs pass: zero
  failures/skips, incremental=off, queries=on (the oracle's exhaustive mode).
- Final runtime suite: **214 unit tests and both integration tests pass**.
  Seven cursor tests include field precedence, aliases, extras, hidden-wrapper
  inheritance, visible/extra inheritance boundaries, descendant jumps, copies,
  and resetting onto an aliased node. Existing reverse-iteration tests preserve
  the eight-bit sentinel and unknown-column behavior.
- `cargo clippy -p ts_port --all-targets -- -D warnings`: clean.
- `cargo clippy --workspace --all-targets`: only the existing generated-YAML
  `unused_assignments` warning at grammars/yaml/src/lex.rs:20, identical to
  the host's main-clippy.json. Host-owned files were not changed.
- `git diff --check main`: clean. All modified files are under runtime/.

## Final benchmark

Three consecutive alternating main/candidate benchmark calls on the final
production code (40 files per language, parse and cursor walk timed separately):

| Run | Main overall port/C | Candidate overall port/C |
| --- | ---: | ---: |
| 1 | 0.811 | 0.720 |
| 2 | 0.813 | 0.720 |
| 3 | 0.815 | 0.723 |
| **Median** | **0.813** | **0.720** |

The median combined workload is **11.4% faster** than main, exceeding the
benchmark's reported 8.3% baseline noise. Every language's combined workload
and cursor walk improved in all three final runs. Parse code was not changed;
its individual timings vary slightly in both directions. The host's own
median-based, per-part acceptance check remains authoritative.

Across the rounded per-language median figures, the walk-only geometric mean
is approximately **0.707 port/C vs main's 1.295**, about **45.4% less walk time**.
Representative median walk ratios:

| Language | Main | Candidate |
| --- | ---: | ---: |
| C | 1.27 | 0.70 |
| C++ | 1.25 | 0.70 |
| HTML | 1.10 | 0.64 |
| JavaScript | 1.28 | 0.70 |
| Markdown | 1.48 | 0.70 |
| Python | 1.29 | 0.71 |
| Rust | 1.34 | 0.73 |
| TypeScript | 1.28 | 0.71 |

The broader project goal of <=0.50 combined port/C is not yet reached.

## Earlier experiments

The operator interrupted the initial cache/sibling implementation before
iterator inlining was added. That version measured overall 0.769 vs 0.820
in a noisy run, including an HTML walk outlier. On resumption, HTML improved
0.89 vs 1.10, and the subsequent final implementation improved it further to
0.64–0.65. The initial regression did not recur. Borrowing child slices with
ordinary inline hints measured 0.758 overall before forced inlining brought
the candidate to approximately 0.720. These earlier results are retained as
context, not substituted for the final three-run comparison.
