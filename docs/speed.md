# Speed

Parse time of the port relative to C tree-sitter 0.25.10 with the same grammar versions
(below 1 is faster). The geometric mean over all 54 grammars is 0.74. Measured again on
repositories that no agent saw while porting, the 25 grammars those repositories cover
come out at 0.72.

| Grammar | Inputs | Port / C | Never-seen repositories | First use |
|---|---|---:|---:|---:|
| `bash` | 300 files, 1.3 MB | 0.67 | 0.68 | 37 ms |
| `c` | 300 files, 5.9 MB | 0.64 | 0.60 | 17 ms |
| `c` (Linux kernel) | 300 files, 3.8 MB | 0.66 |  |  |
| `c_sharp` | 300 files, 2.0 MB | 0.68 | 0.65 | 178 ms |
| `cmake` | 300 files, 0.2 MB | 1.02 |  | 1 ms |
| `cpp` | 300 files, 2.8 MB | 0.66 | 0.63 | 112 ms |
| `css` | 300 files, 2.3 MB | 0.68 | 0.64 | 1 ms |
| `cuda` | 300 files, 4.3 MB | 0.68 |  | 236 ms |
| `dart` | 300 files, 3.5 MB | 0.85 |  | 41 ms |
| `dtd` | 261 files, 0.3 MB | 0.94 |  | 0.3 ms |
| `elixir` | 300 files, 2.5 MB | 0.81 |  | 48 ms |
| `elm` | 300 files, 1.9 MB | 0.63 |  | 4 ms |
| `embedded_template` | 300 files, 0.2 MB | 1.08 |  | 0.1 ms |
| `erlang` | 300 files, 5.3 MB | 0.64 |  | 12 ms |
| `fortran` | 300 files, 3.3 MB | 0.74 |  | 212 ms |
| `fsharp` | 300 files, 2.2 MB | 0.79 |  | 379 ms |
| `glsl` | 300 files, 0.9 MB | 0.59 |  | 25 ms |
| `go` | 300 files, 3.3 MB | 0.81 | 0.79 | 5 ms |
| `haskell` | 300 files, 1.1 MB | 0.88 |  | 132 ms |
| `hcl` | 300 files, 0.1 MB | 0.85 |  | 2 ms |
| `html` | 300 files, 0.9 MB | 0.59 | 0.57 | 0.2 ms |
| `java` | 300 files, 1.6 MB | 0.67 | 0.69 | 12 ms |
| `javascript` | 300 files, 0.2 MB | 0.68 | 0.51 | 12 ms |
| `json` | 300 files, 4.3 MB | 0.86 | 0.85 | 0.1 ms |
| `julia` | 300 files, 3.5 MB | 0.72 |  | 196 ms |
| `kotlin` | 300 files, 0.9 MB | 0.65 | 0.67 | 111 ms |
| `lua` | 300 files, 2.3 MB | 0.65 | 0.61 | 1 ms |
| `make` | 300 files, 0.4 MB | 0.70 |  | 3 ms |
| `markdown` | 300 files, 1.1 MB | 1.03 | 0.99 | 7 ms |
| `markdown_inline` | 300 files, 0.6 MB | 1.10 | 1.22 | 7 ms |
| `objc` | 300 files, 4.7 MB | 0.66 |  | 179 ms |
| `ocaml` | 300 files, 1.3 MB | 0.65 |  | 156 ms |
| `ocaml_interface` | 300 files, 1.2 MB | 0.61 |  | 134 ms |
| `odin` | 300 files, 3.8 MB | 0.76 |  | 75 ms |
| `perl` | 300 files, 1.5 MB | 0.75 |  | 161 ms |
| `php` | 300 files, 1.8 MB | 0.70 | 0.65 | 33 ms |
| `php_only` | 300 files, 1.7 MB | 0.67 | 0.98 | 32 ms |
| `powershell` | 300 files, 2.2 MB | 0.63 |  | 21 ms |
| `proto` | 300 files, 0.8 MB | 0.73 |  | 0.6 ms |
| `python` | 300 files, 4.6 MB | 0.71 | 0.70 | 13 ms |
| `r` | 300 files, 1.1 MB | 0.71 |  | 14 ms |
| `ruby` | 300 files, 1.6 MB | 0.68 | 0.64 | 71 ms |
| `rust` | 300 files, 0.7 MB | 0.70 | 0.70 | 35 ms |
| `scala` | 300 files, 0.4 MB | 0.73 |  | 117 ms |
| `solidity` | 300 files, 0.9 MB | 0.78 |  | 15 ms |
| `sql` | 300 files, 0.1 MB | 0.48 |  | 76 ms |
| `svelte` | 300 files, 0.1 MB | 0.73 |  | 0.6 ms |
| `swift` | 300 files, 2.0 MB | 0.91 | 0.87 | 129 ms |
| `toml` | 300 files, 0.2 MB | 0.86 | 0.88 | 0.3 ms |
| `tsx` | 300 files, 1.0 MB | 0.71 | 0.72 | 47 ms |
| `typescript` | 300 files, 2.5 MB | 0.72 | 0.71 | 47 ms |
| `verilog` | 300 files, 0.4 MB | 0.64 |  | 619 ms |
| `xml` | 300 files, 1.6 MB | 1.39 |  | 0.5 ms |
| `yaml` | 300 files, 0.3 MB | 0.65 | 0.67 | 4 ms |
| `zig` | 300 files, 7.2 MB | 0.71 | 0.70 | 24 ms |

The port is faster on 49 grammars. XML, Markdown and its inline grammar, embedded
templates and CMake are slower.

## Method

- **Machine:** AMD EPYC 9354P (32 cores, 64 threads), 755 GB of memory, Ubuntu 24.04 on
  Linux 6.8. Each job ran pinned to its own physical core with the core's second thread
  idle. Other work ran on the remaining cores at the time.
- **Builds:** both sides in Cargo's release profile, at `opt-level = 3` with 16 codegen
  units and no LTO. GCC 13.3 compiled the C library and grammars through the `cc` crate,
  and Rust 1.99 compiled the port.
- **Inputs:** up to 300 files per grammar, picked by a hash of the path, from the sets
  the correctness checks used: the grammars' test corpora, a corpus of real projects per
  language and the Linux 6.17 sources for C. The never-seen column uses the same number
  of files from repositories cloned for the final verification.
- **What was timed:** parsing alone, with a fresh parser per file created outside the
  timed part. Each file is parsed five times and its fastest parse counts. Each grammar
  runs three rounds on one core, with C first in two of them, and a file's time is its
  fastest round. A grammar's ratio is the port's total time over C's for the files both
  parsed. The overall figure is the geometric mean of the grammars' ratios, with the
  kernel row left out because it repeats the `c` grammar.
- **Noise:** a grammar's three per-round ratios differ by at most 4.4%, and by at most
  6.8% on the never-seen repositories.

## Memory and startup

The grammar tables are decoded from a compact blob the first time a grammar is used,
where C links them as static data. The **First use** column is that one-time cost: the
extra wall time of a grammar's first parse over its second, the median of five runs. It
ranges from 0.1 ms (JSON) to 619 ms (Verilog), with a median of 24 ms and 3.8 s for all
54 grammars. In C it stays under 0.2 ms.

Peak memory of a process that parses one small file with each of the 54 grammars is
224 MB for the port and 45 MB for C. With one grammar (JSON) it is 3.5 MB for the port
and 4.0 MB for C.
