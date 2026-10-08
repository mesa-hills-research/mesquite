# API differences from tree-sitter 0.25.10

Compared with tree-sitter's Rust binding (`binding_rust/lib.rs` at v0.25.10).

## Not available

- **The C API and raw pointers:** the `ffi` module, `from_raw`/`into_raw` on every type,
  `set_allocator` and `PARSER_HEADER`. `LanguageFn` wraps a safe Rust function instead
  of a C function returning a pointer, so upstream's C grammar crates and this runtime
  can't be mixed, in either direction.
- **WebAssembly:** the `wasm` feature (`WasmStore`, `WasmError`, `Language::is_wasm`,
  `Parser::set_wasm_store`/`take_wasm_store`) and `wasm_stdlib_symbols`.
- **Cargo features:** there are none, and the crates always use `std` (no `no_std`
  build).
- **Logging:** `Parser::set_logger` and `Parser::logger`. `LogType` exists, but nothing
  public uses it.
- **DOT graphs:** `Parser::print_dot_graphs`, `Parser::stop_printing_dot_graphs` and
  `Tree::print_dot_graph`.
- **UTF-16 and custom encodings:** `Parser::parse_utf16_le` and `parse_utf16_be` (and
  their `_with_options` forms), the deprecated `parse_utf16` and `parse_utf16_with`,
  `parse_custom_encoding` and the `Decode` trait. Input is UTF-8: `Node::utf16_text`
  exists, but tree offsets are always UTF-8 bytes.
- **Lookahead iterators:** `LookaheadIterator` and `Language::lookahead_iterator`.
- **Cancellation flags:** `Parser::cancellation_flag` and `set_cancellation_flag`
  (unsafe and deprecated upstream). Use the progress callback in `ParseOptions`.
- **Smaller items:** `LossyUtf8` and the hidden `format_sexp`.
- **In the grammar crates:** only the `LANGUAGE` constants. The `NODE_TYPES` and query
  constants (`HIGHLIGHTS_QUERY`, `TAGS_QUERY`, `INJECTIONS_QUERY`, `LOCALS_QUERY` and so
  on) and tree-sitter-md's `parser` feature (`MarkdownParser`) are left out.

## Different

- `Parser` is `Send` but not `Sync` (upstream implements both with `unsafe impl`).
  `Mutex<Parser>` works, sharing a `&Parser` between threads doesn't.
- `Language` is `Copy` (upstream: `Clone`). Its `Debug` output shows the grammar's name,
  and `Tree`'s `Debug` output shows internal fields.
- `Query::capture_names` returns `Vec<&str>` (upstream: `&[&str]`, from a `const fn`).
- `QueryMatch::captures` is a `Vec<QueryCapture>` (upstream: a slice). On a borrowed
  match, iterate over `&m.captures` or `m.captures.iter()`.
- `QueryMatches` has an inherent `next()` that returns an owned `QueryMatch` (a clone).
  It shadows `StreamingIterator::next`. Call `StreamingIterator::next(&mut matches)` to
  borrow the match instead.
- `QueryCursor::matches_with_options` and `captures_with_options` take
  `QueryCursorOptions<'query>`, so the progress callback must outlive the returned
  iterator.
- `QueryMatch::remove` takes effect when the iterator next advances.
- `Language::version`, `Parser::parse_with`, `Parser::timeout_micros` and
  `Parser::set_timeout_micros` are deprecated upstream but not here.
- The error types implement `std::error::Error` unconditionally (upstream: with the
  `std` feature).
- A parse returns `None` and resets the parser when a lex function or external scanner
  advances at the end of the input 2^20 times while lexing one token, a grammar bug on
  which C hangs. With `panic = "abort"` the process aborts with a message instead.

## Additional

- `impl From<&'static LanguageTables> for Language`.
- Nameable iterator types: `Children`, `FieldChildren` and `NodesForCaptureIndex`, and
  the aliases `ChangedRanges`, `ParseProgressCallback` and `QueryProgressCallback`.
- More trait impls: `Clone` for `QueryMatch`, `Debug` for `LanguageRef`, `TreeCursor`,
  `ParseState` and `QueryCursorState`, `Default` for the two state types, and the
  common derives on `LanguageMetadata`.
