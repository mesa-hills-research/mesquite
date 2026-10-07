# Committed reduction and single-version parser paths

## Change

The parser's committed, uniquely owned single-version reduction already moved
stack children instead of cloning them. It nevertheless put those children in a
`Vec<StackSlice>` and entered the general reduction's version-count, slice
selection, and merge machinery. Return the child Vec directly and build/push the
one parent on a separate path. Keep the full general reduction for shared or
branched prefixes, speculative reductions, and multiple actions/versions.

Similarly, condensing one active version cannot merge, prune, or resume anything.
Read its version status and return its minimum error cost directly. This still
performs the error-baseline update in `ts_stack_node_count_since_error`; omitting
that read would not be equivalent to C.

No unsafe code, grammar modifications, limit changes, progress-check changes, or
new persistent caches are involved. General reduction selection still precedes
adding the action's dynamic precedence.

## Measurement

The host's initial full-dumper profile attributed 2.49% self time to
`ts_parser__reduce` and 1.98% to `ts_parser_parse_with_options`. A local `perf
record -e cycles:u` over the host's `profile/bench.list` (hash, repeat 3) after
splitting these paths showed 0.55% in the standalone reduction function and 1.36%
in parse-with-options; some of the smaller reduction path now inlines into
advance. These profiles also include tree traversal: the pinned parse timer,
not profile percentages, determines the speedup.

After merging main's subtree changes at `9edb29b`, three host pinned benchmark
runs gave overall port/C ratios **0.978, 0.973, 0.976** against main's **1.013**.
The median is **0.976**, approximately **3.65% less parse time** than main,
exceeding the host's reported 1.9% overall noise. Every language's median improved:

| Language | Main port/C | Worktree median port/C |
| --- | ---: | ---: |
| c | 0.99 | 0.96 |
| cpp | 1.03 | 1.01 |
| go | 0.99 | 0.95 |
| java | 0.96 | 0.93 |
| javascript | 1.06 | 1.00 |
| python | 1.03 | 0.98 |
| rust | 1.03 | 0.97 |
| tsx | 1.00 | 0.97 |
| typescript | 1.03 | 1.00 |

The benchmark reports rounded per-language ratios; its overall value uses the
unrounded geometric mean.

## Validation

- The combined change, including the updated main, passes all 4,712 differential
  gate inputs with incremental seed 7 and queries enabled, including complete
  trees and progress-callback counts.
- Regression tests compare committed and general reductions, including zero
  children, trailing/internal extras, error-state nonterminal extras, negative
  action precedence, and fragile nodes.
- Single-version condensation tests compare both paths' costs and saved error
  baselines for ordinary and error states, null and non-null links, and baselines
  above/below the current node count.
- Existing in-place stack-pop tests still compare ownership, scanner tokens,
  summaries, free-list order, and refusal of shared/branched prefixes.
- `cargo test -p ts_port`: 185 unit tests and both integration tests pass.
- `cargo clippy --workspace --all-targets -- -D warnings`: clean.

The host's full kernel/fresh-repository acceptance suite remains a separate
merge-time check; the local differential run above is the gate set.
