//! Binding-only tests use inert tables and explicit matches, independently of
//! the query compiler/state machine so their units need not be implemented yet.
use super::*;
use crate::{length::Length, subtree::Subtree, tree::Tree};
use std::sync::OnceLock;
use ts_port_tables::LanguageTables;

fn language() -> Language {
    static TABLES: OnceLock<LanguageTables> = OnceLock::new();
    Language::from(TABLES.get_or_init(|| LanguageTables {
        abi_version: 15,
        name: None,
        metadata: None,
        symbol_count: 0,
        alias_count: 0,
        token_count: 0,
        external_token_count: 0,
        state_count: 0,
        large_state_count: 0,
        production_id_count: 0,
        field_count: 0,
        max_alias_sequence_length: 0,
        parse_table: vec![],
        small_parse_table: vec![],
        small_parse_table_map: vec![],
        parse_actions: vec![],
        symbol_names: vec![],
        field_names: vec![],
        field_map_slices: vec![],
        field_map_entries: vec![],
        symbol_metadata: vec![],
        public_symbol_map: vec![],
        alias_map: vec![],
        alias_sequences: vec![],
        lex_modes: vec![],
        lex_fn: |_, _| false,
        keyword_lex_fn: None,
        keyword_capture_token: 0,
        external_scanner: None,
        primary_state_ids: vec![],
        reserved_words: vec![],
        max_reserved_word_set_size: 0,
        supertype_count: 0,
        supertype_symbols: vec![],
        supertype_map_slices: vec![],
        supertype_map_entries: vec![],
    }))
}

fn query(predicates: Vec<TextPredicateCapture>) -> Query {
    Query {
        compiled: Box::new(CompiledQuery {
            captures: SymbolTable::default(),
            predicate_values: SymbolTable::default(),
            capture_quantifiers: vec![],
            steps: vec![],
            pattern_map: vec![],
            predicate_steps: vec![],
            patterns: vec![],
            step_offsets: vec![],
            negated_fields: vec![],
            string_buffer: vec![],
            repeat_symbols_with_rootless_patterns: vec![],
            language: language(),
            wildcard_root_pattern_count: 0,
        }),
        capture_names: CaptureNames::new(vec![]),
        capture_quantifiers: Box::new([]),
        text_predicates: vec![predicates.into_boxed_slice()].into_boxed_slice(),
        property_settings: Box::new([]),
        property_predicates: Box::new([]),
        general_predicates: Box::new([]),
    }
}

fn evaluate(predicates: Vec<TextPredicateCapture>, texts: &[(u32, &[u8])]) -> (bool, Vec<usize>) {
    let tree = Tree {
        root: Box::new(Subtree::Null),
        language: language(),
        included_ranges: vec![],
    };
    let removals = Mutex::default();
    let captures = texts
        .iter()
        .enumerate()
        .map(|(position, (index, _))| QueryCapture {
            index: *index,
            node: Node {
                tree: &tree,
                subtree: &tree.root,
                position: Length {
                    bytes: position as u32,
                    ..Length::default()
                },
                alias: 0,
            },
        })
        .collect();
    let result = QueryMatch::new(
        QueryMatchData {
            id: 17,
            pattern_index: 0,
            captures,
        },
        &removals,
    );
    let mut reads = Vec::new();
    let mut provider = |node: Node<'_>| {
        let i = node.start_byte();
        reads.push(i);
        iter::once(texts[i].1)
    };
    let satisfied = result.satisfies_text_predicates(
        &query(predicates),
        &mut Vec::new(),
        &mut Vec::new(),
        &mut provider,
    );
    (satisfied, reads)
}

#[test]
fn scalar_predicates_preserve_any_mode_fallthrough() {
    for positive in [false, true] {
        for all in [false, true] {
            let make = || TextPredicateCapture::EqString(0, "a".into(), positive, all);
            assert!(evaluate(vec![make()], &[]).0);
            assert!(evaluate(vec![make()], &[(1, b"unrelated")]).0);
            assert_eq!(evaluate(vec![make()], &[(0, b"a")]).0, !all || positive);
            assert_eq!(evaluate(vec![make()], &[(0, b"b")]).0, !all || !positive);
            let (matched, reads) = evaluate(vec![make()], &[(0, b"a"), (0, b"b")]);
            assert_eq!(matched, !all);
            assert_eq!(reads, if all == positive { vec![0, 1] } else { vec![0] });
        }
    }
}

#[test]
fn regex_predicates_have_the_same_missing_and_any_truth_tables() {
    let regex = regex::bytes::Regex::new("^a+$").unwrap();
    for positive in [false, true] {
        for all in [false, true] {
            let make = || TextPredicateCapture::MatchString(0, regex.clone(), positive, all);
            assert!(evaluate(vec![make()], &[]).0);
            assert_eq!(evaluate(vec![make()], &[(0, b"aaa")]).0, !all || positive);
            assert_eq!(evaluate(vec![make()], &[(0, b"bbb")]).0, !all || !positive);
            assert_eq!(evaluate(vec![make()], &[(0, b"aaa"), (0, b"bbb")]).0, !all);
        }
    }
    let raw_byte = regex::bytes::Regex::new(r"(?-u)^\xFF$").unwrap();
    assert!(
        evaluate(
            vec![TextPredicateCapture::MatchString(0, raw_byte, true, true)],
            &[(0, b"\xff")]
        )
        .0
    );
}

#[test]
fn paired_captures_check_lengths_unless_any_mode_succeeds_early() {
    for positive in [false, true] {
        for all in [false, true] {
            let make = || TextPredicateCapture::EqCapture(0, 1, positive, all);
            assert!(evaluate(vec![make()], &[]).0);
            assert!(!evaluate(vec![make()], &[(0, b"a")]).0);
            assert!(!evaluate(vec![make()], &[(1, b"a")]).0);
            assert_eq!(
                evaluate(vec![make()], &[(0, b"a"), (1, b"a")]).0,
                !all || positive
            );
            assert_eq!(
                evaluate(vec![make()], &[(0, b"a"), (1, b"b")]).0,
                !all || !positive
            );
            // The first pair satisfies either the positive or negative predicate.
            let second: &[u8] = if positive { b"a" } else { b"b" };
            let (matched, reads) = evaluate(vec![make()], &[(0, b"a"), (0, b"extra"), (1, second)]);
            assert_eq!(matched, !all);
            assert_eq!(reads, [0, 2]);
        }
    }
}

#[test]
fn membership_requires_all_captures_and_handles_empty_sets() {
    let make = |positive| {
        TextPredicateCapture::AnyString(0, vec!["a".into(), "b".into()].into(), positive)
    };
    assert!(evaluate(vec![make(true)], &[(0, b"a"), (0, b"b")]).0);
    assert!(!evaluate(vec![make(true)], &[(0, b"a"), (0, b"c")]).0);
    assert!(evaluate(vec![make(false)], &[(0, b"c"), (0, b"d")]).0);
    assert!(!evaluate(vec![make(false)], &[(0, b"c"), (0, b"a")]).0);
    for positive in [false, true] {
        let empty = || TextPredicateCapture::AnyString(0, Box::new([]), positive);
        assert!(evaluate(vec![empty()], &[]).0);
        assert_eq!(evaluate(vec![empty()], &[(0, b"anything")]).0, !positive);
    }
}

#[test]
fn predicates_short_circuit_and_skip_unrelated_captures() {
    let predicates = vec![
        TextPredicateCapture::EqString(1, "a".into(), true, true),
        TextPredicateCapture::EqString(0, "b".into(), true, true),
    ];
    let (satisfied, reads) = evaluate(predicates, &[(0, b"b"), (1, b"wrong")]);
    assert!(!satisfied);
    assert_eq!(reads, [1]);
}

#[test]
fn match_removal_preserves_repeated_ids_and_order() {
    let removals = Mutex::new(vec![]);
    let make = |id| {
        QueryMatch::new(
            QueryMatchData {
                id,
                pattern_index: 0,
                captures: vec![],
            },
            &removals,
        )
    };
    let first = make(17);
    let second = make(2);
    assert_eq!(first.id(), 17);
    first.remove();
    second.remove();
    first.remove();
    assert_eq!(*removals.lock().unwrap(), [17, 2, 17]);
}

#[test]
fn compiler_diagnostics_keep_byte_columns_and_extract_quoted_names() {
    let error = |source, offset, kind| {
        compile_error(&language(), source, QueryCompileError { offset, kind })
    };
    let source = "αβ (bad!)";
    let diagnostic = error(source, 6, QueryErrorCode::NodeType);
    assert_eq!(
        (diagnostic.row, diagnostic.column, diagnostic.offset),
        (0, 6, 6)
    );
    assert_eq!(diagnostic.message, "bad");
    assert_eq!(
        diagnostic.to_string(),
        "Query error at 1:7. Invalid node type bad"
    );
    let diagnostic = error(r#"("no\"such")"#, 2, QueryErrorCode::NodeType);
    assert_eq!(diagnostic.message, r#"no\"such"#);
    let diagnostic = error("\n(name)", 2, QueryErrorCode::Structure);
    assert_eq!(diagnostic.kind, QueryErrorKind::Structure);
    assert_eq!(diagnostic.message, "(name)\n ^");
    assert_eq!((diagnostic.row, diagnostic.column), (1, 1));
    let diagnostic = error("", 0, QueryErrorCode::Syntax);
    assert_eq!(diagnostic.message, "Unexpected EOF");
}
