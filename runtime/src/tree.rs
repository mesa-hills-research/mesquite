use crate::{
    get_changed_ranges::{ts_range_array_get_changed_ranges, ts_subtree_get_changed_ranges},
    language::{Language, ts_language_copy, ts_language_delete},
    length::{Length, length_add},
    node::{Node, ts_node_new},
    point::{POINT_MAX, Point, point_add, point_sub},
    subtree::{
        Subtree, ts_subtree_edit, ts_subtree_padding, ts_subtree_pool_delete, ts_subtree_pool_new,
        ts_subtree_print_dot_graph, ts_subtree_release, ts_subtree_retain,
    },
    tree_cursor::{TreeCursor, ts_tree_cursor_init},
    types::{InputEdit, Range},
};
#[derive(Debug)]
pub struct Tree {
    pub(crate) root: Box<Subtree>,
    pub(crate) language: Language,
    pub(crate) included_ranges: Vec<Range>,
}

pub(crate) fn ts_tree_new(root: Subtree, language: &Language, included_ranges: &[Range]) -> Tree {
    Tree {
        root: Box::new(root),
        language: ts_language_copy(language),
        included_ranges: included_ranges.to_vec(),
    }
}

pub(crate) fn ts_tree_copy(tree: &Tree) -> Tree {
    ts_tree_new(
        ts_subtree_retain(&tree.root),
        &tree.language,
        &tree.included_ranges,
    )
}

pub(crate) fn ts_tree_delete(tree: &mut Tree) {
    let mut pool = ts_subtree_pool_new(0);
    ts_subtree_release(&mut pool, std::mem::take(&mut *tree.root));
    ts_subtree_pool_delete(&mut pool);
    ts_language_delete(tree.language);
    // The caller owns the Tree value and its root slot. Empty them without
    // moving the slot; Rust drops those allocations when the value is dropped.
    drop(std::mem::take(&mut tree.included_ranges));
}

pub(crate) fn ts_tree_root_node(tree: &Tree) -> Node<'_> {
    ts_node_new(tree, &tree.root, ts_subtree_padding(&tree.root), 0)
}

pub(crate) fn ts_tree_root_node_with_offset(
    tree: &Tree,
    offset_bytes: u32,
    offset_extent: Point,
) -> Node<'_> {
    let offset = Length {
        bytes: offset_bytes,
        extent: offset_extent,
    };
    ts_node_new(
        tree,
        &tree.root,
        length_add(offset, ts_subtree_padding(&tree.root)),
        0,
    )
}

pub(crate) fn ts_tree_language(tree: &Tree) -> &Language {
    &tree.language
}

pub(crate) fn ts_tree_edit(tree: &mut Tree, edit: &InputEdit) {
    edit_included_ranges(&mut tree.included_ranges, edit);

    let mut pool = ts_subtree_pool_new(0);
    *tree.root = ts_subtree_edit(std::mem::take(&mut *tree.root), edit, &mut pool);
    ts_subtree_pool_delete(&mut pool);
}

fn edit_included_ranges(ranges: &mut [Range], edit: &InputEdit) {
    for range in ranges {
        if range.end_byte >= edit.old_end_byte {
            // An unbounded end stays unbounded, including its original point.
            if range.end_byte != u32::MAX {
                range.end_byte = edit
                    .new_end_byte
                    .wrapping_add(range.end_byte - edit.old_end_byte);
                range.end_point = point_add(
                    edit.new_end_point,
                    point_sub(range.end_point, edit.old_end_point),
                );
                if range.end_byte < edit.new_end_byte {
                    range.end_byte = u32::MAX;
                    range.end_point = POINT_MAX;
                }
            }
        } else if range.end_byte > edit.start_byte {
            range.end_byte = edit.start_byte;
            range.end_point = edit.start_point;
        }
        if range.start_byte >= edit.old_end_byte {
            // Unlike end_byte, start_byte has no special unbounded case in C.
            range.start_byte = edit
                .new_end_byte
                .wrapping_add(range.start_byte - edit.old_end_byte);
            range.start_point = point_add(
                edit.new_end_point,
                point_sub(range.start_point, edit.old_end_point),
            );
            if range.start_byte < edit.new_end_byte {
                range.start_byte = u32::MAX;
                range.start_point = POINT_MAX;
            }
        } else if range.start_byte > edit.start_byte {
            range.start_byte = edit.start_byte;
            range.start_point = edit.start_point;
        }
    }
}

pub(crate) fn ts_tree_included_ranges(tree: &Tree) -> Vec<Range> {
    tree.included_ranges.clone()
}

pub(crate) fn ts_tree_get_changed_ranges(old_tree: &Tree, new_tree: &Tree) -> Vec<Range> {
    let mut cursor1 = TreeCursor {
        tree: None,
        stack: Vec::new(),
        root_alias_symbol: 0,
    };
    let mut cursor2 = TreeCursor {
        tree: None,
        stack: Vec::new(),
        root_alias_symbol: 0,
    };
    ts_tree_cursor_init(&mut cursor1, ts_tree_root_node(old_tree));
    ts_tree_cursor_init(&mut cursor2, ts_tree_root_node(new_tree));

    let mut included_range_differences = Vec::new();
    ts_range_array_get_changed_ranges(
        &old_tree.included_ranges,
        &new_tree.included_ranges,
        &mut included_range_differences,
    );

    ts_subtree_get_changed_ranges(
        &old_tree.root,
        &new_tree.root,
        &mut cursor1,
        &mut cursor2,
        &old_tree.language,
        &included_range_differences,
    )
}

pub(crate) fn _ts_dup(file: &std::fs::File) -> std::io::Result<std::fs::File> {
    file.try_clone()
}

pub(crate) fn ts_tree_print_dot_graph(
    tree: &Tree,
    output: &mut dyn std::io::Write,
) -> std::io::Result<()> {
    // The caller owns the writer; native descriptor duplication/closing is
    // replaced by borrowing it for the duration of the subtree printer.
    ts_subtree_print_dot_graph(&tree.root, &tree.language, output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::point::point_new;

    fn range(start_byte: u32, end_byte: u32, start_point: Point, end_point: Point) -> Range {
        Range {
            start_byte,
            end_byte,
            start_point,
            end_point,
        }
    }

    fn replacement() -> InputEdit {
        InputEdit {
            start_byte: 10,
            old_end_byte: 20,
            new_end_byte: 15,
            start_point: point_new(1, 4),
            old_end_point: point_new(1, 14),
            new_end_point: point_new(1, 9),
        }
    }

    #[test]
    fn edits_clip_overlaps_and_shift_following_ranges() {
        let mut ranges = [
            range(0, 10, point_new(0, 0), point_new(1, 4)),
            range(5, 15, point_new(0, 5), point_new(1, 9)),
            range(12, 18, point_new(1, 6), point_new(1, 12)),
            range(15, 30, point_new(1, 9), point_new(1, 24)),
            range(20, 30, point_new(1, 14), point_new(1, 24)),
            range(10, 20, point_new(1, 4), point_new(1, 14)),
        ];
        edit_included_ranges(&mut ranges, &replacement());
        assert_eq!(
            ranges,
            [
                range(0, 10, point_new(0, 0), point_new(1, 4)),
                range(5, 10, point_new(0, 5), point_new(1, 4)),
                range(10, 10, point_new(1, 4), point_new(1, 4)),
                range(10, 25, point_new(1, 4), point_new(1, 19)),
                range(15, 25, point_new(1, 9), point_new(1, 19)),
                range(10, 15, point_new(1, 4), point_new(1, 9)),
            ]
        );
    }

    #[test]
    fn insertion_moves_both_endpoints_at_the_insertion() {
        let mut ranges = [range(10, 10, point_new(1, 4), point_new(1, 4))];
        let edit = InputEdit {
            old_end_byte: 10,
            old_end_point: point_new(1, 4),
            ..replacement()
        };
        edit_included_ranges(&mut ranges, &edit);
        assert_eq!(ranges, [range(15, 15, point_new(1, 9), point_new(1, 9))]);
    }

    #[test]
    fn multiline_edit_uses_point_extents_not_vector_addition() {
        let mut ranges = [range(22, 40, point_new(3, 6), point_new(5, 7))];
        let edit = InputEdit {
            old_end_point: point_new(3, 4),
            new_end_point: point_new(2, 8),
            ..replacement()
        };
        edit_included_ranges(&mut ranges, &edit);
        assert_eq!(ranges, [range(17, 35, point_new(2, 10), point_new(4, 7))]);
    }

    #[test]
    fn unbounded_end_is_preserved_but_start_is_shifted() {
        // A non-MAX point also stays untouched for an unbounded end.
        let end_point = point_new(30, 7);
        let mut ranges = [range(u32::MAX, u32::MAX, POINT_MAX, end_point)];
        edit_included_ranges(&mut ranges, &replacement());
        assert_eq!(ranges[0].start_byte, u32::MAX - 5);
        assert_eq!(ranges[0].start_point, POINT_MAX);
        assert_eq!(ranges[0].end_byte, u32::MAX);
        assert_eq!(ranges[0].end_point, end_point);
    }

    #[test]
    fn byte_overflow_sets_both_coordinates_to_max() {
        let mut ranges = [range(
            u32::MAX - 2,
            u32::MAX - 1,
            point_new(5, 0),
            point_new(5, 1),
        )];
        let edit = InputEdit {
            new_end_byte: 25,
            new_end_point: point_new(1, 19),
            ..replacement()
        };
        edit_included_ranges(&mut ranges, &edit);
        assert_eq!(ranges, [range(u32::MAX, u32::MAX, POINT_MAX, POINT_MAX)]);
    }

    #[test]
    fn reaching_max_without_overflow_preserves_computed_point() {
        let mut ranges = [range(20, 20, point_new(1, 14), point_new(1, 14))];
        let edit = InputEdit {
            new_end_byte: u32::MAX,
            ..replacement()
        };
        edit_included_ranges(&mut ranges, &edit);
        assert_eq!(
            ranges,
            [range(u32::MAX, u32::MAX, point_new(1, 9), point_new(1, 9))]
        );
    }

    #[test]
    fn duplicate_shares_file_position_without_taking_ownership() -> std::io::Result<()> {
        use std::io::{Read, Seek, SeekFrom};

        let mut original =
            std::fs::File::open(concat!(env!("CARGO_MANIFEST_DIR"), "/src/tree.rs"))?;
        let mut duplicate = _ts_dup(&original)?;
        duplicate.seek(SeekFrom::Start(3))?;
        assert_eq!(original.stream_position()?, 3);
        drop(duplicate);
        let mut bytes = [0; 5];
        original.read_exact(&mut bytes)?;
        assert_eq!(&bytes, &include_bytes!("tree.rs")[3..8]);
        Ok(())
    }
}
