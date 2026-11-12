# Speed

Parse time of Mesquite relative to C tree-sitter 0.25.10 with the same grammar versions
(below 1 is faster). C's speed depends on the compiler that builds it, so each column
names one. The geometric mean over the 58 grammars is 0.71 against C built with GCC 13.4,
0.72 against Clang 21.1 and 0.53 against GCC 15.2.

| Grammar | Inputs | GCC 15.2 | GCC 13.4 | Clang 21.1 |
|---|---|---:|---:|---:|
| `astro` | 300 files, 0.3 MB | 0.74 | 0.73 | 0.72 |
| `bash` | 300 files, 1.0 MB | 0.36 | 0.69 | 0.70 |
| `c` | 300 files, 2.2 MB | 0.39 | 0.58 | 0.68 |
| `c_sharp` | 300 files, 4.9 MB | 0.36 | 0.65 | 0.71 |
| `cmake` | 300 files, 0.3 MB | 0.76 | 0.77 | 0.70 |
| `cpp` | 300 files, 4.1 MB | 0.34 | 0.72 | 0.73 |
| `css` | 300 files, 1.4 MB | 0.32 | 0.62 | 0.61 |
| `cuda` | 300 files, 4.3 MB | 0.35 | 0.68 | 0.83 |
| `dart` | 300 files, 3.6 MB | 0.84 | 0.80 | 0.92 |
| `diff` | 300 files, 5.3 MB | 0.75 | 0.76 | 0.71 |
| `dtd` | 49 files, 0.3 MB | 0.82 | 0.83 | 0.78 |
| `elixir` | 300 files, 4.2 MB | 0.34 | 0.79 | 0.75 |
| `elm` | 231 files, 3.4 MB | 0.62 | 0.60 | 0.67 |
| `embedded_template` | 300 files, 0.2 MB | 0.86 | 0.86 | 0.79 |
| `erlang` | 300 files, 5.1 MB | 0.45 | 0.63 | 0.61 |
| `fortran` | 300 files, 4.2 MB | 0.13 | 0.74 | 0.73 |
| `fsharp` | 300 files, 1.7 MB | 0.48 | 0.75 | 0.81 |
| `glsl` | 300 files, 0.4 MB | 0.34 | 0.64 | 0.69 |
| `go` | 300 files, 3.6 MB | 0.69 | 0.66 | 0.80 |
| `graphql` | 300 files, 0.04 MB | 0.29 | 0.66 | 0.61 |
| `haskell` | 300 files, 1.9 MB | 0.90 | 0.88 | 0.93 |
| `hcl` | 300 files, 0.2 MB | 0.85 | 0.86 | 0.72 |
| `html` | 300 files, 6.7 MB | 0.57 | 0.58 | 0.54 |
| `java` | 300 files, 1.7 MB | 0.68 | 0.66 | 0.79 |
| `javascript` | 300 files, 0.3 MB | 0.72 | 0.72 | 0.75 |
| `jsdoc` | 300 files, 0.05 MB | 0.92 | 0.93 | 0.89 |
| `json` | 300 files, 3.1 MB | 0.72 | 0.74 | 0.62 |
| `julia` | 300 files, 5.5 MB | 0.47 | 0.67 | 0.67 |
| `kotlin` | 300 files, 1.2 MB | 0.28 | 0.70 | 0.69 |
| `lua` | 300 files, 5.6 MB | 0.62 | 0.61 | 0.66 |
| `make` | 300 files, 0.5 MB | 0.76 | 0.74 | 0.78 |
| `markdown` | 300 files, 4.1 MB | 0.66 | 0.74 | 0.66 |
| `markdown_inline` | 300 files, 4.1 MB | 0.60 | 1.09 | 1.06 |
| `objc` | 300 files, 3.5 MB | 0.27 | 0.69 | 0.77 |
| `ocaml` | 300 files, 1.8 MB | 0.38 | 0.61 | 0.60 |
| `ocaml_interface` | 300 files, 1.3 MB | 0.44 | 0.59 | 0.62 |
| `odin` | 300 files, 4.0 MB | 0.71 | 0.68 | 0.72 |
| `perl` | 300 files, 2.4 MB | 0.42 | 0.73 | 0.71 |
| `php` | 300 files, 1.8 MB | 0.65 | 0.64 | 0.68 |
| `php_only` | 300 files, 1.8 MB | 0.67 | 0.65 | 0.71 |
| `powershell` | 300 files, 2.3 MB | 0.11 | 0.63 | 0.62 |
| `proto` | 300 files, 1.0 MB | 0.33 | 0.71 | 0.69 |
| `python` | 300 files, 2.2 MB | 0.73 | 0.71 | 0.75 |
| `r` | 300 files, 1.8 MB | 0.71 | 0.68 | 0.72 |
| `ruby` | 300 files, 1.4 MB | 0.49 | 0.62 | 0.64 |
| `rust` | 300 files, 1.7 MB | 0.70 | 0.68 | 0.71 |
| `scala` | 300 files, 0.5 MB | 0.51 | 0.75 | 0.77 |
| `solidity` | 300 files, 1.4 MB | 0.80 | 0.77 | 0.93 |
| `sql` | 300 files, 0.2 MB | 0.67 | 0.61 | 0.77 |
| `svelte` | 300 files, 0.06 MB | 0.86 | 0.85 | 0.84 |
| `swift` | 300 files, 4.5 MB | 0.18 | 0.92 | 0.89 |
| `toml` | 300 files, 0.6 MB | 0.69 | 0.69 | 0.64 |
| `tsx` | 300 files, 1.7 MB | 0.73 | 0.72 | 0.75 |
| `typescript` | 300 files, 2.8 MB | 0.71 | 0.71 | 0.75 |
| `verilog` | 300 files, 2.0 MB | 0.58 | 0.65 | 0.71 |
| `xml` | 300 files, 0.6 MB | 0.74 | 0.76 | 0.62 |
| `yaml` | 300 files, 1.9 MB | 0.64 | 0.65 | 0.62 |
| `zig` | 300 files, 5.2 MB | 0.69 | 0.69 | 0.69 |
| `c` (Linux kernel) | 300 files, 5.7 MB | 0.33 | 0.62 | 0.68 |

Mesquite is faster than all three builds of C on 57 grammars. On Markdown's inline
grammar it takes 1.09 times as long as C built with GCC 13.4, and 1.06 times Clang's.

## Method

The machine is an AMD Ryzen 7 9800X3D (8 cores, 96 MB of L3 cache) with Ubuntu 26.04 and
Linux 7.0, running nothing else, with each process pinned to its own core. Both libraries
are built with Cargo's release profile (`opt-level = 3`, no LTO) and Rust 1.97.1. The C
library and grammars are compiled the way their crates compile them, at `-O3` with
assertions on, by GCC 15.2 (Ubuntu's default), GCC 13.4 or Clang 21.1. With `-DNDEBUG`,
GCC 15.2's build takes 0.98 times as long.

Each grammar has up to 300 files, picked by a hash of the path, from its test corpus and
from real projects in its language at fixed commits. The kernel row takes 300 files of the
Linux 6.17 sources, and the JSDoc inputs are doc comments from JavaScript and TypeScript
files.

Only parsing is timed, with a fresh parser per file. Each round parses every file five
times with each library, in a random order of the libraries, from a fresh copy of each
binary. A grammar's ratio is the median over 20 rounds of Mesquite's time over C's, each
file counting its mean parse time. The overall figure is the median over rounds of the
geometric mean of the 58 ratios, without the kernel row. Its 95% interval is within 0.003
of it, and C timed against a copy of itself gives 1.00.

## Memory and startup

The large grammar tables are read in place from the binary, as C reads its static arrays,
so using a grammar for the first time decodes only its small tables. The extra time of a
grammar's first parse over its second is at most 0.10 ms for every grammar, with a median
of 0.04 ms. In C it is at most 0.06 ms, with a median of 0.03 ms.

A process that parses one small file with each of the 58 grammars peaks at 55 MB with
Mesquite and at 48 to 66 MB with C, depending on its compiler. Nearly all of it is the
pages of the binary that the parses read, and anonymous memory is 1.4 MB with Mesquite and
0.6 MB with C. With one grammar (JSON) both use 4 to 5 MB.

A parsed tree takes 1.0 to 1.2 times the memory of C's. For example, a 1.8 MB JSON file
makes a tree of 30 MB with Mesquite and 25 MB with C, and a 1.7 MB C file one of 133 MB
and 115 MB.

## Binary size

The parse tables of a grammar's large states are stored without their zeros, about four
in five of their entries, where C keeps them as dense arrays. A binary that links all 58
grammars is 60 MB with Mesquite and 118 MB with C, stripped. Reading the compact tables
costs about 2% of parse time against dense ones.

## Reparsing after an append

Text appended at the end of a file, as an editor or a stream adds it, is parsed again
from the old tree. The time this takes grows with the file for both libraries.

| Input | C, GCC 15.2 | C, GCC 13.4 | C, Clang 21.1 | Mesquite |
|---|---:|---:|---:|---:|
| Python, 25 KB | 0.67 ms | 0.68 ms | 0.70 ms | 0.58 ms |
| Python, 100 KB | 2.47 ms | 2.57 ms | 2.62 ms | 2.14 ms |
| Python, 400 KB | 9.53 ms | 10.05 ms | 10.20 ms | 8.42 ms |
| JSON, 25 KB | 0.20 ms | 0.21 ms | 0.12 ms | 0.09 ms |
| JSON, 100 KB | 0.75 ms | 0.77 ms | 0.43 ms | 0.35 ms |
| JSON, 400 KB | 3.08 ms | 3.09 ms | 1.83 ms | 1.53 ms |

Each time is the mean of 750 reparses after 4-byte appends to generated code (one
assignment per line, or an array of small objects), each including the edit and freeing
the old tree, the median of 12 processes per library, on the machine above.

## Comparing on your own files

Cargo can rename dependencies, so one program can use both libraries:

```toml
[dependencies]
c_tree_sitter = { package = "tree-sitter", version = "=0.25.10" }
c_rust = { package = "tree-sitter-rust", version = "=0.24.2" }
port_tree_sitter = { package = "mesquite", version = "0.26.1" }
port_rust = { package = "mesquite_rust", version = "0.2.0" }
```

```rust
use std::time::{Duration, Instant};

fn main() {
    let (mut c, mut port) = (Duration::ZERO, Duration::ZERO);
    for path in std::env::args().skip(1) {
        let source = std::fs::read(path).unwrap();

        let mut parser = c_tree_sitter::Parser::new();
        parser.set_language(&c_rust::LANGUAGE.into()).unwrap();
        let start = Instant::now();
        parser.parse(&source, None).unwrap();
        c += start.elapsed();

        let mut parser = port_tree_sitter::Parser::new();
        parser.set_language(&port_rust::LANGUAGE.into()).unwrap();
        let start = Instant::now();
        parser.parse(&source, None).unwrap();
        port += start.elapsed();
    }
    println!("port/C {:.2}", port.as_secs_f64() / c.as_secs_f64());
}
```

Build it with `cargo build --release` (the C side needs a C compiler) and pass it the
files to parse. Pin it to one core (`taskset -c 2 ...`) and run it a few times for
steady numbers. C's time depends on the compiler that builds it (see the table above), and
`CC=clang cargo build --release` builds the C side with another one. Other grammars work
the same way, with the crates and constants listed in [grammars.md](grammars.md).
