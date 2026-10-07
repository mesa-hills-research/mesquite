# Cursor optimization handoff (worker-6)

The operator stopped the run while this candidate was being measured. Production
changes and this handoff are committed; no files outside `runtime/` were changed.

## Candidate

Applied the earlier cursor candidate as commits `b41c613` and `9da3a0b`
(cherry-picks of `1ef0fa2` and `bf82e52`). Main was `84ad1eb`.

Forward cursor movement previously advanced its temporary child iterator even
when immediately returning that child. The candidate seeks the selected edge
without reading its successor's padding or computing unused successor state.
Fields are resolved only for selected edges. Exhausted sibling searches stop
before iterator setup, successful sibling moves replace the existing stack slot,
and leaf rejection and small node readouts inline across the public API boundary.
Reverse iteration remains separate to preserve C's eight-bit sentinel and
structural-index behavior. No unsafe code, public signatures, or tree layouts
were introduced or changed.

## Checks completed during this run

- `cargo test -p ts_port --lib`: 215 passed.
- `cargo clippy -p ts_port --all-targets --all-features -- -D warnings`: passed.
- `git diff --check`: passed.
- All 969 pinned benchmark inputs had identical walk hashes, node counts, root
  error flags, and progress records in three runs of each binary.
- The original candidate had exhaustive oracle validation in its earlier run;
  this run did **not** repeat the full differential, incremental, or query gates.

## Measurements and limitations

Two supported `run_oracle(inputs="benchmark")` calls (unchanged main, then
candidate) returned only candidate nanoseconds, not the advertised alternating
main/candidate comparison. The active host `bench-baseline.json` was absent.
No host state was modified and no archived baseline was installed.

For a local screening comparison, saved both binaries directly from
`targets/worker-6-oracle/release/ts_oracle_port` after their supported builds,
with matching host build features. SHA-256:

- main: `ffaf67a1a64b0f69515cd55345e67a173525e26096a2067b5d8d4ee689c59764`
- candidate: `6f13adf1d16e9fc9a85fdb85fd369834388923c5b80242b329d50286ac230ac2`

Three alternating pairs, pinned to CPU 6, using all 969 pinned files with
`--repeat 3 --workload parse_walk`, produced geometric-mean candidate/main ratios
of 0.96682 parse, 0.60350 walk, and 0.91143 combined. **These are not admission
measurements:** per-language parse ratios ranged from 0.635 to 1.517 despite no
parser changes, showing substantial contention/drift. Do not claim the apparent
39.7% walk or 8.9% combined reduction as a verified beyond-noise speedup.

A separate `perf stat -e instructions:u` single-repeat parse/walk run counted
35,590,176,288 user instructions for main and 34,421,102,846 for the candidate
(3.285% fewer). This covers the whole command, not just the timed regions, and is
not a timing admission result.

Local binaries, scripts, and raw records are in `/tmp/tree-cursor-worker6/` for
this session. No temporary instrumentation remains in production.

## Remaining work

Have the host establish its current-main baseline and run the controlled
alternating benchmark, including per-language parse/walk noise checks. Rerun all
oracle inputs with incremental/query checks on this branch. The requested
port/C <= 0.50 goal and performance admission have not been established. Do not
submit repeated documentation-only changes as a replacement for these checks.
