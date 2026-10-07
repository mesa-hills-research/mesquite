# Parser cache fast paths and GLR code separation

Baseline: main `1ded31d`, pinned port/C geometric mean **0.743**. An unchanged
source control after restart measured 0.741, within the reported 1.9% noise.

## Retained changes

- Inline token-cache lookup and update. A normal cache miss no longer needs an
  out-of-line owning `Subtree::Null` return and subsequent aggregate copy; cache
  updates expose their null/inline cases directly to the caller.
- Borrow the stack's last external token when caching newly lexed lookahead.
  When the cache already owns the identical handle, keep that handle rather than
  performing an atomic clone followed by a release. Distinct handles still
  replace each other even if their serialized scanner bytes are identical.
- Explicitly keep general GLR reduction and condensation out of line. Merely
  placing these in separate private functions allowed the optimizer to fold
  them back into the fast-path routines.

The C reduction, lexing, recovery, and progress algorithms are unchanged. New
handles are retained before releasing the previous token and then its scanner
snapshot, in the original order. No unsafe code, public API, limits, table
layout, scanner format, or host-owned files changed.

## Evidence and experiments

A fresh parse-heavy `perf record -e cycles:u` profile used the host benchmark
list with `--repeat 20`. In the main parse routine, samples concentrated at
aggregate copies following cache misses and leaf construction. Inlining just
cache lookup or outlining just the GLR fallbacks produced small gains below the
noise threshold. Combining lookup/update inlining with GLR separation measured
0.730, 0.731, and 0.738, still insufficient. Avoiding redundant scanner-snapshot
reference-count traffic on top of that combination cleared the threshold.

Other tested changes were removed: owned reduction headers, specialized binary
reductions, inline child buffers, packed branch/leaf metadata, alternate helper
return types, action-run caches, range-search shortcuts, and extra outlining or
inlining of parse/lex/progress routines. They either regressed or failed to beat
noise. The earlier investigation is recorded in `perf_parser_2026_10_07.md`.

## Pinned measurements

Three screening runs of the retained production change measured 0.725, 0.728,
and 0.725. After adding comments and regression tests, three further consecutive
runs measured:

| | Run 1 | Run 2 | Run 3 | Median |
|---|---:|---:|---:|---:|
| Overall port/C | 0.727 | 0.730 | 0.727 | **0.727** |

The final median is approximately **2.2% less parse time** than main's 0.743,
above the 1.9% threshold. The median of all six runs is also 0.727.

| Language | Main port/C | Final-three median port/C |
|---|---:|---:|
| c | 0.72 | 0.71 |
| cpp | 0.76 | 0.75 |
| go | 0.74 | 0.73 |
| java | 0.72 | 0.71 |
| javascript | 0.76 | 0.74 |
| python | 0.75 | 0.74 |
| rust | 0.74 | 0.72 |
| tsx | 0.74 | 0.73 |
| typescript | 0.75 | 0.74 |

The table uses the rounded values returned by the benchmark tool. Every
language's median improved; the overall 0.60 target is not yet met.

## Validation

- `cargo test -p ts_port`: **210** unit tests and both integration tests pass.
- `cargo clippy --workspace --all-targets -- -D warnings`: clean.
- Oracle gate: **4,712/4,712** files pass, with incremental seed 7 and queries on;
  complete trees and progress counts remain identical to C for all languages.
- The new regression compares borrowed cache updates with the original owning
  setter: unchanged and changing scanner handles, distinct handles with equal
  bytes, null/empty snapshots, inline/heap tokens, token/snapshot aliasing,
  reference counts, pool state, and reset behavior.
- Full kernel and fresh-repository validation remain host merge-time checks.
