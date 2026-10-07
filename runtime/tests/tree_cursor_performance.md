# Tree-cursor performance investigation

## The profile and the benchmark cover different work

The `perf-tree_cursor` bucket attributed 10.1% of the full oracle dumper's
self time to `tree_cursor`. This is tree traversal, not parsing:

- `parser.rs` does not use `TreeCursor` or `ts_tree_cursor_*`.
- The oracle's `dump.rs::parse` starts its timer immediately before
  `parser.parse_with_options` and stops it immediately afterward. The reported
  `parse_ns` does not include the subsequent `walk` or query checks.
- A local `perf record` with call stacks confirmed, for example,
  `ts_tree_cursor_is_entry_visible <- TreeCursor::depth <- node_line <- walk`.

Consequently, changing only the cursor cannot directly improve the pinned
parse-only benchmark. In particular, the 10.1% profile share must not be used
as an estimate of potential parsing speedup. Profile a parse-only driver when
selecting parsing optimization targets; benchmark walking and queries separately
when evaluating cursor optimizations.

## Safe experiment (not retained)

Against runtime commit `1bca858`, an experiment:

- borrowed the parent's children slice once in `CursorChildIterator`, avoiding
  repeated parent enum decoding and child-storage lookup;
- made sibling traversal generic over its advance function instead of using a
  function pointer;
- marked the small cursor and alias/field-map helpers `#[inline]`.

It preserved the reverse iterator's eight-bit sentinel and structural-index
rules. All five cursor unit tests and all **4,712 oracle gate inputs** passed,
with `incremental=7` and queries enabled. No unsafe code was added.

An alternating main/candidate comparison of the full dumper's `hash` command
on the 360 pinned benchmark files, pinned locally to CPU 7, gave:

| Runtime | Wall times (seconds) | Median |
| --- | --- | --- |
| Main | 4.0473, 4.0344, 4.0496 | 4.0473 |
| Candidate | 3.9754, 4.0139, 3.9766 | 3.9766 |

This is about **1.75% less full-dumper wall time**, not a parse-time result and
not a dedicated-CPU acceptance measurement. The host's parse-only benchmark
reported **1.363 port/C**, versus the then-current main's **1.352**, with 1.9%
reported noise: no parsing improvement. A second candidate run also reported
1.363; main had meanwhile advanced, so its newer baseline is not comparable
to the original experiment.

The experimental runtime changes were reverted because they did not meet the
required parse-time acceptance gate. That initial investigation retained only
this report, not a parsing optimization.

## Follow-up after the documentation-only result was rejected

A parser-loop profile over the same 360-file input list (no tree walking) showed
approximately 7% self time in reductions and 5% in stack popping. Experiments
transferred exclusively owned stack paths rather than cloning them, separated
committed reductions from the general slice worklist, and exposed the small
reduction/push paths to call-site inlining. These produced measurable gains at
several intermediate main revisions:

| Main revision | Main port/C | Candidate median port/C | Improvement |
| --- | ---: | ---: | ---: |
| `af419b8` | 1.121 | 1.026 | 8.5% |
| `9edb29b` | 1.013 | 0.985 | 2.8% |
| `5aedb91` | 0.974 | 0.950 | 2.5% |

These are historical results, **not a claim of improvement over current main**.
Main acquired overlapping implementations while the experiments were being
tested. Each was merged and remeasured rather than claiming the old gains as a
new contribution. Following main's lexer and one-child-pop optimizations, the
remaining inline hints no longer beat the noise threshold.

An additional safe layout experiment encoded a link's pending flag alongside
its integer arena index. The Vec allocation bound guarantees room for that bit,
so this needed no unsafe or narrower node IDs. It reduced StackLink from 32 to
24 bytes and StackNode from 72 to 64 bytes on the measured target. The combined
runtime still did not beat updated main (both measured about 0.917 port/C), so
this experiment was also withdrawn.

## Final state

The branch retains **only regression tests and this investigation report**.
Production parser and stack code match main `de86b49`; there are no new runtime
algorithms, inlining policies, layout changes, unsafe code, or grammar changes.
Main itself already measures below the requested overall 1.00 port/C target.
There is no distinct current-branch speedup to submit through the optimization
gate, and the earlier measurements must not be used to claim one.

The retained tests exercise the now-shared ownership-transfer implementation:
child-handle transfer, predecessor reference counts, extras/nulls, zero-count
reductions, shared/short/branching fallback, and preservation of head scanner
state, summary, and error baseline compared with pop-then-renumber.
