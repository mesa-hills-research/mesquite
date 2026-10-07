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
    #[inline]
    fn drop(&mut self) {
        // Most headers are leaves or have already been drained by release.
        // Keep that case inline, without bringing the traversal loop into every
        // Arc drop and every replacement of a freshly initialized header.
        if !self.children.is_empty() {
            drop_subtree_children(std::mem::take(&mut self.children));
        }
    }
}

fn drop_subtree_children(mut pending: Vec<Subtree>) {
    while let Some(tree) = pending.pop() {
        if let Subtree::Heap(data) = tree
            && let Ok(mut data) = Arc::try_unwrap(data)
        {
            pending.append(&mut data.children);
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

#[inline]
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

#[inline]
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
    ts_subtree_new_leaf_with(
        pool,
        symbol,
        padding,
        size,
        lookahead_bytes,
        parse_state,
        has_external_tokens,
        depends_on_column,
        is_keyword,
        language,
        |_| {},
    )
}

/// Initialize heap-only token metadata before sharing the header. The callback
/// is intentionally not invoked for inline leaves; external tokens always take
/// the heap path, using the same inline eligibility rules as C.
#[inline]
pub(crate) fn ts_subtree_new_leaf_with(
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
    initialize: impl FnOnce(&mut SubtreeHeapData),
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
        new_heap_leaf(
            pool,
            symbol,
            padding,
            size,
            lookahead_bytes,
            parse_state,
            has_external_tokens,
            depends_on_column,
            is_keyword,
            language,
            initialize,
        )
    }
}

// Keep allocation/pool cleanup off the common inline-leaf construction path.
#[inline(never)]
fn new_heap_leaf(
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
    initialize: impl FnOnce(&mut SubtreeHeapData),
) -> Subtree {
    let metadata = ts_language_symbol_metadata(language, symbol);
    let extra = symbol == BUILTIN_SYM_END;
    let mut header = SubtreeHeapData {
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
        error_cost: 0,
        fragile_left: false,
        fragile_right: false,
        has_changes: false,
        has_external_scanner_state_change: false,
        is_missing: false,
    };
    initialize(&mut header);
    let data = if let Some(mut data) = pool.free_trees.pop() {
        *Arc::get_mut(&mut data).expect("pooled subtrees must be unique") = header;
        data
    } else {
        // Initialize fresh allocations directly: no default header to
        // overwrite, and no uniqueness synchronization before first use.
        Arc::new(header)
    };
    Subtree::Heap(data)
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
    summarize_children(
        tree.heap_mut().expect("cannot summarize an inline leaf"),
        language,
    );
}

// Accumulating a summary does not own the child vector. In particular, fresh
// reductions can compute it before constructing a header with a Drop impl.
struct ChildSummary {
    branch: BranchData,
    padding: Length,
    size: Length,
    lookahead_bytes: u32,
    error_cost: u32,
    parse_state: StateId,
    has_external_tokens: bool,
    has_external_scanner_state_change: bool,
    depends_on_column: bool,
    fragile_left: bool,
    fragile_right: bool,
    symbol: Symbol,
    visible: bool,
    named: bool,
}

#[inline(always)]
fn summarize_children(data: &mut SubtreeHeapData, language: &Language) {
    let SubtreePayload::Branch(branch) = &data.payload else {
        panic!("child summaries require a branch payload");
    };
    let mut summary = ChildSummary {
        branch: branch.clone(),
        padding: data.padding,
        size: data.size,
        lookahead_bytes: data.lookahead_bytes,
        error_cost: data.error_cost,
        parse_state: data.parse_state,
        has_external_tokens: data.has_external_tokens,
        has_external_scanner_state_change: data.has_external_scanner_state_change,
        depends_on_column: data.depends_on_column,
        fragile_left: data.fragile_left,
        fragile_right: data.fragile_right,
        symbol: data.symbol,
        visible: data.visible,
        named: data.named,
    };
    summarize_child_slice(&mut summary, &data.children, language);
    data.payload = SubtreePayload::Branch(summary.branch);
    data.padding = summary.padding;
    data.size = summary.size;
    data.lookahead_bytes = summary.lookahead_bytes;
    data.error_cost = summary.error_cost;
    data.parse_state = summary.parse_state;
    data.has_external_tokens = summary.has_external_tokens;
    data.has_external_scanner_state_change = summary.has_external_scanner_state_change;
    data.depends_on_column = summary.depends_on_column;
    data.fragile_left = summary.fragile_left;
    data.fragile_right = summary.fragile_right;
}

#[inline(always)]
fn summarize_child_slice(data: &mut ChildSummary, children: &[Subtree], language: &Language) {
    let branch = &mut data.branch;
    let mut summary = BranchData {
        production_id: branch.production_id,
        first_leaf: branch.first_leaf,
        ..BranchData::default()
    };
    // Accumulate separately from the owning header. Child Arc pointers cannot
    // alias these locals, so writes need not force reloading child data.
    let mut padding = data.padding;
    let mut size = data.size;
    let mut error_cost: u32 = 0;
    let mut has_external_tokens = false;
    let mut depends_on_column = false;
    let mut has_external_scanner_state_change = false;
    let mut fragile_left = data.fragile_left;
    let mut fragile_right = data.fragile_right;
    let mut parse_state = data.parse_state;

    let mut structural_index = 0;
    let alias_sequence = ts_language_alias_sequence(language, summary.production_id as u32);
    let mut lookahead_end_byte = 0;
    let is_error = data.symbol == BUILTIN_SYM_ERROR || data.symbol == BUILTIN_SYM_ERROR_REPEAT;

    for (i, child) in children.iter().enumerate() {
        // Decode the representation once. In particular, inline leaves have
        // no branch/scanner summaries, so they need none of the heap-only work.
        let (
            child_padding,
            child_size,
            lookahead,
            symbol,
            visible,
            named,
            extra,
            has_children,
            child_branch,
        ) = match child {
            Subtree::Inline(leaf) => {
                if leaf.flags & MISSING != 0 {
                    error_cost = error_cost
                        .wrapping_add(ERROR_COST_PER_MISSING_TREE + ERROR_COST_PER_RECOVERY);
                }
                (
                    ts_subtree_padding(child),
                    ts_subtree_size(child),
                    (leaf.padding_rows_and_lookahead >> 4) as u32,
                    leaf.symbol as Symbol,
                    leaf.flags & VISIBLE != 0,
                    leaf.flags & NAMED != 0,
                    leaf.flags & EXTRA != 0,
                    false,
                    None,
                )
            }
            Subtree::Heap(child) => {
                // C checks the existing size before replacing it for the
                // first child, including when re-summarizing a rotation.
                if size.extent.row == 0 && child.depends_on_column {
                    depends_on_column = true;
                }
                has_external_scanner_state_change |= child.has_external_scanner_state_change;
                has_external_tokens |= child.has_external_tokens;
                if child.symbol != BUILTIN_SYM_ERROR_REPEAT {
                    error_cost = error_cost.wrapping_add(if child.is_missing {
                        ERROR_COST_PER_MISSING_TREE + ERROR_COST_PER_RECOVERY
                    } else {
                        child.error_cost
                    });
                }
                if child.symbol == BUILTIN_SYM_ERROR {
                    fragile_left = true;
                    fragile_right = true;
                    parse_state = TS_TREE_STATE_NONE;
                }
                let child_branch = if let SubtreePayload::Branch(child_branch) = &child.payload {
                    if !child.children.is_empty() {
                        // Empty reductions have a branch payload, but C
                        // treats them as leaves for dynamic precedence.
                        summary.dynamic_precedence = summary
                            .dynamic_precedence
                            .wrapping_add(child_branch.dynamic_precedence);
                    }
                    summary.visible_descendant_count = summary
                        .visible_descendant_count
                        .wrapping_add(child_branch.visible_descendant_count);
                    Some(child_branch)
                } else {
                    None
                };
                (
                    child.padding,
                    child.size,
                    child.lookahead_bytes,
                    child.symbol,
                    child.visible,
                    child.named,
                    child.extra,
                    !child.children.is_empty(),
                    child_branch,
                )
            }
            Subtree::Null => (
                length_zero(),
                length_zero(),
                0,
                0,
                false,
                false,
                false,
                false,
                None,
            ),
        };

        if i == 0 {
            padding = child_padding;
            size = child_size;
        } else {
            size = length_add(size, length_add(child_padding, child_size));
        }
        let child_lookahead_end_byte = padding
            .bytes
            .wrapping_add(size.bytes)
            .wrapping_add(lookahead);
        lookahead_end_byte = lookahead_end_byte.max(child_lookahead_end_byte);

        if is_error && !extra && !(symbol == BUILTIN_SYM_ERROR && !has_children) {
            if visible {
                error_cost = error_cost.wrapping_add(ERROR_COST_PER_SKIPPED_TREE);
            } else if has_children {
                error_cost = error_cost.wrapping_add(
                    ERROR_COST_PER_SKIPPED_TREE
                        .wrapping_mul(child_branch.map_or(0, |b| b.visible_child_count)),
                );
            }
        }

        if !extra
            && symbol != 0
            && !alias_sequence.is_empty()
            && alias_sequence[structural_index] != 0
        {
            summary.visible_descendant_count = summary.visible_descendant_count.wrapping_add(1);
            summary.visible_child_count = summary.visible_child_count.wrapping_add(1);
            if ts_language_symbol_metadata(language, alias_sequence[structural_index]).named {
                summary.named_child_count = summary.named_child_count.wrapping_add(1);
            }
        } else if visible {
            summary.visible_descendant_count = summary.visible_descendant_count.wrapping_add(1);
            summary.visible_child_count = summary.visible_child_count.wrapping_add(1);
            summary.named_child_count = summary.named_child_count.wrapping_add(u32::from(named));
        } else if has_children && let Some(child_branch) = child_branch {
            summary.visible_child_count = summary
                .visible_child_count
                .wrapping_add(child_branch.visible_child_count);
            summary.named_child_count = summary
                .named_child_count
                .wrapping_add(child_branch.named_child_count);
        }
        structural_index += usize::from(!extra);
    }

    let lookahead_bytes = lookahead_end_byte
        .wrapping_sub(size.bytes)
        .wrapping_sub(padding.bytes);
    if is_error {
        error_cost = error_cost
            .wrapping_add(ERROR_COST_PER_RECOVERY)
            .wrapping_add(ERROR_COST_PER_SKIPPED_CHAR.wrapping_mul(size.bytes))
            .wrapping_add(ERROR_COST_PER_SKIPPED_LINE.wrapping_mul(size.extent.row));
    }

    let ends = children.first().zip(children.last());
    if let Some((first_child, last_child)) = ends {
        summary.first_leaf = FirstLeaf {
            symbol: ts_subtree_leaf_symbol(first_child),
            parse_state: ts_subtree_leaf_parse_state(first_child),
        };
        if ts_subtree_fragile_left(first_child) {
            fragile_left = true;
        }
        if ts_subtree_fragile_right(last_child) {
            fragile_right = true;
        }
        if children.len() >= 2
            && !data.visible
            && !data.named
            && ts_subtree_symbol(first_child) == data.symbol
        {
            summary.repeat_depth = ts_subtree_repeat_depth(first_child)
                .max(ts_subtree_repeat_depth(last_child))
                .wrapping_add(1) as u16;
        }
    }
    *branch = summary;
    data.padding = padding;
    data.size = size;
    data.error_cost = error_cost;
    data.has_external_tokens = has_external_tokens;
    data.depends_on_column = depends_on_column;
    data.has_external_scanner_state_change = has_external_scanner_state_change;
    data.fragile_left = fragile_left;
    data.fragile_right = fragile_right;
    data.parse_state = parse_state;
    data.lookahead_bytes = lookahead_bytes;
}

pub(crate) fn ts_subtree_new_node(
    symbol: Symbol,
    children: Vec<Subtree>,
    production_id: u32,
    language: &Language,
) -> Subtree {
    ts_subtree_new_node_with(symbol, children, production_id, language, |_| {})
}

/// Initialize a reduction's header while it is exclusively owned, before Arc
/// introduces the need for copy-on-write uniqueness checks.
pub(crate) fn ts_subtree_new_node_with(
    symbol: Symbol,
    children: Vec<Subtree>,
    production_id: u32,
    language: &Language,
    initialize: impl FnOnce(&mut SubtreeHeapData),
) -> Subtree {
    let metadata = ts_language_symbol_metadata(language, symbol);
    let fragile = symbol == BUILTIN_SYM_ERROR || symbol == BUILTIN_SYM_ERROR_REPEAT;
    // Finish the non-owning summary first, so the hot loop need not initialize
    // and repeatedly update an owning, droppable heap header. Keep the children
    // in their original Vec until the final header is ready.
    let mut summary = ChildSummary {
        symbol,
        visible: metadata.visible,
        named: metadata.named,
        padding: length_zero(),
        size: length_zero(),
        lookahead_bytes: 0,
        error_cost: 0,
        parse_state: 0,
        has_external_tokens: false,
        has_external_scanner_state_change: false,
        depends_on_column: false,
        fragile_left: fragile,
        fragile_right: fragile,
        branch: BranchData {
            production_id: production_id as u16,
            ..BranchData::default()
        },
    };
    summarize_child_slice(&mut summary, &children, language);
    // Explicit fields also avoid constructing a dropping default temporary.
    let mut result = SubtreeHeapData {
        symbol,
        visible: metadata.visible,
        named: metadata.named,
        padding: summary.padding,
        size: summary.size,
        lookahead_bytes: summary.lookahead_bytes,
        error_cost: summary.error_cost,
        parse_state: summary.parse_state,
        has_external_tokens: summary.has_external_tokens,
        has_external_scanner_state_change: summary.has_external_scanner_state_change,
        depends_on_column: summary.depends_on_column,
        fragile_left: summary.fragile_left,
        fragile_right: summary.fragile_right,
        children,
        payload: SubtreePayload::Branch(summary.branch),
        extra: false,
        has_changes: false,
        is_missing: false,
        is_keyword: false,
    };
    initialize(&mut result);
    Subtree::Heap(Arc::new(result))
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
    tree.clone()
}

// Inline and null handles have no allocation or worklist state to release.
// Keep their callers out of the heap traversal's register-save/cleanup path.
#[inline]
pub(crate) fn ts_subtree_release(pool: &mut SubtreePool, tree: Subtree) {
    if let Subtree::Heap(data) = tree {
        release_heap(pool, data);
    }
}

fn release_heap(pool: &mut SubtreePool, mut data: Arc<SubtreeHeapData>) {
    pool.tree_stack.clear();
    // Shared handles only need Arc's decrement. The initial unique handle can
    // be consumed directly, without a round trip through the worklist.
    if Arc::strong_count(&data) > 1 {
        return;
    }
    loop {
        // Queuing only checks the strong count; prove uniqueness once here.
        // Weak observers may prevent pooling, in which case normal Arc Drop
        // still drains the tree iteratively.
        if let Some(header) = Arc::get_mut(&mut data) {
            if header.children.is_empty() {
                if pool.free_trees.capacity() > 0 && pool.free_trees.len() < TS_MAX_TREE_POOL_SIZE {
                    // This already-borrowed header is unique. Avoid a second
                    // uniqueness check just to reset it and move it into the pool.
                    *header = SubtreeHeapData::default();
                    pool.free_trees.push(data);
                }
            } else {
                for child in std::mem::take(&mut header.children) {
                    subtree_queue_release(&mut pool.tree_stack, child);
                }
                // C frees branch allocations, and only pools leaf allocations.
            }
        }
        let Some(Subtree::Heap(next)) = pool.tree_stack.pop() else {
            break;
        };
        data = next;
    }
}

/// Drop shared references immediately, in child order, just as C decrements each
/// refcount before pushing the newly unreferenced heaps on its LIFO worklist.
fn subtree_queue_release(stack: &mut Vec<Subtree>, tree: Subtree) {
    // Shared references need only their normal Arc drop. Avoid get_mut's weak
    // counter synchronization unless this could actually be the final owner.
    if let Subtree::Heap(data) = tree
        && Arc::strong_count(&data) == 1
    {
        stack.push(Subtree::Heap(data));
    }
}

pub(crate) fn ts_subtree_compare(left: &Subtree, right: &Subtree, pool: &mut SubtreePool) -> i32 {
    pool.tree_stack.push(left.clone());
    pool.tree_stack.push(right.clone());
    while let Some(right) = pool.tree_stack.pop() {
        let left = pool
            .tree_stack
            .pop()
            .expect("comparison worklist contains pairs");
        let order = ts_subtree_symbol(&left)
            .cmp(&ts_subtree_symbol(&right))
            .then_with(|| ts_subtree_child_count(&left).cmp(&ts_subtree_child_count(&right)));
        if !order.is_eq() {
            pool.tree_stack.clear();
            return if order.is_lt() { -1 } else { 1 };
        }
        for (left, right) in ts_subtree_children(&left)
            .iter()
            .zip(ts_subtree_children(&right))
            .rev()
        {
            pool.tree_stack.push(left.clone());
            pool.tree_stack.push(right.clone());
        }
    }
    0
}

pub(crate) fn ts_subtree_set_has_changes(tree: &mut Subtree) {
    match tree {
        Subtree::Inline(data) => data.flags |= HAS_CHANGES,
        Subtree::Heap(data) => Arc::make_mut(data).has_changes = true,
        Subtree::Null => panic!("cannot mark a null subtree as changed"),
    }
}

pub(crate) fn ts_subtree_edit(
    mut tree: Subtree,
    edit: &InputEdit,
    pool: &mut SubtreePool,
) -> Subtree {
    let edit = Edit {
        start: Length {
            bytes: edit.start_byte,
            extent: edit.start_point,
        },
        old_end: Length {
            bytes: edit.old_end_byte,
            extent: edit.old_end_point,
        },
        new_end: Length {
            bytes: edit.new_end_byte,
            extent: edit.new_end_point,
        },
    };
    // Each entry borrows a disjoint child slot. No ancestors are borrowed once
    // their children have been queued, and all mutation is shallow copy-on-write.
    let mut stack = vec![(&mut tree, edit)];
    while let Some((tree, mut edit)) = stack.pop() {
        let is_noop =
            edit.old_end.bytes == edit.start.bytes && edit.new_end.bytes == edit.start.bytes;
        let is_pure_insertion = edit.old_end.bytes == edit.start.bytes;
        let parent_depends_on_column = ts_subtree_depends_on_column(tree);
        let column_shifted = edit.new_end.extent.column != edit.old_end.extent.column;
        let mut size = ts_subtree_size(tree);
        let mut padding = ts_subtree_padding(tree);
        let total_size = length_add(padding, size);
        let lookahead_bytes = ts_subtree_lookahead_bytes(tree);
        let end_byte = total_size.bytes.wrapping_add(lookahead_bytes);
        if edit.start.bytes > end_byte || (is_noop && edit.start.bytes == end_byte) {
            continue;
        }

        if edit.old_end.bytes <= padding.bytes {
            padding = length_add(edit.new_end, length_sub(padding, edit.old_end));
        } else if edit.start.bytes < padding.bytes {
            size = length_saturating_sub(size, length_sub(edit.old_end, padding));
            padding = edit.new_end;
        } else if edit.start.bytes < total_size.bytes
            || (edit.start.bytes == total_size.bytes && is_pure_insertion)
        {
            size = length_add(
                length_sub(edit.new_end, padding),
                length_saturating_sub(total_size, edit.old_end),
            );
        }

        *tree = ts_subtree_make_mut(pool, std::mem::take(tree));
        match tree {
            Subtree::Inline(data) => {
                if ts_subtree_can_inline(padding, size, lookahead_bytes) {
                    data.padding_bytes = padding.bytes as u8;
                    data.padding_columns = padding.extent.column as u8;
                    data.padding_rows_and_lookahead =
                        (data.padding_rows_and_lookahead & 0xf0) | padding.extent.row as u8;
                    data.size_bytes = size.bytes as u8;
                } else {
                    let mut heap = ts_subtree_pool_allocate(pool);
                    *Arc::get_mut(&mut heap).expect("pooled heaps are unique") = SubtreeHeapData {
                        padding,
                        size,
                        lookahead_bytes,
                        symbol: data.symbol as Symbol,
                        parse_state: data.parse_state,
                        visible: data.flags & VISIBLE != 0,
                        named: data.flags & NAMED != 0,
                        extra: data.flags & EXTRA != 0,
                        is_missing: data.flags & MISSING != 0,
                        is_keyword: data.flags & KEYWORD != 0,
                        children: Vec::new(),
                        payload: SubtreePayload::Leaf,
                        ..SubtreeHeapData::default()
                    };
                    *tree = Subtree::Heap(heap);
                }
            }
            Subtree::Heap(data) => {
                let data = Arc::make_mut(data);
                data.padding = padding;
                data.size = size;
            }
            Subtree::Null => panic!("cannot edit a null subtree"),
        }
        ts_subtree_set_has_changes(tree);

        let mut child_right = length_zero();
        if let Some(data) = tree.heap_mut() {
            for (i, child) in data.children.iter_mut().enumerate() {
                let child_size = ts_subtree_total_size(child);
                let child_left = child_right;
                child_right = length_add(child_left, child_size);
                if child_right
                    .bytes
                    .wrapping_add(ts_subtree_lookahead_bytes(child))
                    < edit.start.bytes
                {
                    continue;
                }
                if (child_left.bytes > edit.old_end.bytes
                    || (child_left.bytes == edit.old_end.bytes && child_size.bytes > 0 && i > 0))
                    && (!parent_depends_on_column || child_left.extent.row > padding.extent.row)
                    && (!ts_subtree_depends_on_column(child)
                        || !column_shifted
                        || child_left.extent.row > edit.old_end.extent.row)
                {
                    break;
                }
                let mut child_edit = Edit {
                    start: length_saturating_sub(edit.start, child_left),
                    old_end: length_saturating_sub(edit.old_end, child_left),
                    new_end: length_saturating_sub(edit.new_end, child_left),
                };
                // Only the first child touching the edit receives inserted text.
                if child_right.bytes > edit.start.bytes
                    || (child_right.bytes == edit.start.bytes && is_pure_insertion)
                {
                    edit.new_end = edit.start;
                } else {
                    child_edit.old_end = child_edit.start;
                    child_edit.new_end = child_edit.start;
                }
                stack.push((child, child_edit));
            }
        }
    }
    tree
}

pub(crate) fn ts_subtree_last_external_token(mut tree: &Subtree) -> Subtree {
    if !ts_subtree_has_external_tokens(tree) {
        return Subtree::Null;
    }
    while !ts_subtree_children(tree).is_empty() {
        tree = ts_subtree_children(tree)
            .iter()
            .rev()
            .find(|child| ts_subtree_has_external_tokens(child))
            .expect("a branch with external tokens has an external-token child");
    }
    tree.clone()
}

pub(crate) fn ts_subtree__write_char_to_string(output: &mut String, character: i32) {
    use std::fmt::Write;
    match character {
        -1 => output.push_str("INVALID"),
        0 => output.push_str("'\\0'"),
        10 => output.push_str("'\\n'"),
        9 => output.push_str("'\\t'"),
        13 => output.push_str("'\\r'"),
        32..=126 => {
            output.push('\'');
            output.push(character as u8 as char);
            output.push('\'');
        }
        _ => write!(output, "{character}").unwrap(),
    }
}

// As in C, identity (not the field's spelling) marks the root call.
static ROOT_FIELD: &str = "__ROOT__";

pub(crate) fn ts_subtree__write_to_string(
    tree: &Subtree,
    output: &mut String,
    language: &Language,
    include_all: bool,
    alias_symbol: Symbol,
    alias_is_named: bool,
    field_name: Option<&str>,
) {
    use crate::language::{
        ts_language_alias_sequence, ts_language_field_map, ts_language_field_name_for_id,
        ts_language_symbol_metadata, ts_language_symbol_name,
    };
    enum Entry<'a> {
        Node(&'a Subtree, Symbol, bool, Option<&'a str>),
        Close,
    }
    let mut stack = vec![Entry::Node(tree, alias_symbol, alias_is_named, field_name)];
    while let Some(entry) = stack.pop() {
        let Entry::Node(tree, alias_symbol, alias_is_named, field_name) = entry else {
            output.push(')');
            continue;
        };
        if tree.is_null() {
            output.push_str("(NULL)");
            continue;
        }
        let is_root = field_name.is_some_and(|name| std::ptr::eq(name, ROOT_FIELD));
        let is_visible = include_all
            || ts_subtree_missing(tree)
            || if alias_symbol != 0 {
                alias_is_named
            } else {
                ts_subtree_visible(tree) && ts_subtree_named(tree)
            };
        if is_visible {
            if !is_root {
                output.push(' ');
                if let Some(name) = field_name {
                    output.push_str(name);
                    output.push_str(": ");
                }
            }
            if ts_subtree_is_error(tree)
                && ts_subtree_child_count(tree) == 0
                && ts_subtree_size(tree).bytes > 0
            {
                output.push_str("(UNEXPECTED ");
                let Some(SubtreePayload::Error(character)) = tree.heap().map(|d| &d.payload) else {
                    panic!("an error leaf stores its lookahead character");
                };
                ts_subtree__write_char_to_string(output, *character);
            } else {
                let symbol = if alias_symbol != 0 {
                    alias_symbol
                } else {
                    ts_subtree_symbol(tree)
                };
                let name = ts_language_symbol_name(language, symbol).expect("valid subtree symbol");
                if ts_subtree_missing(tree) {
                    output.push_str("(MISSING ");
                    let named = alias_is_named || ts_subtree_named(tree);
                    if !named {
                        output.push('"');
                    }
                    output.push_str(name);
                    if !named {
                        output.push('"');
                    }
                } else {
                    output.push('(');
                    output.push_str(name);
                }
            }
        } else if is_root {
            let symbol = if alias_symbol != 0 {
                alias_symbol
            } else {
                ts_subtree_symbol(tree)
            };
            let name = ts_language_symbol_name(language, symbol).expect("valid subtree symbol");
            output.push('(');
            if ts_subtree_child_count(tree) > 0 {
                output.push_str(name);
            } else {
                let named = ts_subtree_named(tree);
                if !named {
                    output.push('"');
                }
                output.push_str(name);
                if !named {
                    output.push('"');
                }
                output.push(')');
            }
        }
        if is_visible {
            stack.push(Entry::Close);
        }
        if ts_subtree_child_count(tree) > 0 {
            let production = ts_subtree_production_id(tree) as u32;
            let aliases = ts_language_alias_sequence(language, production);
            let fields = ts_language_field_map(language, production);
            let mut structural_index = 0;
            let children_start = stack.len();
            for child in ts_subtree_children(tree) {
                if ts_subtree_extra(child) {
                    stack.push(Entry::Node(child, 0, false, None));
                } else {
                    let alias = if aliases.is_empty() {
                        0
                    } else {
                        aliases[structural_index]
                    };
                    let named = alias != 0 && ts_language_symbol_metadata(language, alias).named;
                    let mut child_field = if is_visible { None } else { field_name };
                    for field in fields {
                        if !field.inherited && field.child_index as usize == structural_index {
                            child_field = ts_language_field_name_for_id(language, field.field_id);
                            break;
                        }
                    }
                    stack.push(Entry::Node(child, alias, named, child_field));
                    structural_index += 1;
                }
            }
            // The C writer recurses left to right; use a worklist to avoid a
            // call-stack overflow when rendering deeply nested trees.
            stack[children_start..].reverse();
        }
    }
}

pub(crate) fn ts_subtree_string(
    tree: &Subtree,
    alias_symbol: Symbol,
    alias_is_named: bool,
    language: &Language,
    include_all: bool,
) -> String {
    let mut output = String::new();
    ts_subtree__write_to_string(
        tree,
        &mut output,
        language,
        include_all,
        alias_symbol,
        alias_is_named,
        Some(ROOT_FIELD),
    );
    output
}

pub(crate) fn ts_subtree__print_dot_graph(
    tree: &Subtree,
    start_offset: u32,
    language: &Language,
    alias_symbol: Symbol,
    output: &mut dyn std::io::Write,
) -> std::io::Result<()> {
    use crate::language::ts_language_write_symbol_as_dot_string;
    enum Entry<'a> {
        Node(&'a Subtree, u32, Symbol),
        Edge(&'a Subtree, &'a Subtree, usize),
    }
    let mut stack = vec![Entry::Node(tree, start_offset, alias_symbol)];
    while let Some(entry) = stack.pop() {
        let (tree, start_offset, alias_symbol) = match entry {
            Entry::Node(tree, offset, alias) => (tree, offset, alias),
            Entry::Edge(parent, child, index) => {
                writeln!(
                    output,
                    "tree_{parent:p} -> tree_{child:p} [tooltip={index}]"
                )?;
                continue;
            }
        };
        let symbol = if alias_symbol != 0 {
            alias_symbol
        } else {
            ts_subtree_symbol(tree)
        };
        let end_offset = start_offset.wrapping_add(ts_subtree_total_bytes(tree));
        write!(output, "tree_{tree:p} [label=\"")?;
        ts_language_write_symbol_as_dot_string(language, output, symbol)?;
        write!(output, "\"")?;
        if ts_subtree_child_count(tree) == 0 {
            write!(output, ", shape=plaintext")?;
        }
        if ts_subtree_extra(tree) {
            write!(output, ", fontcolor=gray")?;
        }
        if ts_subtree_has_changes(tree) {
            write!(output, ", color=green, penwidth=2")?;
        }
        write!(
            output,
            concat!(
                ", tooltip=\"range: {} - {}\nstate: {}\nerror-cost: {}\nhas-changes: {}",
                "\ndepends-on-column: {}\ndescendant-count: {}\nrepeat-depth: {}\nlookahead-bytes: {}"
            ),
            start_offset,
            end_offset,
            ts_subtree_parse_state(tree),
            ts_subtree_error_cost(tree),
            u8::from(ts_subtree_has_changes(tree)),
            u8::from(ts_subtree_depends_on_column(tree)),
            ts_subtree_visible_descendant_count(tree),
            ts_subtree_repeat_depth(tree),
            ts_subtree_lookahead_bytes(tree),
        )?;
        if ts_subtree_is_error(tree)
            && ts_subtree_child_count(tree) == 0
            && let Some(SubtreePayload::Error(character)) = tree.heap().map(|d| &d.payload)
            && *character != 0
        {
            write!(output, "\ncharacter: '")?;
            // fprintf's %c writes the low byte, not a UTF-8 encoding.
            output.write_all(&[*character as u8])?;
            write!(output, "'")?;
        }
        writeln!(output, "\"]")?;
        let mut child_start_offset = start_offset;
        let mut child_info_offset = u32::from(language.tables.max_alias_sequence_length)
            .wrapping_mul(u32::from(ts_subtree_production_id(tree)));
        let children_start = stack.len();
        for (i, child) in ts_subtree_children(tree).iter().enumerate() {
            let mut alias = 0;
            if !ts_subtree_extra(child) && child_info_offset != 0 {
                alias = language.tables.alias_sequences[child_info_offset as usize];
                child_info_offset = child_info_offset.wrapping_add(1);
            }
            stack.push(Entry::Node(child, child_start_offset, alias));
            stack.push(Entry::Edge(tree, child, i));
            child_start_offset = child_start_offset.wrapping_add(ts_subtree_total_bytes(child));
        }
        stack[children_start..].reverse();
    }
    Ok(())
}

pub(crate) fn ts_subtree_print_dot_graph(
    tree: &Subtree,
    language: &Language,
    output: &mut dyn std::io::Write,
) -> std::io::Result<()> {
    writeln!(output, "digraph tree {{")?;
    writeln!(output, "edge [arrowhead=none]")?;
    ts_subtree__print_dot_graph(tree, 0, language, 0, output)?;
    writeln!(output, "}}")
}

pub(crate) fn ts_subtree_external_scanner_state(tree: &Subtree) -> Option<&ExternalScannerState> {
    let data = tree.heap()?;
    if data.has_external_tokens
        && data.children.is_empty()
        && let SubtreePayload::External(state) = &data.payload
    {
        Some(state)
    } else {
        None
    }
}

pub(crate) fn ts_subtree_external_scanner_state_eq(tree: &Subtree, other: &Subtree) -> bool {
    // Stack versions and the token cache commonly share the same snapshot (or
    // both have no scanner). Their states cannot differ; avoid decoding headers
    // and comparing byte slices for identical handles.
    if tree.ptr_eq(other) {
        return true;
    }
    // None represents C's static, zero-length scanner state.
    let left =
        ts_subtree_external_scanner_state(tree).map_or(&[][..], ts_external_scanner_state_data);
    let right =
        ts_subtree_external_scanner_state(other).map_or(&[][..], ts_external_scanner_state_data);
    left == right
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
    fn releasing_weakly_observed_heaps_is_iterative_and_does_not_pool_them() {
        std::thread::Builder::new()
            .stack_size(128 * 1024)
            .spawn(|| {
                let shared_leaf = Subtree::Heap(Arc::new(SubtreeHeapData::default()));
                let mut tree = shared_leaf.clone();
                let mut observers = Vec::new();
                for _ in 0..20_000 {
                    let data = Arc::new(SubtreeHeapData {
                        children: vec![tree],
                        payload: SubtreePayload::Branch(BranchData::default()),
                        ..SubtreeHeapData::default()
                    });
                    observers.push(Arc::downgrade(&data));
                    tree = Subtree::Heap(data);
                }
                let mut pool = ts_subtree_pool_new(32);
                ts_subtree_release(&mut pool, tree);
                assert!(pool.tree_stack.is_empty());
                assert!(pool.free_trees.is_empty());
                assert!(observers.iter().all(|weak| weak.upgrade().is_none()));
                let Subtree::Heap(data) = shared_leaf else {
                    unreachable!()
                };
                assert_eq!(Arc::strong_count(&data), 1);
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
    fn release_fast_paths_preserve_scratch_and_shared_ownership() {
        let shared = Arc::new(SubtreeHeapData::default());
        let mut pool = ts_subtree_pool_new(32);
        pool.tree_stack.push(Subtree::Heap(shared.clone()));
        ts_subtree_release(&mut pool, Subtree::Null);
        ts_subtree_release(&mut pool, leaf(1, false));
        assert_eq!(pool.tree_stack.len(), 1);
        assert_eq!(Arc::strong_count(&shared), 2);

        ts_subtree_release(&mut pool, Subtree::Heap(shared.clone()));
        assert!(pool.tree_stack.is_empty());
        assert!(pool.free_trees.is_empty());
        assert_eq!(Arc::strong_count(&shared), 1);
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

#[cfg(test)]
mod subtree_2_tests {
    use super::*;

    fn leaf(symbol: u8, size: u8) -> Subtree {
        Subtree::Inline(InlineLeaf {
            symbol,
            size_bytes: size,
            flags: VISIBLE | NAMED,
            ..InlineLeaf::default()
        })
    }

    fn branch(symbol: Symbol, children: Vec<Subtree>) -> Subtree {
        let size = children.iter().fold(length_zero(), |size, child| {
            length_add(size, ts_subtree_total_size(child))
        });
        Subtree::Heap(Arc::new(SubtreeHeapData {
            symbol,
            size,
            children,
            visible: true,
            named: true,
            payload: SubtreePayload::Branch(BranchData::default()),
            ..SubtreeHeapData::default()
        }))
    }

    #[test]
    fn retain_and_change_flags_use_shallow_copy_on_write() {
        let original = branch(2, vec![branch(3, vec![leaf(4, 1)])]);
        let mut retained = ts_subtree_retain(&original);
        assert!(original.ptr_eq(&retained));
        ts_subtree_set_has_changes(&mut retained);
        assert!(!original.ptr_eq(&retained));
        assert!(!ts_subtree_has_changes(&original));
        assert!(ts_subtree_has_changes(&retained));
        assert!(ts_subtree_children(&original)[0].ptr_eq(&ts_subtree_children(&retained)[0]));
        let mut inline = leaf(1, 2);
        ts_subtree_set_has_changes(&mut inline);
        assert!(ts_subtree_has_changes(&inline));
        assert!(ts_subtree_visible(&inline));
        assert!(ts_subtree_named(&inline));
    }

    #[test]
    fn compare_orders_symbols_then_counts_then_children_left_to_right() {
        let mut pool = SubtreePool::default();
        let left = branch(1, vec![leaf(2, 4), leaf(4, 1)]);
        let right = branch(1, vec![leaf(3, 2), leaf(1, 1)]);
        assert_eq!(ts_subtree_compare(&left, &right, &mut pool), -1);
        assert_eq!(ts_subtree_compare(&right, &left, &mut pool), 1);
        assert!(pool.tree_stack.is_empty());
        assert_eq!(ts_subtree_compare(&left, &left, &mut pool), 0);
        assert_eq!(ts_subtree_compare(&leaf(1, 1), &left, &mut pool), -1);
        assert_eq!(ts_subtree_compare(&leaf(1, 2), &leaf(1, 30), &mut pool), 0);
        assert_eq!(
            Arc::strong_count(match &left {
                Subtree::Heap(d) => d,
                _ => unreachable!(),
            }),
            1
        );
    }

    #[test]
    fn releasing_branches_preserves_shared_descendants() {
        let child = branch(2, vec![leaf(3, 1)]);
        let parent = branch(1, vec![child.clone(), child.clone()]);
        let mut pool = SubtreePool::default();
        ts_subtree_release(&mut pool, parent);
        assert_eq!(
            Arc::strong_count(match &child {
                Subtree::Heap(d) => d,
                _ => unreachable!(),
            }),
            1
        );
        assert_eq!(ts_subtree_child_count(&child), 1);
        ts_subtree_release(&mut pool, child);
        assert!(pool.tree_stack.is_empty());
        assert!(pool.free_trees.is_empty());
    }

    #[test]
    fn comparison_and_release_do_not_recurse() {
        std::thread::Builder::new()
            .stack_size(128 * 1024)
            .spawn(|| {
                let mut left = leaf(1, 1);
                let mut right = leaf(1, 2);
                for _ in 0..20_000 {
                    left = branch(2, vec![left]);
                    right = branch(2, vec![right]);
                }
                let mut pool = SubtreePool::default();
                assert_eq!(ts_subtree_compare(&left, &right, &mut pool), 0);
                ts_subtree_release(&mut pool, left);
                ts_subtree_release(&mut pool, right);
                assert!(pool.tree_stack.is_empty());
            })
            .unwrap()
            .join()
            .unwrap();
    }

    #[test]
    fn edits_beyond_lookahead_or_noops_at_its_end_leave_identity_unchanged() {
        let mut original = branch(1, vec![leaf(2, 10)]);
        original.heap_mut().unwrap().lookahead_bytes = 4;
        let mut pool = SubtreePool::default();
        let edit = InputEdit {
            start_byte: 15,
            old_end_byte: 15,
            new_end_byte: 20,
            ..InputEdit::default()
        };
        let result = ts_subtree_edit(original.clone(), &edit, &mut pool);
        assert!(result.ptr_eq(&original));
        let edit = InputEdit {
            start_byte: 14,
            old_end_byte: 14,
            new_end_byte: 14,
            ..InputEdit::default()
        };
        let result = ts_subtree_edit(original.clone(), &edit, &mut pool);
        assert!(result.ptr_eq(&original));
        assert!(!ts_subtree_has_changes(&result));
    }

    #[test]
    fn last_external_token_uses_rightmost_external_descendant() {
        let mut external = branch(3, Vec::new());
        let data = external.heap_mut().unwrap();
        data.has_external_tokens = true;
        data.payload = SubtreePayload::External(ExternalScannerState::default());
        let mut internal = branch(2, vec![external.clone(), leaf(4, 1)]);
        internal.heap_mut().unwrap().has_external_tokens = true;
        let mut root = branch(1, vec![external.clone(), internal.clone(), leaf(5, 1)]);
        root.heap_mut().unwrap().has_external_tokens = true;
        let last = ts_subtree_last_external_token(&root);
        assert!(last.ptr_eq(&external));
        assert!(ts_subtree_last_external_token(&leaf(1, 1)).is_null());
        assert!(ts_subtree_external_scanner_state(&root).is_none());
        assert!(ts_subtree_external_scanner_state(&last).is_some());
        assert!(ts_subtree_external_scanner_state_eq(&root, &Subtree::Null));
    }

    #[test]
    fn scanner_state_equality_handles_identity_distinct_heaps_and_empty_states() {
        let external = |bytes: &[u8]| {
            Subtree::Heap(Arc::new(SubtreeHeapData {
                has_external_tokens: true,
                payload: SubtreePayload::External(ts_external_scanner_state_init(bytes)),
                children: Vec::new(),
                ..SubtreeHeapData::default()
            }))
        };
        let token = external(&[1, 2, 3]);
        let retained = token.clone();
        assert!(ts_subtree_external_scanner_state_eq(&token, &retained));
        assert!(ts_subtree_external_scanner_state_eq(&token, &external(&[1, 2, 3])));
        assert!(!ts_subtree_external_scanner_state_eq(&token, &external(&[1, 2])));
        assert!(!ts_subtree_external_scanner_state_eq(&token, &external(&[1, 2, 4])));
        assert!(!ts_subtree_external_scanner_state_eq(&token, &Subtree::Null));
        assert!(ts_subtree_external_scanner_state_eq(&external(&[]), &Subtree::Null));
        assert!(ts_subtree_external_scanner_state_eq(&Subtree::Null, &Subtree::Null));
        assert!(ts_subtree_external_scanner_state_eq(&leaf(1, 2), &leaf(3, 4)));
    }

    #[test]
    fn unexpected_characters_match_c_escaping_and_ascii_printability() {
        for (character, expected) in [
            (-1, "INVALID"),
            (0, "'\\0'"),
            (10, "'\\n'"),
            (9, "'\\t'"),
            (13, "'\\r'"),
            (32, "' '"),
            (39, "'''"),
            (92, "'\\'"),
            (126, "'~'"),
            (127, "127"),
            (233, "233"),
            (11, "11"),
        ] {
            let mut output = String::new();
            ts_subtree__write_char_to_string(&mut output, character);
            assert_eq!(output, expected);
        }
    }
}

#[cfg(test)]
mod summary_tests {
    use super::*;
    use std::sync::LazyLock;
    use ts_port_tables::{LanguageTables, SymbolMetadata};

    fn language() -> Language {
        static TABLES: LazyLock<LanguageTables> = LazyLock::new(|| {
            let mut tables = LanguageTables::decode(
                include_bytes!("../../grammars/c/src/tables.bin"),
                |_, _| panic!("summary tests do not lex"),
                Some(|_, _| panic!("summary tests do not lex keywords")),
                None,
            );
            tables.symbol_metadata[1] = SymbolMetadata {
                visible: true,
                named: true,
                supertype: false,
            };
            tables.symbol_metadata[2] = SymbolMetadata::default();
            tables.max_alias_sequence_length = 3;
            tables.alias_sequences = vec![0, 0, 0, 1, 2, 1];
            tables
        });
        Language::from(&*TABLES)
    }

    #[test]
    fn heap_leaf_initialization_precedes_sharing_for_fresh_and_pooled_headers() {
        let language = language();
        let mut pool = ts_subtree_pool_new(32);
        let inline = ts_subtree_new_leaf_with(
            &mut pool, 1, length_zero(), length_zero(), 0, 7, false, false, false,
            &language, |_| panic!("inline leaves do not construct a heap header"),
        );
        assert!(matches!(inline, Subtree::Inline(_)));

        for length in [0, 24, 25, 1024] {
            let bytes: Vec<_> = (0..length).map(|i| i as u8).collect();
            let mut initialized = false;
            let tree = ts_subtree_new_leaf_with(
                &mut pool, 1, length_zero(), length_zero(), 0, 7, true, true, false,
                &language,
                |header| {
                    assert!(header.has_external_tokens);
                    assert!(header.children.is_empty());
                    header.payload = SubtreePayload::External(ts_external_scanner_state_init(&bytes));
                    header.has_external_scanner_state_change = true;
                    initialized = true;
                },
            );
            assert!(initialized);
            assert!(pool.free_trees.is_empty());
            let Subtree::Heap(header) = &tree else { panic!("external tokens are heap leaves") };
            assert_eq!(Arc::strong_count(header), 1);
            assert!(header.has_external_scanner_state_change && header.depends_on_column);
            assert_eq!(header.parse_state, 7);
            assert_eq!(
                ts_external_scanner_state_data(ts_subtree_external_scanner_state(&tree).unwrap()),
                bytes,
            );
            ts_subtree_release(&mut pool, tree);
            assert_eq!(pool.free_trees.len(), 1);
        }
    }

    #[test]
    fn fresh_summary_matches_resummarization_for_empty_unary_and_multiple_children() {
        let language = language();
        let inline = Subtree::Inline(InlineLeaf {
            symbol: 1,
            parse_state: 7,
            flags: VISIBLE | NAMED | MISSING,
            padding_bytes: 2,
            padding_columns: 2,
            size_bytes: 3,
            padding_rows_and_lookahead: 7 << 4,
        });
        let mut external = ts_subtree_new_node(2, Vec::new(), 0, &language);
        let data = external.heap_mut().unwrap();
        data.extra = true;
        data.has_external_tokens = true;
        data.has_external_scanner_state_change = true;
        data.depends_on_column = true;
        data.payload = SubtreePayload::External(ts_external_scanner_state_init(&[1, 2]));
        let hidden = ts_subtree_new_node(2, vec![inline.clone()], 0, &language);
        let children = [inline, external, hidden];

        for count in 0..=children.len() {
            for symbol in [3, BUILTIN_SYM_ERROR, BUILTIN_SYM_ERROR_REPEAT] {
                for production_id in [0, 1] {
                    let metadata = ts_language_symbol_metadata(&language, symbol);
                    let fragile = symbol == BUILTIN_SYM_ERROR || symbol == BUILTIN_SYM_ERROR_REPEAT;
                    let mut expected = SubtreeHeapData {
                        symbol,
                        visible: metadata.visible,
                        named: metadata.named,
                        fragile_left: fragile,
                        fragile_right: fragile,
                        children: children[..count].to_vec(),
                        payload: SubtreePayload::Branch(BranchData {
                            production_id,
                            ..BranchData::default()
                        }),
                        ..SubtreeHeapData::default()
                    };
                    summarize_children(&mut expected, &language);
                    let fresh = ts_subtree_new_node(
                        symbol,
                        children[..count].to_vec(),
                        production_id as u32,
                        &language,
                    );
                    // Debug compares every header/payload field and the child
                    // contents, not the Arc allocation's address.
                    assert_eq!(
                        format!("{:?}", fresh.heap().unwrap()),
                        format!("{expected:?}"),
                        "count={count}, symbol={symbol}, production_id={production_id}",
                    );
                }
            }
        }
    }

    #[test]
    fn summaries_combine_missing_errors_hidden_branches_and_external_extras() {
        let missing = Subtree::Inline(InlineLeaf {
            symbol: 1,
            parse_state: 7,
            flags: VISIBLE | NAMED | MISSING,
            padding_bytes: 2,
            padding_columns: 2,
            size_bytes: 3,
            padding_rows_and_lookahead: 7 << 4,
        });
        let external = Subtree::Heap(Arc::new(SubtreeHeapData {
            symbol: 2,
            visible: true,
            extra: true,
            padding: Length {
                bytes: 1,
                extent: Point { row: 0, column: 1 },
            },
            size: Length {
                bytes: 4,
                extent: Point { row: 1, column: 1 },
            },
            lookahead_bytes: 2,
            error_cost: 5,
            has_external_tokens: true,
            has_external_scanner_state_change: true,
            depends_on_column: true,
            children: Vec::new(),
            payload: SubtreePayload::External(ExternalScannerState::default()),
            ..SubtreeHeapData::default()
        }));
        let hidden = Subtree::Heap(Arc::new(SubtreeHeapData {
            symbol: 3,
            padding: Length {
                bytes: 2,
                extent: Point { row: 0, column: 2 },
            },
            size: Length {
                bytes: 5,
                extent: Point { row: 0, column: 5 },
            },
            lookahead_bytes: 1,
            error_cost: 7,
            children: vec![missing.clone()],
            payload: SubtreePayload::Branch(BranchData {
                visible_child_count: 2,
                named_child_count: 1,
                visible_descendant_count: 3,
                dynamic_precedence: -4,
                ..BranchData::default()
            }),
            ..SubtreeHeapData::default()
        }));
        let error = Subtree::Heap(Arc::new(SubtreeHeapData {
            symbol: BUILTIN_SYM_ERROR,
            visible: true,
            named: true,
            size: Length {
                bytes: 1,
                extent: Point { row: 0, column: 1 },
            },
            error_cost: 11,
            children: Vec::new(),
            payload: SubtreePayload::Error(0),
            ..SubtreeHeapData::default()
        }));
        let tree = ts_subtree_new_node(
            BUILTIN_SYM_ERROR,
            vec![missing, external, hidden, error],
            0,
            &language(),
        );
        let data = tree.heap().unwrap();
        let summary = tree.branch().unwrap();
        assert_eq!(
            data.padding,
            Length {
                bytes: 2,
                extent: Point { row: 0, column: 2 }
            }
        );
        assert_eq!(
            data.size,
            Length {
                bytes: 16,
                extent: Point { row: 1, column: 9 }
            }
        );
        assert_eq!(data.lookahead_bytes, 0);
        assert_eq!(data.error_cost, 1479);
        assert!(
            data.has_external_tokens
                && data.has_external_scanner_state_change
                && data.depends_on_column
        );
        assert!(data.fragile_left && data.fragile_right);
        assert_eq!(data.parse_state, TS_TREE_STATE_NONE);
        assert_eq!(summary.dynamic_precedence, -4);
        assert_eq!(summary.visible_child_count, 5);
        assert_eq!(summary.named_child_count, 3);
        assert_eq!(summary.visible_descendant_count, 6);
        assert_eq!(summary.first_leaf.symbol, 1);
        assert_eq!(summary.first_leaf.parse_state, 7);
    }

    #[test]
    fn resummarizing_uses_old_rows_and_empty_reductions_have_leaf_precedence() {
        let child = Subtree::Heap(Arc::new(SubtreeHeapData {
            symbol: 2,
            parse_state: 23,
            depends_on_column: true,
            children: Vec::new(),
            payload: SubtreePayload::Branch(BranchData {
                dynamic_precedence: 99,
                ..BranchData::default()
            }),
            ..SubtreeHeapData::default()
        }));
        let mut data = SubtreeHeapData {
            symbol: 3,
            size: Length {
                bytes: 3,
                extent: Point { row: 3, column: 0 },
            },
            parse_state: 9,
            fragile_left: true,
            has_external_tokens: true,
            has_external_scanner_state_change: true,
            children: vec![child],
            payload: SubtreePayload::Branch(BranchData::default()),
            ..SubtreeHeapData::default()
        };
        summarize_children(&mut data, &language());
        assert!(!data.depends_on_column);
        assert!(!data.has_external_tokens && !data.has_external_scanner_state_change);
        assert_eq!(data.size, length_zero());
        assert!(data.fragile_left);
        assert_eq!(data.parse_state, 9);
        summarize_children(&mut data, &language());
        assert!(data.depends_on_column);
        let SubtreePayload::Branch(summary) = &data.payload else {
            unreachable!()
        };
        assert_eq!(summary.dynamic_precedence, 0);
        assert_eq!(summary.first_leaf.symbol, 2);
        assert_eq!(summary.first_leaf.parse_state, 23);
    }

    #[test]
    fn aliases_skip_extras_and_never_make_eof_visible() {
        let leaf = |symbol, flags| {
            Subtree::Inline(InlineLeaf {
                symbol,
                flags,
                ..InlineLeaf::default()
            })
        };
        let tree = ts_subtree_new_node(
            3,
            vec![
                leaf(1, 0),
                leaf(2, EXTRA | VISIBLE | NAMED),
                leaf(3, 0),
                leaf(0, 0),
            ],
            1,
            &language(),
        );
        let summary = tree.branch().unwrap();
        assert_eq!(summary.visible_child_count, 3);
        assert_eq!(summary.named_child_count, 2);
        assert_eq!(summary.visible_descendant_count, 3);
    }
}
