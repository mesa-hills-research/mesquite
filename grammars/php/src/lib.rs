//! The `php` tree-sitter grammar for the ported runtime: its tables (decoded from
//! `tables.bin`) and lexer (`lex.rs`), converted from the grammar's `src/parser.c`.
//!
//! Upstream: tree-sitter-php 0.25.1 (<https://github.com/tree-sitter/tree-sitter-php>, tag
//! `v0.25.1`). Generated: do not edit by hand, except the scanner (`scanner.rs`).
#![forbid(unsafe_code)]

mod lex;
mod scanner;

use std::sync::LazyLock;

use tree_sitter_language::{LanguageFn, LanguageTables, LexFn, ScannerCreateFn};

/// The grammar's tables, written by the converter from the compiled C grammar.
static BLOB: &[u8] = include_bytes!("tables.bin");

static TABLES: LazyLock<LanguageTables> = LazyLock::new(decode);

/// The grammar, for `tree_sitter::Language::new(LANGUAGE_PHP)` or `LANGUAGE_PHP.into()`, as
/// with tree-sitter-php's `tree_sitter_php::LANGUAGE_PHP`.
pub const LANGUAGE_PHP: LanguageFn = LanguageFn::from_fn(language);

/// The PHP (without HTML) grammar (crate `mesquite_php_only`), as with
/// tree-sitter-php's `tree_sitter_php::LANGUAGE_PHP_ONLY`.
pub use tree_sitter_php_only::LANGUAGE as LANGUAGE_PHP_ONLY;

/// The grammar's tables (the C `tree_sitter_php()`), decoded on first use.
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
        let language = tree_sitter::Language::new(super::LANGUAGE_PHP);
        assert_eq!(language, super::LANGUAGE_PHP.into());
        assert!(std::ptr::eq(
            super::LANGUAGE_PHP.tables(),
            super::language()
        ));
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&language).unwrap();
        assert!(parser.parse("", None).is_some());
    }

    #[test]
    fn reexported_php_only() {
        let reexported = tree_sitter::Language::new(super::LANGUAGE_PHP_ONLY);
        assert_eq!(reexported, tree_sitter_php_only::LANGUAGE.into());
        assert_ne!(reexported, super::LANGUAGE_PHP.into());
    }

    #[test]
    fn tables_decode() {
        let language = super::language();
        assert_eq!(language.abi_version, 15);
        assert_eq!(language.state_count, 3469);
        assert_eq!(language.symbol_count, 433);
    }
}
