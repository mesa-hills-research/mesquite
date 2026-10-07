# Language lookup optimization measurements

## Retained changes

- Keep the compressed-table cache's miss scan in a cold, non-inlined helper.
  Profiling main showed that its inlined scan required saving six callee-saved
  registers even on cache hits. The short hit path can now inline independently.
- Use 8,192 eight-byte cache slots (64 KiB per parser, previously 32 KiB).
  The state/symbol pair plus one is a nonzero 33-bit key; the low 16 bits of a
  slot remain its value. Checking the entire key preserves zero results and
  makes collisions harmless. A stride of 131 spreads neighboring states better
  than 31 in the measured lookup stream. The cache still bypasses dense rows
  and is cleared on every language-setting attempt.
- Represent `TableEntry` as one borrowed slice including the action-list header,
  rather than an action slice and a separately copied bool. This reduces the
  handle from three machine words to two. The actions accessor returns the same
  header-free slice as before; the reusable accessor reads the immutable header.
  The empty default borrows a static zero-action, non-reusable header.

There are no new allocations during parsing, no new unsafe code, no changes to
host-owned grammar data, and no changes to parsing decisions, callback timing,
error-recovery limits, or action order. Only the parser's fixed cache allocation
is larger.

## Measurements

Pinned `run_oracle(inputs="benchmark")` against main `4a40c35`:

| Run | Main overall port/C | Branch overall port/C |
| --- | ---: | ---: |
| 1 | 0.939 | 0.915 |
| 2 | 0.939 | 0.941 |
| 3 | 0.939 | 0.918 |

The median is **0.918**, about **2.2% less parse time** than main, exceeding the
reported 1.9% overall noise. Run 2 had isolated Go/Rust timing spikes; per-language
medians from these three runs were:

| Language | Main | Branch median |
| --- | ---: | ---: |
| C | 0.93 | 0.90 |
| C++ | 0.99 | 0.96 |
| Go | 0.94 | 0.92 |
| Java | 0.89 | 0.87 |
| JavaScript | 0.97 | 0.94 |
| Python | 0.93 | 0.90 |
| Rust | 0.94 | 0.92 |
| TSX | 0.93 | 0.91 |
| TypeScript | 0.95 | 0.93 |

## Rejected experiments

A slice/contains scan, dense table expansion, lazy decoded rows, cached action
headers or full action slices, and forced inlining of all lookup helpers did
not provide a repeatable sufficient gain when combined with current main. They
are not retained. The earlier 4,096-slot cache was already merged by another
worker; none of its original speedup is attributed to these changes.

## Equivalence coverage

Unit tests compare raw cached lookups for all state/symbol combinations in all
nine generated grammars. The expanded checks also compare each terminal's
header-backed `TableEntry` with the host table decoder's action slice and flag.
Targeted tests cover the two-word representation, borrowed slice identity,
default entries, zero values, colliding keys, the all-ones state/symbol pair,
language changes, and existing first-action/last-action semantics.
