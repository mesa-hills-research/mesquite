//! The `markdown` tree-sitter grammar for the ported runtime: its tables (decoded from
//! `tables.bin`) and lexer (`lex.rs`), converted from the grammar's `src/parser.c`.
//!
//! Upstream: tree-sitter-md 0.5.3
//! (<https://github.com/tree-sitter-grammars/tree-sitter-markdown>, tag `v0.5.3`).
//! Generated: do not edit by hand, except the scanner (`scanner.rs`).
#![forbid(unsafe_code)]

mod lex;
mod scanner;

use std::sync::LazyLock;

use tree_sitter_language::{LanguageFn, LanguageTables, LexFn, ScannerCreateFn};

/// The grammar's tables, written by the converter from the compiled C grammar.
static BLOB: &[u8] = include_bytes!("tables.bin");

static TABLES: LazyLock<LanguageTables> = LazyLock::new(decode);

/// The grammar, for `tree_sitter::Language::new(LANGUAGE)` or `LANGUAGE.into()`, as with
/// tree-sitter-md's `tree_sitter_md::LANGUAGE`.
pub const LANGUAGE: LanguageFn = LanguageFn::from_fn(language);

/// The Markdown (inline) grammar (crate `mesquite_markdown_inline`), as with
/// tree-sitter-md's `tree_sitter_md::INLINE_LANGUAGE`.
pub use tree_sitter_markdown_inline::LANGUAGE as INLINE_LANGUAGE;

/// The grammar's tables (the C `tree_sitter_markdown()`), decoded on first use.
pub fn language() -> &'static LanguageTables {
    &TABLES
}

fn decode() -> LanguageTables {
    let keyword_lex_fn: Option<LexFn> = None;
    let scanner_create: Option<ScannerCreateFn> = Some(scanner::create);
    LanguageTables::decode(BLOB, lex::ts_lex, keyword_lex_fn, scanner_create)
}

#[cfg(test)]
mod tests {
    #[test]
    fn language_constant() {
        let language = tree_sitter::Language::new(super::LANGUAGE);
        assert_eq!(language, super::LANGUAGE.into());
        assert!(std::ptr::eq(super::LANGUAGE.tables(), super::language()));
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&language).unwrap();
        assert!(parser.parse("", None).is_some());
    }

    #[test]
    fn reexported_markdown_inline() {
        let reexported = tree_sitter::Language::new(super::INLINE_LANGUAGE);
        assert_eq!(reexported, tree_sitter_markdown_inline::LANGUAGE.into());
        assert_ne!(reexported, super::LANGUAGE.into());
    }

    #[test]
    fn deep_nesting_parses() {
        // From 255 nested blocks on, the scanner's state is larger than the runtime's
        // buffer (C writes past its end).
        let language = tree_sitter::Language::new(super::LANGUAGE);
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&language).unwrap();
        for depth in [254, 255, 256, 300, 1000] {
            let indented = |marker: &str| -> String {
                (0..depth)
                    .map(|i| format!("{}{marker} item\n", "  ".repeat(i)))
                    .collect()
            };
            for body in [
                format!("{} quoted text\n", ">".repeat(depth)),
                format!("{}quoted text\n", "> ".repeat(depth)),
                format!("{}item\n", "- ".repeat(depth)),
                format!("{}item\n", "> - ".repeat(depth / 2 + 1)),
                indented("-"),
                (0..depth)
                    .map(|i| format!("{}1. x\n", "   ".repeat(i)))
                    .collect(),
            ] {
                for text in [
                    format!("# Title\n\n{body}\nAfter.\n"),
                    format!("# Title\n\n{body}"),
                    body.trim_end().to_owned(),
                ] {
                    let tree = parser.parse(&text, None).expect("a tree");
                    assert_eq!(tree.root_node().end_byte(), text.len(), "{depth}: {text:?}");
                }
            }
        }
    }

    #[test]
    fn tables_decode() {
        let language = super::language();
        assert_eq!(language.abi_version, 15);
        assert_eq!(language.state_count, 925);
        assert_eq!(language.symbol_count, 203);
    }
}
