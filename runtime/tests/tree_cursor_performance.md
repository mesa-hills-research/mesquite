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
reductions and 5% in stack popping. The ordinary deterministic reduction cloned
child handles, created a temporary head, then released the old path and
immediately renumbered the new head back to the original version.

An ownership-transfer experiment removed this work for a single non-speculative
reduction on the sole active version. It first verified that every removed node
had exactly one graph owner and one predecessor, then moved the children and
transferred the final predecessor reference back to the original head. Against
main `af419b8`, this measured about 8.5% less parse time. Main subsequently gained
an overlapping optimization, so that result is **not** the final incremental
gain from this branch.

### Retained change after merging current main

The final change reuses main's ownership-transfer algorithm but lets the common
reduction take its children directly, instead of staging a `StackSlice` and
running the general ambiguity-selection loop. The small reduction entry point
can inline independently of the out-of-line general path. A failed preflight is
not attempted a second time without intervening mutation. Empty trailing-extra
buffers avoid drain setup as on main's general reduction path.

The transferred prefix preserves extras/null counting and child order. Shared
or branching paths, speculative reductions, multiple versions, and diagnostics
retain the general reduction path. Head scanner state, summary and error
baseline are unchanged, and no progress checkpoint is skipped. No recovery
costs, parse limits, tree semantics or callback ordering change. All code is
safe Rust.

After merging main's overlapping pop-transfer and subtree improvements (main
`9edb29b`), three pinned `run_oracle(inputs="benchmark")` runs gave:

| | Overall port/C |
| --- | --- |
| Main | 1.013 |
| Candidate run 1 | 0.983 |
| Candidate run 2 | 0.989 |
| Candidate run 3 | 0.985 |

The median is **about 2.8% less parse time**, beyond the reported 1.9% noise.
Every language improved in the median, and the measured overall port/C ratio
is now below 1.00.

Validation: all **4,712 oracle inputs passed**, with incremental seed 7 and
queries on; **188 runtime unit tests** and the runtime integration tests passed;
`cargo clippy --workspace --all-targets -- -D warnings` was clean. New regression
tests cover child-handle transfer, predecessor reference counts, extras/nulls,
zero-count reductions, shared/short/branching fallback, and preservation of head
scanner state, summary, and error baseline compared with pop-then-renumber.
