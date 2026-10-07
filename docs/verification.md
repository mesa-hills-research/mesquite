# Verification

A differential oracle ran the port and the C library (tree-sitter 0.25.10, with the
same grammar versions) on the same inputs and compared four things:

- the syntax trees
- incremental parsing: an edit, then a reparse with the edited old tree
- query results
- the calls to the progress callback and the positions they report

They were identical on more than 600,000 inputs across the 54 grammars:

- the gate's sets, used during development: the grammars' test corpora, a development
  corpus per language and fuzzed variants of both
- the Linux kernel sources (C)
- repositories that were never seen during development
- a separate fuzzing run with its own fuzzer and seed

The comparison ran on an overflow-checked build and on a normal build.

<!-- TODO: the per-set input counts and the write-up of the run. -->

`cargo test --workspace` runs the port's own tests.

## Where the port and C differ on purpose

The comparison turned up bugs in C code. Where identical trees need C's behaviour, the
port keeps it. Otherwise both sides of the comparison carried the same fixes:

- **Runtime:** C's `ts_subtree_repeat_depth` reads a field of external-token leaves that
  holds scanner state there. The port reads the same bytes, so its trees match C's.
- **Perl scanner:** resetting the scanner (deserializing empty state) also clears
  `recovery_emitted`, and serialized state carries no uninitialized padding. Without
  these fixes the C scanner's incremental results depend on earlier parses.
- **CMake scanner:** a new scanner starts zeroed, and a reset also clears the pending
  token, for the same reason.
