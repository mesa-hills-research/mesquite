# Subtree hot-path optimization

The original subtree path repeatedly decoded child representations, synchronized
Arc uniqueness when both queuing and releasing a header, and called its iterative
destructor even for already-drained headers. More subtly, constructing a header
with `..SubtreeHeapData::default()` created a second temporary with a destructor.
A local `perf record` profile (the oracle dumper, 50 parses per benchmark input)
and annotated assembly exposed that redundant initialization/destruction in the
reduction constructor.

The optimized implementation:

- Constructs complete leaf/reduction headers without temporary dropping defaults;
  freshly allocated leaves are initialized before entering an Arc.
- Decodes inline/heap children once and accumulates summaries locally, publishing
  them after traversal. Keeps C's error costs, alias/extra indexing, old-row
  dependency check, empty-reduction behavior, and sticky fragility flags.
- Keeps the empty destructor path inline and the deep iterative traversal out of
  line. Destruction remains safe for shared children and weak observers.
- Consumes the initial release handle directly, checks uniqueness once per queued
  header, and pools already-borrowed leaf headers without another uniqueness check.

After merging the lexer and reduction-constructor changes from main, three
consecutive pinned benchmark runs reported overall port/C ratios **1.097, 1.108,
1.096**, against main's **1.121**. The median is **1.097**, about **2.1% faster**
than main (reported overall noise: 1.9%). Per-language medians:

| Language | Main | Optimized |
|---|---:|---:|
| C | 1.05 | 1.03 |
| C++ | 1.10 | 1.07 |
| Go | 1.06 | 1.04 |
| Java | 1.04 | 1.03 |
| JavaScript | 1.20 | 1.17 |
| Python | 1.18 | 1.14 |
| Rust | 1.20 | 1.17 |
| TSX | 1.13 | 1.09 |
| TypeScript | 1.16 | 1.14 |

Validation: 4,712/4,712 oracle gate files match C, with incremental seed 7 and
queries enabled. Workspace tests pass and workspace/all-target clippy is clean.
Added unit coverage for summary edge cases and deeply nested weakly observed
release. No unsafe code or host-owned file changes. The overall <= 1.00 goal is
not yet reached; whole-kernel/fresh-repository acceptance remains with the host.
