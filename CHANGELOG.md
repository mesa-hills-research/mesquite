# Changelog

## 0.26.0

This release is `mesquite` 0.26.0, `mesquite_language` 0.2.0 and a 0.2.0 crate for each of the 58 grammars. The runtime still follows tree-sitter 0.25.10. Its version moves to 0.26 because its API takes `mesquite_language` types, which changed, so it needs the 0.2 grammar crates. Update the runtime and every grammar crate together.

Grammar tables are read in place from the binary, and the large parse tables are stored without their zeros. Parsing, reparsing and balancing do less work, and nodes keep up to three children inline. The table compares the two releases with all 58 grammars, on the machine and with the method of [docs/speed.md](docs/speed.md).

| | 0.25.10 | 0.26.0 |
|---|---:|---:|
| Parse time against C built by GCC 13.4, geometric mean | 0.78 | 0.71 |
| Parse time against C built by Clang 21.1, geometric mean | 0.79 | 0.72 |
| A grammar's first use, median and slowest | 11.5 ms, 391 ms | 0.04 ms, 0.10 ms |
| Peak memory of a process using every grammar | 224 MB | 55 MB |
| Stripped binary with every grammar | 119 MB | 60 MB |
| Reparse of 400 KB of Python after a 4-byte append | 10.95 ms | 8.42 ms |
| Reparse of 400 KB of JSON after a 4-byte append | 2.45 ms | 1.53 ms |

Mesquite is now faster than C on every grammar but Markdown's inline grammar, which takes 1.09 times as long as C built by GCC 13.4.

Changes to `mesquite_language`, which only code that builds or reads grammar tables itself sees:

- The large tables of `LanguageTables` (`parse_table`, `small_parse_table`, `small_parse_table_map`, `parse_actions`, `lex_modes` and `primary_state_ids`) are `Table` views into the blob, and `parse_table` is a `LargeStates`. Names are `&'static str`.
- `LanguageTables::decode` and `try_decode` take a `&'static [u8]` blob in format 2 and reject format 1. They check the blob's layout and lengths but not its values. `LanguageTables::validate` checks those, and each grammar crate's tests run it on its blob.
- `LanguageTables::action_list` returns a `Table<ParseAction>`.
- `Lexer::accept_token` sets the result symbol and marks the token's end in one call. It has a default, so existing lexers need no change.

## 0.25.10

The first release, on crates.io as `mesquite` 0.25.10, `mesquite_language` 0.1.0 and a 0.1.0 crate for each of the 58 grammars. The runtime follows tree-sitter 0.25.10, and [docs/grammars.md](docs/grammars.md) lists each grammar's upstream version.
