# C parsing investigation — worker 8

## Disposition

No independent optimization is proposed for merging. The useful change found in
this investigation was independently merged on main before validation finished.
The other experiments failed the performance threshold and were removed. The
final runtime, grammar and table sources are identical to main commit `4a40c35`.
Only this investigation report is retained.

The task's original C ratio of 1.33 and overall ratio of 1.387 were stale by the
start of this investigation. Main has since achieved the overall <=1.00 goal.
This report does not attribute main's gains to this branch.

## Profile and experiments

A temporary parse-only harness parsed the 40 pinned C benchmark files 100 times,
without walking or formatting the trees. `perf record` attributed approximately
9.6% of samples to reduction-node construction, 9.3% to parser advance, 5.9% to
the general reduction routine, 4.1% to in-place stack popping and 3.9% to stack
node construction. Tree destruction was included in this diagnostic harness;
acceptance measurements used the host's pinned parse benchmark instead.

1. **Direct committed reductions.** Return the children directly from a unique
   in-place stack pop rather than constructing a one-element slice worklist.
   Handle this single result without general version grouping, halted-version
   counting and merge checks. Two pinned runs measured overall port/C 0.985
   against main's then-recorded 1.013, approximately 2.8% faster. All 4,712 oracle
   gate files passed, including incremental and query checks. Main then acquired
   the equivalent change in `5caeb8d`, together with single-version condensation
   in `f22e209`; the duplicate implementation was discarded and main merged.
2. **Compact stack links.** Pack the pending flag into a spare bit of an arena
   index (integer arithmetic only, no pointers or unsafe), reducing 64-bit link
   storage from 32 to 24 bytes and stack nodes from 72 to 64 bytes. One pinned
   run measured 0.969 against main's 0.974: only about 0.5%, below the reported
   1.9% noise. Removed rather than retaining an unproven representation change.
3. **Unary in-place pop shortcut.** Inline a single-child/no-extra pop to avoid
   the general preflight and reverse loops. Tests passed, but the pinned run
   measured 0.981, showing no gain even against the pre-lexer baseline of 0.974.
   Main's lexer optimization also landed during this experiment. Removed.

## Final verification after merging main

With no production source differences from `4a40c35`, three consecutive pinned
runs measured overall port/C **0.969, 0.958, 0.959** (median **0.959**) and C
**0.96, 0.94, 0.94** (median **0.94**). The host's recorded main overall ratio was
0.939. At this point HEAD and main had the same Git tree
`8fb2d883721e40769940eeac1e99ebea16478537`; the timing difference cannot be
attributed to a source change on this branch. No independent improvement over
main is claimed.

- `cargo check --workspace`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test -p ts_port --all-targets`: 187 unit tests and both integration
  tests passed.
- Oracle gate: 4,712/4,712 files passed; incremental=7, queries=on; no failures,
  crashes, timeouts or skips.
- No new unsafe, host-owned changes, or retained algorithm deviations.
- The complete kernel and fresh-repository suites were not independently rerun
  here; there is no production-code delta requiring a new equivalence claim.
