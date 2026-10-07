# Non-owning child summaries

## Profile and implementation

A parse-only `perf` profile over the pinned JavaScript inputs attributed roughly
8% of samples to the fresh reduction constructor (including child summarization).
The constructor built a `SubtreeHeapData` first, then summarized into it before
allocating its Arc. That header owns a Vec and implements iterative Drop; its
partially initialized fields therefore participate in cleanup throughout the
summary calculation. Annotation showed substantial stack initialization/store
traffic in this path.

Compute the same summary in a separate, non-owning `ChildSummary`, borrowing the
child slice. Only construct the complete heap header after the calculation. The
existing re-summarization entry point seeds this accumulator from the old header
and writes the result back. Preserve the old size/padding, first leaf, parse state
and fragility: C consults or preserves them in first-child/empty cases. Allocation,
sharing, child order, alias handling, costs and progress checks do not change.

The accumulator uses the original iterator loop. Experiments with compact stack
links, unary/binary summary specialization, forced lexing call boundaries and
earlier leaf initialization changes were initially removed: their gains did not
survive measurement or composition with newer main. The follow-up below combines
heap-only initialization with an outlined allocation path. There is no unsafe
code or grammar/table change.

## Original pinned measurements (historical)

Baseline main: `de86b49` (including the lexer and one-child stack-pop optimizations).
The three final `run_oracle(inputs="benchmark")` measurements were:

| Language | Main port/C | Run 1 | Run 2 | Run 3 | Median |
|---|---:|---:|---:|---:|---:|
| c | 0.91 | 0.89 | 0.89 | 0.89 | 0.89 |
| cpp | 0.97 | 0.95 | 0.95 | 0.94 | 0.95 |
| go | 0.92 | 0.89 | 0.90 | 0.89 | 0.89 |
| java | 0.87 | 0.85 | 0.87 | 0.88 | 0.87 |
| javascript | 0.94 | 0.91 | 0.91 | 0.91 | 0.91 |
| python | 0.90 | 0.89 | 0.88 | 0.88 | 0.88 |
| rust | 0.91 | 0.89 | 0.89 | 0.89 | 0.89 |
| tsx | 0.91 | 0.87 | 0.88 | 0.87 | 0.87 |
| typescript | 0.93 | 0.90 | 0.90 | 0.91 | 0.90 |
| **Geometric mean** | **0.917** | **0.894** | **0.896** | **0.895** | **0.895** |

The overall median is about 2.4% faster than main, versus the tool's reported
1.9% noise. JavaScript is approximately 3.2% faster using the rounded displayed
ratios. No language's displayed median regressed; Java is unchanged.

## Follow-up after the host rejection

The grammar-cache merge changed main to `298df50`; the host then measured only
1.2% improvement for the summary-only branch (0.885 vs 0.895), below the 1.9%
acceptance threshold. The older measurements above are not gains over this main.

The follow-up separates the common inline-leaf constructor/release paths from
heap allocation and traversal. `ts_subtree_new_leaf_with` is inlineable while its
heap/pool implementation stays out of line. External scanner payloads and their
state-change flag are initialized before placing the header in an Arc, avoiding
the parser's immediate COW uniqueness check. The callback runs only for heap
leaves; the external-token inline exclusion is unchanged. Release now dispatches
to a heap-only helper, so null/inline handles require no traversal-function call.
All heap release, scratch clearing, shared-reference, and pooling order is intact.

Three consecutive final measurements against `298df50`:

| Language | Main port/C | Run 1 | Run 2 | Run 3 | Median |
|---|---:|---:|---:|---:|---:|
| c | 0.88 | 0.92 | 0.86 | 0.87 | 0.87 |
| cpp | 0.93 | 0.91 | 0.91 | 0.92 | 0.91 |
| go | 0.90 | 0.87 | 0.87 | 0.87 | 0.87 |
| java | 0.85 | 0.83 | 0.83 | 0.83 | 0.83 |
| javascript | 0.92 | 0.91 | 0.90 | 0.91 | 0.91 |
| python | 0.88 | 0.87 | 0.87 | 0.87 | 0.87 |
| rust | 0.90 | 0.86 | 0.86 | 0.87 | 0.86 |
| tsx | 0.89 | 0.86 | 0.86 | 0.87 | 0.86 |
| typescript | 0.90 | 0.88 | 0.89 | 0.89 | 0.89 |
| **Geometric mean** | **0.895** | **0.879** | **0.871** | **0.875** | **0.875** |

The median improves overall time by approximately 2.2%, with no language's
rounded median slower. The first C measurement was a high outlier; all three
runs are reported rather than discarding it. These are pre-merge measurements,
not a substitute for the host's independent acceptance benchmark.

## Equivalence

- Added a fresh-vs-existing-header summarization test over empty, unary and
  multi-child parents, ordinary/error/error-repeat symbols, aliases, missing
  inline leaves, hidden children and external extras. It compares every header
  and payload field.
- New tests cover heap-only leaf initialization for fresh and pooled headers,
  scanner snapshots of length 0/24/25/1024, unique ownership, and scratch clearing
  when releasing null, inline, and shared heap handles.
- Existing tests cover first-child column dependencies using the old row count,
  empty reductions' precedence, alias/extra/EOF handling, and combined errors.
- Workspace tests and strict workspace/all-targets clippy pass.
- The oracle gate passes all 4,712 inputs with seven incremental checks and query
  checks enabled, including exact progress-callback parity.
- Full kernel/fresh-repository performance/correctness acceptance remains the
  host's independent merge gate.
