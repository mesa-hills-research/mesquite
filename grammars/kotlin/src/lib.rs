//! The `kotlin` tree-sitter grammar for the ported runtime: its tables (decoded from
//! `tables.bin`) and lexer (`lex.rs`), converted from the grammar's `src/parser.c`.
//!
//! Upstream: tree-sitter-kotlin-ng 1.1.0
//! (<https://github.com/tree-sitter-grammars/tree-sitter-kotlin>, tag `v1.1.0`). Generated:
//! do not edit by hand, except the scanner (`scanner.rs`).
#![forbid(unsafe_code)]

mod lex;
mod scanner;

use std::sync::LazyLock;

use tree_sitter_language::{LanguageFn, LanguageTables, LexFn, ScannerCreateFn};

/// The grammar's tables, written by the converter from the compiled C grammar.
static BLOB: &[u8] = include_bytes!("tables.bin");

static TABLES: LazyLock<LanguageTables> = LazyLock::new(decode);

/// The grammar, for `tree_sitter::Language::new(LANGUAGE)` or `LANGUAGE.into()`, as with
/// tree-sitter-kotlin-ng's `tree_sitter_kotlin_ng::LANGUAGE`.
pub const LANGUAGE: LanguageFn = LanguageFn::from_fn(language);

/// The grammar's tables (the C `tree_sitter_kotlin()`), decoded on first use.
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
    fn annotation_at_end_of_input_parses() {
        // The C scanner loops forever on each of these files (no newline at the end).
        let language = tree_sitter::Language::new(super::LANGUAGE);
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&language).unwrap();
        for text in [
            "val a\n@",
            "class A\n@",
            "class A\n@Suppress(\"x\")",
            "class Repo {\n    val db: Db\n    @Inject",
            "package p\n\nclass A {\n    var x: Int = 0\n        @JvmName(\"getX\")",
            "val x: Int\n    @Deprecated(\"a b\")",
            "val x = 1\n@file:JvmName(\"A\")",
        ] {
            let tree = parser.parse(text, None).expect(text);
            assert_eq!(tree.root_node().end_byte(), text.len(), "{text:?}");
        }
    }

    #[test]
    fn tables_decode() {
        let language = super::language();
        assert_eq!(language.abi_version, 14);
        assert_eq!(language.state_count, 11432);
        assert_eq!(language.symbol_count, 289);
    }
}
