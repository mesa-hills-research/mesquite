# Direct stack-slot construction and full-word subtree tags

## Investigation

Baseline: `cc15596` (main at the start of this task), pinned port/C **0.765**.
A same-tree control measured **0.764**. The requested longer-reduction hotspot
includes the inlined parent constructor, not just stack traversal.

A local `perf record -e cycles:u` run of the oracle's `profile/bench.list`, with
`--repeat 10`, exposed another concrete bottleneck: about 75% of the samples
inside `ts_stack_push` landed at the copy from a temporary node's tail to the
recycled arena slot. The preceding 64-bit load overlapped separate 32-bit
precedence and 16-bit state/link-count stores. Fresh subtree handles also used
narrow tag stores followed by full-handle copies. These instruction sequences
are susceptible to store-forwarding stalls.

## Retained changes

- Choose the vacant arena slot **before** computing cumulative metrics. This
  also avoids keeping all the metrics live across arena growth.
- Compute the non-owning scalar header, then construct the owning `StackNode`
  directly in the destination. Do not build, update, and copy a local node.
- Give `Subtree` a `u64` discriminant. On the benchmark's 64-bit target this uses
  existing padding without increasing its 16-byte size (including
  `Option<Subtree>`), while avoiding narrow-tag/full-handle store forwarding.
- For committed single-link replacement, assert that overflow storage is absent
  rather than assigning `None` and compiling an unnecessary dropping assignment.
  Link count only increases on live nodes; overflow storage is created only when
  adding a second link, and shared/branching prefixes still take the general path.

All changes are safe Rust. Arena IDs, free-list order, logical reference counts,
subtree ownership, arithmetic, traversal order, GLR preflight, and callback
checkpoints are unchanged. The C algorithms and limits have not changed. The
representation change is internal; no tree or serialized scanner format uses
raw enum memory.

## Discarded experiments

Moving entire removed nodes out during longer reductions, specializing ordinary
two-child productions, and packing the pending bit into arena IDs did not beat
the baseline and were removed. Scalar-header computation alone and word-sized
tags alone were not enough to clear the benchmark's 1.9% noise threshold.
Selecting the arena destination only *after* computing metrics also lost much
of the gain; the allocation-before-header ordering matters.

## Measurements

Three consecutive pinned runs of the final implementation:

| | Run 1 | Run 2 | Run 3 | Median |
| --- | ---: | ---: | ---: | ---: |
| Overall port/C | 0.744 | 0.743 | 0.745 | **0.744** |

This is **2.75% less parse time** than main's 0.765, beyond the reported 1.9%
overall noise. Every language's median improved:

| Language | Main port/C | Worktree median port/C |
| --- | ---: | ---: |
| c | 0.74 | 0.72 |
| cpp | 0.78 | 0.76 |
| go | 0.77 | 0.74 |
| java | 0.74 | 0.72 |
| javascript | 0.78 | 0.76 |
| python | 0.78 | 0.75 |
| rust | 0.77 | 0.74 |
| tsx | 0.75 | 0.73 |
| typescript | 0.78 | 0.76 |

Ratios above are rounded by the benchmark tool. The same local profiling command
put `ts_stack_push` at **0.67%** of full-dumper samples, down from **3.84%**;
the overlapping temporary-tail copy is gone. These profile shares include tree
walking and are diagnostic only, not the parse-only speed measurement.

## Validation

- All **4,712** oracle gate files match C, including complete trees, progress,
  incremental seed 7, and query checks.
- `cargo test -p ts_port`: 209 unit tests and both integration tests pass.
- `cargo clippy --workspace --all-targets -- -D warnings`: clean.
- The new test compares fresh and recycled node construction, including a slot
  previously owning overflow links, null/inline/missing/heap subtrees, both
  pending states, cumulative counters, wrapping extents and negative precedence.
- Layout tests continue to enforce 8-byte inline leaves and 16-byte handles on
  64-bit targets, and now check that optional handles remain 16 bytes as well.
- No unsafe code or host-owned files changed. The complete kernel and fresh-repo
  suites remain host merge-time checks; the overall 0.60 target is not yet met.
