# Parser profiling — stopped before implementation

Status: **investigation only; not an optimization/admission candidate**.
The operator stopped this run before a production change was implemented.
Production sources remain identical to main `84ad1eb`; merging main reported
already up to date. No unsafe code or host-owned files were changed.

## Measurements

The supported `run_oracle(inputs="benchmark")` command was run on the unchanged
worktree. It returned candidate nanoseconds for all 25 languages, not alternating
main/candidate ratios. `/work/ts-port-run5/oracle/bench-baseline.json` was still
absent. No baseline was created, replaced, or promoted from an archive.
Consequently no gain, per-language non-regression, or admission result is claimed.

A separate diagnostic profile used the oracle-built binary (including its
`queries` feature), not a differently configured manual build:

```text
binary: /tmp/worker3-perf/main
SHA-256: fd73efb5140e8d90b96715c56e977616a196ce0f3bd3415027a7914223bfeeae
perf record -q -e cycles:u -o /tmp/worker3-perf/base.data -- \
  taskset -c 8 /tmp/worker3-perf/main bench \
  /work/ts-port-run5/oracle/profile/bench.list --repeat 10 --workload parse_walk
```

The 969-file profile reported these whole-command self-time shares:

| Symbol | Self time |
| --- | ---: |
| `ts_parser_parse_with_options` | 9.85% |
| `ts_stack_reduce_many_for_version` (including inlined constructor) | 7.13% |
| `ts_parser__reduce` | 6.12% |
| `ts_parser__balance_subtree` | 2.87% |
| `ts_parser__reduce_general` | 2.45% |

These are samples, **not** parse-timer speedup measurements. The shell CPU's SMT
sibling was not isolated, so timings from this profile are unsuitable for an
admission claim. Formatting is not among the dominant hot functions in this
parse/walk workload.

Within `ts_parser_parse_with_options`, 17.44% of that symbol's samples landed on
the result-copy sequence following `new_heap_leaf`; other sampled sites include
lex-mode loads and stack arena/head checks. This is a hypothesis to investigate,
not proof that a copy rewrite will improve performance. Earlier lessons already
record a heap-leaf Arc-return experiment, so a successor should consult those
measurements rather than assume this is an unexplored optimization.

Raw profile, annotation, and output are temporarily in `/tmp/worker3-perf/`.
No correctness run was needed for this unchanged-source investigation. A future
implementation still needs runtime tests/clippy, full oracle equivalence
(including progress/incremental/query behavior), and a functioning alternating
main/candidate benchmark with separate parse/walk non-regression checks.
