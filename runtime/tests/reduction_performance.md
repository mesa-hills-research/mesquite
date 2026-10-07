# Final committed-pop optimization

Baseline: main `4a40c35`. Earlier cache and direct-reduction experiments on this
branch were superseded by equivalent main changes; those historical gains are
not claimed as additional gains here.

## Final changes

- For a committed one-symbol pop, a unique top node with one non-extra edge can
  transfer that edge immediately. It needs no second ownership/predecessor walk.
  Null links produce an empty child Vec; ordinary tokens produce one owned child.
  Leading extras, larger reductions, sharing, and branching keep the existing
  preflight/general paths. Free-list order, head metadata, and refcounts remain
  unchanged.
- Ordinary `#[inline]` hints expose the small committed reduction, in-place pop,
  push, and node-construction paths to call-site optimization. No forced inlining,
  unsafe code, or host-owned changes are used.

## Final pinned measurements

Three runs against main's overall port/C **0.939**:

| | Run 1 | Run 2 | Run 3 | Median |
|---|---:|---:|---:|---:|
| Overall port/C | 0.918 | 0.921 | 0.917 | **0.918** |
| C++ port/C | 0.96 | 0.97 | 0.97 | **0.97** |

Median overall improvement is approximately **2.24%**, beyond the reported
**1.9%** noise threshold. C++ improves from main's 0.99 to 0.97. No language's
median regresses; all nine language medians and the geometric mean are below C.

## Final validation

- **4,712/4,712** oracle gate inputs match, with incremental seed 7 and queries on.
- **189** runtime unit tests pass; clippy is clean.
- Added focused coverage of null and visible one-edge pops, including predecessor
  ownership and free-list order. Existing tests cover extras, shared/branching
  fallback, missing/error paths, zero-count pops, metadata, and scratch reuse.

Full kernel/fresh reference checks also passed on the preceding direct-reduction
revision, but must be rerun by the host for final acceptance of this revision.
