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

## Retained optimization: fuse stack-slot replacement with reductions

A fresh parse-only profile against `298df50` still showed 5.4% self time in
`ts_stack_push`, 2.9% in the longer owned-pop path, and roughly 1% each in
renumbering, trailing-extra removal, and version-status bookkeeping. The
committed reduction already transferred child ownership, but still removed its
last stack slot and immediately reconstructed a new node in that same slot.

The retained implementation now:

- Specializes a committed unary reduction with a unique, single-link top and no
  trailing extra. It moves the child into the constructor without moving the
  stack head, recycling its slot, or rebuilding the predecessor link.
- Preflights longer committed prefixes exactly as before, then preserves the
  last removed slot for the parent. Other slots are recycled in the same order;
  the final arena/head/free-list state matches separate pop-then-push. Zero-child
  reductions still add a new slot.
- Computes cumulative position, error cost, node count, and dynamic precedence
  from the predecessor and the new parent, using the same wrapping arithmetic
  and subtree accessors as normal push. Head scanner state, summary, status,
  and error baseline are unchanged.
- Keeps shared, branching, inactive, and incomplete prefixes on the general
  graph traversal. The unary shortcut delegates extra-bearing tops to the
  longer-prefix path. Neither shortcut changes progress checkpoints.
- Exposes no-op renumbering as a small inline wrapper, leaving actual version
  removal out of line, and inlines version-status computation. Trailing-extra
  removal also stays inline (now shared with main).

The constructor is called only after eligibility is proven and must return a
non-null parent. There are no new unsafe blocks, dependencies, grammar changes,
public API changes, or changes to cursor traversal. The separate owned-pop
implementation remains test-only as an independent reference for stack-state
comparisons.

### Final benchmark against merged main `4ab0fd1`

Main advanced during development; its child-summary and inline-leaf changes
were merged before the final measurements. Three consecutive pinned benchmark
runs on the final production code reported:

| Language | Main | Run 1 | Run 2 | Run 3 |
| --- | ---: | ---: | ---: | ---: |
| C | 0.85 | 0.82 | 0.83 | 0.82 |
| C++ | 0.91 | 0.87 | 0.87 | 0.87 |
| Go | 0.85 | 0.82 | 0.82 | 0.82 |
| Java | 0.82 | 0.77 | 0.78 | 0.78 |
| JavaScript | 0.88 | 0.83 | 0.84 | 0.83 |
| Python | 0.84 | 0.80 | 0.80 | 0.80 |
| Rust | 0.84 | 0.80 | 0.79 | 0.79 |
| TSX | 0.84 | 0.81 | 0.80 | 0.81 |
| TypeScript | 0.86 | 0.83 | 0.82 | 0.82 |
| **Overall port/C** | **0.854** | **0.818** | **0.817** | **0.816** |

The median overall ratio is **0.817**, about **4.3% less parse time than main**
(`1 - 0.817 / 0.854`), beyond the reported 1.9% noise threshold. All nine
languages improved. This is a new gain over merged main, unlike the historical
experiments above.

### Validation

- Full oracle gate: **4,712/4,712 pass**, no failures or skips,
  `incremental=7 queries=on`, including complete trees and progress counts.
- `cargo test --workspace`: all tests pass, including 203 runtime unit tests
  and both runtime integration tests.
- `cargo clippy --workspace --all-targets -- -D warnings`: clean.
- New direct tests compare complete stack snapshots against pop-then-push for
  unary and multi-child replacements, including nulls, extras, zero-count
  reductions, cumulative headers, predecessor counts, and head metadata. They
  also check child transfer without retaining and unchanged rejected paths.
- Existing reduction tests compare committed and general paths, including
  fragility, dynamic precedence, extras, empty nodes, and error baselines.

The host's exhaustive kernel and fresh-repository checks remain merge-time
validation; the local gate result does not claim to include those additional
sets.
