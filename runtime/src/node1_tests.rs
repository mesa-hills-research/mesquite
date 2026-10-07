use super::*;
use std::sync::{Arc, OnceLock};
use ts_port_tables::LanguageTables;

// These tests use hand-built subtrees so they can check the node helpers without
// depending on the parser, subtree constructors, or grammar scanners.
fn tree(root: Subtree) -> Tree {
    static TABLES: OnceLock<LanguageTables> = OnceLock::new();
    let tables = TABLES.get_or_init(|| LanguageTables {
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
    });
    Tree {
        root: Box::new(root),
        language: Language::from(tables),
        included_ranges: vec![],
    }
}

fn length(bytes: u32, row: u32, column: u32) -> Length {
    Length {
        bytes,
        extent: Point { row, column },
    }
}

fn leaf(symbol: u8, size: u8, padding: u8, flags: u8) -> Subtree {
    Subtree::Inline(InlineLeaf {
        symbol,
        size_bytes: size,
        padding_bytes: padding,
        padding_columns: padding,
        flags,
        ..InlineLeaf::default()
    })
}

fn branch(children: Vec<Subtree>) -> Subtree {
    Subtree::Heap(Arc::new(SubtreeHeapData {
        children: children.into(),
        payload: SubtreePayload::Branch(BranchData::default()),
        ..SubtreeHeapData::default()
    }))
}

#[test]
fn constructors_and_leaf_navigation() {
    let tree = tree(leaf(1, 3, 7, VISIBLE));
    let node = ts_node_new(&tree, &tree.root, length(20, 4, 7), 9);
    assert!(std::ptr::eq(node.tree, &tree));
    assert!(std::ptr::eq(ts_node__subtree(node), &*tree.root));
    assert_eq!(ts_node_start_byte(node), 20);
    assert_eq!(ts_node_start_point(node), Point { row: 4, column: 7 });
    assert_eq!(ts_node__alias(&node), 9);
    assert!(ts_node__null().is_none());

    let mut iterator = ts_node_iterate_children(&node);
    assert!(iterator.parent.is_null());
    assert!(iterator.alias_sequence.is_empty());
    assert_eq!(iterator.position, length_zero());
    assert!(ts_node_child_iterator_next(&mut iterator).is_none());
    assert!(ts_node__child(node, 0, true).is_none());
    assert!(ts_node__first_child_for_byte(node, 0, true).is_none());

    // The initial node is returned even when it is anonymous or the requested
    // range lies outside it. Only a reversed range produces a null node.
    for include_anonymous in [false, true] {
        let result = ts_node__descendant_for_byte_range(node, 0, 0, include_anonymous).unwrap();
        assert!(std::ptr::eq(result.subtree, node.subtree));
        assert!(ts_node__descendant_for_byte_range(node, 1, 0, include_anonymous).is_none());
        let start = Point { row: 1, column: 2 };
        let end = Point { row: 2, column: 1 };
        let result =
            ts_node__descendant_for_point_range(node, start, end, include_anonymous).unwrap();
        assert!(std::ptr::eq(result.subtree, node.subtree));
        assert!(ts_node__descendant_for_point_range(node, end, start, include_anonymous).is_none());
    }
}

#[test]
fn child_iterator_tracks_padding_extents_and_structural_aliases() {
    let extra = Subtree::Heap(Arc::new(SubtreeHeapData {
        padding: length(2, 1, 1),
        size: length(4, 1, 2),
        extra: true,
        children: vec![].into(),
        payload: SubtreePayload::Leaf,
        ..SubtreeHeapData::default()
    }));
    let tree = tree(branch(vec![
        leaf(1, 3, 7, VISIBLE),
        extra,
        leaf(2, 2, 1, VISIBLE),
    ]));
    let children = ts_subtree_children(&tree.root);
    let mut iterator = NodeChildIterator {
        parent: &tree.root,
        tree: &tree,
        position: length(10, 2, 3),
        child_index: 0,
        structural_child_index: 0,
        alias_sequence: &[11, 12],
    };
    assert!(!ts_node_child_iterator_done(&iterator));
    let first = ts_node_child_iterator_next(&mut iterator).unwrap();
    assert!(std::ptr::eq(first.subtree, &children[0]));
    assert_eq!(first.position, length(10, 2, 3));
    assert_eq!(first.alias, 11);
    assert_eq!(iterator.position, length(13, 2, 6));
    assert_eq!(iterator.structural_child_index, 1);

    let extra = ts_node_child_iterator_next(&mut iterator).unwrap();
    assert!(std::ptr::eq(extra.subtree, &children[1]));
    assert_eq!(extra.position, length(15, 3, 1));
    assert_eq!(extra.alias, 0);
    assert_eq!(iterator.position, length(19, 4, 2));
    assert_eq!(iterator.structural_child_index, 1);

    let last = ts_node_child_iterator_next(&mut iterator).unwrap();
    assert!(std::ptr::eq(last.subtree, &children[2]));
    assert_eq!(last.position, length(20, 4, 3));
    assert_eq!(last.alias, 12);
    assert_eq!(iterator.position, length(22, 4, 5));
    assert_eq!(iterator.structural_child_index, 2);
    assert!(ts_node_child_iterator_done(&iterator));
    assert!(ts_node_child_iterator_next(&mut iterator).is_none());
    assert_eq!(iterator.child_index, 3);
}

#[test]
fn relevance_and_cached_child_counts() {
    for flags in [0, VISIBLE, NAMED, VISIBLE | NAMED] {
        let tree = tree(leaf(1, 1, 0, flags));
        let node = ts_node_new(&tree, &tree.root, length_zero(), 0);
        assert_eq!(ts_node__is_relevant(node, true), flags & VISIBLE != 0);
        assert_eq!(ts_node__is_relevant(node, false), flags == VISIBLE | NAMED);
        assert_eq!(ts_node__relevant_child_count(node, true), 0);
        assert_eq!(ts_node__relevant_child_count(node, false), 0);
        let aliased = ts_node_new(&tree, &tree.root, length_zero(), 7);
        assert!(ts_node__is_relevant(aliased, true));
    }

    let tree = tree(Subtree::Heap(Arc::new(SubtreeHeapData {
        children: vec![leaf(1, 1, 0, VISIBLE)].into(),
        payload: SubtreePayload::Branch(BranchData {
            visible_child_count: 9,
            named_child_count: 4,
            ..BranchData::default()
        }),
        ..SubtreeHeapData::default()
    })));
    let node = ts_node_new(&tree, &tree.root, length_zero(), 0);
    assert_eq!(ts_node__relevant_child_count(node, true), 9);
    assert_eq!(ts_node__relevant_child_count(node, false), 4);
}

#[test]
fn trailing_empty_descendants_use_subtree_word_identity() {
    let empty = leaf(1, 0, 0, VISIBLE);
    let equal_inline = empty.clone();
    assert!(!std::ptr::eq(&empty, &equal_inline));
    let nested = branch(vec![equal_inline]);
    let parent = branch(vec![leaf(2, 1, 0, VISIBLE), nested.clone()]);
    assert!(ts_subtree_has_trailing_empty_descendant(&parent, &empty));
    assert!(ts_subtree_has_trailing_empty_descendant(&parent, &nested));
    assert!(!ts_subtree_has_trailing_empty_descendant(
        &parent,
        &branch(vec![])
    ));
    assert!(!ts_subtree_has_trailing_empty_descendant(&empty, &empty));

    // A zero-size child with nonzero padding terminates the suffix search.
    let parent = branch(vec![empty.clone(), leaf(3, 0, 1, VISIBLE)]);
    assert!(!ts_subtree_has_trailing_empty_descendant(&parent, &empty));
    let parent = branch(vec![empty.clone(), leaf(3, 1, 0, VISIBLE)]);
    assert!(!ts_subtree_has_trailing_empty_descendant(&parent, &empty));
}
