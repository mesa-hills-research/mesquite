# C parsing investigation — worker 8

## Current candidate (against main `298df50`)

This supersedes the earlier report-only submission. There is now an independent
production change in `subtree.rs`; the discarded stack/child-storage experiments
are not included.

A parse-only `perf record` harness over the pinned C files identified reduction
node construction as a major remaining runtime cost (about 9.6% of samples on
the original investigation baseline). Its general summary loop handled unary
reductions with the same accumulation, alias indexing and boundary logic needed
for arbitrary child lists. The leaf constructor also contained its allocating
path, making the very common inline-value path harder to inline into callers.

## Changes

- Specialize summaries for one child, production ID zero (no aliases), and a
  non-error parent. Decode the child representation once, inherit its extent and
  lookahead directly, and accumulate metadata in locals before publishing it.
- Preserve the general routine for aliases, error parents, empty reductions and
  multiple children. Both routes preserve all C metadata. In particular, the
  shortcut checks the parent's **old** row count for column dependencies,
  preserves sticky fragility/parse-state values, and retains an empty child
  reduction's descendant count without adding its dynamic precedence.
- Separate allocating/pooling leaves into an out-of-line helper. The inline leaf
  eligibility conditions, flags and allocation/pool behavior are unchanged.
- Add ordinary inline hints to the small leaf constructor, reduction constructor
  and trailing-extra helper, exposing value construction and ownership transfers
  at their call sites. There is no new unsafe or changed parser/progress logic.

## Measurements

Six pinned runs of this candidate, in order, gave overall port/C:

**0.875, 0.888, 0.870, 0.871, 0.868, 0.871**.

The first three preceded formatting and the additional regression test; the last
three followed them. Production logic did not change between these runs. The
six-run median and final three-run median are both **0.871**, versus main's
**0.895**, about **2.7% lower parse time**, exceeding the reported 1.9% noise.
The first group included Java and Rust timing outliers; they are included above,
not discarded. Every language's six-run median is faster than recorded main.

Final three-run per-language medians (ratios are rounded by the host):

| Language | Main port/C | Candidate port/C |
| --- | ---: | ---: |
| c | 0.88 | 0.86 |
| cpp | 0.93 | 0.91 |
| go | 0.90 | 0.87 |
| java | 0.85 | 0.83 |
| javascript | 0.92 | 0.89 |
| python | 0.88 | 0.86 |
| rust | 0.90 | 0.87 |
| tsx | 0.89 | 0.85 |
| typescript | 0.90 | 0.88 |

## Validation

- Oracle gate: **4,712/4,712** files pass, incremental=7 and queries=on, with no
  failures, crashes, timeouts or skips.
- `cargo test -p ts_port --all-targets`: **191 unit tests** and both integration
  tests pass.
- New comparison test exercises the specialized and general summary functions
  over null/inline/heap children, visible/named/extra/missing flags, errors and
  error repeats, empty/nonempty child reductions, alias/error-parent fallbacks,
  wrapping byte/lookahead values, sticky flags, and repeated summarization with
  different old row counts.
- `cargo check --workspace` and
  `cargo clippy --workspace --all-targets -- -D warnings` pass.
- No host-owned files, unsafe code, persistent caches or C limits were changed.
  The complete kernel/fresh-repository suites remain host merge-time checks.

## Earlier rejected experiments

The earlier direct-reduction optimization was independently merged on main and
was not submitted twice. Compact arena links, inline unary child storage,
additional stack shortcuts, fused version-status reads and a balance traversal
probe did not establish sufficient independent gains and were removed. The
current candidate changes only subtree construction/summarization and tests.
