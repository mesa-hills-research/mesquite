# Language lookup optimization measurements

## Retained changes

- Keep the compressed-table cache's miss scan in a cold, non-inlined helper.
  Profiling main showed that its inlined scan required saving six callee-saved
  registers even on cache hits. The short hit path can now inline independently.
- Use 16,384 eight-byte cache slots (128 KiB per parser, previously 32 KiB).
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

Pinned `run_oracle(inputs="benchmark")` against main `de86b49`, after merging
its new stack fast paths and rechecking the combined optimization:

| Run | Main overall port/C | Branch overall port/C |
| --- | ---: | ---: |
| 1 | 0.917 | 0.899 |
| 2 | 0.917 | 0.896 |
| 3 | 0.917 | 0.892 |

The median is **0.896**, about **2.3% less parse time** than main, exceeding the
reported 1.9% overall noise. Run 1 had an isolated Java timing spike; per-language
medians from these three runs were:

| Language | Main | Branch median |
| --- | ---: | ---: |
| C | 0.91 | 0.88 |
| C++ | 0.97 | 0.93 |
| Go | 0.92 | 0.89 |
| Java | 0.87 | 0.85 |
| JavaScript | 0.94 | 0.92 |
| Python | 0.90 | 0.87 |
| Rust | 0.91 | 0.90 |
| TSX | 0.91 | 0.88 |
| TypeScript | 0.93 | 0.90 |

An earlier 8,192-slot version measured 2.2% faster than main `4a40c35` but only
1.7% after the stack changes. The retained 16,384-slot version was measured
against the newer combined baseline, not against an outdated main.

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

Final validation: all 4,712 oracle gate files passed with incremental edits and
query checks enabled; 190 runtime unit tests passed and runtime all-targets
clippy was clean. Whole-kernel/fresh-repository validation remains with the host.
