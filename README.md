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
- **Same results as C.** Trees, incremental reparses, query results and progress
  callbacks match the C library's, apart from a few [C bugs it fixes](docs/api-differences.md#fixes).

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

| Inputs | Port / C |
|---|---:|
| All 54 grammars (geometric mean) | 0.74 |
| C (Linux kernel) | 0.66 |
| C++ | 0.66 |
| Rust | 0.70 |
| Python | 0.71 |
| TypeScript | 0.72 |
| JavaScript | 0.68 |
| Go | 0.81 |
| Java | 0.67 |

The port is faster on 49 of the 54 grammars. Each grammar's tables are decoded the first
time it is used, which takes a median of 24 ms. Every grammar, the machine, the method and
how to compare on your own files are in [docs/speed.md](docs/speed.md).

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

The crates use the 2024 edition. They are tested with Rust 1.97 and use no
standard-library API newer than Rust 1.89.

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
- [Differences](docs/api-differences.md) from tree-sitter 0.25.10: chiefly the missing C
  API and WebAssembly support, a `Parser` that is `Send` but not `Sync`, and the C bugs
  the port fixes.
- [Speed](docs/speed.md): the full comparison with C.

## Tests

```sh
cargo test --workspace
```

## License

MIT, like tree-sitter and most of the grammars. Three grammars are Apache-2.0 (Elixir,
Erlang and HCL). The copyright notices are in [NOTICE](NOTICE), and the license texts in
[LICENSE](LICENSE), `runtime/LICENSE`, `tables/LICENSE` and `grammars/<key>/LICENSE`.
