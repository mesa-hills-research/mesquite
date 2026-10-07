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

## Follow-up: optimize parsing, not the tree walk

After the documentation-only result was rejected, a parse-only driver over the
same 360-file input list showed approximately 7% self time in reductions and 5%
in stack popping. The ordinary deterministic reduction cloned child handles,
created a temporary head, then released the old path and immediately renumbered
the new head back to the original version.

The retained optimization fuses that sequence for a single non-speculative
reduction on the sole active version. It first verifies that every removed node
has exactly one graph owner and one predecessor. It then moves, rather than
clones, the children, preserving extra/null counting and child order, and
transfers the final predecessor reference back to the original head. Shared or
branching paths, speculative reductions, multiple versions, and diagnostic
output retain the original algorithm. No progress checkpoint is skipped.

This is an ownership optimization, not a change to recovery costs, parse limits,
tree semantics, or callback ordering. Arena slots can be recycled earlier, and
no transient stack version is needed on the fused path. All code is safe Rust.

After merging main's lexer and reduction-header improvements (main `af419b8`),
three pinned `run_oracle(inputs="benchmark")` runs gave:

| | Overall port/C |
| --- | --- |
| Main | 1.121 |
| Candidate run 1 | 1.031 |
| Candidate run 2 | 1.025 |
| Candidate run 3 | 1.026 |

The median is **about 8.5% less parse time**, beyond the reported 1.9% noise.
Every language improved. This does not yet meet the ultimate overall <= 1.00
port/C goal.

Validation: all **4,712 oracle inputs passed**, with incremental seed 7 and
queries on; **182 runtime unit tests** and the runtime integration tests passed;
`cargo clippy -p ts_port --all-targets -- -D warnings` was clean. New regression
tests cover child-handle transfer, predecessor reference counts, extras/nulls,
zero-width reductions, shared/short/branching fallback, and preservation of head
scanner state, summary, and error baseline compared with pop-then-renumber.
