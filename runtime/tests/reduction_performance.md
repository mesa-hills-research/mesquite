# Direct completion of committed reductions

Baseline: main `9edb29b`, including compact-table caching, ASCII lexer changes,
pre-Arc reduction initialization, in-place unique-prefix popping, and subtree
summary/release improvements.

A sole committed reduction already knows it has one active version, one pop
result, and no alternatives to compare or merge. Previously the in-place pop
still wrapped its child Vec in a `StackSlice`, transferred the slice buffer, and
ran general slice/version bookkeeping. It now returns the child Vec directly;
`ts_parser__reduce` completes that reduction and returns its existing version.
The original general reduction remains the fallback for speculative reductions,
multiple versions, branching, and shared prefixes. Header metadata, extra-token
order, ownership, error baselines, scanner state, and progress checkpoints are
unchanged. No unsafe code or host-owned files were changed.

## Measurement

Three pinned oracle benchmark runs after merging the baseline:

| | Run 1 | Run 2 | Run 3 | Median |
|---|---:|---:|---:|---:|
| Overall port/C | 0.985 | 0.986 | 0.992 | **0.986** |
| C++ port/C | 1.02 | 1.02 | 1.07 | **1.02** |

Main's overall port/C was 1.013 and C++ was 1.03. The median overall improvement
is approximately **2.7%**, exceeding the reported **1.9%** noise threshold.
Per-language medians were c 0.97, cpp 1.02, go 0.97, java 0.93, javascript 1.03,
python 0.99, rust 0.99, tsx 0.97, typescript 1.00; none regressed versus main.
The geometric mean is now below the requested 1.00 port/C target.

## Validation

- Oracle gate: **4,712/4,712**, incremental seed 7 and queries enabled.
- Full fresh and kernel manifests: **146,326/146,326**, compared tree, range,
  field, progress, and query hashes against the host's cached C reference tag
  `714d8a240b152b0f+q` (incremental off, matching those cached checks).
- All fresh C++ files: **880/880**, run directly against the C dumper with
  incremental seed 7 and queries enabled.
- **185** runtime unit tests pass; `cargo clippy -p ts_port --all-targets` clean.
- Added direct/general reduction equivalence coverage for empty reductions,
  nonterminal extras, fragile headers, and negative dynamic precedence. Existing
  tests cover trailing extras, ambiguity, sharing/branch fallback, allocation
  reuse, and head metadata. The prior cache-collision regression is retained.
