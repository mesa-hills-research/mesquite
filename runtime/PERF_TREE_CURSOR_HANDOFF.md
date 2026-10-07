# Cursor optimization results (worker-6)

## Candidate

Applied the earlier cursor candidate as commits `b41c613` and `9da3a0b`
(cherry-picks of `1ef0fa2` and `bf82e52`), based on main `84ad1eb`.

Forward cursor movement previously advanced its temporary child iterator even
when immediately returning that child. The candidate seeks the selected edge
without reading its successor's padding or computing unused successor state.
Fields are resolved only for selected edges. Exhausted sibling searches stop
before iterator setup, successful sibling moves replace the existing stack slot,
and leaf rejection and small node readouts inline across the public API boundary.
Reverse iteration remains separate to preserve C's eight-bit sentinel and
structural-index behavior. No unsafe code, public signatures, or tree layouts
were introduced or changed. All branch changes are under `runtime/`.

## Validation after the operator restarted the run

- `cargo test -p ts_port --all-features`: 215 unit tests and both integration
  tests passed.
- `cargo clippy -p ts_port --all-targets --all-features -- -D warnings`: passed.
- `git diff --check`: passed.
- All 175,323 oracle inputs passed with zero failures, panics, crashes, timeouts,
  or skipped inputs: 11,816 gate inputs (incremental=7, queries enabled) and
  163,507 kernel/fresh inputs (queries enabled, incremental disabled by the
  host's all-input mode).
- All 969 pinned benchmark inputs had identical walk hashes, node counts, root
  error flags, and progress records in three local runs of each binary.

## Supported alternating benchmark

The missing host baseline was present after restart, at main `84ad1eb`. Three
successive `run_oracle(inputs="benchmark")` calls each timed main and candidate
alternately over all 25 languages, producing these overall port/C ratios:

| Comparison | Main | Candidate |
|---|---:|---:|
| 1 | 0.725 | 0.687 |
| 2 | 0.727 | 0.686 |
| 3 | 0.726 | 0.688 |

The median of these reported overall ratios indicates approximately **5.4% less
combined parse/walk time**, exceeding the reported 2.0% overall noise. Taking the
per-language medians of the displayed walk ratios gives approximately **36.1%
less walk time** geometrically across the languages. These percentages are
approximate because the tool displays rounded ratios. Every language's combined
and walk time improved in each comparison. Parse time is largely unchanged;
individual runs fluctuate, so the host's final median-of-three admission check
must still judge each parse/walk component against its own noise threshold.
These three preview comparisons do not themselves assert final host admission.

The requested combined port/C <= 0.50 goal has **not** been reached. Cursor work
speeds up traversal, not the parser, and does not solve the remaining parse cost.

## Supplemental local evidence (not admission measurements)

Binaries were copied directly from the supported oracle builds with matching
host features. SHA-256:

- main: `ffaf67a1a64b0f69515cd55345e67a173525e26096a2067b5d8d4ee689c59764`
- candidate: `6f13adf1d16e9fc9a85fdb85fd369834388923c5b80242b329d50286ac230ac2`

`perf stat -e instructions:u`, all 969 files, single-repeat parse/walk, counted
35,590,176,288 user instructions for main and 34,421,102,846 for the candidate
(3.285% fewer). This covers the whole command, not just the timed regions.
A five-repeat candidate `perf record` profile attributed 2.41% self time to
next-sibling traversal and 2.36% to nonempty first-child traversal; profiling the
whole command is not equivalent to the host's separate parse/walk timers.

The initial local timings before the stop had severe contention/drift and are
superseded by the supported alternating comparisons above. No host baseline or
other host state was edited. Temporary local binaries, scripts, and profiles are
in `/tmp/tree-cursor-worker6/`; no instrumentation remains in production.
