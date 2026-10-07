# mhr_tree_sitter

A pure, safe Rust port of the [tree-sitter](https://github.com/tree-sitter/tree-sitter)
0.25.10 runtime and 54 tree-sitter grammars.

- **Pure Rust.** No C sources, build scripts or C compiler. The only external
  dependencies are `regex` and `streaming-iterator`.
- **Safe.** Every crate has `#![forbid(unsafe_code)]`.
- **Drop-in names.** The runtime's library is `tree_sitter` and follows tree-sitter's
  Rust API. Each grammar crate's library and constants are named like the upstream
  crate's (`tree_sitter_rust::LANGUAGE`), so most code that uses tree-sitter only needs
  its `Cargo.toml` changed.
- **Same trees as C.** Trees, incremental reparses, query results and progress
  callbacks were identical to the C library's on more than 600,000 inputs.

It was made by an AI agent swarm. Agents translated the C runtime and the grammars'
external scanners unit by unit, and a test gate compared every change with the C
library. <!-- TODO: link the write-up of the run. -->

## Languages

Bash, C, C#, C++, CMake, CSS, CUDA, Dart, Elixir, Elm, Embedded templates (ERB, EJS),
Erlang, F#, Fortran, GLSL, Go, Haskell, HCL, HTML, Java, JavaScript, JSON, Julia,
Kotlin, Lua, Make, Markdown, Objective-C, OCaml, Odin, Perl, PHP, PowerShell, Protocol
Buffers, Python, R, Ruby, Rust, Scala, Solidity, SQL, Svelte, Swift, TOML, TypeScript
and TSX, Verilog, XML and DTD, YAML, Zig.

The crates, their upstream versions and their constants are listed in
[docs/grammars.md](docs/grammars.md).

## Speed

Parse time relative to C tree-sitter 0.25.10 with the same grammars (below 1 is faster).

<!-- TODO: fill in from the box timings. -->

| Inputs | Port / C |
|---|---:|
| All 54 grammars (geometric mean) | _TBD_ |
| C (Linux kernel) | _TBD_ |
| C++ | _TBD_ |
| Rust | _TBD_ |
| Python | _TBD_ |
| TypeScript | _TBD_ |
| JavaScript | _TBD_ |
| Go | _TBD_ |
| Java | _TBD_ |

Every language, the inputs, the machine and the method are in [docs/speed.md](docs/speed.md).

## Installation

The crates are on GitHub for now:

```toml
[dependencies]
mhr_tree_sitter = { git = "https://github.com/mesa-hills-research/mhr_tree_sitter" }
mhr_tree_sitter_rust = { git = "https://github.com/mesa-hills-research/mhr_tree_sitter" }
```

The library names are `tree_sitter` and `tree_sitter_rust`, so `use tree_sitter::...`
lines stay as they are. To keep upstream's dependency names as well, rename the packages:

```toml
[dependencies]
tree-sitter = { package = "mhr_tree_sitter", git = "https://github.com/mesa-hills-research/mhr_tree_sitter" }
tree-sitter-rust = { package = "mhr_tree_sitter_rust", git = "https://github.com/mesa-hills-research/mhr_tree_sitter" }
```

The crates use the 2024 edition and are tested with Rust 1.97. Clippy's
`incompatible_msrv` check finds no standard-library API newer than Rust 1.89.

## Usage

```rust
use tree_sitter::{Parser, Query, QueryCursor, StreamingIterator};

let mut parser = Parser::new();
parser
    .set_language(&tree_sitter_rust::LANGUAGE.into())
    .expect("Error loading Rust grammar");
let source = "fn main() {}";
let tree = parser.parse(source, None).unwrap();
assert_eq!(
    tree.root_node().to_sexp(),
    "(source_file (function_item name: (identifier) parameters: (parameters) body: (block)))"
);

let query = Query::new(
    &tree_sitter_rust::LANGUAGE.into(),
    "(function_item name: (identifier) @name)",
)
.unwrap();
let mut cursor = QueryCursor::new();
let mut matches = cursor.matches(&query, tree.root_node(), source.as_bytes());
while let Some(found) = matches.next() {
    for capture in found.captures.iter() {
        println!("{}", capture.node.utf8_text(source.as_bytes()).unwrap());
    }
}
```

## Documentation

- [Grammars](docs/grammars.md): every grammar crate, its upstream crate and constants,
  and the repository layout.
- [API differences](docs/api-differences.md) from tree-sitter 0.25.10's Rust binding,
  chiefly the missing C API and WebAssembly support, and a `Parser` that is `Send` but
  not `Sync`.
- [Verification](docs/verification.md): how the port was compared with the C library.
- [Speed](docs/speed.md): the full comparison with C.

## License

MIT, like tree-sitter and most of the grammars. Three grammars are Apache-2.0 (Elixir,
Erlang and HCL). The copyright notices are in [NOTICE](NOTICE), and the license texts in
[LICENSE](LICENSE), `runtime/LICENSE`, `tables/LICENSE` and `grammars/<key>/LICENSE`.
