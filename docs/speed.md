# Speed

Parse time of the port relative to C tree-sitter 0.25.10 with the same grammar versions
(below 1 is faster).

<!-- TODO: fill in from the box timings: every grammar, the inputs, the machine and the
build settings. -->

| Grammar | Inputs | Port / C |
|---|---|---:|
| _TBD_ | _TBD_ | _TBD_ |

## Method

_TBD_: the machine and its cores, the inputs per language, the build settings of both
sides (optimization level, LTO, codegen units), what was timed (parsing alone, or
parsing and a full tree walk) and the run-to-run noise.

## Memory and startup

The grammar tables are decoded from a compact blob on first use, where C has static
data. That costs a fixed amount of time and memory the first time a grammar is used.
<!-- TODO: measured numbers. -->
