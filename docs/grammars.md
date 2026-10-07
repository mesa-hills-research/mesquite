# Grammars

Each grammar is a crate, `mhr_tree_sitter_<key>` in `grammars/<key>/`. Its `lib.rs`,
`lex.rs` and `tables.bin` are generated from the grammar's `parser.c`, and `scanner.rs`
is the translated external scanner, where the grammar has one.

| Language | Package | Library | Constant | Upstream crate | Version | License |
|---|---|---|---|---|---|---|
| Bash | `mhr_tree_sitter_bash` | `tree_sitter_bash` | `LANGUAGE` | [`tree-sitter-bash`](https://github.com/tree-sitter/tree-sitter-bash/tree/v0.25.1) | 0.25.1 | MIT |
| C | `mhr_tree_sitter_c` | `tree_sitter_c` | `LANGUAGE` | [`tree-sitter-c`](https://github.com/tree-sitter/tree-sitter-c/tree/v0.24.2) | 0.24.2 | MIT |
| C# | `mhr_tree_sitter_c_sharp` | `tree_sitter_c_sharp` | `LANGUAGE` | [`tree-sitter-c-sharp`](https://github.com/tree-sitter/tree-sitter-c-sharp/tree/v0.23.5) | 0.23.5 | MIT |
| CMake | `mhr_tree_sitter_cmake` | `tree_sitter_cmake` | `LANGUAGE` | [`tree-sitter-cmake`](https://github.com/uyha/tree-sitter-cmake/tree/v0.7.5) | 0.7.5 | MIT |
| C++ | `mhr_tree_sitter_cpp` | `tree_sitter_cpp` | `LANGUAGE` | [`tree-sitter-cpp`](https://github.com/tree-sitter/tree-sitter-cpp/tree/v0.23.4) | 0.23.4 | MIT |
| CSS | `mhr_tree_sitter_css` | `tree_sitter_css` | `LANGUAGE` | [`tree-sitter-css`](https://github.com/tree-sitter/tree-sitter-css/tree/v0.25.0) | 0.25.0 | MIT |
| CUDA | `mhr_tree_sitter_cuda` | `tree_sitter_cuda` | `LANGUAGE` | [`tree-sitter-cuda`](https://github.com/tree-sitter-grammars/tree-sitter-cuda/tree/v0.21.2) | 0.21.2 | MIT |
| Dart | `mhr_tree_sitter_dart` | `tree_sitter_dart` | `LANGUAGE` | [`tree-sitter-dart`](https://github.com/nielsenko/tree-sitter-dart/tree/v0.2.0) | 0.2.0 | MIT |
| DTD | `mhr_tree_sitter_dtd` | `tree_sitter_dtd` | `LANGUAGE` | [`tree-sitter-xml`](https://github.com/tree-sitter-grammars/tree-sitter-xml/tree/v0.7.0) | 0.7.0 | MIT |
| Elixir | `mhr_tree_sitter_elixir` | `tree_sitter_elixir` | `LANGUAGE` | [`tree-sitter-elixir`](https://github.com/elixir-lang/tree-sitter-elixir/tree/v0.3.5) | 0.3.5 | Apache-2.0 |
| Elm | `mhr_tree_sitter_elm` | `tree_sitter_elm` | `LANGUAGE` | [`tree-sitter-elm`](https://github.com/elm-tooling/tree-sitter-elm/tree/v5.9.4) | 5.9.4 | MIT |
| Embedded templates (ERB, EJS) | `mhr_tree_sitter_embedded_template` | `tree_sitter_embedded_template` | `LANGUAGE` | [`tree-sitter-embedded-template`](https://github.com/tree-sitter/tree-sitter-embedded-template/tree/v0.25.0) | 0.25.0 | MIT |
| Erlang | `mhr_tree_sitter_erlang` | `tree_sitter_erlang` | `LANGUAGE` | [`tree-sitter-erlang`](https://github.com/WhatsApp/tree-sitter-erlang/tree/0.20) | 0.20.0 | Apache-2.0 |
| Fortran | `mhr_tree_sitter_fortran` | `tree_sitter_fortran` | `LANGUAGE` | [`tree-sitter-fortran`](https://github.com/stadelmanma/tree-sitter-fortran/tree/v0.6.0) | 0.6.0 | MIT |
| F# | `mhr_tree_sitter_fsharp` | `tree_sitter_fsharp` | `LANGUAGE_FSHARP` | [`tree-sitter-fsharp`](https://github.com/ionide/tree-sitter-fsharp/tree/0.3.12) | 0.3.12 | MIT |
| GLSL | `mhr_tree_sitter_glsl` | `tree_sitter_glsl` | `LANGUAGE_GLSL` | [`tree-sitter-glsl`](https://github.com/tree-sitter-grammars/tree-sitter-glsl/tree/v0.2.0) | 0.2.0 | MIT |
| Go | `mhr_tree_sitter_go` | `tree_sitter_go` | `LANGUAGE` | [`tree-sitter-go`](https://github.com/tree-sitter/tree-sitter-go/tree/v0.25.0) | 0.25.0 | MIT |
| Haskell | `mhr_tree_sitter_haskell` | `tree_sitter_haskell` | `LANGUAGE` | [`tree-sitter-haskell`](https://github.com/tree-sitter/tree-sitter-haskell/tree/v0.24.1) | 0.24.1 | MIT |
| HCL | `mhr_tree_sitter_hcl` | `tree_sitter_hcl` | `LANGUAGE` | [`tree-sitter-hcl`](https://github.com/tree-sitter-grammars/tree-sitter-hcl/tree/v1.1.1) | 1.1.0 | Apache-2.0 |
| HTML | `mhr_tree_sitter_html` | `tree_sitter_html` | `LANGUAGE` | [`tree-sitter-html`](https://github.com/tree-sitter/tree-sitter-html/tree/v0.23.2) | 0.23.2 | MIT |
| Java | `mhr_tree_sitter_java` | `tree_sitter_java` | `LANGUAGE` | [`tree-sitter-java`](https://github.com/tree-sitter/tree-sitter-java/tree/v0.23.5) | 0.23.5 | MIT |
| JavaScript | `mhr_tree_sitter_javascript` | `tree_sitter_javascript` | `LANGUAGE` | [`tree-sitter-javascript`](https://github.com/tree-sitter/tree-sitter-javascript/tree/v0.25.0) | 0.25.0 | MIT |
| JSON | `mhr_tree_sitter_json` | `tree_sitter_json` | `LANGUAGE` | [`tree-sitter-json`](https://github.com/tree-sitter/tree-sitter-json/tree/v0.24.8) | 0.24.8 | MIT |
| Julia | `mhr_tree_sitter_julia` | `tree_sitter_julia` | `LANGUAGE` | [`tree-sitter-julia`](https://github.com/tree-sitter/tree-sitter-julia/tree/v0.23.1) | 0.23.1 | MIT |
| Kotlin | `mhr_tree_sitter_kotlin` | `tree_sitter_kotlin_ng` | `LANGUAGE` | [`tree-sitter-kotlin-ng`](https://github.com/tree-sitter-grammars/tree-sitter-kotlin/tree/v1.1.0) | 1.1.0 | MIT |
| Lua | `mhr_tree_sitter_lua` | `tree_sitter_lua` | `LANGUAGE` | [`tree-sitter-lua`](https://github.com/tree-sitter-grammars/tree-sitter-lua/tree/v0.5.0) | 0.5.0 | MIT |
| Make | `mhr_tree_sitter_make` | `tree_sitter_make` | `LANGUAGE` | [`tree-sitter-make`](https://github.com/tree-sitter-grammars/tree-sitter-make/tree/v1.1.1) | 1.1.1 | MIT |
| Markdown (block) | `mhr_tree_sitter_markdown` | `tree_sitter_md` | `LANGUAGE`, `INLINE_LANGUAGE` | [`tree-sitter-md`](https://github.com/tree-sitter-grammars/tree-sitter-markdown/tree/v0.5.3) | 0.5.3 | MIT |
| Markdown (inline) | `mhr_tree_sitter_markdown_inline` | `tree_sitter_markdown_inline` | `LANGUAGE` | [`tree-sitter-md`](https://github.com/tree-sitter-grammars/tree-sitter-markdown/tree/v0.5.3) | 0.5.3 | MIT |
| Objective-C | `mhr_tree_sitter_objc` | `tree_sitter_objc` | `LANGUAGE` | [`tree-sitter-objc`](https://github.com/tree-sitter-grammars/tree-sitter-objc/tree/v3.0.2) | 3.0.2 | MIT |
| OCaml | `mhr_tree_sitter_ocaml` | `tree_sitter_ocaml` | `LANGUAGE_OCAML`, `LANGUAGE_OCAML_INTERFACE` | [`tree-sitter-ocaml`](https://github.com/tree-sitter/tree-sitter-ocaml/tree/v0.26.0) | 0.26.0 | MIT |
| OCaml (interface) | `mhr_tree_sitter_ocaml_interface` | `tree_sitter_ocaml_interface` | `LANGUAGE` | [`tree-sitter-ocaml`](https://github.com/tree-sitter/tree-sitter-ocaml/tree/v0.26.0) | 0.26.0 | MIT |
| Odin | `mhr_tree_sitter_odin` | `tree_sitter_odin` | `LANGUAGE` | [`tree-sitter-odin`](https://github.com/tree-sitter-grammars/tree-sitter-odin/tree/v1.3.0) | 1.3.0 | MIT |
| Perl | `mhr_tree_sitter_perl` | `ts_parser_perl` | `LANGUAGE` | [`ts-parser-perl`](https://github.com/tree-sitter-perl/tree-sitter-perl/tree/v2.0.0) | 2.0.0 | MIT |
| PHP | `mhr_tree_sitter_php` | `tree_sitter_php` | `LANGUAGE_PHP`, `LANGUAGE_PHP_ONLY` | [`tree-sitter-php`](https://github.com/tree-sitter/tree-sitter-php/tree/v0.25.1) | 0.25.1 | MIT |
| PHP (without HTML) | `mhr_tree_sitter_php_only` | `tree_sitter_php_only` | `LANGUAGE` | [`tree-sitter-php`](https://github.com/tree-sitter/tree-sitter-php/tree/v0.25.1) | 0.25.1 | MIT |
| PowerShell | `mhr_tree_sitter_powershell` | `tree_sitter_powershell` | `LANGUAGE` | [`tree-sitter-powershell`](https://github.com/airbus-cert/tree-sitter-powershell/tree/v0.26.3) | 0.26.3 | MIT |
| Protocol Buffers | `mhr_tree_sitter_proto` | `tree_sitter_proto` | `LANGUAGE` | [`tree-sitter-proto`](https://github.com/coder3101/tree-sitter-proto/tree/0.6.0) | 0.6.0 | MIT |
| Python | `mhr_tree_sitter_python` | `tree_sitter_python` | `LANGUAGE` | [`tree-sitter-python`](https://github.com/tree-sitter/tree-sitter-python/tree/v0.25.0) | 0.25.0 | MIT |
| R | `mhr_tree_sitter_r` | `tree_sitter_r` | `LANGUAGE` | [`tree-sitter-r`](https://github.com/r-lib/tree-sitter-r/tree/v1.3.0) | 1.3.0 | MIT |
| Ruby | `mhr_tree_sitter_ruby` | `tree_sitter_ruby` | `LANGUAGE` | [`tree-sitter-ruby`](https://github.com/tree-sitter/tree-sitter-ruby/tree/v0.23.1) | 0.23.1 | MIT |
| Rust | `mhr_tree_sitter_rust` | `tree_sitter_rust` | `LANGUAGE` | [`tree-sitter-rust`](https://github.com/tree-sitter/tree-sitter-rust/tree/v0.24.2) | 0.24.2 | MIT |
| Scala | `mhr_tree_sitter_scala` | `tree_sitter_scala` | `LANGUAGE` | [`tree-sitter-scala`](https://github.com/tree-sitter/tree-sitter-scala/tree/v0.26.2) | 0.26.2 | MIT |
| Solidity | `mhr_tree_sitter_solidity` | `tree_sitter_solidity` | `LANGUAGE` | [`tree-sitter-solidity`](https://github.com/JoranHonig/tree-sitter-solidity/tree/v1.2.13) | 1.2.13 | MIT |
| SQL | `mhr_tree_sitter_sql` | `tree_sitter_sequel` | `LANGUAGE` | [`tree-sitter-sequel`](https://github.com/DerekStride/tree-sitter-sql/tree/v0.3.11) | 0.3.11 | MIT |
| Svelte | `mhr_tree_sitter_svelte` | `tree_sitter_svelte_ng` | `LANGUAGE` | [`tree-sitter-svelte-ng`](https://github.com/tree-sitter-grammars/tree-sitter-svelte/tree/v1.0.2) | 1.0.2 | MIT |
| Swift | `mhr_tree_sitter_swift` | `tree_sitter_swift` | `LANGUAGE` | [`tree-sitter-swift`](https://github.com/alex-pinkus/tree-sitter-swift/tree/0.7.4) | 0.7.4 | MIT |
| TOML | `mhr_tree_sitter_toml` | `tree_sitter_toml_ng` | `LANGUAGE` | [`tree-sitter-toml-ng`](https://github.com/tree-sitter-grammars/tree-sitter-toml/tree/v0.7.0) | 0.7.0 | MIT |
| TSX | `mhr_tree_sitter_tsx` | `tree_sitter_tsx` | `LANGUAGE` | [`tree-sitter-typescript`](https://github.com/tree-sitter/tree-sitter-typescript/tree/v0.23.2) | 0.23.2 | MIT |
| TypeScript | `mhr_tree_sitter_typescript` | `tree_sitter_typescript` | `LANGUAGE_TYPESCRIPT`, `LANGUAGE_TSX` | [`tree-sitter-typescript`](https://github.com/tree-sitter/tree-sitter-typescript/tree/v0.23.2) | 0.23.2 | MIT |
| Verilog | `mhr_tree_sitter_verilog` | `tree_sitter_verilog` | `LANGUAGE` | [`tree-sitter-verilog`](https://github.com/tree-sitter/tree-sitter-verilog/tree/v1.0.3) | 1.0.3 | MIT |
| XML | `mhr_tree_sitter_xml` | `tree_sitter_xml` | `LANGUAGE_XML`, `LANGUAGE_DTD` | [`tree-sitter-xml`](https://github.com/tree-sitter-grammars/tree-sitter-xml/tree/v0.7.0) | 0.7.0 | MIT |
| YAML | `mhr_tree_sitter_yaml` | `tree_sitter_yaml` | `LANGUAGE` | [`tree-sitter-yaml`](https://github.com/tree-sitter-grammars/tree-sitter-yaml/tree/v0.7.2) | 0.7.2 | MIT |
| Zig | `mhr_tree_sitter_zig` | `tree_sitter_zig` | `LANGUAGE` | [`tree-sitter-zig`](https://github.com/tree-sitter-grammars/tree-sitter-zig/tree/v1.1.2) | 1.1.2 | MIT |

## Upstream crates with two grammars

Five upstream crates hold two grammars each: tree-sitter-typescript (TypeScript and
TSX), tree-sitter-php (PHP, and PHP without HTML), tree-sitter-md (Markdown block and
inline), tree-sitter-ocaml (OCaml and OCaml interfaces) and tree-sitter-xml (XML and
DTD). Here each grammar has its own crate, and the first crate of each pair re-exports
the second under upstream's name. So `tree_sitter_typescript::LANGUAGE_TSX`,
`tree_sitter_php::LANGUAGE_PHP_ONLY`, `tree_sitter_md::INLINE_LANGUAGE`,
`tree_sitter_ocaml::LANGUAGE_OCAML_INTERFACE` and `tree_sitter_xml::LANGUAGE_DTD` work
as upstream, and the second crate can also be used alone (`tree_sitter_tsx::LANGUAGE`).

tree-sitter-fsharp's signature grammar (`LANGUAGE_SIGNATURE`) and tree-sitter-ocaml's
type grammar (`LANGUAGE_OCAML_TYPE`) aren't included.

## Library names

Library names follow the upstream crates, suffixes included: `tree_sitter_kotlin_ng`
(tree-sitter-kotlin-ng), `tree_sitter_toml_ng` (tree-sitter-toml-ng), `tree_sitter_md`
(tree-sitter-md), `tree_sitter_svelte_ng` (tree-sitter-svelte-ng), `tree_sitter_sequel`
(tree-sitter-sequel, the SQL grammar) and `ts_parser_perl` (ts-parser-perl).

## Repository layout

- `runtime/`: the runtime (`mhr_tree_sitter`). `runtime/PORTING.md` is the translation
  contract the swarm worked from, and `runtime/tests/*.md` are its optimization notes.
  Both use the crates' names from before the rename (`ts_port`, `ts_port_tables`,
  `ts_port_<key>`).
- `tables/`: `mhr_tree_sitter_language` (library `tree_sitter_language`), the
  counterpart of upstream's tree-sitter-language crate. The runtime and the grammar
  crates share it. It holds the grammar tables and `LanguageFn`.
- `grammars/<key>/`: the grammar crates.
