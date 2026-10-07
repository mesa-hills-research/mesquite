# Rust parse-performance investigation (no retained production change)

## Profile and experiments

A local parse-only Rust harness, using the host benchmark's Rust inputs and the
same input/progress callback shape, attributed approximately 20% of samples to
subtree construction and stack popping. Unlike the full dumper profile, this
harness did not walk or format the resulting trees. Its sampling still included
tree destruction outside the parse timer; pinned oracle timings were the
acceptance criterion.

The following safe-Rust experiments were measured and then removed:

- Inline storage for a unary branch's child, avoiding its child Vec allocation
  on committed single-version reductions. The local Rust measurement improved,
  but the multi-language benchmark did not improve beyond noise. An inline
  two-child variant was also unsuccessful.
- Flattening the nested external-scanner payload enum to reduce header size.
  Tested independently against main, its overall ratio was 0.974 versus main's
  0.974: no measured gain.
- Separating reduction control flow and accumulating child summaries locally.
  The initial variants did not improve the local baseline; newer main changes
  subsequently optimized these paths more effectively.
- Ordinary inline hints on stack node construction and push measured overall
  ratios of 0.922 and 0.921 against the then-main 0.939 (about 1.8–1.9%). This
  was marginal against the reported 1.9% noise. Forced inlining, additional pop
  and trailing-extra inline hints, and a different constructor initialization
  order did not improve it. Main's later combined one-child pop specialization
  and inline hints superseded this candidate.

All experimental data-layout and production changes have been removed. The
runtime, scanners, and existing tests match main at `de86b49`. Only this report
is additional. No new unsafe code or host-owned file changes remain, and no
additive speedup is claimed.

## Final validation

After merging the latest main:

- The pinned benchmark reported main **0.917** and this worktree **0.920** overall
  port/C, within noise for identical runtime code. Rust was **0.91** and **0.92**,
  respectively. Main already meets the overall <= 1.00 goal.
- All **4,712** differential gate inputs passed, with incremental seed 7 and
  queries enabled; there were no failures, skips, crashes, or timeouts.
- `cargo test -p ts_port` passed (188 unit tests and both integration tests).
- `cargo clippy --workspace --all-targets -- -D warnings` passed.

The complete kernel/fresh-repository suite was not rerun locally. There is no
remaining production delta for it to validate relative to that main commit.
