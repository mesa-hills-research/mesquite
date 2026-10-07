# Parser optimization experiments (worker-2)

Baseline: `cc1559622c7e5ddd98e3838cf549ccc9ef2d1856`.

## Outcome

No production change retained. None of the safe-Rust experiments demonstrated
an improvement beyond the host's 1.9% overall noise threshold. In particular,
the best candidate's 0.762 port/C result was reproduced by an unchanged-source
control. The overall target of 0.60 remains unmet; this report does not claim a
speedup.

## Investigation

Inspected the current parser/stack/subtree implementations, the C runtime, and
the host profile with `perf annotate`. The profile attributes 4.28% of full-dumper
samples to parse-with-options, 3.81% to reduce, 0.85% to balancing, and 0.62% to
condensation. These are not percentages of parse-only time: the dumper also
walks and formats the resulting trees. The annotated reduction routine has an
848-byte stack frame and extensive inlined construction/GLR bookkeeping, making
code separation and argument passing plausible candidates, but measurements
did not substantiate a gain.

Each row below is a separate experiment against the same main baseline, not a
cumulative series. These are screening runs, not three-run acceptance medians.
All measurements used `run_oracle(inputs="benchmark")` and the host's pinned
benchmark. Main's reported geometric mean was 0.765 throughout.

| Experiment | Overall port/C |
|---|---:|
| Const-generic single-version vs multi-version reduction specialization | 0.772 |
| Ordinary reduction inline hint | 0.765 |
| Shared out-of-line reduction header constructor, grouped header inputs, inline reduction entry | 0.775 |
| Group reduction arguments in a borrowed settings struct | 0.764 |
| Outline lex and advance from the large parse routine | 0.787 |
| Force reduction inlining | 0.763 |
| Balance via reverse sibling-prefix iterators and materialize paths only on cancellation | 0.774 |
| Avoid cloning/releasing an unchanged external-scanner token in the token cache | 0.768 |
| Separate sole-action reductions from the GLR action loop, with forced reduction inlining | 0.762 |
| Unmodified-source control after reverting every experiment | 0.762 |

All experimental code was removed rather than retaining additional complexity
or optimization hints based on noise. No unsafe code, host-owned files, tree
semantics, reduction ordering, limits, or progress behavior changed.

## Final validation

Before adding this report, both `HEAD^{tree}` and `main^{tree}` were
`327e3f6768e0dea3c2f1cfcdf653054f0228fc3a`.

- `cargo test -p ts_port --lib --quiet`: 208 passed.
- `cargo clippy -p ts_port --lib --message-format short`: clean.
- Unmodified-source benchmark control: c 0.74, cpp 0.78, go 0.77, java 0.73,
  javascript 0.78, python 0.77, rust 0.76, tsx 0.76, typescript 0.78;
  geometric mean 0.762.
- The differential gate was not rerun: final production sources are exactly
  the baseline's, and the only committed change is this report.
