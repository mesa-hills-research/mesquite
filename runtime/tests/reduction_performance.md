# C++ performance investigation — final disposition

**No additional qualifying performance change remains on this branch.** All
production runtime sources match main `298df50`. The remaining differences are
this report and a regression test exercising colliding parse-table cache entries,
cached zero transitions, and clearing the cache before switching grammars.

## Historical improvements, now superseded

Earlier versions of this branch measured improvements from compressed-table
memoization, direct completion of committed reductions, and a specialized
one-child committed pop with ordinary inlining hints. Equivalent optimizations
entered main independently before final acceptance. Their historical gains must
not be counted as gains over current main.

In particular, the previous report's 2.24% improvement compared against main
`4a40c35`, not current main. The subsequent host measurement found only 1.0%
additional improvement, below the required 1.9% noise threshold. This report
replaces that stale claim.

## Follow-up experiments

The current C++ profile still showed substantial time in stack pushes (~7.5%),
general count pops (~5.3%), and subtree construction. Experiments included:

- Constructing complete stack-node records from local summaries rather than
  updating partially initialized records.
- Storing one-child reduction children inline in the heap header, avoiding the
  separate child-buffer allocation on the committed one-child path.
- Caching decoded action slices in addition to compact-table values.
- Combining stack state/link-count fields into one integer to change record-copy
  code generation.
- Specializing zero/one-child general pops without changing speculative ownership.
- Deferring byte-column updates until the lexer's position is observed.

None demonstrated a gain beyond the acceptance threshold; all were reverted.
Representative single-run overall port/C measurements for the last four
experiments were 0.905, 0.896, 0.898, and 0.901 respectively, against current main's
0.895. These are screening measurements, not three-run acceptance medians.

Main's latest reported C++ port/C is 0.93 and its overall geometric mean is 0.895,
already below the task's original 1.00 goal. **No incremental gain is claimed for
this final branch; it should not be accepted as a new performance optimization.**

## Final validation

- Production runtime sources match main `298df50` exactly.
- All 191 runtime unit tests pass.
- `cargo clippy -p ts_port --all-targets --quiet` is clean.
- No unsafe code or host-owned files were changed.
- Prior optimized revisions passed the differential gate, but those results are
  historical. No unvalidated experimental production changes remain.
