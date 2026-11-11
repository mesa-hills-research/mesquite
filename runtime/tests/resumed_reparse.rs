//! A reparse that is canceled again and again, and resumed each time, takes
//! up node reuse where it stopped. It ends with the same tree and changed
//! ranges as a reparse that runs through.

use tree_sitter::{InputEdit, ParseOptions, ParseState, Parser, Point, Range, Tree};

fn items(count: usize) -> String {
    let mut text = String::from("[\n");
    for i in 0..count {
        text.push_str(&format!(
            "  {{\"id\": {i}, \"name\": \"item {i}\", \"tags\": [\"a\", \"b\"]}},\n"
        ));
    }
    text.push_str("  null\n]\n");
    text
}

fn point_at(text: &str, offset: usize) -> Point {
    let before = &text[..offset];
    let row = before.matches('\n').count();
    let column = offset - before.rfind('\n').map_or(0, |ix| ix + 1);
    Point::new(row, column)
}

/// The edited text, and the edit that turns `old` into it.
fn splice(old: &str, start: usize, removed: usize, inserted: &str) -> (String, InputEdit) {
    let new = format!("{}{}{}", &old[..start], inserted, &old[start + removed..]);
    let edit = InputEdit {
        start_byte: start,
        old_end_byte: start + removed,
        new_end_byte: start + inserted.len(),
        start_position: point_at(old, start),
        old_end_position: point_at(old, start + removed),
        new_end_position: point_at(&new, start + inserted.len()),
    };
    (new, edit)
}

fn ranges(old: &Tree, new: &Tree) -> Vec<Range> {
    old.changed_ranges(new).collect()
}

#[test]
fn a_reparse_resumed_after_each_cancellation_matches_one_that_runs_through() {
    let language = tree_sitter_json::LANGUAGE.into();
    let mut parser = Parser::new();
    parser.set_language(&language).unwrap();
    let old_text = items(400);
    let middle = old_text.find("\"item 200\"").unwrap();
    let end = old_text.len() - "  null\n]\n".len();
    let cases = [
        splice(&old_text, middle, 0, "\"x\": 1, \"y\": "),
        splice(&old_text, middle, 12, ""),
        splice(&old_text, end, 0, "  {\"id\": -1},\n"),
        splice(&old_text, 3, 0, "[[[["),
        splice(&old_text, middle, 1, "{"),
    ];
    for (new_text, edit) in cases {
        let mut old = parser.parse(&old_text, None).unwrap();
        old.edit(&edit);
        let expected = parser.parse(&new_text, Some(&old)).unwrap();

        let bytes = new_text.as_bytes();
        let mut read = |offset: usize, _: Point| bytes.get(offset..).unwrap_or_default();
        let mut calls = 0;
        let tree = loop {
            let mut cancel = |_: &ParseState| true;
            let options = ParseOptions::new().progress_callback(&mut cancel);
            calls += 1;
            if let Some(tree) = parser.parse_with_options(&mut read, Some(&old), Some(options)) {
                break tree;
            }
        };
        assert!(calls > 10, "{calls} calls");
        assert_eq!(tree.root_node().to_sexp(), expected.root_node().to_sexp());
        assert_eq!(ranges(&old, &tree), ranges(&old, &expected));
    }
}
