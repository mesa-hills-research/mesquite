# Cross-version committed-reduction optimization

Final comparison baseline: main `b97f71b` (including its fused stack-slot,
unary-summary, and quiet-parser optimizations).

## Remaining hot path and change

Main fused owned reductions only when the stack had one version. C++ frequently
has several GLR versions whose current prefixes are nevertheless uniquely owned.
For a sole reduce action, those prefixes can transfer children and replace their
existing slot without a temporary head, retained child handles, a slice worklist,
or the eventual renumber/release walk.

The version-aware fused helpers retain the existing preflight, constructor,
slot-reuse, child-order and cumulative-header algorithms. Shared or branching
prefixes still use the normal general reduction. Multiple-version parents retain
C's fragile marking. The optimization checks the same temporary-version limit,
including the halted-version allowance, even though no temporary head is created.

After replacement, merges visit all other original versions in index order. A
successful merge leaves the source present for advance to halt. Its original
scalar header and null-link status are restored: the outer scheduling loop reads
its position, and error recovery can inspect halted error states before stack
condensation. Scanner token, summary, and error baseline stay with the head.
Speculative/multiple-action reductions never consume their original prefix.

An experiment consuming partial prefixes was removed. It did not improve timing
and complicated preservation of the original halted head. No unsafe code, new
dependencies, or host-owned file changes are present.

## Final pinned measurements

Three runs against main's overall port/C **0.791**:

| | Run 1 | Run 2 | Run 3 | Median |
|---|---:|---:|---:|---:|
| Overall port/C | 0.770 | 0.768 | 0.771 | **0.770** |
| C++ port/C | 0.80 | 0.78 | 0.79 | **0.79** |

Overall improvement is approximately **2.65%**, exceeding the reported **1.9%**
noise threshold. C++ improves from 0.86 to 0.79, approximately **8.1%**. Language
medians are c 0.75, cpp 0.79, go 0.77, java 0.74, javascript 0.78, python 0.78,
rust 0.77, tsx 0.76, typescript 0.78. Rust's rounded median is about 1.3% above
main's 0.76, within the reported threshold; no other language median increases.

Earlier speedups and measurements in this branch's history have been superseded
by main and are not claimed as additional gains.

## Validation

- 208 runtime unit tests pass; runtime clippy (all targets) is clean.
- New tests compare committed and general reductions across both version-index
  orders, zero/unary/multi-child reductions, null links, trailing extras, merges,
  error-state headers, and temporary-version limits with halted allowances.
- All 4,712 gate inputs passed with incremental seed 7 and query checks.
- All 880 fresh C++ files passed with incremental seed 7 and query checks.
- All 146,326 fresh-repository/kernel files matched the host C reference's tree,
  progress, and query hashes (incremental off, matching the reference cache).

The broad fresh/kernel checks above ran immediately before merging main's
quiet-parser specialization; no cross-version optimization code changed in that
merge. The final gate passed again after the merge. Host acceptance must still apply
its complete final-worktree checks.
