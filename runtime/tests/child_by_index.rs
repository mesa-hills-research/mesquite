//! Children looked up by index, in any order, are the children a cursor
//! steps through. A node's long list of children is held in a balanced tree
//! of hidden nodes, so a lookup takes time in proportion to the logarithm of
//! their number, and a loop over them by index in proportion to the number.

use std::time::Instant;

use tree_sitter::{InputEdit, Language, Node, Parser, Point, Tree};

fn parse(source: &str) -> Tree {
    let mut parser = Parser::new();
    parser
        .set_language(&Language::from(tree_sitter_cmake::language()))
        .unwrap();
    parser.parse(source, None).unwrap()
}

/// A JSON array of `items` objects cut off inside the next one: error
/// recovery makes the root an ERROR node that holds every object, comma and
/// bracket directly.
fn unclosed_json(items: usize) -> Tree {
    let mut source = (0..items).fold(String::from("[\n"), |mut source, ix| {
        source.push_str(&format!("  {{\"id\": {ix}, \"name\": \"item {ix}\"}},\n"));
        source
    });
    source.push_str("  {\"id\": 0, \"na");
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_json::LANGUAGE.into())
        .unwrap();
    parser.parse(&source, None).unwrap()
}

/// CMake with `commands` commands at the top level, some of them nested.
fn source(commands: usize) -> String {
    (0..commands)
        .map(|ix| match ix % 4 {
            0 => format!("set(VALUE_{ix} \"value {ix}\")\n"),
            1 => format!("if(VALUE_{ix})\n  message(STATUS \"{ix}\")\nendif()\n"),
            2 => format!("# comment {ix}\n"),
            _ => format!("add_library(lib{ix} STATIC a{ix}.c b{ix}.c)\n"),
        })
        .collect()
}

/// Every node under `node`, depth first.
fn nodes<'tree>(node: Node<'tree>, out: &mut Vec<Node<'tree>>) {
    out.push(node);
    let mut cursor = node.walk();
    for child in node.children(&mut cursor).collect::<Vec<_>>() {
        nodes(child, out);
    }
}

fn assert_lookups_match_the_cursor(node: Node<'_>) {
    let mut cursor = node.walk();
    let children = node.children(&mut cursor).collect::<Vec<_>>();
    let named = node.named_children(&mut cursor).collect::<Vec<_>>();
    let count = children.len();
    // In order, backwards, alternating between the two kinds of lookup, and
    // past the end.
    let orders = [
        (0..count).collect::<Vec<_>>(),
        (0..count).rev().collect(),
        (0..count).flat_map(|ix| [ix, ix / 2]).collect(),
    ];
    for order in orders {
        for ix in order {
            assert_eq!(node.child(ix), Some(children[ix]), "child {ix} of {node:?}");
            if let Some(named_child) = named.get(ix / 2) {
                assert_eq!(node.named_child(ix / 2), Some(*named_child));
            }
        }
    }
    assert_eq!(node.child(count), None);
    assert_eq!(node.named_child(named.len()), None);
}

#[test]
fn children_by_index_are_the_children_a_cursor_steps_through() {
    let tree = parse(&source(200));
    let mut all = Vec::new();
    nodes(tree.root_node(), &mut all);
    for node in all {
        assert_lookups_match_the_cursor(node);
    }
}

#[test]
fn children_of_a_flat_error_node_by_index_are_the_children_a_cursor_steps_through() {
    let tree = unclosed_json(300);
    let root = tree.root_node();
    assert!(root.is_error() && root.child_count() > 600);
    assert_lookups_match_the_cursor(root);
}

#[test]
fn children_by_index_follow_an_edit() {
    let source = source(50);
    let mut tree = parse(&source);
    let root = tree.root_node();
    let last = root.child_count() - 1;
    // Remember a place near the end, then append a command.
    assert!(root.child(last).is_some());

    let appended = format!("{source}message(\"appended\")\n");
    let end = |text: &str| Point::new(text.lines().count(), 0);
    tree.edit(&InputEdit {
        start_byte: source.len(),
        old_end_byte: source.len(),
        new_end_byte: appended.len(),
        start_position: end(&source),
        old_end_position: end(&source),
        new_end_position: end(&appended),
    });
    let mut parser = Parser::new();
    parser
        .set_language(&Language::from(tree_sitter_cmake::language()))
        .unwrap();
    let tree = parser.parse(&appended, Some(&tree)).unwrap();
    assert_lookups_match_the_cursor(tree.root_node());
    let root = tree.root_node();
    let last = root.child(root.child_count() - 1).unwrap();
    assert_eq!(&appended[last.byte_range()], "message(\"appended\")");
}

/// Run with `cargo test -p mesquite --release --test child_by_index --
/// --ignored --nocapture`.
#[test]
#[ignore = "timing, run by hand in release"]
fn a_loop_over_children_by_index() {
    // A root of commands, which hidden nodes hold in a balanced tree, and an
    // ERROR root, which holds them all directly.
    for (items, flat) in [
        (1_000, false),
        (10_000, false),
        (40_000, false),
        (1_000, true),
        (5_000, true),
        (20_000, true),
    ] {
        let tree = if flat {
            unclosed_json(items)
        } else {
            parse(&source(items))
        };
        let root = tree.root_node();
        let start = Instant::now();
        let mut bytes = 0;
        for ix in 0..root.child_count() {
            bytes += root.child(ix).unwrap().byte_range().len();
        }
        let by_index = start.elapsed();
        let start = Instant::now();
        let mut cursor = root.walk();
        for child in root.children(&mut cursor) {
            bytes -= child.byte_range().len();
        }
        let by_cursor = start.elapsed();
        assert_eq!(bytes, 0);
        println!(
            "{} children of {}: by index {by_index:.2?}, with a cursor {by_cursor:.2?}",
            root.child_count(),
            root.kind()
        );
    }
}
