# One-child stack pop specialization

## Motivation and implementation

The host profile attributed 1.75% of full-dumper self time to
`ts_stack_pop_count_in_place` and 1.60% to `stack_node_new`. Inspection with
`perf annotate` showed the pop's two predecessor walks and node-drop bookkeeping,
plus node-construction temporaries and call overhead. Those profile percentages
include tree traversal, so pinned parse timings are the performance criterion.

A committed one-child reduction whose top node has one owner and one non-extra
(or null) link needs neither a separate prefix validation walk nor reversal of a
child list. Transfer the owned node/edge directly and return its zero- or
one-element child Vec. Keep the existing two-pass implementation in a separate
helper for longer prefixes and extras. Add ordinary inline hints to the small
pop entry point, node construction, and push; do not force inlining the general
walk.

The specialization preserves null-link counting, subtree identity/reference
counts, predecessor ownership, free-list order, head metadata, and scratch-array
clearing. All ambiguity, sharing, iterator limits, error costs, and progress
checkpoints remain unchanged. There is no new unsafe code or persistent storage.
Experiments with narrower IDs and retained vacant-node storage were discarded
because they did not improve the benchmark.

## Pinned measurements

After merging main's committed-reduction changes (`5aedb91`), three consecutive
runs gave overall port/C ratios **0.953, 0.951, 0.950**, versus main's **0.974**:
about **2.36%** less parse time at the median.

After also merging the newer lexer main (`4a40c35`), the final three consecutive
runs gave **0.916, 0.920, 0.919**, versus main's **0.939**. The median improvement
is **2.13%**, above the reported 1.9% overall noise. Every language's median was
faster:

| Language | Main port/C | Worktree median port/C |
| --- | ---: | ---: |
| c | 0.93 | 0.91 |
| cpp | 0.99 | 0.96 |
| go | 0.94 | 0.92 |
| java | 0.89 | 0.88 |
| javascript | 0.97 | 0.94 |
| python | 0.93 | 0.90 |
| rust | 0.94 | 0.91 |
| tsx | 0.93 | 0.91 |
| typescript | 0.95 | 0.93 |

Per-language ratios are rounded by the benchmark tool. The final local
`perf record -e cycles:u` run over `profile/bench.list --repeat 3` confirms that
the specialized pop and node constructor no longer appear as standalone hot
functions; their work is inlined into callers. This is not itself a parse-time
measurement.

## Validation

- All **4,712** differential gate inputs pass after the final merge, with
  incremental seed 7 and queries enabled, including tree/progress comparisons.
- `cargo test -p ts_port`: **188** unit tests and both integration tests pass.
- Added direct comparison with pop-then-renumber for null, inline, and heap
  one-child links, with both pending flags. Assertions cover child identity,
  heap refcounts, scanner token, summary, error baseline, predecessor refcount,
  and free-list order. Existing tests cover extras, larger/zero pops, and
  refusal of shared and branching prefixes.
- `cargo clippy --workspace --all-targets -- -D warnings`: clean.
- The host's complete kernel/fresh-repository suite remains a merge-time check;
  the local oracle validation above is the gate set.
