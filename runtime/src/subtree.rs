use crate::error_costs::*;
use crate::{language::Language, length::*, types::*};
use std::sync::Arc;

/// Eight bytes, just as in C. The enum containing it is 16 bytes on 64-bit hosts.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C)]
pub(crate) struct InlineLeaf {
    pub parse_state: StateId,
    pub symbol: u8,
    pub flags: u8,
    pub padding_bytes: u8,
    pub padding_columns: u8,
    pub size_bytes: u8,
    pub padding_rows_and_lookahead: u8,
}
// padding_rows_and_lookahead: low 4 bits = rows, high 4 bits = lookahead bytes.
pub(crate) const VISIBLE: u8 = 1;
pub(crate) const NAMED: u8 = 2;
pub(crate) const EXTRA: u8 = 4;
pub(crate) const HAS_CHANGES: u8 = 8;
pub(crate) const MISSING: u8 = 16;
pub(crate) const KEYWORD: u8 = 32;

#[derive(Clone, Debug, Default)]
pub(crate) enum Subtree {
    #[default]
    Null,
    Inline(InlineLeaf),
    Heap(Arc<SubtreeHeapData>),
}
/// A mutable subtree is an owned handle. `Arc::make_mut` before heap mutation.
pub(crate) type MutableSubtree = Subtree;
pub(crate) type SubtreeArray = Vec<Subtree>;
pub(crate) type MutableSubtreeArray = Vec<Subtree>;
#[derive(Clone, Debug, Default)]
pub(crate) struct SubtreeHeapData {
    pub padding: Length,
    pub size: Length,
    pub lookahead_bytes: u32,
    pub error_cost: u32,
    pub symbol: Symbol,
    pub parse_state: StateId,
    pub visible: bool,
    pub named: bool,
    pub extra: bool,
    pub fragile_left: bool,
    pub fragile_right: bool,
    pub has_changes: bool,
    pub has_external_tokens: bool,
    pub has_external_scanner_state_change: bool,
    pub depends_on_column: bool,
    pub is_missing: bool,
    pub is_keyword: bool,
    pub children: Vec<Subtree>,
    pub payload: SubtreePayload,
}
#[derive(Clone, Debug, Default)]
pub(crate) enum SubtreePayload {
    #[default]
    Leaf,
    Branch(BranchData),
    External(ExternalScannerState),
    Error(i32),
}
#[derive(Clone, Debug, Default)]
pub(crate) struct BranchData {
    pub visible_child_count: u32,
    pub named_child_count: u32,
    pub visible_descendant_count: u32,
    pub dynamic_precedence: i32,
    pub repeat_depth: u16,
    pub production_id: u16,
    pub first_leaf: FirstLeaf,
}
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct FirstLeaf {
    pub symbol: Symbol,
    pub parse_state: StateId,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum ExternalScannerState {
    Inline { length: u8, data: [u8; 24] },
    Heap(Arc<[u8]>),
}
impl Default for ExternalScannerState {
    fn default() -> Self {
        Self::Inline {
            length: 0,
            data: [0; 24],
        }
    }
}
#[derive(Default, Debug)]
pub(crate) struct SubtreePool {
    pub free_trees: Vec<Arc<SubtreeHeapData>>,
    pub tree_stack: Vec<Subtree>,
}
/// Never recursively destroy an arbitrarily deep syntax tree. Shared children
/// stay shared; uniquely owned descendants are drained on an explicit worklist.
impl Drop for SubtreeHeapData {
    fn drop(&mut self) {
        let mut pending = std::mem::take(&mut self.children);
        while let Some(tree) = pending.pop() {
            if let Subtree::Heap(data) = tree
                && let Ok(mut data) = Arc::try_unwrap(data)
            {
                pending.append(&mut data.children);
            }
        }
    }
}

pub(crate) fn ts_external_scanner_state_init(data: &[u8]) -> ExternalScannerState {
    todo!("subtree-1: ts_external_scanner_state_init")
}

pub(crate) fn ts_external_scanner_state_copy(state: &ExternalScannerState) -> ExternalScannerState {
    todo!("subtree-1: ts_external_scanner_state_copy")
}

pub(crate) fn ts_external_scanner_state_delete(state: ExternalScannerState) {
    todo!("subtree-1: ts_external_scanner_state_delete")
}

pub(crate) fn ts_external_scanner_state_data(state: &ExternalScannerState) -> &[u8] {
    todo!("subtree-1: ts_external_scanner_state_data")
}

pub(crate) fn ts_external_scanner_state_eq(state: &ExternalScannerState, buffer: &[u8]) -> bool {
    todo!("subtree-1: ts_external_scanner_state_eq")
}

pub(crate) fn ts_subtree_array_copy(source: &[Subtree], destination: &mut Vec<Subtree>) {
    todo!("subtree-1: ts_subtree_array_copy")
}

pub(crate) fn ts_subtree_array_clear(pool: &mut SubtreePool, trees: &mut Vec<Subtree>) {
    todo!("subtree-1: ts_subtree_array_clear")
}

pub(crate) fn ts_subtree_array_delete(pool: &mut SubtreePool, trees: &mut Vec<Subtree>) {
    todo!("subtree-1: ts_subtree_array_delete")
}

pub(crate) fn ts_subtree_array_remove_trailing_extras(
    trees: &mut Vec<Subtree>,
    destination: &mut Vec<Subtree>,
) {
    todo!("subtree-1: ts_subtree_array_remove_trailing_extras")
}

pub(crate) fn ts_subtree_array_reverse(trees: &mut [Subtree]) {
    todo!("subtree-1: ts_subtree_array_reverse")
}

pub(crate) fn ts_subtree_pool_new(capacity: u32) -> SubtreePool {
    todo!("subtree-1: ts_subtree_pool_new")
}

pub(crate) fn ts_subtree_pool_delete(pool: &mut SubtreePool) {
    todo!("subtree-1: ts_subtree_pool_delete")
}

pub(crate) fn ts_subtree_pool_allocate(pool: &mut SubtreePool) -> Arc<SubtreeHeapData> {
    todo!("subtree-1: ts_subtree_pool_allocate")
}

pub(crate) fn ts_subtree_pool_free(pool: &mut SubtreePool, tree: Arc<SubtreeHeapData>) {
    todo!("subtree-1: ts_subtree_pool_free")
}

pub(crate) fn ts_subtree_can_inline(padding: Length, size: Length, lookahead_bytes: u32) -> bool {
    todo!("subtree-1: ts_subtree_can_inline")
}

pub(crate) fn ts_subtree_new_leaf(
    pool: &mut SubtreePool,
    symbol: Symbol,
    padding: Length,
    size: Length,
    lookahead_bytes: u32,
    parse_state: StateId,
    has_external_tokens: bool,
    depends_on_column: bool,
    is_keyword: bool,
    language: &Language,
) -> Subtree {
    todo!("subtree-1: ts_subtree_new_leaf")
}

pub(crate) fn ts_subtree_set_symbol(tree: &mut Subtree, symbol: Symbol, language: &Language) {
    todo!("subtree-1: ts_subtree_set_symbol")
}

pub(crate) fn ts_subtree_new_error(
    pool: &mut SubtreePool,
    lookahead_char: i32,
    padding: Length,
    size: Length,
    bytes_scanned: u32,
    parse_state: StateId,
    language: &Language,
) -> Subtree {
    todo!("subtree-1: ts_subtree_new_error")
}

pub(crate) fn ts_subtree_clone(tree: &Subtree) -> Subtree {
    todo!("subtree-1: ts_subtree_clone")
}

pub(crate) fn ts_subtree_make_mut(pool: &mut SubtreePool, tree: Subtree) -> Subtree {
    todo!("subtree-1: ts_subtree_make_mut")
}

pub(crate) fn ts_subtree_compress(
    tree: &mut Subtree,
    count: u32,
    language: &Language,
    stack: &mut Vec<Subtree>,
) {
    todo!("subtree-1: ts_subtree_compress")
}

pub(crate) fn ts_subtree_summarize_children(tree: &mut Subtree, language: &Language) {
    todo!("subtree-1: ts_subtree_summarize_children")
}

pub(crate) fn ts_subtree_new_node(
    symbol: Symbol,
    children: Vec<Subtree>,
    production_id: u32,
    language: &Language,
) -> Subtree {
    todo!("subtree-1: ts_subtree_new_node")
}

pub(crate) fn ts_subtree_new_error_node(
    children: Vec<Subtree>,
    extra: bool,
    language: &Language,
) -> Subtree {
    todo!("subtree-1: ts_subtree_new_error_node")
}

pub(crate) fn ts_subtree_new_missing_leaf(
    pool: &mut SubtreePool,
    symbol: Symbol,
    padding: Length,
    lookahead_bytes: u32,
    language: &Language,
) -> Subtree {
    todo!("subtree-1: ts_subtree_new_missing_leaf")
}

pub(crate) fn ts_subtree_retain(tree: &Subtree) -> Subtree {
    todo!("subtree-2: ts_subtree_retain")
}

pub(crate) fn ts_subtree_release(pool: &mut SubtreePool, tree: Subtree) {
    todo!("subtree-2: ts_subtree_release")
}

pub(crate) fn ts_subtree_compare(left: &Subtree, right: &Subtree, pool: &mut SubtreePool) -> i32 {
    todo!("subtree-2: ts_subtree_compare")
}

pub(crate) fn ts_subtree_set_has_changes(tree: &mut Subtree) {
    todo!("subtree-2: ts_subtree_set_has_changes")
}

pub(crate) fn ts_subtree_edit(tree: Subtree, edit: &InputEdit, pool: &mut SubtreePool) -> Subtree {
    todo!("subtree-2: ts_subtree_edit")
}

pub(crate) fn ts_subtree_last_external_token(tree: &Subtree) -> Subtree {
    todo!("subtree-2: ts_subtree_last_external_token")
}

pub(crate) fn ts_subtree__write_char_to_string(output: &mut String, character: i32) {
    todo!("subtree-2: ts_subtree__write_char_to_string")
}

pub(crate) fn ts_subtree__write_to_string(
    tree: &Subtree,
    output: &mut String,
    language: &Language,
    include_all: bool,
    alias_symbol: Symbol,
    alias_is_named: bool,
    field_name: Option<&str>,
) {
    todo!("subtree-2: ts_subtree__write_to_string")
}

pub(crate) fn ts_subtree_string(
    tree: &Subtree,
    alias_symbol: Symbol,
    alias_is_named: bool,
    language: &Language,
    include_all: bool,
) -> String {
    todo!("subtree-2: ts_subtree_string")
}

pub(crate) fn ts_subtree__print_dot_graph(
    tree: &Subtree,
    start_offset: u32,
    language: &Language,
    alias_symbol: Symbol,
    output: &mut dyn std::io::Write,
) -> std::io::Result<()> {
    todo!("subtree-2: ts_subtree__print_dot_graph")
}

pub(crate) fn ts_subtree_print_dot_graph(
    tree: &Subtree,
    language: &Language,
    output: &mut dyn std::io::Write,
) -> std::io::Result<()> {
    todo!("subtree-2: ts_subtree_print_dot_graph")
}

pub(crate) fn ts_subtree_external_scanner_state(tree: &Subtree) -> Option<&ExternalScannerState> {
    todo!("subtree-2: ts_subtree_external_scanner_state")
}

pub(crate) fn ts_subtree_external_scanner_state_eq(tree: &Subtree, other: &Subtree) -> bool {
    todo!("subtree-2: ts_subtree_external_scanner_state_eq")
}

pub(crate) const TS_TREE_STATE_NONE: StateId = StateId::MAX;
pub(crate) const TS_MAX_INLINE_TREE_LENGTH: u32 = 255;
impl Subtree {
    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }
    pub fn heap(&self) -> Option<&SubtreeHeapData> {
        if let Self::Heap(data) = self {
            Some(data)
        } else {
            None
        }
    }
    pub fn heap_mut(&mut self) -> Option<&mut SubtreeHeapData> {
        if let Self::Heap(data) = self {
            Some(Arc::make_mut(data))
        } else {
            None
        }
    }
    pub fn branch(&self) -> Option<&BranchData> {
        match self.heap().map(|d| &d.payload) {
            Some(SubtreePayload::Branch(data)) => Some(data),
            _ => None,
        }
    }
    /// C's `a.ptr == b.ptr`: value equality for inline words, identity for heaps.
    pub fn ptr_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Null, Self::Null) => true,
            (Self::Inline(a), Self::Inline(b)) => a == b,
            (Self::Heap(a), Self::Heap(b)) => Arc::ptr_eq(a, b),
            _ => false,
        }
    }
}

pub(crate) fn ts_subtree_visible(tree: &Subtree) -> bool {
    match tree {
        Subtree::Inline(d) => d.flags & VISIBLE != 0,
        Subtree::Heap(d) => d.visible,
        Subtree::Null => false,
    }
}

pub(crate) fn ts_subtree_named(tree: &Subtree) -> bool {
    match tree {
        Subtree::Inline(d) => d.flags & NAMED != 0,
        Subtree::Heap(d) => d.named,
        Subtree::Null => false,
    }
}

pub(crate) fn ts_subtree_extra(tree: &Subtree) -> bool {
    match tree {
        Subtree::Inline(d) => d.flags & EXTRA != 0,
        Subtree::Heap(d) => d.extra,
        Subtree::Null => false,
    }
}

pub(crate) fn ts_subtree_has_changes(tree: &Subtree) -> bool {
    match tree {
        Subtree::Inline(d) => d.flags & HAS_CHANGES != 0,
        Subtree::Heap(d) => d.has_changes,
        Subtree::Null => false,
    }
}

pub(crate) fn ts_subtree_missing(tree: &Subtree) -> bool {
    match tree {
        Subtree::Inline(d) => d.flags & MISSING != 0,
        Subtree::Heap(d) => d.is_missing,
        Subtree::Null => false,
    }
}

pub(crate) fn ts_subtree_is_keyword(tree: &Subtree) -> bool {
    match tree {
        Subtree::Inline(d) => d.flags & KEYWORD != 0,
        Subtree::Heap(d) => d.is_keyword,
        Subtree::Null => false,
    }
}

pub(crate) fn ts_subtree_symbol(tree: &Subtree) -> Symbol {
    match tree {
        Subtree::Inline(d) => d.symbol as Symbol,
        Subtree::Heap(d) => d.symbol,
        Subtree::Null => 0,
    }
}

pub(crate) fn ts_subtree_parse_state(tree: &Subtree) -> StateId {
    match tree {
        Subtree::Inline(d) => d.parse_state,
        Subtree::Heap(d) => d.parse_state,
        Subtree::Null => 0,
    }
}

pub(crate) fn ts_subtree_lookahead_bytes(tree: &Subtree) -> u32 {
    match tree {
        Subtree::Inline(d) => (d.padding_rows_and_lookahead >> 4) as u32,
        Subtree::Heap(d) => d.lookahead_bytes,
        Subtree::Null => 0,
    }
}

pub(crate) fn ts_subtree_fragile_left(tree: &Subtree) -> bool {
    tree.heap().is_some_and(|d| d.fragile_left)
}

pub(crate) fn ts_subtree_fragile_right(tree: &Subtree) -> bool {
    tree.heap().is_some_and(|d| d.fragile_right)
}

pub(crate) fn ts_subtree_has_external_tokens(tree: &Subtree) -> bool {
    tree.heap().is_some_and(|d| d.has_external_tokens)
}

pub(crate) fn ts_subtree_has_external_scanner_state_change(tree: &Subtree) -> bool {
    tree.heap()
        .is_some_and(|d| d.has_external_scanner_state_change)
}

pub(crate) fn ts_subtree_depends_on_column(tree: &Subtree) -> bool {
    tree.heap().is_some_and(|d| d.depends_on_column)
}

pub(crate) fn ts_subtree_visible_child_count(tree: &Subtree) -> u32 {
    tree.branch().map_or(0, |d| d.visible_child_count)
}

pub(crate) fn ts_subtree_named_child_count(tree: &Subtree) -> u32 {
    tree.branch().map_or(0, |d| d.named_child_count)
}

pub(crate) fn ts_subtree_visible_descendant_count(tree: &Subtree) -> u32 {
    tree.branch().map_or(0, |d| d.visible_descendant_count)
}

pub(crate) fn ts_subtree_dynamic_precedence(tree: &Subtree) -> i32 {
    tree.branch().map_or(0, |d| d.dynamic_precedence)
}

pub(crate) fn ts_subtree_production_id(tree: &Subtree) -> u16 {
    tree.branch().map_or(0, |d| d.production_id)
}

pub(crate) fn ts_subtree_repeat_depth(tree: &Subtree) -> u32 {
    tree.branch().map_or(0, |d| d.repeat_depth as u32)
}

pub(crate) fn ts_subtree_children(tree: &Subtree) -> &[Subtree] {
    tree.heap().map_or(&[], |d| d.children.as_slice())
}
pub(crate) fn ts_subtree_child_count(tree: &Subtree) -> u32 {
    ts_subtree_children(tree).len() as u32
}
pub(crate) fn ts_subtree_set_extra(tree: &mut Subtree, extra: bool) {
    match tree {
        Subtree::Inline(d) => {
            if extra {
                d.flags |= EXTRA;
            } else {
                d.flags &= !EXTRA;
            }
        }
        Subtree::Heap(d) => Arc::make_mut(d).extra = extra,
        Subtree::Null => panic!("cannot set extra on a null subtree"),
    }
}
pub(crate) fn ts_subtree_leaf_symbol(tree: &Subtree) -> Symbol {
    tree.branch()
        .map_or_else(|| ts_subtree_symbol(tree), |d| d.first_leaf.symbol)
}
pub(crate) fn ts_subtree_leaf_parse_state(tree: &Subtree) -> StateId {
    tree.branch().map_or_else(
        || ts_subtree_parse_state(tree),
        |d| d.first_leaf.parse_state,
    )
}
pub(crate) fn ts_subtree_padding(tree: &Subtree) -> Length {
    match tree {
        Subtree::Inline(d) => Length {
            bytes: d.padding_bytes as u32,
            extent: Point {
                row: (d.padding_rows_and_lookahead & 15) as u32,
                column: d.padding_columns as u32,
            },
        },
        Subtree::Heap(d) => d.padding,
        Subtree::Null => length_zero(),
    }
}
pub(crate) fn ts_subtree_size(tree: &Subtree) -> Length {
    match tree {
        Subtree::Inline(d) => Length {
            bytes: d.size_bytes as u32,
            extent: Point {
                row: 0,
                column: d.size_bytes as u32,
            },
        },
        Subtree::Heap(d) => d.size,
        Subtree::Null => length_zero(),
    }
}
pub(crate) fn ts_subtree_total_size(tree: &Subtree) -> Length {
    length_add(ts_subtree_padding(tree), ts_subtree_size(tree))
}
pub(crate) fn ts_subtree_total_bytes(tree: &Subtree) -> u32 {
    ts_subtree_total_size(tree).bytes
}
pub(crate) fn ts_subtree_is_repetition(tree: &Subtree) -> bool {
    !ts_subtree_named(tree) && !ts_subtree_visible(tree) && ts_subtree_child_count(tree) > 0
}
pub(crate) fn ts_subtree_error_cost(tree: &Subtree) -> u32 {
    if ts_subtree_missing(tree) {
        ERROR_COST_PER_MISSING_TREE + ERROR_COST_PER_RECOVERY
    } else {
        tree.heap().map_or(0, |d| d.error_cost)
    }
}
pub(crate) fn ts_subtree_is_fragile(tree: &Subtree) -> bool {
    ts_subtree_fragile_left(tree) || ts_subtree_fragile_right(tree)
}
pub(crate) fn ts_subtree_is_error(tree: &Subtree) -> bool {
    ts_subtree_symbol(tree) == ts_port_tables::BUILTIN_SYM_ERROR
}
pub(crate) fn ts_subtree_is_eof(tree: &Subtree) -> bool {
    ts_subtree_symbol(tree) == ts_port_tables::BUILTIN_SYM_END
}
pub(crate) fn ts_subtree_from_mut(tree: MutableSubtree) -> Subtree {
    tree
}
// Unlike the C reinterpretation, Rust transfers ownership; subsequent heap mutation
// still goes through Arc::make_mut. This function needs no unsafe code.
pub(crate) fn ts_subtree_to_mut_unsafe(tree: Subtree) -> MutableSubtree {
    tree
}
#[cfg(test)]
mod layout_tests {
    use super::*;
    #[test]
    fn compact_handles() {
        assert_eq!(std::mem::size_of::<InlineLeaf>(), 8);
        #[cfg(target_pointer_width = "64")]
        assert_eq!(std::mem::size_of::<Subtree>(), 16);
    }
    #[test]
    fn small_leaf_extents() {
        let tree = Subtree::Inline(InlineLeaf {
            padding_bytes: 21,
            padding_columns: 200,
            padding_rows_and_lookahead: 0x73,
            size_bytes: 10,
            flags: MISSING,
            ..InlineLeaf::default()
        });
        assert_eq!(
            ts_subtree_padding(&tree).extent,
            Point {
                row: 3,
                column: 200
            }
        );
        assert_eq!(ts_subtree_lookahead_bytes(&tree), 7);
        assert_eq!(ts_subtree_size(&tree).bytes, 10);
        assert_eq!(ts_subtree_error_cost(&tree), 610);
    }
}

pub(crate) const TS_MAX_TREE_POOL_SIZE: usize = 32;
#[derive(Clone, Copy, Debug)]
pub(crate) struct Edit {
    pub start: Length,
    pub old_end: Length,
    pub new_end: Length,
}
