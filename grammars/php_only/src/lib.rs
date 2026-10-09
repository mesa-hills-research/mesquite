//! The `php_only` tree-sitter grammar for the ported runtime: its tables (decoded from
//! `tables.bin`) and lexer (`lex.rs`), converted from the grammar's `src/parser.c`.
//!
//! Upstream: tree-sitter-php 0.25.1 (<https://github.com/tree-sitter/tree-sitter-php>, tag
//! `v0.25.1`). Generated: do not edit by hand (the scanner is php's `scanner.rs`).
#![forbid(unsafe_code)]

mod lex;
#[path = "../../php/src/scanner.rs"]
mod scanner;

use std::sync::LazyLock;

use tree_sitter_language::{LanguageFn, LanguageTables, LexFn, ScannerCreateFn};

/// The grammar's tables, written by the converter from the compiled C grammar.
static BLOB: &[u8] = include_bytes!("tables.bin");

static TABLES: LazyLock<LanguageTables> = LazyLock::new(decode);

/// The grammar, for `tree_sitter::Language::new(LANGUAGE)` or `LANGUAGE.into()`.
/// `mesquite_php` re-exports it as `tree_sitter_php::LANGUAGE_PHP_ONLY`, as in
/// tree-sitter-php.
pub const LANGUAGE: LanguageFn = LanguageFn::from_fn(language);

/// The grammar's tables (the C `tree_sitter_php_only()`), decoded on first use.
pub fn language() -> &'static LanguageTables {
    &TABLES
}

fn decode() -> LanguageTables {
    let keyword_lex_fn: Option<LexFn> = Some(lex::ts_lex_keywords);
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
    fn tables_decode() {
        let language = super::language();
        assert_eq!(language.abi_version, 15);
        assert_eq!(language.state_count, 3385);
        assert_eq!(language.symbol_count, 428);
    }
}
