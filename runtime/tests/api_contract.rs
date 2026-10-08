//! Code written against tree-sitter's Rust API that must keep compiling: signatures,
//! borrows and lifetimes. The one runnable test checks the thread-safety traits.
#![allow(dead_code)]

use std::{cell::Cell, num::NonZeroU16, rc::Rc};
use tree_sitter::{
    InputEdit, Language, LanguageError, LanguageRef, Node, ParseOptions, ParseState, Parser, Point,
    Range, Tree, TreeCursor,
};

fn parser_contract(
    parser: &mut Parser,
    language: &Language,
    source: &[u8],
    old_tree: Option<&Tree>,
    ranges: &[Range],
) -> Result<Option<Tree>, LanguageError> {
    parser.set_language(language)?;
    let _: Option<LanguageRef<'_>> = parser.language();
    parser.reset();
    let _ = parser.set_included_ranges(ranges);
    Ok(parser.parse(source, old_tree))
}

fn callbacks_can_borrow_non_send_data(parser: &mut Parser, source: &[u8]) -> Option<Tree> {
    let count = Rc::new(Cell::new(0));
    let mut progress = |state: &ParseState| {
        let _: usize = state.current_byte_offset();
        let _: bool = state.has_error();
        count.set(count.get() + 1);
        false
    };
    let mut read = |byte: usize, _: Point| &source[byte.min(source.len())..];
    let options = ParseOptions::new().progress_callback(&mut progress);
    parser.parse_with_options(&mut read, None, Some(options))
}

fn children_of_temporary_node<'cursor, 'tree>(
    tree: &'tree Tree,
    cursor: &'cursor mut TreeCursor<'tree>,
) -> impl ExactSizeIterator<Item = Node<'tree>> + 'cursor + use<'cursor, 'tree> {
    tree.root_node().children(cursor)
}

fn named_children_of_temporary_node<'cursor, 'tree>(
    tree: &'tree Tree,
    cursor: &'cursor mut TreeCursor<'tree>,
) -> impl ExactSizeIterator<Item = Node<'tree>> + 'cursor + use<'cursor, 'tree> {
    tree.root_node().named_children(cursor)
}

fn field_children_do_not_borrow_name<'cursor, 'tree>(
    tree: &'tree Tree,
    cursor: &'cursor mut TreeCursor<'tree>,
) -> impl Iterator<Item = Node<'tree>> + 'cursor + use<'cursor, 'tree> {
    let name = String::from("body");
    tree.root_node().children_by_field_name(&name, cursor)
}

fn field_id_contract<'cursor, 'tree>(
    node: Node<'tree>,
    cursor: &'cursor mut TreeCursor<'tree>,
    field: NonZeroU16,
) -> impl Iterator<Item = Node<'tree>> + 'cursor + use<'cursor, 'tree> {
    node.children_by_field_id(field, cursor)
}

fn changed_ranges_do_not_borrow_trees(
    old: Tree,
    new: Tree,
) -> impl ExactSizeIterator<Item = Range> {
    old.changed_ranges(&new)
}

fn tree_contract(tree: &mut Tree, edit: &InputEdit) {
    let _: Tree = tree.clone();
    let _: LanguageRef<'_> = tree.language();
    let _: Node<'_> = tree.root_node_with_offset(1, Point::new(0, 1));
    let _: TreeCursor<'_> = tree.walk();
    tree.edit(edit);
}

fn grammar_contract(tables: &'static tree_sitter_language::LanguageTables) -> Language {
    let language: Language = tables.into();
    let _: Option<&'static str> = language.name();
    let _: usize = language.abi_version();
    let _: usize = language.node_kind_count();
    let _: Option<&'static str> = language.node_kind_for_id(0);
    let _: u16 = language.id_for_node_kind("identifier", true);
    let _: bool = language.node_kind_is_named(0);
    let _: bool = language.node_kind_is_visible(0);
    let _: usize = language.field_count();
    let _: Option<&'static str> = language.field_name_for_id(1);
    let _: Option<NonZeroU16> = language.field_id_for_name(b"body");
    language
}

#[test]
fn thread_traits_are_structural() {
    fn send<T: Send>() {}
    fn shared<T: Send + Sync + Clone>() {}
    send::<Parser>();
    shared::<Language>();
    shared::<Tree>();
    shared::<Node<'static>>();
    shared::<TreeCursor<'static>>();
}
