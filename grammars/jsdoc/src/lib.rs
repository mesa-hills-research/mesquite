//! The `jsdoc` tree-sitter grammar for the ported runtime: its tables (decoded from
//! `tables.bin`) and lexer (`lex.rs`), converted from the grammar's `src/parser.c`.
//!
//! Upstream: tree-sitter-jsdoc 0.23.2
//! (<https://github.com/tree-sitter/tree-sitter-jsdoc>, tag `v0.23.2`). Generated: do not
//! edit by hand, except the scanner (`scanner.rs`).
#![forbid(unsafe_code)]

mod lex;
mod scanner;

use std::sync::LazyLock;

use tree_sitter_language::{LanguageFn, LanguageTables, LexFn, ScannerCreateFn};

/// The grammar's tables, written by the converter from the compiled C grammar.
static BLOB: &[u8] = include_bytes!("tables.bin");

static TABLES: LazyLock<LanguageTables> = LazyLock::new(decode);

/// The grammar, for `tree_sitter::Language::new(LANGUAGE)` or `LANGUAGE.into()`, as with
/// tree-sitter-jsdoc's `tree_sitter_jsdoc::LANGUAGE`.
pub const LANGUAGE: LanguageFn = LanguageFn::from_fn(language);

/// The syntax highlighting query (`queries/highlights.scm`), as with tree-sitter-jsdoc's
/// `tree_sitter_jsdoc::HIGHLIGHTS_QUERY`.
pub const HIGHLIGHTS_QUERY: &str = include_str!("../queries/highlights.scm");

/// The grammar's tables (the C `tree_sitter_jsdoc()`), decoded on first use.
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
    fn queries_compile() {
        let language = tree_sitter::Language::new(super::LANGUAGE);
        tree_sitter::Query::new(&language, super::HIGHLIGHTS_QUERY).unwrap();
    }

    #[test]
    fn tables_decode() {
        let language = super::language();
        assert_eq!(language.abi_version, 14);
        assert_eq!(language.state_count, 51);
        assert_eq!(language.symbol_count, 31);
    }
}
