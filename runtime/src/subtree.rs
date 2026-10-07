use crate::error_costs::*;
use crate::{
    language::{Language, ts_language_alias_sequence, ts_language_symbol_metadata},
    length::*,
    types::*,
};
use std::sync::Arc;
use ts_port_tables::{BUILTIN_SYM_END, BUILTIN_SYM_ERROR};

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
    if data.len() <= 24 {
        let mut short_data = [0; 24];
        short_data[..data.len()].copy_from_slice(data);
        ExternalScannerState::Inline {
            length: data.len() as u8,
            data: short_data,
        }
    } else {
        ExternalScannerState::Heap(Arc::from(data))
    }
}

pub(crate) fn ts_external_scanner_state_copy(state: &ExternalScannerState) -> ExternalScannerState {
    // Long snapshots are immutable, so sharing the bytes replaces C's memcpy.
    state.clone()
}

pub(crate) fn ts_external_scanner_state_delete(state: ExternalScannerState) {
    drop(state);
}

pub(crate) fn ts_external_scanner_state_data(state: &ExternalScannerState) -> &[u8] {
    match state {
        ExternalScannerState::Inline { length, data } => &data[..*length as usize],
        ExternalScannerState::Heap(data) => data,
    }
}

pub(crate) fn ts_external_scanner_state_eq(state: &ExternalScannerState, buffer: &[u8]) -> bool {
    ts_external_scanner_state_data(state) == buffer
}

pub(crate) fn ts_subtree_array_copy(source: &[Subtree], destination: &mut Vec<Subtree>) {
    // Cloning a handle retains the subtree, not its descendants recursively.
    *destination = source.to_vec();
}

pub(crate) fn ts_subtree_array_clear(pool: &mut SubtreePool, trees: &mut Vec<Subtree>) {
    for tree in trees.drain(..) {
        ts_subtree_release(pool, tree);
    }
}

pub(crate) fn ts_subtree_array_delete(pool: &mut SubtreePool, trees: &mut Vec<Subtree>) {
    ts_subtree_array_clear(pool, trees);
    *trees = Vec::new();
}

pub(crate) fn ts_subtree_array_remove_trailing_extras(
    trees: &mut Vec<Subtree>,
    destination: &mut Vec<Subtree>,
) {
    destination.clear();
    while trees.last().is_some_and(ts_subtree_extra) {
        destination.push(trees.pop().unwrap());
    }
    ts_subtree_array_reverse(destination);
}

pub(crate) fn ts_subtree_array_reverse(trees: &mut [Subtree]) {
    trees.reverse();
}

pub(crate) fn ts_subtree_pool_new(capacity: u32) -> SubtreePool {
    SubtreePool {
        free_trees: Vec::with_capacity(capacity as usize),
        tree_stack: Vec::new(),
    }
}

pub(crate) fn ts_subtree_pool_delete(pool: &mut SubtreePool) {
    pool.free_trees = Vec::new();
    pool.tree_stack = Vec::new();
}

pub(crate) fn ts_subtree_pool_allocate(pool: &mut SubtreePool) -> Arc<SubtreeHeapData> {
    pool.free_trees
        .pop()
        .unwrap_or_else(|| Arc::new(SubtreeHeapData::default()))
}

pub(crate) fn ts_subtree_pool_free(pool: &mut SubtreePool, tree: Arc<SubtreeHeapData>) {
    let mut tree = tree;
    if pool.free_trees.capacity() > 0
        && pool.free_trees.len() < TS_MAX_TREE_POOL_SIZE
        && let Some(data) = Arc::get_mut(&mut tree)
        && data.children.is_empty()
    {
        // Only uniquely owned, drained allocations may enter the pool. Clear
        // the payload as well, so cached headers do not retain scanner states.
        *data = SubtreeHeapData::default();
        pool.free_trees.push(tree);
    }
}

pub(crate) fn ts_subtree_can_inline(padding: Length, size: Length, lookahead_bytes: u32) -> bool {
    padding.bytes < TS_MAX_INLINE_TREE_LENGTH
        && padding.extent.row < 16
        && padding.extent.column < TS_MAX_INLINE_TREE_LENGTH
        && size.bytes < TS_MAX_INLINE_TREE_LENGTH
        && size.extent.row == 0
        && size.extent.column < TS_MAX_INLINE_TREE_LENGTH
        && lookahead_bytes < 16
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
    let metadata = ts_language_symbol_metadata(language, symbol);
    let extra = symbol == BUILTIN_SYM_END;
    if symbol <= u8::MAX as Symbol
        && !has_external_tokens
        && ts_subtree_can_inline(padding, size, lookahead_bytes)
    {
        Subtree::Inline(InlineLeaf {
            parse_state,
            symbol: symbol as u8,
            flags: ((metadata.visible as u8) * VISIBLE)
                | ((metadata.named as u8) * NAMED)
                | ((extra as u8) * EXTRA)
                | ((is_keyword as u8) * KEYWORD),
            padding_bytes: padding.bytes as u8,
            padding_columns: padding.extent.column as u8,
            size_bytes: size.bytes as u8,
            padding_rows_and_lookahead: padding.extent.row as u8 | ((lookahead_bytes as u8) << 4),
        })
    } else {
        let mut data = ts_subtree_pool_allocate(pool);
        *Arc::get_mut(&mut data).expect("pooled subtrees must be unique") = SubtreeHeapData {
            padding,
            size,
            lookahead_bytes,
            symbol,
            parse_state,
            visible: metadata.visible,
            named: metadata.named,
            extra,
            has_external_tokens,
            depends_on_column,
            is_keyword,
            children: Vec::new(),
            payload: if has_external_tokens {
                SubtreePayload::External(ExternalScannerState::default())
            } else {
                SubtreePayload::Leaf
            },
            ..SubtreeHeapData::default()
        };
        Subtree::Heap(data)
    }
}

pub(crate) fn ts_subtree_set_symbol(tree: &mut Subtree, symbol: Symbol, language: &Language) {
    let metadata = ts_language_symbol_metadata(language, symbol);
    match tree {
        Subtree::Inline(data) => {
            assert!(symbol < u8::MAX as Symbol);
            data.symbol = symbol as u8;
            data.flags = (data.flags & !(NAMED | VISIBLE))
                | ((metadata.named as u8) * NAMED)
                | ((metadata.visible as u8) * VISIBLE);
        }
        Subtree::Heap(data) => {
            let data = Arc::make_mut(data);
            data.symbol = symbol;
            data.named = metadata.named;
            data.visible = metadata.visible;
        }
        Subtree::Null => panic!("cannot set the symbol of a null subtree"),
    }
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
    let mut result = ts_subtree_new_leaf(
        pool,
        BUILTIN_SYM_ERROR,
        padding,
        size,
        bytes_scanned,
        parse_state,
        false,
        false,
        false,
        language,
    );
    let data = result.heap_mut().expect("error symbols cannot be inline");
    data.fragile_left = true;
    data.fragile_right = true;
    data.payload = SubtreePayload::Error(lookahead_char);
    result
}

pub(crate) fn ts_subtree_clone(tree: &Subtree) -> Subtree {
    match tree {
        // A new header and child buffer, with retained child handles. Immutable
        // external scanner bytes can remain shared even when the header is not.
        Subtree::Heap(data) => Subtree::Heap(Arc::new((**data).clone())),
        _ => tree.clone(),
    }
}

pub(crate) fn ts_subtree_make_mut(pool: &mut SubtreePool, tree: Subtree) -> Subtree {
    if let Subtree::Heap(data) = &tree
        && Arc::strong_count(data) > 1
    {
        let result = ts_subtree_clone(&tree);
        ts_subtree_release(pool, tree);
        result
    } else {
        tree
    }
}

pub(crate) fn ts_subtree_compress(
    tree: &mut Subtree,
    count: u32,
    language: &Language,
    stack: &mut Vec<Subtree>,
) {
    let initial_stack_size = stack.len();
    let symbol = tree.heap().expect("cannot compress an inline leaf").symbol;
    let mut current = std::mem::take(tree);

    for _ in 0..count {
        // Inspect without cloning: an extra Arc handle here would change C's
        // ref_count checks and prevent the very rotation we are considering.
        let can_rotate = match &current {
            Subtree::Heap(data) if Arc::strong_count(data) == 1 && data.children.len() >= 2 => {
                match &data.children[0] {
                    Subtree::Heap(child)
                        if Arc::strong_count(child) == 1
                            && child.children.len() >= 2
                            && child.symbol == symbol =>
                    {
                        matches!(
                            &child.children[0],
                            Subtree::Heap(grandchild)
                                if Arc::strong_count(grandchild) == 1
                                    && grandchild.children.len() >= 2
                                    && grandchild.symbol == symbol
                        )
                    }
                    _ => false,
                }
            }
            _ => false,
        };
        if !can_rotate {
            break;
        }

        let mut child = std::mem::take(&mut current.heap_mut().unwrap().children[0]);
        let mut grandchild = std::mem::take(&mut child.heap_mut().unwrap().children[0]);
        let last = grandchild.heap_mut().unwrap().children.last_mut().unwrap();
        child.heap_mut().unwrap().children[0] = std::mem::take(last);
        *last = child;
        // The stack owns the detached ancestors. Their first-child slots are
        // restored on the way back up, avoiding both aliases and recursion.
        stack.push(current);
        current = grandchild;
    }

    while stack.len() > initial_stack_size {
        let mut parent = stack.pop().unwrap();
        let child = &mut parent.heap_mut().unwrap().children[0];
        *child = current;
        let grandchild = child.heap_mut().unwrap().children.last_mut().unwrap();
        ts_subtree_summarize_children(grandchild, language);
        ts_subtree_summarize_children(child, language);
        ts_subtree_summarize_children(&mut parent, language);
        current = parent;
    }
    *tree = current;
}

pub(crate) fn ts_subtree_summarize_children(tree: &mut Subtree, language: &Language) {
    let data = tree.heap_mut().expect("cannot summarize an inline leaf");
    let SubtreePayload::Branch(branch) = &mut data.payload else {
        panic!("child summaries require a branch payload");
    };
    branch.named_child_count = 0;
    branch.visible_child_count = 0;
    data.error_cost = 0;
    branch.repeat_depth = 0;
    branch.visible_descendant_count = 0;
    data.has_external_tokens = false;
    data.depends_on_column = false;
    data.has_external_scanner_state_change = false;
    branch.dynamic_precedence = 0;

    let mut structural_index = 0;
    let alias_sequence = ts_language_alias_sequence(language, branch.production_id as u32);
    let mut lookahead_end_byte = 0;
    let is_error = data.symbol == BUILTIN_SYM_ERROR || data.symbol == BUILTIN_SYM_ERROR_REPEAT;

    for (i, child) in data.children.iter().enumerate() {
        // C checks the existing size before replacing it for the first child.
        // In particular, do not reset size before re-summarizing a rotation.
        if data.size.extent.row == 0 && ts_subtree_depends_on_column(child) {
            data.depends_on_column = true;
        }
        if ts_subtree_has_external_scanner_state_change(child) {
            data.has_external_scanner_state_change = true;
        }

        if i == 0 {
            data.padding = ts_subtree_padding(child);
            data.size = ts_subtree_size(child);
        } else {
            data.size = length_add(data.size, ts_subtree_total_size(child));
        }

        let child_lookahead_end_byte = data
            .padding
            .bytes
            .wrapping_add(data.size.bytes)
            .wrapping_add(ts_subtree_lookahead_bytes(child));
        lookahead_end_byte = lookahead_end_byte.max(child_lookahead_end_byte);

        if ts_subtree_symbol(child) != BUILTIN_SYM_ERROR_REPEAT {
            data.error_cost = data.error_cost.wrapping_add(ts_subtree_error_cost(child));
        }

        let grandchild_count = ts_subtree_child_count(child);
        if is_error
            && !ts_subtree_extra(child)
            && !(ts_subtree_is_error(child) && grandchild_count == 0)
        {
            if ts_subtree_visible(child) {
                data.error_cost = data.error_cost.wrapping_add(ERROR_COST_PER_SKIPPED_TREE);
            } else if grandchild_count > 0 {
                data.error_cost = data.error_cost.wrapping_add(
                    ERROR_COST_PER_SKIPPED_TREE.wrapping_mul(ts_subtree_visible_child_count(child)),
                );
            }
        }

        branch.dynamic_precedence = branch
            .dynamic_precedence
            .wrapping_add(ts_subtree_dynamic_precedence(child));
        branch.visible_descendant_count = branch
            .visible_descendant_count
            .wrapping_add(ts_subtree_visible_descendant_count(child));

        if !ts_subtree_extra(child)
            && ts_subtree_symbol(child) != 0
            && !alias_sequence.is_empty()
            && alias_sequence[structural_index] != 0
        {
            branch.visible_descendant_count = branch.visible_descendant_count.wrapping_add(1);
            branch.visible_child_count = branch.visible_child_count.wrapping_add(1);
            if ts_language_symbol_metadata(language, alias_sequence[structural_index]).named {
                branch.named_child_count = branch.named_child_count.wrapping_add(1);
            }
        } else if ts_subtree_visible(child) {
            branch.visible_descendant_count = branch.visible_descendant_count.wrapping_add(1);
            branch.visible_child_count = branch.visible_child_count.wrapping_add(1);
            if ts_subtree_named(child) {
                branch.named_child_count = branch.named_child_count.wrapping_add(1);
            }
        } else if grandchild_count > 0 {
            branch.visible_child_count = branch
                .visible_child_count
                .wrapping_add(ts_subtree_visible_child_count(child));
            branch.named_child_count = branch
                .named_child_count
                .wrapping_add(ts_subtree_named_child_count(child));
        }

        if ts_subtree_has_external_tokens(child) {
            data.has_external_tokens = true;
        }
        if ts_subtree_is_error(child) {
            data.fragile_left = true;
            data.fragile_right = true;
            data.parse_state = TS_TREE_STATE_NONE;
        }
        if !ts_subtree_extra(child) {
            structural_index += 1;
        }
    }

    data.lookahead_bytes = lookahead_end_byte
        .wrapping_sub(data.size.bytes)
        .wrapping_sub(data.padding.bytes);
    if is_error {
        data.error_cost = data
            .error_cost
            .wrapping_add(ERROR_COST_PER_RECOVERY)
            .wrapping_add(ERROR_COST_PER_SKIPPED_CHAR.wrapping_mul(data.size.bytes))
            .wrapping_add(ERROR_COST_PER_SKIPPED_LINE.wrapping_mul(data.size.extent.row));
    }

    if let (Some(first_child), Some(last_child)) = (data.children.first(), data.children.last()) {
        branch.first_leaf = FirstLeaf {
            symbol: ts_subtree_leaf_symbol(first_child),
            parse_state: ts_subtree_leaf_parse_state(first_child),
        };
        if ts_subtree_fragile_left(first_child) {
            data.fragile_left = true;
        }
        if ts_subtree_fragile_right(last_child) {
            data.fragile_right = true;
        }
        if data.children.len() >= 2
            && !data.visible
            && !data.named
            && ts_subtree_symbol(first_child) == data.symbol
        {
            branch.repeat_depth = ts_subtree_repeat_depth(first_child)
                .max(ts_subtree_repeat_depth(last_child))
                .wrapping_add(1) as u16;
        }
    }
}

pub(crate) fn ts_subtree_new_node(
    symbol: Symbol,
    children: Vec<Subtree>,
    production_id: u32,
    language: &Language,
) -> Subtree {
    let metadata = ts_language_symbol_metadata(language, symbol);
    let fragile = symbol == BUILTIN_SYM_ERROR || symbol == BUILTIN_SYM_ERROR_REPEAT;
    let mut result = Subtree::Heap(Arc::new(SubtreeHeapData {
        symbol,
        visible: metadata.visible,
        named: metadata.named,
        fragile_left: fragile,
        fragile_right: fragile,
        children,
        payload: SubtreePayload::Branch(BranchData {
            production_id: production_id as u16,
            ..BranchData::default()
        }),
        ..SubtreeHeapData::default()
    }));
    ts_subtree_summarize_children(&mut result, language);
    result
}

pub(crate) fn ts_subtree_new_error_node(
    children: Vec<Subtree>,
    extra: bool,
    language: &Language,
) -> Subtree {
    let mut result = ts_subtree_new_node(BUILTIN_SYM_ERROR, children, 0, language);
    result.heap_mut().unwrap().extra = extra;
    result
}

pub(crate) fn ts_subtree_new_missing_leaf(
    pool: &mut SubtreePool,
    symbol: Symbol,
    padding: Length,
    lookahead_bytes: u32,
    language: &Language,
) -> Subtree {
    let mut result = ts_subtree_new_leaf(
        pool,
        symbol,
        padding,
        length_zero(),
        lookahead_bytes,
        0,
        false,
        false,
        false,
        language,
    );
    match &mut result {
        Subtree::Inline(data) => data.flags |= MISSING,
        Subtree::Heap(data) => Arc::make_mut(data).is_missing = true,
        Subtree::Null => unreachable!(),
    }
    result
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
    tree.branch()
        .filter(|_| ts_subtree_child_count(tree) > 0)
        .map_or(0, |d| d.dynamic_precedence)
}

pub(crate) fn ts_subtree_production_id(tree: &Subtree) -> u16 {
    tree.branch()
        .filter(|_| ts_subtree_child_count(tree) > 0)
        .map_or(0, |d| d.production_id)
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
        .filter(|_| ts_subtree_child_count(tree) > 0)
        .map_or_else(|| ts_subtree_symbol(tree), |d| d.first_leaf.symbol)
}
pub(crate) fn ts_subtree_leaf_parse_state(tree: &Subtree) -> StateId {
    tree.branch()
        .filter(|_| ts_subtree_child_count(tree) > 0)
        .map_or_else(
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
    fn heap_mutation_is_shallow_copy_on_write() {
        let child = Subtree::Heap(Arc::new(SubtreeHeapData::default()));
        let mut data = SubtreeHeapData::default();
        data.children.push(child);
        data.payload = SubtreePayload::Branch(BranchData::default());
        let original = Subtree::Heap(Arc::new(data));
        let mut edited = original.clone();
        assert!(edited.ptr_eq(&original));
        edited.heap_mut().unwrap().symbol = 17;
        assert!(!edited.ptr_eq(&original));
        assert_eq!(ts_subtree_symbol(&original), 0);
        assert_eq!(ts_subtree_symbol(&edited), 17);
        assert!(ts_subtree_children(&original)[0].ptr_eq(&ts_subtree_children(&edited)[0]));
    }

    #[test]
    fn deeply_nested_heap_drop_is_iterative() {
        std::thread::Builder::new()
            .stack_size(128 * 1024)
            .spawn(|| {
                let leaf = Arc::new(SubtreeHeapData::default());
                let weak = Arc::downgrade(&leaf);
                let mut tree = Subtree::Heap(leaf);
                for _ in 0..50_000 {
                    let mut data = SubtreeHeapData::default();
                    data.children.push(tree);
                    data.payload = SubtreePayload::Branch(BranchData::default());
                    tree = Subtree::Heap(Arc::new(data));
                }
                drop(tree);
                assert!(weak.upgrade().is_none());
            })
            .unwrap()
            .join()
            .unwrap();
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

#[cfg(test)]
mod construction_tests {
    use super::*;

    fn leaf(symbol: u8, extra: bool) -> Subtree {
        Subtree::Inline(InlineLeaf {
            symbol,
            flags: if extra { EXTRA } else { 0 },
            ..InlineLeaf::default()
        })
    }

    #[test]
    fn scanner_state_inline_boundary_and_byte_equality() {
        for length in [0, 1, 23, 24, 25, 1024] {
            let bytes: Vec<_> = (0..length).map(|i| i as u8).collect();
            let state = ts_external_scanner_state_init(&bytes);
            assert_eq!(
                matches!(state, ExternalScannerState::Inline { .. }),
                length <= 24
            );
            assert_eq!(ts_external_scanner_state_data(&state), bytes);
            assert!(ts_external_scanner_state_eq(&state, &bytes));
            let copy = ts_external_scanner_state_copy(&state);
            ts_external_scanner_state_delete(state);
            assert!(ts_external_scanner_state_eq(&copy, &bytes));
            if !bytes.is_empty() {
                assert!(!ts_external_scanner_state_eq(&copy, &bytes[1..]));
                let mut changed = bytes.clone();
                changed[0] ^= 1;
                assert!(!ts_external_scanner_state_eq(&copy, &changed));
            }
        }
        // Unused bytes of the short buffer do not participate in equality.
        let state = ExternalScannerState::Inline {
            length: 1,
            data: [42; 24],
        };
        assert!(ts_external_scanner_state_eq(&state, &[42]));
    }

    #[test]
    fn arrays_retain_handles_and_move_trailing_extras_in_order() {
        let mut trees = vec![
            Subtree::Heap(Arc::new(SubtreeHeapData::default())),
            leaf(1, false),
            leaf(2, true),
            leaf(3, true),
        ];
        let mut copy = Vec::new();
        ts_subtree_array_copy(&trees, &mut copy);
        for (a, b) in trees.iter().zip(&copy) {
            assert!(a.ptr_eq(b));
        }
        let mut extras = vec![leaf(99, false)];
        ts_subtree_array_remove_trailing_extras(&mut trees, &mut extras);
        assert_eq!(trees.len(), 2);
        assert_eq!(copy.len(), 4);
        assert_eq!(
            extras.iter().map(ts_subtree_symbol).collect::<Vec<_>>(),
            [2, 3]
        );
        ts_subtree_array_reverse(&mut extras);
        assert_eq!(
            extras.iter().map(ts_subtree_symbol).collect::<Vec<_>>(),
            [3, 2]
        );
        ts_subtree_array_remove_trailing_extras(&mut trees, &mut extras);
        assert!(extras.is_empty());
        ts_subtree_array_reverse(&mut extras);

        ts_subtree_array_remove_trailing_extras(&mut copy, &mut extras);
        let mut all_extras = Vec::new();
        ts_subtree_array_remove_trailing_extras(&mut extras, &mut all_extras);
        assert!(extras.is_empty());
        assert_eq!(
            all_extras.iter().map(ts_subtree_symbol).collect::<Vec<_>>(),
            [2, 3]
        );
    }

    #[test]
    fn pool_reuses_unique_allocations_and_honors_its_limit() {
        let mut pool = ts_subtree_pool_new(1);
        let mut allocation = ts_subtree_pool_allocate(&mut pool);
        let address = Arc::as_ptr(&allocation);
        Arc::get_mut(&mut allocation).unwrap().symbol = 12;
        ts_subtree_pool_free(&mut pool, allocation);
        assert_eq!(pool.free_trees.len(), 1);
        let reused = ts_subtree_pool_allocate(&mut pool);
        assert_eq!(Arc::as_ptr(&reused), address);
        assert_eq!(reused.symbol, 0);
        assert!(pool.free_trees.is_empty());

        // A still-live owner prevents the allocation from entering the pool.
        ts_subtree_pool_free(&mut pool, reused.clone());
        assert!(pool.free_trees.is_empty());
        ts_subtree_pool_free(&mut pool, reused);
        for _ in 0..40 {
            ts_subtree_pool_free(&mut pool, Arc::new(SubtreeHeapData::default()));
        }
        assert_eq!(pool.free_trees.len(), TS_MAX_TREE_POOL_SIZE);
        ts_subtree_pool_delete(&mut pool);
        assert_eq!(pool.free_trees.capacity(), 0);
        assert_eq!(pool.tree_stack.capacity(), 0);

        let mut disabled = ts_subtree_pool_new(0);
        let allocation = ts_subtree_pool_allocate(&mut disabled);
        ts_subtree_pool_free(&mut disabled, allocation);
        assert!(disabled.free_trees.is_empty());
    }

    #[test]
    fn inline_eligibility_uses_strict_c_bounds() {
        let padding = Length {
            bytes: 254,
            extent: Point {
                row: 15,
                column: 254,
            },
        };
        let size = Length {
            bytes: 254,
            extent: Point {
                row: 0,
                column: 254,
            },
        };
        assert!(ts_subtree_can_inline(padding, size, 15));
        assert!(!ts_subtree_can_inline(padding, size, 16));
        let mut changed = padding;
        changed.bytes = 255;
        assert!(!ts_subtree_can_inline(changed, size, 15));
        changed = padding;
        changed.extent.row = 16;
        assert!(!ts_subtree_can_inline(changed, size, 15));
        changed = padding;
        changed.extent.column = 255;
        assert!(!ts_subtree_can_inline(changed, size, 15));
        changed = size;
        changed.bytes = 255;
        assert!(!ts_subtree_can_inline(padding, changed, 15));
        changed = size;
        changed.extent.row = 1;
        assert!(!ts_subtree_can_inline(padding, changed, 15));
        changed = size;
        changed.extent.column = 255;
        assert!(!ts_subtree_can_inline(padding, changed, 15));
    }

    #[test]
    fn cloning_copies_the_header_but_retains_child_and_scanner_handles() {
        let state = ts_external_scanner_state_init(&[42; 25]);
        let child = Subtree::Heap(Arc::new(SubtreeHeapData {
            has_external_tokens: true,
            payload: SubtreePayload::External(state),
            children: Vec::new(),
            ..SubtreeHeapData::default()
        }));
        let copied_child = ts_subtree_clone(&child);
        assert!(!copied_child.ptr_eq(&child));
        let SubtreePayload::External(original) = &child.heap().unwrap().payload else {
            unreachable!()
        };
        let SubtreePayload::External(copied) = &copied_child.heap().unwrap().payload else {
            unreachable!()
        };
        assert_eq!(
            ts_external_scanner_state_data(original),
            ts_external_scanner_state_data(copied)
        );

        let parent = Subtree::Heap(Arc::new(SubtreeHeapData {
            children: vec![child],
            payload: SubtreePayload::Branch(BranchData::default()),
            ..SubtreeHeapData::default()
        }));
        let copied_parent = ts_subtree_clone(&parent);
        assert!(!copied_parent.ptr_eq(&parent));
        assert!(ts_subtree_children(&copied_parent)[0].ptr_eq(&ts_subtree_children(&parent)[0]));
        let mut pool = ts_subtree_pool_new(0);
        let address = if let Subtree::Heap(data) = &parent {
            Arc::as_ptr(data)
        } else {
            unreachable!()
        };
        let unique = ts_subtree_make_mut(&mut pool, parent);
        assert_eq!(
            Arc::as_ptr(match &unique {
                Subtree::Heap(data) => data,
                _ => unreachable!(),
            }),
            address
        );
    }

    #[test]
    fn empty_reductions_are_leaves_for_header_accessors() {
        let tree = Subtree::Heap(Arc::new(SubtreeHeapData {
            symbol: 73,
            parse_state: 12,
            children: Vec::new(),
            payload: SubtreePayload::Branch(BranchData {
                production_id: 4,
                dynamic_precedence: 10,
                ..BranchData::default()
            }),
            ..SubtreeHeapData::default()
        }));
        assert_eq!(ts_subtree_leaf_symbol(&tree), 73);
        assert_eq!(ts_subtree_leaf_parse_state(&tree), 12);
        assert_eq!(ts_subtree_production_id(&tree), 0);
        assert_eq!(ts_subtree_dynamic_precedence(&tree), 0);
    }
}
