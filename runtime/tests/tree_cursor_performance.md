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

After the documentation-only result was rejected, a parser-loop profile over the
same 360-file input list (no tree walking) showed approximately 7% self time in
reductions and 5% in stack popping. Experiments transferred exclusively owned
stack paths instead of cloning them and separated committed reductions from the
general slice worklist. Main acquired overlapping implementations while these
experiments were being tested. Those changes were merged rather than duplicated,
and their earlier gains are **not** claimed as this branch's final contribution.

### Retained optimization after merging current main

The final runtime delta is ordinary `#[inline]` hints on four functions:

- `ts_parser__reduce`: expose its small committed-reduction path to advance,
  while leaving the large general reduction worklist out of line;
- `ts_stack_pop_count_in_place`: expose ownership transfer to its immediate
  consumer;
- `ts_stack_push` and `stack_node_new`: specialize push sites for an always-present
  predecessor and remove extra call layers carrying owned subtree/node state.

This keeps the algorithms, operation order, ownership rules, recovery costs,
limits and progress checkpoints unchanged. No unsafe code, forced inlining,
new allocations, or grammar-table changes are introduced.

After merging main's direct reduction, stack condensation and subtree changes
(main `5aedb91`), three pinned `run_oracle(inputs="benchmark")` runs gave:

| | Overall port/C |
| --- | --- |
| Main | 0.974 |
| Candidate run 1 | 0.950 |
| Candidate run 2 | 0.950 |
| Candidate run 3 | 0.947 |

The median is **about 2.5% less parse time**, beyond the reported 1.9% noise.
No language is slower; the measured overall port/C ratio is below 1.00.

Validation: all **4,712 oracle inputs passed**, with incremental seed 7 and
queries on; **190 runtime unit tests** and the runtime integration tests passed;
`cargo clippy --workspace --all-targets -- -D warnings` was clean. New regression
tests retained from the ownership experiments cover child-handle transfer,
predecessor reference counts, extras/nulls, zero-count reductions,
shared/short/branching fallback, and preservation of head scanner state, summary,
and error baseline compared with pop-then-renumber.
