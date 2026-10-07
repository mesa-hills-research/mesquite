//! The `powershell` tree-sitter grammar for the ported runtime: its tables (decoded from
//! `tables.bin`) and lexer (`lex.rs`), converted from the grammar's `src/parser.c`.
//!
//! Upstream: tree-sitter-powershell 0.26.3
//! (<https://github.com/airbus-cert/tree-sitter-powershell>, tag `v0.26.3`). Generated: do
//! not edit by hand, except the scanner (`scanner.rs`).
#![forbid(unsafe_code)]

mod lex;
mod scanner;

use std::sync::LazyLock;

use tree_sitter_language::{LanguageFn, LanguageTables, LexFn, ScannerCreateFn};

/// The grammar's tables, written by the converter from the compiled C grammar.
static BLOB: &[u8] = include_bytes!("tables.bin");

static TABLES: LazyLock<LanguageTables> = LazyLock::new(decode);

/// The grammar, for `tree_sitter::Language::new(LANGUAGE)` or `LANGUAGE.into()`, as with
/// tree-sitter-powershell's `tree_sitter_powershell::LANGUAGE`.
pub const LANGUAGE: LanguageFn = LanguageFn::from_fn(language);

/// The grammar's tables (the C `tree_sitter_powershell()`), decoded on first use.
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
    fn tables_decode() {
        let language = super::language();
        assert_eq!(language.abi_version, 15);
        assert_eq!(language.state_count, 2127);
        assert_eq!(language.symbol_count, 415);
    }
}
