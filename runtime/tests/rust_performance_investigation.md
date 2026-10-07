# Diagnostic-free parser and exhausted-reuse fast paths

## Profile and retained changes

A local parse-only `perf` harness used the host benchmark's Rust inputs and the
same input/progress-callback shape. It attributed roughly 10–13% of samples to
parser advance and another 5% to the parse loop; stack and subtree construction
were also prominent. The harness did not walk or format the resulting trees.
Sampling included tree destruction outside the parse timer, so the host's pinned
parse timings, not the sampling percentages, determine the gain.

The retained safe-Rust changes are:

- Skip `ts_parser__reuse_node` when its cursor stack is empty. Previously every
  token of a fresh parse entered that large traversal only to read a null current
  tree and return, without changing any state. Nonempty incremental-reuse cursors
  still follow the original traversal.
- Select a const-generic diagnostic mode once at parse entry, propagating it into
  advance and lex. The quiet specialization removes repeated optional logger/DOT
  tests and their diagnostic blocks from those hot functions. The enabled path
  retains the existing lazy formatting, messages and DOT writes. Configuration
  cannot change during the exclusive parser borrow, and selection is repeated on
  every call, including cancellation/resume.
- For a sole active stack version, read its condensation cost directly rather
  than constructing an entire version-comparison record. Still lower the saved
  error-node baseline, and return `u32::MAX` for an error-state head. Multiple or
  inactive versions retain the full condensation algorithm.

There are no grammar/scanner changes, unsafe additions, progress-check changes,
new persistent caches, or adjusted error costs/limits. The compiler specializes
control flow; the parser's decisions and order of operations remain unchanged.

## Pinned measurement

The candidate includes main **4ab0fd1**, including its latest subtree and grammar
cache optimizations. Three consecutive runs after final production-code polish
reported overall port/C **0.829, 0.830, 0.830**, versus main **0.854**. The median is
**0.830**, approximately **2.8% less parse time**, above the reported 1.9% overall
noise. Every language's median improved:

| Language | Main port/C | Candidate median port/C |
| --- | ---: | ---: |
| c | 0.85 | 0.83 |
| cpp | 0.91 | 0.88 |
| go | 0.85 | 0.83 |
| java | 0.82 | 0.79 |
| javascript | 0.88 | 0.85 |
| python | 0.84 | 0.82 |
| rust | 0.84 | 0.82 |
| tsx | 0.84 | 0.82 |
| typescript | 0.86 | 0.83 |

Per-language values are rounded by the benchmark. Its overall value uses the
unrounded geometric mean. An earlier post-merge run before comment/formatting
polish measured 0.825; it is not used in the final three-run median above.

Earlier inline-child storage, external-payload layout, lexer adapter, scanner
cache and additional inline-hint experiments were discarded: their gains did
not exceed noise or did not survive composition with newer main changes. The
previous documentation-only submission's no-speedup result does not describe
this retained production candidate.

## Validation

- All **4,712** differential gate files pass with incremental seed 7 and queries
  enabled, including complete trees and progress-callback counts. No failures,
  skips, crashes, or timeouts.
- Regression tests compare complete subtree metadata and progress sequences
  between quiet, logger-only, DOT-only and combined modes for valid and malformed
  input, both fresh and edited incremental parses. They check that lexer/parser
  diagnostics and DOT output remain present when enabled.
- Cancellation/resume tests cover all four initial/resumed logger combinations,
  comparing trees and progress sequences and checking resumed diagnostics.
- Existing condensation tests compare the shortcut and general algorithm for
  error costs and saved baselines, ordinary/error states and null/non-null links.
- `cargo test -p ts_port`: **196 unit tests and both integration tests pass**.
- `cargo clippy --workspace --all-targets -- -D warnings`: **clean**.

The full kernel/fresh-repository acceptance suite remains a host merge-time
check; the local differential run above covers the gate set.
