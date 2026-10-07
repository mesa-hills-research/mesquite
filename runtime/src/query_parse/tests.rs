use super::*;
use std::sync::LazyLock;
use tree_sitter_language::{ExternalScanner, LanguageTables, Lexer};

fn unused_lex(_: &mut dyn Lexer, _: StateId) -> bool {
    panic!("query parser tests never lex source code")
}

fn unused_scanner() -> Box<dyn ExternalScanner> {
    panic!("query parser tests never create a scanner")
}

fn tables() -> LanguageTables {
    LanguageTables::decode(
        include_bytes!("../../../grammars/javascript/src/tables.bin"),
        unused_lex,
        Some(unused_lex),
        Some(unused_scanner),
    )
}

fn new_query() -> CompiledQuery {
    static TABLES: LazyLock<LanguageTables> = LazyLock::new(tables);
    CompiledQuery {
        captures: SymbolTable::default(),
        predicate_values: SymbolTable::default(),
        capture_quantifiers: Vec::new(),
        steps: Vec::new(),
        pattern_map: Vec::new(),
        predicate_steps: Vec::new(),
        patterns: Vec::new(),
        step_offsets: Vec::new(),
        negated_fields: vec![0],
        string_buffer: Vec::new(),
        repeat_symbols_with_rootless_patterns: Vec::new(),
        language: Language::from(&*TABLES),
        wildcard_root_pattern_count: 0,
    }
}

fn parse(source: &[u8]) -> (CompiledQuery, CaptureQuantifiers) {
    let mut query = new_query();
    let mut stream = stream_new(source);
    let mut quantifiers = capture_quantifiers_new();
    assert_eq!(
        ts_query__parse_pattern(&mut query, &mut stream, 0, false, &mut quantifiers),
        QueryErrorCode::None,
        "input: {:?}, stopped at {}",
        String::from_utf8_lossy(source),
        stream.input,
    );
    assert_eq!(stream.input, source.len());
    (query, quantifiers)
}

#[test]
fn negated_field_lists_reuse_only_exact_ordered_matches() {
    let mut query = new_query();
    query.steps.push(query_step__new(0, 0, false));
    for (mut fields, expected_id, expected_len) in [
        (vec![2, 1], 1, 4),
        (vec![2, 1], 1, 4),
        (vec![2], 4, 6),
        (vec![1, 2], 6, 9),
        (vec![2, 1], 1, 9),
        (vec![2, 1, 1], 9, 13),
        (vec![1], 13, 15),
        (vec![], 0, 15),
    ] {
        let original = fields.clone();
        ts_query__add_negated_fields(&mut query, 0, &mut fields);
        assert_eq!(query.steps[0].negated_field_list_id, expected_id);
        assert_eq!(query.negated_fields.len(), expected_len);
        assert_eq!(fields, original);
    }
}

#[test]
fn strings_preserve_bytes_and_decode_only_the_four_special_escapes() {
    let mut query = new_query();
    let source = "\"before\\n\\r\\t\\0\\\"\\\\\\q\\é🙂after\"tail";
    let mut stream = stream_new(source.as_bytes());
    assert_eq!(
        ts_query__parse_string_literal(&mut query, &mut stream),
        QueryErrorCode::None
    );
    assert_eq!(
        query.string_buffer,
        "before\n\r\t\0\"\\qé🙂after".as_bytes()
    );
    assert_eq!(&source.as_bytes()[stream.input..], b"tail");

    // Escaping a literal newline is allowed, and malformed UTF-8 is copied.
    for (source, expected) in [
        (&b"\"a\\\nb\""[..], &b"a\nb"[..]),
        (&b"\"a\\\xffb\""[..], &b"a\xffb"[..]),
        (&b"\"a\0b\""[..], &b"a\0b"[..]),
        (&b"\"\""[..], &b""[..]),
    ] {
        let mut stream = stream_new(source);
        assert_eq!(
            ts_query__parse_string_literal(&mut query, &mut stream),
            QueryErrorCode::None
        );
        assert_eq!(query.string_buffer, expected);
    }
}

#[test]
fn invalid_strings_reset_to_the_opening_quote() {
    let mut query = new_query();
    for source in [
        &b"\"missing"[..],
        &b"\"line\nbreak\""[..],
        &b"\"escape\\"[..],
        &b"\""[..],
        &b"not-a-string"[..],
    ] {
        let mut stream = stream_new(source);
        assert_eq!(
            ts_query__parse_string_literal(&mut query, &mut stream),
            QueryErrorCode::Syntax
        );
        assert_eq!(stream.input, 0);
        assert_eq!(stream.next, i32::from(source[0]));
    }
}

#[test]
fn predicates_keep_argument_kinds_order_and_embedded_nuls() {
    let mut query = new_query();
    symbol_table_insert_name(&mut query.captures, b"node");
    let source = b"eq? @node \"x\\0y\" bare) ; comment\n";
    let mut stream = stream_new(source);
    assert_eq!(
        ts_query__parse_predicate(&mut query, &mut stream),
        QueryErrorCode::None
    );
    let steps: Vec<_> = query
        .predicate_steps
        .iter()
        .map(|step| (step.kind, step.value_id))
        .collect();
    assert_eq!(
        steps,
        vec![
            (PredicateStepKind::String, 0),
            (PredicateStepKind::Capture, 0),
            (PredicateStepKind::String, 1),
            (PredicateStepKind::String, 2),
            (PredicateStepKind::Done, 0),
        ]
    );
    assert_eq!(symbol_table_name_for_id(&query.predicate_values, 0), b"eq?");
    assert_eq!(
        symbol_table_name_for_id(&query.predicate_values, 1),
        b"x\0y"
    );
    assert_eq!(
        symbol_table_name_for_id(&query.predicate_values, 2),
        b"bare"
    );
    assert_eq!(stream.input, source.len());

    let mut stream = stream_new(b"eq? @unknown)");
    assert_eq!(
        ts_query__parse_predicate(&mut query, &mut stream),
        QueryErrorCode::Capture
    );
    assert_eq!(stream.input, 5);
    assert_eq!(stream.next, 'u' as i32);
}

#[test]
fn anchors_mark_both_alternatives_and_preserve_placeholder_links() {
    let (query, quantifiers) = parse(b"(program . [(identifier) (string)] @child .)");
    assert_eq!(query.steps.len(), 4);
    assert_eq!(query.steps[0].depth, 0);
    assert_eq!(query.steps[1].depth, 1);
    assert_eq!(query.steps[3].depth, 1);
    assert_eq!(query.steps[1].alternative_index, 3);
    assert_eq!(query.steps[2].alternative_index, 4);
    assert!(query.steps[2].is_dead_end);
    for index in [1, 3] {
        assert!(query.steps[index].is_immediate);
        assert!(query.steps[index].is_last_child);
        assert_eq!(query.steps[index].capture_ids, [0, NONE, NONE]);
    }
    assert_eq!(quantifiers, vec![CaptureQuantifier::One]);
}

#[test]
fn field_and_capture_prefixes_apply_to_each_alternative_not_placeholders() {
    let (query, quantifiers) = parse(b"name: [(identifier) (string)] @node");
    let field = ts_language_field_id_for_name(&query.language, b"name");
    assert_ne!(field, 0);
    assert_eq!(query.steps.len(), 3);
    for index in [0, 2] {
        assert_eq!(query.steps[index].field, field);
        assert_eq!(query.steps[index].capture_ids, [0, NONE, NONE]);
    }
    assert_eq!(query.steps[1].field, 0);
    assert_eq!(query.steps[1].capture_ids, [NONE; MAX_STEP_CAPTURE_COUNT]);
    assert_eq!(quantifiers, vec![CaptureQuantifier::One]);
    // The field prefix owns the offset for its first step.
    assert_eq!(query.step_offsets[0].byte_offset, 0);
    assert_eq!(query.step_offsets[0].step_index, 0);
}

#[test]
fn quantifiers_combine_across_sequences_alternations_and_suffixes() {
    let (query, quantifiers) = parse(b"((identifier) @item (string)? @item)");
    assert_eq!(quantifiers, vec![CaptureQuantifier::OneOrMore]);
    assert_eq!(query.steps.len(), 2);
    assert_eq!(query.steps[1].alternative_index, 2);

    let (query, quantifiers) = parse(b"[(identifier) @left (string) @right] @both*");
    assert_eq!(quantifiers, vec![CaptureQuantifier::ZeroOrMore; 3]);
    assert_eq!(query.steps.len(), 4);
    assert_eq!(query.steps[0].alternative_index, 2);
    assert_eq!(query.steps[1].alternative_index, 3);
    assert_eq!(query.steps[2].alternative_index, 4);
    assert_eq!(query.steps[3].alternative_index, 0);
    assert!(query.steps[3].is_pass_through);
    assert!(query.steps[3].alternative_is_immediate);
    assert_eq!(query.steps[0].capture_ids, [0, 2, NONE]);
    assert_eq!(query.steps[2].capture_ids, [1, 2, NONE]);

    let (query, quantifiers) = parse(b"(identifier)+ ? @node");
    assert_eq!(quantifiers, vec![CaptureQuantifier::ZeroOrMore]);
    assert_eq!(query.steps.len(), 2);
    assert_eq!(query.steps[0].alternative_index, 2);
    assert_eq!(query.steps[1].alternative_index, 0);
    assert!(query.steps[1].is_pass_through);
}

#[test]
fn named_wildcards_missing_patterns_and_missing_prefixes() {
    let (query, _) = parse(b"_");
    assert_eq!(query.steps[0].symbol, WILDCARD_SYMBOL);
    assert!(!query.steps[0].is_named);
    let (query, _) = parse(b"(_)");
    assert_eq!(query.steps[0].symbol, WILDCARD_SYMBOL);
    assert!(query.steps[0].is_named);
    for source in [&b"(MISSING)"[..], &b"(M)"[..], &b"(MISS)"[..]] {
        let (query, _) = parse(source);
        assert_eq!(query.steps[0].symbol, WILDCARD_SYMBOL);
        assert!(query.steps[0].is_named);
        assert!(query.steps[0].is_missing);
    }
    for source in [&b"(MISSING identifier)"[..], &b"(M identifier)"[..]] {
        let (query, _) = parse(source);
        assert_eq!(
            query.steps[0].symbol,
            ts_language_symbol_for_name(&query.language, b"identifier", true)
        );
        assert!(query.steps[0].is_missing);
    }
    let (query, _) = parse(b"(MISSING \";\")");
    assert!(query.steps[0].is_missing);
    assert_eq!(
        query.steps[0].symbol,
        ts_language_symbol_for_name(&query.language, b";", false)
    );
}

#[test]
fn supertype_constraints_and_abi_gated_subtype_validation() {
    let (query, _) = parse(b"(primary_expression/identifier)");
    assert_eq!(
        query.steps[0].supertype_symbol,
        ts_language_symbol_for_name(&query.language, b"primary_expression", true)
    );
    assert_eq!(
        query.steps[0].symbol,
        ts_language_symbol_for_name(&query.language, b"identifier", true)
    );

    let mut query = new_query();
    let mut stream = stream_new(b"(primary_expression/program)");
    assert_eq!(
        ts_query__parse_pattern(&mut query, &mut stream, 0, false, &mut Vec::new()),
        QueryErrorCode::Structure
    );
    assert_eq!(stream.input, 0);

    static OLD_TABLES: LazyLock<LanguageTables> = LazyLock::new(|| {
        let mut tables = tables();
        tables.abi_version = 14;
        tables
    });
    query.language = Language::from(&*OLD_TABLES);
    let mut stream = stream_new(b"(primary_expression/program)");
    assert_eq!(
        ts_query__parse_pattern(&mut query, &mut stream, 0, false, &mut Vec::new()),
        QueryErrorCode::None
    );
}

#[test]
fn negated_field_limit_validates_even_the_discarded_ninth_field() {
    let (query, _) = parse(b"(identifier !name !body !name !body !name !body !name !body !value)");
    let name = ts_language_field_id_for_name(&query.language, b"name");
    let body = ts_language_field_id_for_name(&query.language, b"body");
    assert_eq!(query.steps[0].negated_field_list_id, 1);
    assert_eq!(
        query.negated_fields,
        vec![0, name, body, name, body, name, body, name, body, 0]
    );

    let mut query = new_query();
    let source = b"(identifier !name !body !name !body !name !body !name !body !bad)";
    let mut stream = stream_new(source);
    assert_eq!(
        ts_query__parse_pattern(&mut query, &mut stream, 0, false, &mut Vec::new()),
        QueryErrorCode::Field
    );
    assert_eq!(&source[stream.input..], b"bad)");
    // Field errors update only input, leaving the already-scanned lookahead.
    assert_eq!(stream.next, ')' as i32);
}

#[test]
fn syntax_and_lookup_errors_keep_c_error_positions() {
    for (source, kind, offset) in [
        ("[]", QueryErrorCode::Syntax, 1),
        ("[(identifier))", QueryErrorCode::Syntax, 13),
        ("(_ .)", QueryErrorCode::Syntax, 4),
        ("(bad_node)", QueryErrorCode::NodeType, 1),
        ("\"bad_token\"", QueryErrorCode::NodeType, 1),
        ("(MISSING bad_node)", QueryErrorCode::NodeType, 9),
        ("(MISSING \"bad_token\")", QueryErrorCode::NodeType, 10),
        ("(  identifier/program)", QueryErrorCode::Structure, 2),
        ("body:", QueryErrorCode::Syntax, 5),
        ("body:)", QueryErrorCode::Syntax, 5),
        ("(identifier) @", QueryErrorCode::Syntax, 14),
        (
            "((identifier) (#eq? @unknown \"a\"))",
            QueryErrorCode::Capture,
            21,
        ),
    ] {
        let mut query = new_query();
        let mut stream = stream_new(source.as_bytes());
        let result = ts_query__parse_pattern(&mut query, &mut stream, 0, false, &mut Vec::new());
        assert_eq!((result, stream.input), (kind, offset), "{source:?}");
    }
}

#[test]
fn nested_predicates_do_not_add_steps_and_capture_slots_stay_bounded() {
    let (query, quantifiers) = parse(b"((identifier) @a @b @c @d (#eq? @a \"x\") (.custom! @d))");
    assert_eq!(query.steps.len(), 1);
    assert_eq!(query.steps[0].capture_ids, [0, 1, 2]);
    assert_eq!(query.captures.slices.len(), 4);
    assert_eq!(quantifiers, vec![CaptureQuantifier::One; 4]);
    assert_eq!(query.predicate_steps.len(), 7);
    assert_eq!(query.predicate_steps[3].kind, PredicateStepKind::Done);
    assert_eq!(query.predicate_steps[6].kind, PredicateStepKind::Done);
}
