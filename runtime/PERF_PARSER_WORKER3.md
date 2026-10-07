# Parser optimization: combined stack and token hot-path improvements

## Retained candidate

Main at final synchronization: `84ad1eb`. This runtime-only candidate combines
previously retained work that had been blocked by the missing host benchmark
baseline; that baseline is now present and the supported alternating comparison
works again.

- **Direct recycled stack-slot initialization** (from tidy-8, resumed in
  `2cf650d`): represent vacancy by the existing zero refcount rather than an
  additional `Option` discriminant. Vacant slots own no links. Initialize all
  live scalar fields directly and preserve C's graph ownership, release order,
  merge limits, and stable usize indices. Tests exercise stale nonzero metrics
  when reusing a slot as both a base node and an ordinary node.
- **Small metadata accessor inlining** (also from tidy-8): expose related
  immutable node/cursor properties to callers, avoiding repeated call overhead
  in parse-and-walk consumers. Large traversal algorithms are unchanged.
- **Packed inline-token construction and pointer-sized heap-leaf return**
  (tidy-1's `317d252`, cherry-picked as `7197c31`): build the eight-byte inline
  payload as one word converted safely to bytes, retaining cheap byte-field
  reads. Return the Arc alone from the heap constructor instead of a whole
  Subtree aggregate. Inline limits, fields, flags, COW and pooling are unchanged.
  Tests cover all 65,536 state bit patterns and each flag mutation.

No unsafe code, pointer tagging, C reinterpretation, new dependencies, grammar
changes, or host-owned artifact changes were introduced. The combined changes
preserve algorithms, operation/progress counts, and tree results; representation
and code-generation choices differ from C as documented in PORTING.md.

## Correctness and hygiene

Validation was repeated on the combination, not inferred from its components:

- `cargo test -p ts_port`: **216 unit tests and both integration tests pass**.
- `cargo clippy -p ts_port --all-targets -- -D warnings`: clean.
- Differential gate: **11,816/11,816 pass**, incremental seed 7 and queries on.
- Kernel and fresh repositories: **163,507/163,507 pass**, queries on;
  the tool reports incremental checks off for this set.
- Total oracle inputs: **175,323**, no failures, panics, crashes, or skips.
- `git diff --check main`: clean; all changed paths are under `runtime/`.
- Final `git merge main`: already up to date.

## Supported alternating benchmark

Three calls to `run_oracle(inputs="benchmark")` on the retained combination
reported the following overall port/C geometric means (lower is better):

| Comparison call | Main | Candidate |
| --- | ---: | ---: |
| 1 | 0.724 | 0.730 |
| 2 | 0.751 | 0.702 |
| 3 | 0.729 | 0.703 |

The first call had large candidate Python/TOML/TypeScript outliers; the second
had main Java/Lua outliers. All results are included here rather than silently
excluding those calls. The median of these overall ratios is **0.729 vs 0.703**,
about **3.6% lower combined time**. The last comparison independently gives the
same improvement, and reports **2.0% overall noise**. In that last comparison,
all language combined ratios and walks improved, with no parse regression in the
printed two-decimal ratios. Walk improvements account for a substantial portion
of the gain; parse improvements are smaller.

These tool calls each compare one main and candidate run. They do not replace
the host's final admission calculation, which takes per-language/per-part
medians and checks their noise margins. The requested overall port/C <= 0.50
remains unmet; this is a smaller measurable improvement, not completion of that
long-term performance target.

## Withdrawn unary-storage experiment

Before resuming the retained candidates, worker-3 tried allocation-free unary
child storage (`11f1ed3`, reverted by `8654bb6`). Empty/One/Many and One/Many enums
fit in Vec's 24-byte footprint without enlarging the subtree header. Unary
reductions passed the child directly instead of creating a one-element Vec.

The three-variant form measured 0.724 vs main 0.727 (about 0.4% better), with
Markdown and walk regressions. The two-variant form measured 0.727 vs main 0.726
(about 0.1% worse). Unit tests passed and clippy was clean, but neither established
a gain above noise. No inline-child storage remains in this candidate.

## Initial interrupted profile

The first workflow stopped before implementation; its then-missing benchmark
baseline is historical, not a current blocker. The unchanged-main 969-file,
10-repeat `bench ... --workload parse_walk` cycles profile reported self time:
parser driver 9.85%, fused multi-child reduction (including its inlined
constructor) 7.13%, reduction entry 6.12%, balancing 2.87%, general reduction
2.45%. A large sampled site followed the aggregate return from new_heap_leaf.
These samples guided investigation; they were not speedup evidence. The shell
CPU was not SMT-isolated. Raw diagnostic artifacts remain temporarily under
`/tmp/worker3-perf/`, and none are required to build or validate the candidate.
