# Verification

A differential oracle ran the port and the C library (tree-sitter 0.25.10, with the
same grammar versions) on the same inputs and compared four things:

- the syntax trees
- incremental parsing: an edit, then a reparse with the edited old tree
- query results
- the calls to the progress callback and the positions they report

They were identical on 727,612 inputs across the 54 grammars:

- the gate's sets, used during development: the grammars' test corpora, a development
  corpus per language and fuzzed variants of both
- the Linux kernel sources (C)
- a further set of open-source repositories
- a separate fuzzing run with its own fuzzer and seed

The comparison ran on an overflow-checked build and on a normal build.

| Set | Inputs | Identical |
|---|---:|---:|
| Development sets: the gate's sets, real projects per language and the Linux kernel | 267,285 | 267,283 |
| Further open-source repositories | 166,254 | 166,254 |
| Earlier development and safety sets (25 grammars) | 179,033 | 179,033 |
| Perl and CMake sets, at two incremental seeds | 15,043 | 15,043 |
| Separate fuzzing run | 100,000 | 99,999 |

The 3 inputs not counted as identical are ones the C library gave no result for, so there
was nothing to compare. Every set ran on the normal build and, except Perl and CMake, on
the overflow-checked build too, all at the final commit.

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

The port also fixes inputs on which C never returns or writes past a buffer:

- **Kotlin scanner:** a file that ends right after an annotation (`val a` then `@` on
  the last line) parses. C's scanner loops forever at the end of the input.
