# Changelog

## 0.26.1

This release changes only the runtime, `mesquite` 0.26.1, which works with the 0.2.0 grammar crates.

A node now takes a block sized for its children. Leaves take 96-byte blocks of glibc's allocator, as in C, and nodes with one, two or three children 112, 128 or 144 bytes. In 0.26.0 every node took a 160-byte block. A tree takes 1.0 to 1.2 times the memory of C's, down from 1.3 to 1.5 times.

| Tree of | 0.26.0 | 0.26.1 | C |
|---|---:|---:|---:|
| a 1.8 MB JSON file | 36.0 MB | 30.2 MB | 24.5 MB |
| a 1.7 MB C file | 172.1 MB | 132.7 MB | 114.8 MB |

On an AMD EPYC 4345P, parsing takes 0.98 of 0.26.0's time in the geometric mean over the 58 grammars, and walking a tree with a cursor 0.86 to 0.93 of it on Rust, Markdown and Go files. Julia takes 1.05 times as long as with 0.26.0, and JSON 1.03.

On the same machine, reparsing Python after a 4-byte append takes 1.04 times 0.26.0's time at 100 and 200 KB, 0.96 at 300 KB and 0.86 to 0.97 from 400 to 800 KB. Reparsing 400 KB of JSON takes 0.92 of it. Mesquite reparses faster than C at every size.

glibc keeps freed blocks of up to 128 bytes in its fastbins, unmerged, and hands out the newest first. After a large tree was dropped, the next parse got its nodes scattered over that tree's memory, and merged them at its first large request. Dropping a tree that frees 4,096 nodes or more, and shares none of its nodes with another tree, now makes one 64 KB request, which merges them at the drop. The old tree of a reparse shares most of its nodes with the new one, so its drop makes no request, and the next reparse reuses its blocks as they are. Other allocators serve the request and free it.

Verilog's and Julia's parse times depend on where the linker places a few hot functions (error recovery's table lookup for Verilog, the lexer and Julia's scanner for Julia). On an Intel i9-13980HX, the same code placed differently takes up to 8% longer on Verilog's largest file and 15% longer on Julia's two largest, and with every function aligned to 64 bytes 0.26.0 and 0.26.1 parse these files within about 1% of each other. A program that links Mesquite gets placements of its own, so its times on these grammars can differ either way.

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
