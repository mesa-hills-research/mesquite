# Speed

Parse time of the port relative to C tree-sitter 0.25.10 with the same grammar versions
(below 1 is faster). The geometric mean over all 58 grammars is 0.73.

| Grammar | Inputs | Port / C | First use |
|---|---|---:|---:|
| `astro` | 300 files, 0.2 MB | 0.67 | 0.2 ms |
| `bash` | 300 files, 1.3 MB | 0.67 | 37 ms |
| `c` | 300 files, 5.9 MB | 0.64 | 17 ms |
| `c` (Linux kernel) | 300 files, 3.8 MB | 0.66 |  |
| `c_sharp` | 300 files, 2.0 MB | 0.68 | 178 ms |
| `cmake` | 300 files, 0.2 MB | 1.02 | 1 ms |
| `cpp` | 300 files, 2.8 MB | 0.66 | 112 ms |
| `css` | 300 files, 2.3 MB | 0.68 | 1 ms |
| `cuda` | 300 files, 4.3 MB | 0.68 | 236 ms |
| `dart` | 300 files, 3.5 MB | 0.85 | 41 ms |
| `diff` | 300 files, 13.1 MB | 0.93 | 0.1 ms |
| `dtd` | 261 files, 0.3 MB | 0.94 | 0.3 ms |
| `elixir` | 300 files, 2.5 MB | 0.81 | 48 ms |
| `elm` | 300 files, 1.9 MB | 0.63 | 4 ms |
| `embedded_template` | 300 files, 0.2 MB | 1.08 | 0.1 ms |
| `erlang` | 300 files, 5.3 MB | 0.64 | 12 ms |
| `fortran` | 300 files, 3.3 MB | 0.74 | 212 ms |
| `fsharp` | 300 files, 2.2 MB | 0.79 | 379 ms |
| `glsl` | 300 files, 0.9 MB | 0.59 | 25 ms |
| `go` | 300 files, 3.3 MB | 0.81 | 5 ms |
| `graphql` | 300 files, 3.3 MB | 0.40 | 0.5 ms |
| `haskell` | 300 files, 1.1 MB | 0.88 | 132 ms |
| `hcl` | 300 files, 0.1 MB | 0.85 | 2 ms |
| `html` | 300 files, 0.9 MB | 0.59 | 0.2 ms |
| `java` | 300 files, 1.6 MB | 0.67 | 12 ms |
| `javascript` | 300 files, 0.2 MB | 0.68 | 12 ms |
| `jsdoc` | 300 files, 0.05 MB | 0.86 | 0.1 ms |
| `json` | 300 files, 4.3 MB | 0.86 | 0.1 ms |
| `julia` | 300 files, 3.5 MB | 0.72 | 196 ms |
| `kotlin` | 300 files, 0.9 MB | 0.65 | 111 ms |
| `lua` | 300 files, 2.3 MB | 0.65 | 1 ms |
| `make` | 300 files, 0.4 MB | 0.70 | 3 ms |
| `markdown` | 300 files, 1.1 MB | 1.03 | 7 ms |
| `markdown_inline` | 300 files, 0.6 MB | 1.10 | 7 ms |
| `objc` | 300 files, 4.7 MB | 0.66 | 179 ms |
| `ocaml` | 300 files, 1.3 MB | 0.65 | 156 ms |
| `ocaml_interface` | 300 files, 1.2 MB | 0.61 | 134 ms |
| `odin` | 300 files, 3.8 MB | 0.76 | 75 ms |
| `perl` | 300 files, 1.5 MB | 0.75 | 161 ms |
| `php` | 300 files, 1.8 MB | 0.70 | 33 ms |
| `php_only` | 300 files, 1.7 MB | 0.67 | 32 ms |
| `powershell` | 300 files, 2.2 MB | 0.63 | 21 ms |
| `proto` | 300 files, 0.8 MB | 0.73 | 0.6 ms |
| `python` | 300 files, 4.6 MB | 0.71 | 13 ms |
| `r` | 300 files, 1.1 MB | 0.71 | 14 ms |
| `ruby` | 300 files, 1.6 MB | 0.68 | 71 ms |
| `rust` | 300 files, 0.7 MB | 0.70 | 35 ms |
| `scala` | 300 files, 0.4 MB | 0.73 | 117 ms |
| `solidity` | 300 files, 0.9 MB | 0.78 | 15 ms |
| `sql` | 300 files, 0.1 MB | 0.48 | 76 ms |
| `svelte` | 300 files, 0.1 MB | 0.73 | 0.6 ms |
| `swift` | 300 files, 2.0 MB | 0.91 | 129 ms |
| `toml` | 300 files, 0.2 MB | 0.86 | 0.3 ms |
| `tsx` | 300 files, 1.0 MB | 0.71 | 47 ms |
| `typescript` | 300 files, 2.5 MB | 0.72 | 47 ms |
| `verilog` | 300 files, 0.4 MB | 0.64 | 619 ms |
| `xml` | 300 files, 1.6 MB | 1.39 | 0.5 ms |
| `yaml` | 300 files, 0.3 MB | 0.65 | 4 ms |
| `zig` | 300 files, 7.2 MB | 0.71 | 24 ms |

The port is faster on 53 grammars. XML, Markdown and its inline grammar, embedded
templates and CMake are slower.

## Method

- **Machine:** AMD EPYC 9354P with Ubuntu 24.04, each job pinned to its own physical
  core. Astro, diff, GraphQL and JSDoc were timed on an Intel Core i9-13980HX (a 6-core
  virtual machine with Ubuntu 26.04), pinned to one core.
- **Builds:** Cargo's release profile (`opt-level = 3`, no LTO) for both. GCC 13.3
  compiled the C library and grammars, and Rust 1.99 compiled the port (GCC 15.2 and
  Rust 1.97 for Astro, diff, GraphQL and JSDoc).
- **Inputs:** up to 300 files per grammar from the grammars' test corpora and real
  projects in each language, and the Linux 6.17 sources for the kernel row. The JSDoc
  inputs are doc comments from JavaScript and TypeScript files.
- **Timing:** parsing alone, with a fresh parser per file. A file's time is its fastest
  of 15 parses, and a grammar's ratio is the port's total time over C's. The overall
  figure is the geometric mean of the grammars' ratios, without the kernel row.

## Memory and startup

The grammar tables are decoded from a compact blob the first time a grammar is used,
where C links them as static data. The **First use** column is that one-time cost: the
extra wall time of a grammar's first parse over its second, the median of five runs. It
ranges from 0.1 ms (JSON) to 619 ms (Verilog), with a median of 19 ms and 3.8 s for all
58 grammars. In C it stays under 0.2 ms.

Peak memory of a process that parses one small file with each of 54 grammars (all but
Astro, diff, GraphQL and JSDoc) is 224 MB for the port and 45 MB for C. With one grammar
(JSON) it is 3.5 MB for the port and 4.0 MB for C.

## Comparing on your own files

Cargo can rename dependencies, so one program can use both libraries:

```toml
[dependencies]
c_tree_sitter = { package = "tree-sitter", version = "=0.25.10" }
c_rust = { package = "tree-sitter-rust", version = "=0.24.2" }
port_tree_sitter = { package = "mesquite", git = "https://github.com/mesa-hills-research/mesquite" }
port_rust = { package = "mesquite_rust", git = "https://github.com/mesa-hills-research/mesquite" }
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
steady numbers. Other grammars work the same way, with the crates and constants listed
in [grammars.md](grammars.md).
