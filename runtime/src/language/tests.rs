use super::*;
use std::sync::LazyLock;
use ts_port_tables::{ExternalScannerTables, FieldMapEntry, MapSlice};

fn unused_lex(_: &mut dyn ts_port_tables::Lexer, _: StateId) -> bool {
    panic!("language table tests do not lex")
}

fn unused_scanner() -> Box<dyn ts_port_tables::ExternalScanner> {
    panic!("language table tests do not create scanners")
}

fn leak(tables: LanguageTables) -> Language {
    Language::from(&*Box::leak(Box::new(tables)))
}

static LANGUAGE: LazyLock<Language> = LazyLock::new(|| {
    use ParseActionEntry::{Action, Header};
    let named = SymbolMetadata {
        visible: true,
        named: true,
        supertype: false,
    };
    let reduce = ParseAction::Reduce {
        symbol: 4,
        child_count: 1,
        dynamic_precedence: -1,
        production_id: 1,
    };
    let shift = ParseAction::Shift {
        state: 2,
        extra: false,
        repetition: true,
    };
    leak(LanguageTables {
        abi_version: 15,
        name: Some("fixture".into()),
        metadata: Some(LanguageMetadata {
            major_version: 1,
            minor_version: 2,
            patch_version: 3,
        }),
        symbol_count: 6,
        alias_count: 1,
        token_count: 4,
        external_token_count: 2,
        state_count: 4,
        large_state_count: 2,
        production_id_count: 2,
        field_count: 2,
        max_alias_sequence_length: 3,
        parse_table: vec![1, 3, 6, 8, 1, 1, 0, 10, 12, 0, 2, 2],
        // State 2 visits symbols in group order: 2, 3, 1, 4, 5. State 3 is empty.
        small_parse_table: vec![3, 6, 2, 2, 3, 3, 1, 1, 1, 2, 4, 5, 0],
        small_parse_table_map: vec![0, 12],
        parse_actions: vec![
            Header {
                count: 0,
                reusable: false,
            },
            Header {
                count: 1,
                reusable: false,
            },
            Action(ParseAction::Accept),
            Header {
                count: 2,
                reusable: true,
            },
            Action(reduce),
            Action(shift),
            Header {
                count: 1,
                reusable: true,
            },
            Action(ParseAction::Shift {
                state: 0,
                extra: true,
                repetition: false,
            }),
            Header {
                count: 1,
                reusable: false,
            },
            Action(ParseAction::Recover),
            Header {
                count: 1,
                reusable: false,
            },
            Action(reduce),
            Header {
                count: 2,
                reusable: true,
            },
            Action(shift),
            Action(reduce),
        ],
        symbol_names: [
            "end",
            "word",
            ";",
            "_hidden",
            "expr",
            "_expression",
            "alias",
        ]
        .map(str::to_owned)
        .to_vec(),
        field_names: vec![None, Some("body".into()), Some("type".into())],
        field_map_slices: vec![
            MapSlice::default(),
            MapSlice {
                index: 0,
                length: 2,
            },
        ],
        field_map_entries: vec![
            FieldMapEntry {
                field_id: 1,
                child_index: 0,
                inherited: false,
            },
            FieldMapEntry {
                field_id: 2,
                child_index: 2,
                inherited: true,
            },
        ],
        symbol_metadata: vec![
            SymbolMetadata::default(),
            named,
            SymbolMetadata {
                visible: true,
                ..SymbolMetadata::default()
            },
            SymbolMetadata::default(),
            named,
            SymbolMetadata {
                visible: false,
                named: true,
                supertype: true,
            },
            named,
        ],
        public_symbol_map: vec![0, 6, 2, 3, 4, 5, 6],
        alias_map: vec![1, 2, 1, 6, 4, 1, 6, 0],
        alias_sequences: vec![0, 0, 0, 0, 6, 0],
        lex_modes: vec![
            LexMode::default(),
            LexMode {
                lex_state: 7,
                external_lex_state: 2,
                reserved_word_set_id: 2,
            },
            LexMode {
                lex_state: 8,
                external_lex_state: 1,
                reserved_word_set_id: 1,
            },
            LexMode::default(),
        ],
        lex_fn: unused_lex,
        keyword_lex_fn: None,
        keyword_capture_token: 0,
        external_scanner: Some(ExternalScannerTables {
            states: vec![false, false, true, false, false, true],
            symbol_map: vec![1, 2],
            create: unused_scanner,
        }),
        primary_state_ids: vec![0, 1, 1, 3],
        reserved_words: vec![0, 0, 0, 2, 0, 1, 1, 2, 3],
        max_reserved_word_set_size: 3,
        supertype_count: 1,
        supertype_symbols: vec![5],
        supertype_map_slices: vec![
            MapSlice::default(),
            MapSlice::default(),
            MapSlice::default(),
            MapSlice::default(),
            MapSlice::default(),
            MapSlice {
                index: 0,
                length: 2,
            },
        ],
        supertype_map_entries: vec![1, 4],
    })
});

#[test]
fn native_language_identity_and_abi_gates() {
    let language = &*LANGUAGE;
    let copied = ts_language_copy(language);
    assert_eq!(*language, copied);
    ts_language_delete(copied);
    assert_eq!(ts_language_version(language), 15);
    assert_eq!(ts_language_abi_version(language), 15);
    assert_eq!(ts_language_symbol_count(language), 7);
    assert_eq!(ts_language_state_count(language), 4);
    assert_eq!(ts_language_field_count(language), 2);
    assert_eq!(ts_language_name(language), Some("fixture"));
    assert_eq!(ts_language_metadata(language).unwrap().minor_version, 2);
    assert_eq!(ts_language_supertypes(language), &[5]);
    assert_eq!(ts_language_subtypes(language, 5), &[1, 4]);
    assert!(ts_language_subtypes(language, 4).is_empty());
    assert!(ts_language_subtypes(language, BUILTIN_SYM_ERROR).is_empty());
    assert!(ts_language_state_is_primary(language, 1));
    assert!(!ts_language_state_is_primary(language, 2));

    let mut tables = language.tables.clone();
    tables.abi_version = 14;
    let old = leak(tables.clone());
    assert_eq!(ts_language_name(&old), None);
    assert_eq!(ts_language_metadata(&old), None);
    assert!(ts_language_supertypes(&old).is_empty());
    assert!(ts_language_subtypes(&old, 5).is_empty());
    assert_eq!(
        ts_language_lex_mode_for_state(&old, 2),
        LexMode {
            lex_state: 8,
            external_lex_state: 1,
            reserved_word_set_id: 0,
        }
    );
    assert!(!ts_language_is_reserved_word(&old, 2, 2));
    assert!(!ts_language_state_is_primary(&old, 2));
    tables.abi_version = 13;
    tables.primary_state_ids.clear();
    assert!(ts_language_state_is_primary(&leak(tables), 2));
}

#[test]
fn symbol_names_metadata_and_error_prefixes() {
    let language = &*LANGUAGE;
    for name in [b"".as_slice(), b"E", b"ERR", b"ERROR", b"ERROR\0ignored"] {
        assert_eq!(
            ts_language_symbol_for_name(language, name, true),
            BUILTIN_SYM_ERROR
        );
    }
    for name in [
        b"ERRORS".as_slice(),
        b"ER\0",
        b"_hidden",
        b"missing",
        b"wor",
    ] {
        assert_eq!(ts_language_symbol_for_name(language, name, true), 0);
    }
    assert_eq!(ts_language_symbol_for_name(language, b"ERROR", false), 0);
    assert_eq!(ts_language_symbol_for_name(language, b"word", true), 6);
    assert_eq!(ts_language_symbol_for_name(language, b"word", false), 0);
    assert_eq!(
        ts_language_symbol_for_name(language, b"_expression", true),
        5
    );
    assert_eq!(ts_language_symbol_for_name(language, b";", false), 2);
    assert_eq!(ts_language_symbol_name(language, 6), Some("alias"));
    assert_eq!(ts_language_symbol_name(language, 7), None);
    assert_eq!(
        ts_language_symbol_name(language, BUILTIN_SYM_ERROR),
        Some("ERROR")
    );
    assert_eq!(
        ts_language_symbol_name(language, BUILTIN_SYM_ERROR_REPEAT),
        Some("_ERROR")
    );
    assert_eq!(ts_language_public_symbol(language, 1), 6);
    assert_eq!(
        ts_language_public_symbol(language, BUILTIN_SYM_ERROR),
        BUILTIN_SYM_ERROR
    );
    for (symbol, expected) in [
        (1, SymbolType::Regular),
        (2, SymbolType::Anonymous),
        (3, SymbolType::Auxiliary),
        (5, SymbolType::Supertype),
        (BUILTIN_SYM_ERROR, SymbolType::Regular),
        (BUILTIN_SYM_ERROR_REPEAT, SymbolType::Auxiliary),
    ] {
        assert_eq!(ts_language_symbol_type(language, symbol), expected);
    }
}

#[test]
fn fields_aliases_and_external_tokens() {
    let language = &*LANGUAGE;
    assert_eq!(ts_language_field_name_for_id(language, 0), None);
    assert_eq!(ts_language_field_name_for_id(language, 2), Some("type"));
    assert_eq!(ts_language_field_name_for_id(language, 3), None);
    for (name, id) in [
        ("body", 1),
        ("type", 2),
        ("", 0),
        ("bod", 0),
        ("a", 0),
        ("z", 0),
    ] {
        assert_eq!(ts_language_field_id_for_name(language, name.as_bytes()), id);
    }
    assert_eq!(ts_language_field_id_for_name(language, b"body\0"), 0);
    assert_eq!(
        ts_language_field_map(language, 1),
        &language.tables.field_map_entries
    );
    assert!(ts_language_field_map(language, 0).is_empty());
    assert_eq!(ts_language_alias_sequence(language, 1), &[0, 6, 0]);
    assert!(ts_language_alias_sequence(language, 0).is_empty());
    assert_eq!(ts_language_alias_at(language, 0, 999), 0);
    assert_eq!(ts_language_alias_at(language, 1, 1), 6);
    assert_eq!(ts_language_aliases_for_symbol(language, 1), &[1, 6]);
    assert_eq!(ts_language_aliases_for_symbol(language, 4), &[6]);
    assert_eq!(ts_language_aliases_for_symbol(language, 2), &[2]);
    assert_eq!(ts_language_aliases_for_symbol(language, 5), &[5]);
    assert_eq!(ts_language_enabled_external_tokens(language, 0), None);
    assert_eq!(
        ts_language_enabled_external_tokens(language, 1),
        Some([true, false].as_slice())
    );
    assert_eq!(
        ts_language_enabled_external_tokens(language, 2),
        Some([false, true].as_slice())
    );

    let mut tables = language.tables.clone();
    tables.field_count = 0;
    tables.field_names.clear();
    tables.field_map_slices.clear();
    tables.field_map_entries.clear();
    let no_fields = leak(tables);
    assert_eq!(ts_language_field_name_for_id(&no_fields, 0), None);
    assert_eq!(ts_language_field_id_for_name(&no_fields, b"body"), 0);
    assert!(ts_language_field_map(&no_fields, 1).is_empty());
}

#[test]
fn reserved_words_stop_at_zero_after_comparing() {
    let language = &*LANGUAGE;
    assert!(!ts_language_is_reserved_word(language, 0, 0));
    assert!(ts_language_is_reserved_word(language, 2, 2));
    assert!(ts_language_is_reserved_word(language, 2, 0));
    assert!(!ts_language_is_reserved_word(language, 2, 1));
    assert!(ts_language_is_reserved_word(language, 1, 3));
    assert!(!ts_language_is_reserved_word(language, 1, 0));
}

#[test]
fn table_actions_and_successors() {
    let language = &*LANGUAGE;
    let entry = ts_language_table_entry(language, 0, 1);
    assert!(entry.is_reusable());
    assert_eq!(entry.actions().len(), 2);
    assert!(ts_language_has_reduce_action(language, 0, 1));
    assert_eq!(ts_language_next_state(language, 0, 1), 2);
    // Only the first action matters for has_reduce; only the last for next_state.
    assert!(!ts_language_has_reduce_action(language, 1, 2));
    assert_eq!(ts_language_next_state(language, 1, 2), 0);
    assert_eq!(ts_language_next_state(language, 1, 1), 0);
    assert_eq!(ts_language_next_state(language, 0, 0), 0);
    assert_eq!(ts_language_next_state(language, 0, 3), 0);
    assert_eq!(ts_language_next_state(language, 2, 2), 2); // extra shift
    assert_eq!(ts_language_next_state(language, 1, 4), 2); // nonterminal
    assert_eq!(ts_language_next_state(language, 2, 4), 1);
    assert_eq!(ts_language_lookup(language, 2, 0), 0);
    assert!(!ts_language_has_actions(language, 2, 0));
    assert!(ts_language_has_actions(language, 2, 1));
    assert!(ts_language_table_entry(language, 1, 0).actions().is_empty());
    for symbol in [BUILTIN_SYM_ERROR, BUILTIN_SYM_ERROR_REPEAT] {
        let entry = ts_language_table_entry(language, StateId::MAX, symbol);
        assert!(entry.actions().is_empty());
        assert!(!entry.is_reusable());
        assert_eq!(ts_language_next_state(language, StateId::MAX, symbol), 0);
    }
}

#[test]
fn lookahead_order_reset_and_group_reuse() {
    let language = &*LANGUAGE;
    assert!(ts_lookahead_iterator_new(language, 4).is_none());
    let mut iterator = ts_lookahead_iterator_new(language, 0).unwrap();
    assert_eq!(*ts_lookahead_iterator_language(&iterator), *language);
    assert_eq!(
        ts_lookahead_iterator_current_symbol(&iterator),
        BUILTIN_SYM_ERROR
    );
    assert_eq!(
        ts_lookahead_iterator_current_symbol_name(&iterator),
        Some("ERROR")
    );
    for (state, expected) in [
        (0, vec![0, 1, 2, 3, 4, 5]),
        (1, vec![1, 2, 4, 5]),
        (2, vec![2, 3, 1, 4, 5]),
        (3, vec![]),
    ] {
        assert!(ts_lookahead_iterator_reset_state(&mut iterator, state));
        let mut symbols = Vec::new();
        while ts_lookahead_iterator_next(&mut iterator) {
            symbols.push(iterator.symbol);
            if u32::from(iterator.symbol) < language.tables.token_count {
                assert_eq!(
                    iterator.actions,
                    ts_language_actions(language, state, iterator.symbol)
                );
                assert_eq!(iterator.next_state, 0);
            } else {
                assert!(iterator.actions.is_empty());
                assert_eq!(
                    iterator.next_state,
                    ts_language_lookup(language, state, iterator.symbol)
                );
            }
        }
        assert_eq!(symbols, expected);
    }
    assert!(ts_lookahead_iterator_reset_state(&mut iterator, 2));
    assert!(ts_lookahead_iterator_next(&mut iterator));
    let index = iterator.data_index;
    let symbol = iterator.symbol;
    assert!(!ts_lookahead_iterator_reset_state(&mut iterator, 4));
    let other = leak(language.tables.clone());
    assert!(!ts_lookahead_iterator_reset(&mut iterator, &other, 4));
    assert_eq!(iterator.data_index, index);
    assert_eq!(iterator.symbol, symbol);
    assert_eq!(iterator.language, *language);
    assert!(ts_lookahead_iterator_reset(&mut iterator, &other, 1));
    assert_eq!(iterator.language, other);
    assert_eq!(iterator.symbol, BUILTIN_SYM_ERROR);
    ts_lookahead_iterator_delete(iterator);
}

#[test]
fn dot_names_escape_bytes_and_propagate_write_errors() {
    let mut tables = LANGUAGE.tables.clone();
    tables.symbol_names[2] = "a\"\\\n\t\ré\0ignored".into();
    let language = leak(tables);
    let mut output = Vec::new();
    ts_language_write_symbol_as_dot_string(&language, &mut output, 2).unwrap();
    assert_eq!(output, "a\\\"\\\\\\n\\t\ré".as_bytes());
    let mut too_small = [0u8; 1];
    assert!(
        ts_language_write_symbol_as_dot_string(&language, &mut too_small.as_mut_slice(), 2)
            .is_err()
    );
}

#[test]
fn all_grammar_states_agree_between_lookup_and_iteration() {
    // Decode the real host tables without compiling lexers or invoking scanners.
    let grammars: &[(&str, &[u8], bool)] = &[
        (
            "c",
            include_bytes!("../../../grammars/c/src/tables.bin"),
            false,
        ),
        (
            "cpp",
            include_bytes!("../../../grammars/cpp/src/tables.bin"),
            true,
        ),
        (
            "go",
            include_bytes!("../../../grammars/go/src/tables.bin"),
            false,
        ),
        (
            "java",
            include_bytes!("../../../grammars/java/src/tables.bin"),
            false,
        ),
        (
            "javascript",
            include_bytes!("../../../grammars/javascript/src/tables.bin"),
            true,
        ),
        (
            "python",
            include_bytes!("../../../grammars/python/src/tables.bin"),
            true,
        ),
        (
            "rust",
            include_bytes!("../../../grammars/rust/src/tables.bin"),
            true,
        ),
        (
            "typescript",
            include_bytes!("../../../grammars/typescript/src/tables.bin"),
            true,
        ),
        (
            "tsx",
            include_bytes!("../../../grammars/tsx/src/tables.bin"),
            true,
        ),
    ];
    let mut cache = ParseTableCache::default();
    for &(name, blob, has_scanner) in grammars {
        cache.clear();
        let language = leak(LanguageTables::decode(
            blob,
            unused_lex,
            Some(unused_lex),
            has_scanner.then_some(unused_scanner),
        ));
        let tables = language.tables;
        for state in 0..tables.state_count as StateId {
            let mut expected = vec![0; tables.symbol_count as usize];
            let mut expected_order = Vec::new();
            if u32::from(state) < tables.large_state_count {
                let start = state as usize * tables.symbol_count as usize;
                expected.copy_from_slice(
                    &tables.parse_table[start..start + tables.symbol_count as usize],
                );
                expected_order.extend(
                    expected
                        .iter()
                        .enumerate()
                        .filter_map(|(symbol, &value)| (value != 0).then_some(symbol as Symbol)),
                );
            } else {
                let offset = tables.small_parse_table_map
                    [(u32::from(state) - tables.large_state_count) as usize]
                    as usize;
                let mut data = &tables.small_parse_table[offset..];
                let groups = data[0];
                data = &data[1..];
                for _ in 0..groups {
                    let value = data[0];
                    let count = data[1] as usize;
                    for &symbol in &data[2..2 + count] {
                        expected[symbol as usize] = value;
                        expected_order.push(symbol);
                    }
                    data = &data[2 + count..];
                }
            }
            for (symbol, &value) in expected.iter().enumerate() {
                assert_eq!(
                    ts_language_lookup(&language, state, symbol as Symbol),
                    value,
                    "{name} state {state} symbol {symbol}"
                );
            }
            // Check both misses and hits, including zero entries. Reusing the
            // cache across states exercises collisions without changing results.
            for (symbol, &value) in expected.iter().enumerate() {
                for _ in 0..2 {
                    assert_eq!(cache.lookup(&language, state, symbol as Symbol), value);
                }
            }
            // Compare the thin header-backed handle with the independently
            // decoded host action run for every terminal in every state.
            for (symbol, &value) in expected
                .iter()
                .enumerate()
                .take(tables.token_count as usize)
            {
                let entry = cache.table_entry(&language, state, symbol as Symbol);
                let (reusable, actions) = tables.action_list(value as usize);
                assert_eq!(
                    entry.actions(), actions,
                    "{name} state {state} symbol {symbol}"
                );
                assert_eq!(
                    entry.is_reusable(), reusable,
                    "{name} state {state} symbol {symbol}"
                );
            }
            let mut iterator = ts_language_lookaheads(&language, state);
            for symbol in expected_order {
                assert!(
                    ts_lookahead_iterator__next(&mut iterator),
                    "{name} state {state}"
                );
                assert_eq!(iterator.symbol, symbol, "{name} state {state}");
                assert_eq!(iterator.table_value, expected[symbol as usize]);
                if u32::from(symbol) < tables.token_count {
                    assert_eq!(
                        iterator.actions,
                        tables.action_list(expected[symbol as usize] as usize).1
                    );
                    assert_eq!(iterator.next_state, 0);
                } else {
                    assert!(iterator.actions.is_empty());
                    assert_eq!(iterator.next_state, expected[symbol as usize]);
                }
            }
            assert!(
                !ts_lookahead_iterator__next(&mut iterator),
                "{name} state {state}"
            );
        }
    }
}

#[test]
fn parse_table_cache_clears_on_language_change() {
    let language = &*LANGUAGE;
    let mut cache = ParseTableCache::default();
    assert_eq!(cache.lookup(language, 2, 2), 6);
    let mut changed = language.tables.clone();
    changed.small_parse_table[1] = 3;
    let changed = leak(changed);
    cache.clear();
    assert_eq!(cache.lookup(&changed, 2, 2), 3);
    for language in [language, &changed] {
        cache.clear();
        for state in 0..language.tables.state_count as StateId {
            for symbol in 0..language.tables.symbol_count as Symbol {
                assert_eq!(cache.next_state(language, state, symbol), ts_language_next_state(language, state, symbol));
                if u32::from(symbol) < language.tables.token_count {
                    let cached = cache.table_entry(language, state, symbol);
                    let expected = ts_language_table_entry(language, state, symbol);
                    assert_eq!(cached.actions(), expected.actions());
                    assert_eq!(cached.is_reusable(), expected.is_reusable());
                }
            }
        }
        for symbol in [BUILTIN_SYM_ERROR, BUILTIN_SYM_ERROR_REPEAT] {
            assert_eq!(cache.next_state(language, StateId::MAX, symbol), 0);
            assert!(cache.table_entry(language, StateId::MAX, symbol).actions().is_empty());
        }
    }
}

#[test]
fn parser_setting_language_invalidates_parse_table_cache() {
    use crate::parser::{ts_parser_new, ts_parser_set_language};
    let language = &*LANGUAGE;
    let mut parser = ts_parser_new();
    assert!(ts_parser_set_language(&mut parser, Some(language)));
    assert_eq!(parser.parse_table_cache.lookup(language, 2, 2), 6);
    let mut changed = language.tables.clone();
    changed.small_parse_table[1] = 3;
    let changed = leak(changed);
    assert!(ts_parser_set_language(&mut parser, Some(&changed)));
    assert_eq!(parser.parse_table_cache.lookup(&changed, 2, 2), 3);
    assert!(ts_parser_set_language(&mut parser, None));
    assert!(ts_parser_set_language(&mut parser, Some(language)));
    assert_eq!(parser.parse_table_cache.lookup(language, 2, 2), 6);
}

#[test]
fn table_entry_borrows_its_header_without_widening_the_handle() {
    assert_eq!(size_of::<TableEntry>(), size_of::<&[ParseActionEntry]>());
    let empty = TableEntry::default();
    assert!(empty.actions().is_empty());
    assert!(!empty.is_reusable());

    for index in [0, 1, 3, 6, 8, 10, 12] {
        let entry = TableEntry::new(&LANGUAGE, index);
        let (reusable, actions) = LANGUAGE.tables.action_list(index as usize);
        assert_eq!(entry.actions(), actions);
        assert_eq!(entry.is_reusable(), reusable);
        assert!(std::ptr::eq(entry.actions(), actions));
    }
}

#[test]
fn parse_table_cache_checks_full_keys_and_caches_zero_values() {
    // The raw lookup accepts every u16 state/symbol pair in compressed rows.
    // In particular (MAX, MAX) needs the 33rd bit of the nonzero cache key.
    let mut tables = LANGUAGE.tables.clone();
    tables.state_count = u32::from(StateId::MAX) + 1;
    tables.large_state_count = 0;
    tables.symbol_count = u32::from(Symbol::MAX) + 1;
    tables.parse_table.clear();
    tables.small_parse_table = vec![1, 7, 1, 0, 1, u16::MAX, 2, 0, u16::MAX];
    tables.small_parse_table_map = vec![0; tables.state_count as usize];
    tables.small_parse_table_map[0] = 4;
    tables.small_parse_table_map[StateId::MAX as usize] = 4;
    let language = leak(tables);
    let mut cache = ParseTableCache::default();

    // These two distinct keys collide under the cache's state*131+symbol
    // index. One is an absent symbol, so zero must be cached unambiguously.
    for _ in 0..3 {
        assert_eq!(cache.lookup(&language, 0, 131), 0);
        assert_eq!(cache.lookup(&language, 0, 131), 0);
        assert_eq!(cache.lookup(&language, 1, 0), 7);
        assert_eq!(cache.lookup(&language, 1, 131), 0);
        assert_eq!(cache.lookup(&language, 0, 0), u16::MAX);
        assert_eq!(cache.lookup(&language, StateId::MAX, Symbol::MAX), u16::MAX);
        assert_eq!(cache.lookup(&language, StateId::MAX, Symbol::MAX), u16::MAX);
        cache.clear();
    }
}
