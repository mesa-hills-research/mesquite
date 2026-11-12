use crate::error_costs::*;
use crate::{
    language::{Language, ts_language_alias_sequence, ts_language_symbol_metadata},
    length::*,
    types::*,
};
use std::sync::Arc;
use tree_sitter_language::{BUILTIN_SYM_END, BUILTIN_SYM_ERROR};

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
/// Construct the payload as one packed word, but expose bytes to field readers.
/// Separate struct-field stores followed by a word load stall the parser's hot
/// lex/shift path. Conversely, storing a u64 here makes traversal unpack fields
/// with shifts instead of byte loads. Packing into bytes avoids both costs, with
/// no pointer tagging, transmutation, or changes to C's inline limits.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub(crate) struct PackedInlineLeaf([u8; 8]);

impl PackedInlineLeaf {
    #[inline]
    fn new(data: InlineLeaf) -> Self {
        Self(
            (u64::from(data.parse_state)
                | (u64::from(data.symbol) << 16)
                | (u64::from(data.flags) << 24)
                | (u64::from(data.padding_bytes) << 32)
                | (u64::from(data.padding_columns) << 40)
                | (u64::from(data.size_bytes) << 48)
                | (u64::from(data.padding_rows_and_lookahead) << 56))
                .to_le_bytes(),
        )
    }

    #[inline]
    fn fields(self) -> InlineLeaf {
        InlineLeaf {
            parse_state: StateId::from_le_bytes([self.0[0], self.0[1]]),
            symbol: self.0[2],
            flags: self.0[3],
            padding_bytes: self.0[4],
            padding_columns: self.0[5],
            size_bytes: self.0[6],
            padding_rows_and_lookahead: self.0[7],
        }
    }

    #[inline]
    pub(crate) fn update(&mut self, update: impl FnOnce(&mut InlineLeaf)) {
        let mut fields = self.fields();
        update(&mut fields);
        *self = Self::new(fields);
    }

    #[inline]
    fn set_flag(&mut self, flag: u8, value: bool) {
        if value {
            self.0[3] |= flag;
        } else {
            self.0[3] &= !flag;
        }
    }
}

// padding_rows_and_lookahead: low 4 bits = rows, high 4 bits = lookahead bytes.
pub(crate) const VISIBLE: u8 = 1;
pub(crate) const NAMED: u8 = 2;
pub(crate) const EXTRA: u8 = 4;
pub(crate) const HAS_CHANGES: u8 = 8;
pub(crate) const MISSING: u8 = 16;
pub(crate) const KEYWORD: u8 = 32;

// On 64-bit hosts this fills the padding before a heap pointer without growing
// the 16-byte handle. The tag is the heap handle's u64 tag word, whose spare
// values mark null and inline handles. Word-sized tag stores avoid partial-store
// forwarding stalls when stack links move a freshly constructed handle as a
// whole value.
#[derive(Clone, Debug, Default)]
pub(crate) enum Subtree {
    #[default]
    Null,
    Inline(PackedInlineLeaf),
    Heap(HeapSubtree),
}
/// A mutable subtree is an owned handle. `HeapSubtree::make_mut` before heap
/// mutation.
pub(crate) type MutableSubtree = Subtree;
pub(crate) type SubtreeArray = Vec<Subtree>;
pub(crate) type MutableSubtreeArray = Vec<Subtree>;

/// A heap subtree's header and its children. Up to three children share the
/// header's allocation, as in C, which puts a node's children right before its
/// header, and more are kept in a Vec. The type gives the number of children, so
/// a leaf has no child slots and a node only as many as it uses.
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub(crate) struct HeapNode<C: AsMut<[Subtree]>> {
    pub data: SubtreeHeapData,
    pub children: C,
}

pub(crate) type LeafNode = HeapNode<[Subtree; 0]>;

/// A handle to a heap subtree. Nodes with up to three children keep them in an
/// array, and longer lists in a Vec. Nine in ten branch nodes of typical trees
/// have three children or fewer. A node's child count is fixed when it is made.
///
/// The tags sit at the top of the u64 range, and the null and inline handles
/// take the two values below them. Then a match on a `Subtree` compares the
/// tag with constants. With tags 0 to 4, every match first turned the tag into
/// a variant index with a subtraction and a conditional move.
#[derive(Clone, Debug)]
#[repr(u64)]
pub(crate) enum HeapSubtree {
    Leaf(Arc<LeafNode>) = u64::MAX - 4,
    One(Arc<HeapNode<[Subtree; 1]>>) = u64::MAX - 3,
    Two(Arc<HeapNode<[Subtree; 2]>>) = u64::MAX - 2,
    Three(Arc<HeapNode<[Subtree; 3]>>) = u64::MAX - 1,
    Many(Arc<HeapNode<Vec<Subtree>>>) = u64::MAX,
}

/// Runs `$body` with `$node` bound to the handle's Arc, whatever its child count.
macro_rules! with_node {
    ($tree:expr, $node:ident => $body:expr) => {
        match $tree {
            HeapSubtree::Leaf($node) => $body,
            HeapSubtree::One($node) => $body,
            HeapSubtree::Two($node) => $body,
            HeapSubtree::Three($node) => $body,
            HeapSubtree::Many($node) => $body,
        }
    };
}

impl HeapSubtree {
    /// A node with these children, in the variant that fits their count.
    #[inline(always)]
    pub(crate) fn new(data: SubtreeHeapData, mut children: Children) -> Self {
        children.compact();
        match children {
            // The slots past `len` are null, so forgetting them skips three
            // calls to `Subtree`'s drop and leaks nothing.
            Children::Inline {
                len,
                items: [a, b, c],
            } => match len {
                0 => {
                    std::mem::forget([a, b, c]);
                    Self::Leaf(Arc::new(HeapNode { data, children: [] }))
                }
                1 => {
                    std::mem::forget([b, c]);
                    Self::One(Arc::new(HeapNode {
                        data,
                        children: [a],
                    }))
                }
                2 => {
                    std::mem::forget(c);
                    Self::Two(Arc::new(HeapNode {
                        data,
                        children: [a, b],
                    }))
                }
                _ => Self::Three(Arc::new(HeapNode {
                    data,
                    children: [a, b, c],
                })),
            },
            Children::Heap(children) => {
                debug_assert!(children.len() > INLINE_CHILD_CAPACITY);
                Self::Many(Arc::new(HeapNode { data, children }))
            }
        }
    }

    #[inline]
    pub(crate) fn children(&self) -> &[Subtree] {
        with_node!(self, node => &node.children[..])
    }

    /// Whether the node has children. It compares the tag once, where
    /// `children` branches on every variant. A `Many` node always has more
    /// than three children, since `new` moves shorter lists into arrays.
    #[inline]
    pub(crate) fn has_children(&self) -> bool {
        !matches!(self, Self::Leaf(_))
    }

    /// The number of children, without a load for the arrays.
    #[inline]
    pub(crate) fn child_count(&self) -> usize {
        match self {
            Self::Leaf(_) => 0,
            Self::One(_) => 1,
            Self::Two(_) => 2,
            Self::Three(_) => 3,
            Self::Many(node) => node.children.len(),
        }
    }

    /// The header, copied first if the node is shared.
    #[inline]
    pub(crate) fn make_mut(&mut self) -> &mut SubtreeHeapData {
        with_node!(self, node => &mut Arc::make_mut(node).data)
    }

    /// The header and the children, copied first if the node is shared.
    #[inline]
    pub(crate) fn make_mut_with_children(&mut self) -> (&mut SubtreeHeapData, &mut [Subtree]) {
        with_node!(self, node => {
            let node = Arc::make_mut(node);
            (&mut node.data, &mut node.children[..])
        })
    }

    /// The header and the children, if no other handle shares the node.
    #[inline]
    pub(crate) fn get_mut(&mut self) -> Option<(&mut SubtreeHeapData, &mut [Subtree])> {
        with_node!(self, node => {
            let node = Arc::get_mut(node)?;
            Some((&mut node.data, &mut node.children[..]))
        })
    }

    /// The children, mutably, if one of them passes `wanted`. One branch on
    /// the arity covers the test and the borrow. The node must have no other
    /// handle.
    #[inline]
    pub(crate) fn children_mut_if_any(
        &mut self,
        wanted: impl Fn(&Subtree) -> bool,
    ) -> Option<&mut [Subtree]> {
        with_node!(self, node => {
            if !node.children[..].iter().any(&wanted) {
                return None;
            }
            let node = Arc::get_mut(node).expect("the node has no other handle");
            Some(&mut node.children[..])
        })
    }

    #[inline]
    pub(crate) fn strong_count(&self) -> usize {
        with_node!(self, node => Arc::strong_count(node))
    }

    /// The header's address, which identifies the node.
    #[inline]
    pub(crate) fn as_ptr(&self) -> *const SubtreeHeapData {
        &**self
    }

    /// A new allocation with the same header and retained child handles.
    pub(crate) fn clone_node(&self) -> Self {
        match self {
            Self::Leaf(node) => Self::Leaf(Arc::new((**node).clone())),
            Self::One(node) => Self::One(Arc::new((**node).clone())),
            Self::Two(node) => Self::Two(Arc::new((**node).clone())),
            Self::Three(node) => Self::Three(Arc::new((**node).clone())),
            Self::Many(node) => Self::Many(Arc::new((**node).clone())),
        }
    }
}

impl std::ops::Deref for HeapSubtree {
    type Target = SubtreeHeapData;

    #[inline]
    fn deref(&self) -> &SubtreeHeapData {
        with_node!(self, node => &node.data)
    }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct SubtreeHeapData {
    pub padding: Length,
    pub size: Length,
    pub lookahead_bytes: u32,
    pub error_cost: u32,
    pub symbol: Symbol,
    pub parse_state: StateId,
    pub flags: HeapFlags,
    pub payload: SubtreePayload,
}

/// The header's boolean fields, packed into two bytes as C packs them into
/// bitfields. Eleven bytes of booleans would push every node into the next
/// block size of glibc's allocator.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct HeapFlags(u16);

macro_rules! heap_flags {
    ($($bit:literal $name:ident $set:ident $with:ident),* $(,)?) => {
        impl HeapFlags {
            $(
                #[inline]
                pub(crate) const fn $name(self) -> bool {
                    self.0 & (1 << $bit) != 0
                }

                #[inline]
                pub(crate) const fn $with(self, value: bool) -> Self {
                    Self(if value { self.0 | (1 << $bit) } else { self.0 & !(1 << $bit) })
                }
            )*
        }

        impl SubtreeHeapData {
            $(
                #[inline]
                pub(crate) const fn $name(&self) -> bool {
                    self.flags.$name()
                }

                #[inline]
                pub(crate) fn $set(&mut self, value: bool) {
                    self.flags = self.flags.$with(value);
                }
            )*
        }

        impl std::fmt::Debug for HeapFlags {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct("HeapFlags")
                    $(.field(stringify!($name), &self.$name()))*
                    .finish()
            }
        }
    };
}

heap_flags! {
    0 visible set_visible with_visible,
    1 named set_named with_named,
    2 extra set_extra with_extra,
    3 fragile_left set_fragile_left with_fragile_left,
    4 fragile_right set_fragile_right with_fragile_right,
    5 has_changes set_has_changes with_has_changes,
    6 has_external_tokens set_has_external_tokens with_has_external_tokens,
    7 has_external_scanner_state_change set_has_external_scanner_state_change with_has_external_scanner_state_change,
    8 depends_on_column set_depends_on_column with_depends_on_column,
    9 is_missing set_is_missing with_is_missing,
    10 is_keyword set_is_keyword with_is_keyword,
}

#[cfg(test)]
impl SubtreeHeapData {
    /// A heap subtree with this header and these children.
    pub(crate) fn with_children(self, children: impl Into<Children>) -> Subtree {
        Subtree::Heap(HeapSubtree::new(self, children.into()))
    }
}

/// The number of children a node keeps in an array in its own allocation.
pub(crate) const INLINE_CHILD_CAPACITY: usize = 3;

/// The children of a node being made. Up to three are kept inline and more in
/// a Vec, as the node will keep them.
#[derive(Clone)]
pub(crate) enum Children {
    Inline {
        len: u8,
        items: [Subtree; INLINE_CHILD_CAPACITY],
    },
    Heap(Vec<Subtree>),
}

/// Lists the children, the same however they are stored.
impl std::fmt::Debug for Children {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.as_slice()).finish()
    }
}

impl Default for Children {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Children {
    #[inline]
    pub(crate) const fn new() -> Self {
        Self::Inline {
            len: 0,
            items: [const { Subtree::Null }; INLINE_CHILD_CAPACITY],
        }
    }

    #[inline]
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        if capacity <= INLINE_CHILD_CAPACITY {
            Self::new()
        } else {
            Self::Heap(Vec::with_capacity(capacity))
        }
    }

    #[inline]
    pub(crate) fn as_slice(&self) -> &[Subtree] {
        match self {
            Self::Inline { len, items } => &items[..usize::from(*len)],
            Self::Heap(children) => children,
        }
    }

    #[inline]
    pub(crate) fn as_mut_slice(&mut self) -> &mut [Subtree] {
        match self {
            Self::Inline { len, items } => &mut items[..usize::from(*len)],
            Self::Heap(children) => children,
        }
    }

    #[inline]
    pub(crate) fn push(&mut self, tree: Subtree) {
        match self {
            Self::Inline { len, items } if usize::from(*len) < INLINE_CHILD_CAPACITY => {
                items[usize::from(*len)].fill(tree);
                *len += 1;
            }
            Self::Inline { .. } => self.spill(tree),
            Self::Heap(children) => children.push(tree),
        }
    }

    /// Moves a full inline list into a Vec, followed by `tree`. Reductions
    /// size their list from the child count, so this is rare, and kept out of
    /// line so that `push` stays small enough to inline.
    #[cold]
    #[inline(never)]
    fn spill(&mut self, tree: Subtree) {
        let mut children = Vec::with_capacity(INLINE_CHILD_CAPACITY * 2);
        if let Self::Inline { items, .. } = self {
            children.extend(items.iter_mut().map(std::mem::take));
        }
        children.push(tree);
        *self = Self::Heap(children);
    }

    #[inline]
    pub(crate) fn pop(&mut self) -> Option<Subtree> {
        match self {
            Self::Inline { len, items } => {
                if *len == 0 {
                    return None;
                }
                *len -= 1;
                Some(std::mem::take(&mut items[usize::from(*len)]))
            }
            Self::Heap(children) => children.pop(),
        }
    }

    /// Moves a spilled list that has shrunk to the inline capacity, such as a
    /// reduction's children after its trailing extras were removed, back inline.
    #[inline]
    pub(crate) fn compact(&mut self) {
        if let Self::Heap(children) = self
            && children.len() <= INLINE_CHILD_CAPACITY
        {
            *self = Self::from(std::mem::take(children));
        }
    }

    /// Moves the children to the end of `out`, leaving this list empty.
    #[inline]
    pub(crate) fn append_to(&mut self, out: &mut Vec<Subtree>) {
        match self {
            Self::Inline { len, items } => {
                out.extend(items[..usize::from(*len)].iter_mut().map(std::mem::take));
                *len = 0;
            }
            Self::Heap(children) => out.append(children),
        }
    }

    /// The children as a Vec, keeping a spilled list's allocation.
    pub(crate) fn into_vec(self) -> Vec<Subtree> {
        match self {
            Self::Heap(children) => children,
            inline => inline.into_iter().collect(),
        }
    }
}

impl From<Vec<Subtree>> for Children {
    #[inline]
    fn from(mut children: Vec<Subtree>) -> Self {
        if children.len() <= INLINE_CHILD_CAPACITY {
            let mut items = [const { Subtree::Null }; INLINE_CHILD_CAPACITY];
            let len = children.len() as u8;
            for (slot, child) in items.iter_mut().zip(children.drain(..)) {
                *slot = child;
            }
            Self::Inline { len, items }
        } else {
            Self::Heap(children)
        }
    }
}

impl Children {
    /// Takes the children out of `children`. Up to three are moved inline and
    /// leave the Vec empty with its allocation, for reuse. A longer list takes
    /// the Vec itself.
    #[inline]
    pub(crate) fn take_from_vec(children: &mut Vec<Subtree>) -> Self {
        if children.len() <= INLINE_CHILD_CAPACITY {
            let mut items = [const { Subtree::Null }; INLINE_CHILD_CAPACITY];
            let len = children.len() as u8;
            for (slot, child) in items.iter_mut().zip(children.drain(..)) {
                *slot = child;
            }
            Self::Inline { len, items }
        } else {
            Self::Heap(std::mem::take(children))
        }
    }
}

impl From<&[Subtree]> for Children {
    #[inline]
    fn from(children: &[Subtree]) -> Self {
        if children.len() <= INLINE_CHILD_CAPACITY {
            let mut items = [const { Subtree::Null }; INLINE_CHILD_CAPACITY];
            items[..children.len()].clone_from_slice(children);
            Self::Inline {
                len: children.len() as u8,
                items,
            }
        } else {
            Self::Heap(children.to_vec())
        }
    }
}

impl std::ops::Deref for Children {
    type Target = [Subtree];

    #[inline]
    fn deref(&self) -> &[Subtree] {
        self.as_slice()
    }
}

impl std::ops::DerefMut for Children {
    #[inline]
    fn deref_mut(&mut self) -> &mut [Subtree] {
        self.as_mut_slice()
    }
}

impl<'a> IntoIterator for &'a Children {
    type Item = &'a Subtree;
    type IntoIter = std::slice::Iter<'a, Subtree>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.as_slice().iter()
    }
}

impl IntoIterator for Children {
    type Item = Subtree;
    type IntoIter = ChildrenIntoIter;

    #[inline]
    fn into_iter(self) -> ChildrenIntoIter {
        match self {
            Self::Inline { len, items } => {
                ChildrenIntoIter::Inline(items.into_iter().take(usize::from(len)))
            }
            Self::Heap(children) => ChildrenIntoIter::Heap(children.into_iter()),
        }
    }
}

pub(crate) enum ChildrenIntoIter {
    Inline(std::iter::Take<std::array::IntoIter<Subtree, INLINE_CHILD_CAPACITY>>),
    Heap(std::vec::IntoIter<Subtree>),
}

impl Iterator for ChildrenIntoIter {
    type Item = Subtree;

    #[inline]
    fn next(&mut self) -> Option<Subtree> {
        match self {
            Self::Inline(children) => children.next(),
            Self::Heap(children) => children.next(),
        }
    }
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
    pub free_trees: Vec<Arc<LeafNode>>,
    pub tree_stack: Vec<Subtree>,
}
/// Never recursively destroy an arbitrarily deep syntax tree. Shared children
/// stay shared; uniquely owned descendants are drained on an explicit worklist.
impl<C: AsMut<[Subtree]>> Drop for HeapNode<C> {
    #[inline]
    fn drop(&mut self) {
        // Most nodes are leaves or have already been drained by release.
        // Keep that case inline, without bringing the traversal loop into every
        // Arc drop.
        let children = self.children.as_mut();
        if children
            .iter()
            .any(|child| matches!(child, Subtree::Heap(_)))
        {
            drop_subtree_children(children);
        }
    }
}

fn drop_subtree_children(children: &mut [Subtree]) {
    // Uniquely owned descendants, whose children are released in turn. The
    // worklist allocates only once a dropped node has a heap child of its own.
    let mut pending: Vec<HeapSubtree> = Vec::new();
    queue_unique_children(children, &mut pending);
    let mut freed: usize = 0;
    while let Some(node) = pending.pop() {
        // Only a weak reference upgraded on another thread can share it
        // again, and then that owner's drop drains it.
        with_node!(node, node => {
            if let Ok(mut node) = Arc::try_unwrap(node) {
                queue_unique_children(node.children.as_mut(), &mut pending);
                freed += 1;
            }
        });
    }
    if freed >= MERGE_FREED_NODES_AT {
        merge_freed_nodes();
    }
}

/// How many nodes a drop frees before it asks the allocator to merge them,
/// about half a megabyte. The small drops of an incremental reparse stay
/// below it.
const MERGE_FREED_NODES_AT: usize = 4096;

/// glibc keeps freed blocks of up to 128 bytes unmerged in its fastbins and
/// hands them out again newest first, so after a large tree is dropped the
/// next parse would get its nodes scattered over that tree's memory. A
/// request of this size makes glibc merge the freed blocks first. Other
/// allocators serve it and free it.
#[cold]
#[inline(never)]
fn merge_freed_nodes() {
    drop(std::hint::black_box(Vec::<u8>::with_capacity(1 << 16)));
}

fn queue_unique_children(children: &mut [Subtree], pending: &mut Vec<HeapSubtree>) {
    for child in children {
        let Subtree::Heap(node) = std::mem::take(child) else {
            continue;
        };
        // A child that is still shared, such as a node a newer tree reuses,
        // only loses this reference. Dropping it costs one atomic decrement,
        // where a failed try_unwrap costs a compare-exchange and then the same
        // decrement. If another thread drops its reference meanwhile, this
        // drop frees the child, and the child's own drop drains its
        // descendants without recursion.
        if node.strong_count() > 1 {
            drop(node);
        } else {
            pending.push(node);
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

/// ts_subtree_array_remove_trailing_extras for a node's child list.
#[inline]
pub(crate) fn ts_subtree_children_remove_trailing_extras(
    trees: &mut Children,
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

pub(crate) fn ts_subtree_pool_allocate(pool: &mut SubtreePool) -> Arc<LeafNode> {
    pool.free_trees
        .pop()
        .unwrap_or_else(|| Arc::new(LeafNode::default()))
}

pub(crate) fn ts_subtree_pool_free(pool: &mut SubtreePool, tree: Arc<LeafNode>) {
    let mut tree = tree;
    if pool.free_trees.capacity() > 0
        && pool.free_trees.len() < TS_MAX_TREE_POOL_SIZE
        && let Some(node) = Arc::get_mut(&mut tree)
    {
        // Only uniquely owned allocations may enter the pool. Clear the payload
        // as well, so cached headers do not retain scanner states.
        node.data = SubtreeHeapData::default();
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
        Subtree::inline(InlineLeaf {
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
        Subtree::Heap(HeapSubtree::Leaf(new_heap_leaf(
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
        )))
    }
}

// Keep allocation/pool cleanup off the common inline-leaf construction path.
// Return only the Arc, not an aggregate Subtree return slot: the caller supplies
// the known heap tag and keeps the inline-token path independent of that slot.
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
) -> Arc<LeafNode> {
    let metadata = ts_language_symbol_metadata(language, symbol);
    let extra = symbol == BUILTIN_SYM_END;
    let mut header = SubtreeHeapData {
        padding,
        size,
        lookahead_bytes,
        symbol,
        parse_state,
        flags: HeapFlags::default()
            .with_visible(metadata.visible)
            .with_named(metadata.named)
            .with_extra(extra)
            .with_has_external_tokens(has_external_tokens)
            .with_depends_on_column(depends_on_column)
            .with_is_keyword(is_keyword),
        payload: if has_external_tokens {
            SubtreePayload::External(ExternalScannerState::default())
        } else {
            SubtreePayload::Leaf
        },
        error_cost: 0,
    };
    initialize(&mut header);
    if let Some(mut node) = pool.free_trees.pop() {
        Arc::get_mut(&mut node)
            .expect("pooled subtrees must be unique")
            .data = header;
        node
    } else {
        // Initialize fresh allocations directly: no default header to
        // overwrite, and no uniqueness synchronization before first use.
        Arc::new(HeapNode {
            data: header,
            children: [],
        })
    }
}

pub(crate) fn ts_subtree_set_symbol(tree: &mut Subtree, symbol: Symbol, language: &Language) {
    let metadata = ts_language_symbol_metadata(language, symbol);
    match tree {
        Subtree::Inline(data) => {
            assert!(symbol < u8::MAX as Symbol);
            data.update(|data| {
                data.symbol = symbol as u8;
                data.flags = (data.flags & !(NAMED | VISIBLE))
                    | ((metadata.named as u8) * NAMED)
                    | ((metadata.visible as u8) * VISIBLE);
            });
        }
        Subtree::Heap(data) => {
            let data = data.make_mut();
            data.symbol = symbol;
            data.set_named(metadata.named);
            data.set_visible(metadata.visible);
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
    data.set_fragile_left(true);
    data.set_fragile_right(true);
    data.payload = SubtreePayload::Error(lookahead_char);
    result
}

pub(crate) fn ts_subtree_clone(tree: &Subtree) -> Subtree {
    match tree {
        // A new header and child buffer, with retained child handles. Immutable
        // external scanner bytes can remain shared even when the header is not.
        Subtree::Heap(data) => Subtree::Heap(data.clone_node()),
        _ => tree.clone(),
    }
}

pub(crate) fn ts_subtree_make_mut(pool: &mut SubtreePool, tree: Subtree) -> Subtree {
    if let Subtree::Heap(data) = &tree
        && data.strong_count() > 1
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
            Subtree::Heap(data) if data.strong_count() == 1 && data.child_count() >= 2 => {
                match &data.children()[0] {
                    Subtree::Heap(child)
                        if child.strong_count() == 1
                            && child.child_count() >= 2
                            && child.symbol == symbol =>
                    {
                        matches!(
                            &child.children()[0],
                            Subtree::Heap(grandchild)
                                if grandchild.strong_count() == 1
                                    && grandchild.child_count() >= 2
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

        // Each mutable access proves an Arc unique with an atomic operation, so
        // every node is accessed once on the way down and once on the way up.
        let (_, children) = unique_heap_mut(&mut current);
        let mut child = std::mem::take(&mut children[0]);
        let (_, child_children) = unique_heap_mut(&mut child);
        let mut grandchild = std::mem::take(&mut child_children[0]);
        let (_, grandchild_children) = unique_heap_mut(&mut grandchild);
        let last = grandchild_children.last_mut().unwrap();
        child_children[0] = std::mem::take(last);
        *last = child;
        // The stack owns the detached ancestors. Their first-child slots are
        // restored on the way back up, avoiding both aliases and recursion.
        stack.push(current);
        current = grandchild;
    }

    while stack.len() > initial_stack_size {
        let mut parent = stack.pop().unwrap();
        let (parent_data, parent_children) = unique_heap_mut(&mut parent);
        let child = &mut parent_children[0];
        *child = current;
        let (child_data, child_children) = unique_heap_mut(child);
        let (grandchild_data, grandchild_children) =
            unique_heap_mut(child_children.last_mut().unwrap());
        summarize_children(grandchild_data, grandchild_children, language);
        summarize_children(child_data, child_children, language);
        if stack.len() > initial_stack_size {
            // C summarizes this node now and again in the next step, as that
            // step's child, after its last child is summarized. Nothing reads
            // it in between. The second summary sets every field again: the
            // sticky flags from the same children, whose own flags only grew,
            // and the rest afresh, except that it tests this node's previous
            // size to tell whether the first child depends on the column. So
            // only the size is computed now.
            summarize_size(parent_data, parent_children);
        } else {
            summarize_children(parent_data, parent_children, language);
        }
        current = parent;
    }
    *tree = current;
}

/// The padding and size a summary of the node's children would give it.
#[inline]
fn summarize_size(data: &mut SubtreeHeapData, children: &[Subtree]) {
    let mut size = ts_subtree_size(&children[0]);
    for child in &children[1..] {
        size = length_add(size, ts_subtree_total_size(child));
    }
    data.padding = ts_subtree_padding(&children[0]);
    data.size = size;
}

/// The header and children of a heap subtree that has no other owner.
#[inline]
fn unique_heap_mut(tree: &mut Subtree) -> (&mut SubtreeHeapData, &mut [Subtree]) {
    let Subtree::Heap(data) = tree else {
        unreachable!("only heap branches are rotated");
    };
    data.get_mut().expect("rotated subtrees are uniquely owned")
}

pub(crate) fn ts_subtree_summarize_children(tree: &mut Subtree, language: &Language) {
    let Subtree::Heap(data) = tree else {
        panic!("cannot summarize an inline leaf");
    };
    let (data, children) = data.make_mut_with_children();
    summarize_children(data, children, language);
}

// Accumulating a summary does not own the child vector. In particular, fresh
// reductions can compute it before constructing the node, which has a Drop impl.
#[cfg_attr(test, derive(Clone, Debug))]
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
fn summarize_children(data: &mut SubtreeHeapData, children: &[Subtree], language: &Language) {
    let SubtreePayload::Branch(branch) = &mut data.payload else {
        panic!("child summaries require a branch payload");
    };
    // Only the inputs are read from the header. The summary sets every output.
    let mut summary = ChildSummary {
        branch: BranchData {
            production_id: branch.production_id,
            first_leaf: branch.first_leaf,
            ..BranchData::default()
        },
        padding: data.padding,
        size: data.size,
        lookahead_bytes: 0,
        error_cost: 0,
        parse_state: data.parse_state,
        has_external_tokens: false,
        has_external_scanner_state_change: false,
        depends_on_column: false,
        fragile_left: data.flags.fragile_left(),
        fragile_right: data.flags.fragile_right(),
        symbol: data.symbol,
        visible: data.flags.visible(),
        named: data.flags.named(),
    };
    summarize_child_slice(&mut summary, children, language);
    *branch = summary.branch;
    data.padding = summary.padding;
    data.size = summary.size;
    data.lookahead_bytes = summary.lookahead_bytes;
    data.error_cost = summary.error_cost;
    data.parse_state = summary.parse_state;
    data.set_has_external_tokens(summary.has_external_tokens);
    data.set_has_external_scanner_state_change(summary.has_external_scanner_state_change);
    data.set_depends_on_column(summary.depends_on_column);
    data.set_fragile_left(summary.fragile_left);
    data.set_fragile_right(summary.fragile_right);
}

#[inline(always)]
fn summarize_child_slice(data: &mut ChildSummary, children: &[Subtree], language: &Language) {
    // Apply the unary shortcut to the non-owning accumulator, both before fresh
    // header construction and when re-summarizing an existing header. No owning
    // header or child Vec needs to be created or moved during this computation.
    if children.len() == 1
        && data.branch.production_id == 0
        && data.symbol != BUILTIN_SYM_ERROR
        && data.symbol != BUILTIN_SYM_ERROR_REPEAT
    {
        summarize_unary(data, &children[0]);
    } else {
        summarize_child_slice_general(data, children, language);
    }
}

#[inline(always)]
fn summarize_unary(data: &mut ChildSummary, child: &Subtree) {
    // Decode once and finish reading the child before publishing the summary.
    // Preserve the old-row dependency test and sticky flags on re-summarization.
    let mut summary = BranchData::default();
    let mut fragile_left = data.fragile_left;
    let mut fragile_right = data.fragile_right;
    let mut parse_state = data.parse_state;
    let (
        padding,
        size,
        lookahead_bytes,
        error_cost,
        depends_on_column,
        has_external_tokens,
        has_external_scanner_state_change,
    ) = match child {
        Subtree::Inline(leaf) => {
            let leaf = leaf.fields();
            summary.first_leaf = FirstLeaf {
                symbol: leaf.symbol as Symbol,
                parse_state: leaf.parse_state,
            };
            if leaf.flags & VISIBLE != 0 {
                summary.visible_child_count = 1;
                summary.named_child_count = u32::from(leaf.flags & NAMED != 0);
                summary.visible_descendant_count = 1;
            }
            let error_cost = if leaf.flags & MISSING != 0 {
                ERROR_COST_PER_MISSING_TREE + ERROR_COST_PER_RECOVERY
            } else {
                0
            };
            (
                ts_subtree_padding(child),
                ts_subtree_size(child),
                (leaf.padding_rows_and_lookahead >> 4) as u32,
                error_cost,
                false,
                false,
                false,
            )
        }
        Subtree::Heap(child) => {
            let has_children = child.has_children();
            summary.first_leaf = FirstLeaf {
                symbol: child.symbol,
                parse_state: child.parse_state,
            };
            if let SubtreePayload::Branch(branch) = &child.payload {
                summary.visible_descendant_count = branch.visible_descendant_count;
                if has_children {
                    summary.first_leaf = branch.first_leaf;
                    summary.dynamic_precedence = branch.dynamic_precedence;
                    if !child.visible() {
                        summary.visible_child_count = branch.visible_child_count;
                        summary.named_child_count = branch.named_child_count;
                    }
                }
            }
            if child.visible() {
                summary.visible_child_count = 1;
                summary.named_child_count = u32::from(child.named());
                summary.visible_descendant_count = summary.visible_descendant_count.wrapping_add(1);
            }
            let error_cost = if child.symbol == BUILTIN_SYM_ERROR_REPEAT {
                0
            } else if child.is_missing() {
                ERROR_COST_PER_MISSING_TREE + ERROR_COST_PER_RECOVERY
            } else {
                child.error_cost
            };
            fragile_left |= child.fragile_left();
            fragile_right |= child.fragile_right();
            if child.symbol == BUILTIN_SYM_ERROR {
                fragile_left = true;
                fragile_right = true;
                parse_state = TS_TREE_STATE_NONE;
            }
            (
                child.padding,
                child.size,
                child.lookahead_bytes,
                error_cost,
                data.size.extent.row == 0 && child.depends_on_column(),
                child.has_external_tokens(),
                child.has_external_scanner_state_change(),
            )
        }
        Subtree::Null => (length_zero(), length_zero(), 0, 0, false, false, false),
    };
    data.error_cost = error_cost;
    data.depends_on_column = depends_on_column;
    data.has_external_tokens = has_external_tokens;
    data.has_external_scanner_state_change = has_external_scanner_state_change;
    data.fragile_left = fragile_left;
    data.fragile_right = fragile_right;
    data.parse_state = parse_state;
    data.padding = padding;
    data.size = size;
    data.lookahead_bytes = lookahead_bytes;
    data.branch = summary;
}

/// The running totals of a summary over a node's children.
struct ChildTotals<'a> {
    padding: Length,
    size: Length,
    error_cost: u32,
    visible_child_count: u32,
    named_child_count: u32,
    visible_descendant_count: u32,
    dynamic_precedence: i32,
    has_external_tokens: bool,
    depends_on_column: bool,
    has_external_scanner_state_change: bool,
    fragile_left: bool,
    fragile_right: bool,
    parse_state: StateId,
    lookahead_end_byte: u32,
    structural_index: usize,
    alias_sequence: &'a [Symbol],
    is_error: bool,
    // What the summary needs from the first and the last child, read while
    // those children are decoded instead of decoding them again afterwards.
    first_leaf: FirstLeaf,
    first_symbol: Symbol,
    first_repeat_depth: u32,
    last_repeat_depth: u32,
}

impl ChildTotals<'_> {
    /// Add one child. Repetition nodes, which balancing rotates, have two
    /// children, as do many reductions, so that case passes constant positions.
    #[inline(always)]
    fn add(&mut self, child: &Subtree, is_first: bool, is_last: bool, language: &Language) {
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
            child_visible_child_count,
            child_named_child_count,
        ) = match child {
            Subtree::Inline(leaf) => {
                let leaf = leaf.fields();
                if leaf.flags & MISSING != 0 {
                    self.error_cost = self
                        .error_cost
                        .wrapping_add(ERROR_COST_PER_MISSING_TREE + ERROR_COST_PER_RECOVERY);
                }
                let symbol = Symbol::from(leaf.symbol);
                if is_first {
                    self.first_leaf = FirstLeaf {
                        symbol,
                        parse_state: leaf.parse_state,
                    };
                    self.first_symbol = symbol;
                    self.first_repeat_depth = 0;
                }
                if is_last {
                    self.last_repeat_depth = 0;
                }
                (
                    Length {
                        bytes: u32::from(leaf.padding_bytes),
                        extent: Point {
                            row: u32::from(leaf.padding_rows_and_lookahead & 15),
                            column: u32::from(leaf.padding_columns),
                        },
                    },
                    Length {
                        bytes: u32::from(leaf.size_bytes),
                        extent: Point {
                            row: 0,
                            column: u32::from(leaf.size_bytes),
                        },
                    },
                    u32::from(leaf.padding_rows_and_lookahead >> 4),
                    symbol,
                    leaf.flags & VISIBLE != 0,
                    leaf.flags & NAMED != 0,
                    leaf.flags & EXTRA != 0,
                    false,
                    0,
                    0,
                )
            }
            Subtree::Heap(child) => {
                // C checks the existing size before replacing it for the
                // first child, including when re-summarizing a rotation.
                if self.size.extent.row == 0 && child.depends_on_column() {
                    self.depends_on_column = true;
                }
                self.has_external_scanner_state_change |= child.has_external_scanner_state_change();
                self.has_external_tokens |= child.has_external_tokens();
                if child.symbol != BUILTIN_SYM_ERROR_REPEAT {
                    self.error_cost = self.error_cost.wrapping_add(if child.is_missing() {
                        ERROR_COST_PER_MISSING_TREE + ERROR_COST_PER_RECOVERY
                    } else {
                        child.error_cost
                    });
                }
                if child.symbol == BUILTIN_SYM_ERROR {
                    self.fragile_left = true;
                    self.fragile_right = true;
                    self.parse_state = TS_TREE_STATE_NONE;
                }
                let has_children = child.has_children();
                let mut first_leaf = FirstLeaf {
                    symbol: child.symbol,
                    parse_state: child.parse_state,
                };
                let (child_visible_child_count, child_named_child_count, repeat_depth) =
                    match &child.payload {
                        SubtreePayload::Branch(child_branch) => {
                            if has_children {
                                // Empty reductions have a branch payload, but C
                                // treats them as leaves for dynamic precedence.
                                self.dynamic_precedence = self
                                    .dynamic_precedence
                                    .wrapping_add(child_branch.dynamic_precedence);
                                first_leaf = child_branch.first_leaf;
                            }
                            self.visible_descendant_count = self
                                .visible_descendant_count
                                .wrapping_add(child_branch.visible_descendant_count);
                            (
                                child_branch.visible_child_count,
                                child_branch.named_child_count,
                                u32::from(child_branch.repeat_depth),
                            )
                        }
                        // See ts_subtree_repeat_depth for this overlap with C's union.
                        SubtreePayload::External(ExternalScannerState::Inline { data, .. }) => {
                            (0, 0, u32::from(u16::from_ne_bytes([data[16], data[17]])))
                        }
                        _ => (0, 0, 0),
                    };
                if is_first {
                    self.first_leaf = first_leaf;
                    self.first_symbol = child.symbol;
                    self.first_repeat_depth = repeat_depth;
                    self.fragile_left |= child.fragile_left();
                }
                if is_last {
                    self.last_repeat_depth = repeat_depth;
                    self.fragile_right |= child.fragile_right();
                }
                (
                    child.padding,
                    child.size,
                    child.lookahead_bytes,
                    child.symbol,
                    child.visible(),
                    child.named(),
                    child.extra(),
                    has_children,
                    child_visible_child_count,
                    child_named_child_count,
                )
            }
            Subtree::Null => {
                if is_first {
                    self.first_leaf = FirstLeaf::default();
                    self.first_symbol = 0;
                    self.first_repeat_depth = 0;
                }
                if is_last {
                    self.last_repeat_depth = 0;
                }
                (
                    length_zero(),
                    length_zero(),
                    0,
                    0,
                    false,
                    false,
                    false,
                    false,
                    0,
                    0,
                )
            }
        };

        if is_first {
            self.padding = child_padding;
            self.size = child_size;
        } else {
            self.size = length_add(self.size, length_add(child_padding, child_size));
        }
        let child_lookahead_end_byte = self
            .padding
            .bytes
            .wrapping_add(self.size.bytes)
            .wrapping_add(lookahead);
        self.lookahead_end_byte = self.lookahead_end_byte.max(child_lookahead_end_byte);

        if self.is_error && !extra && !(symbol == BUILTIN_SYM_ERROR && !has_children) {
            if visible {
                self.error_cost = self.error_cost.wrapping_add(ERROR_COST_PER_SKIPPED_TREE);
            } else if has_children {
                self.error_cost = self.error_cost.wrapping_add(
                    ERROR_COST_PER_SKIPPED_TREE.wrapping_mul(child_visible_child_count),
                );
            }
        }

        if !extra
            && symbol != 0
            && !self.alias_sequence.is_empty()
            && self.alias_sequence[self.structural_index] != 0
        {
            self.visible_descendant_count = self.visible_descendant_count.wrapping_add(1);
            self.visible_child_count = self.visible_child_count.wrapping_add(1);
            if ts_language_symbol_metadata(language, self.alias_sequence[self.structural_index])
                .named
            {
                self.named_child_count = self.named_child_count.wrapping_add(1);
            }
        } else if visible {
            self.visible_descendant_count = self.visible_descendant_count.wrapping_add(1);
            self.visible_child_count = self.visible_child_count.wrapping_add(1);
            self.named_child_count = self.named_child_count.wrapping_add(u32::from(named));
        } else if has_children {
            self.visible_child_count = self
                .visible_child_count
                .wrapping_add(child_visible_child_count);
            self.named_child_count = self.named_child_count.wrapping_add(child_named_child_count);
        }
        self.structural_index += usize::from(!extra);
    }
}

#[inline(always)]
fn summarize_child_slice_general(
    data: &mut ChildSummary,
    children: &[Subtree],
    language: &Language,
) {
    let production_id = data.branch.production_id;
    let mut totals = ChildTotals {
        padding: data.padding,
        size: data.size,
        error_cost: 0,
        visible_child_count: 0,
        named_child_count: 0,
        visible_descendant_count: 0,
        dynamic_precedence: 0,
        has_external_tokens: false,
        depends_on_column: false,
        has_external_scanner_state_change: false,
        fragile_left: data.fragile_left,
        fragile_right: data.fragile_right,
        parse_state: data.parse_state,
        lookahead_end_byte: 0,
        structural_index: 0,
        alias_sequence: ts_language_alias_sequence(language, u32::from(production_id)),
        is_error: data.symbol == BUILTIN_SYM_ERROR || data.symbol == BUILTIN_SYM_ERROR_REPEAT,
        first_leaf: data.branch.first_leaf,
        first_symbol: 0,
        first_repeat_depth: 0,
        last_repeat_depth: 0,
    };
    if let [first, last] = children {
        totals.add(first, true, false, language);
        totals.add(last, false, true, language);
    } else {
        let last_index = children.len().wrapping_sub(1);
        for (i, child) in children.iter().enumerate() {
            totals.add(child, i == 0, i == last_index, language);
        }
    }

    let mut error_cost = totals.error_cost;
    let (padding, size) = (totals.padding, totals.size);
    let lookahead_bytes = totals
        .lookahead_end_byte
        .wrapping_sub(size.bytes)
        .wrapping_sub(padding.bytes);
    if totals.is_error {
        error_cost = error_cost
            .wrapping_add(ERROR_COST_PER_RECOVERY)
            .wrapping_add(ERROR_COST_PER_SKIPPED_CHAR.wrapping_mul(size.bytes))
            .wrapping_add(ERROR_COST_PER_SKIPPED_LINE.wrapping_mul(size.extent.row));
    }

    let repeat_depth = if children.len() >= 2
        && !data.visible
        && !data.named
        && totals.first_symbol == data.symbol
    {
        totals
            .first_repeat_depth
            .max(totals.last_repeat_depth)
            .wrapping_add(1) as u16
    } else {
        0
    };
    data.branch = BranchData {
        visible_child_count: totals.visible_child_count,
        named_child_count: totals.named_child_count,
        visible_descendant_count: totals.visible_descendant_count,
        dynamic_precedence: totals.dynamic_precedence,
        repeat_depth,
        production_id,
        first_leaf: totals.first_leaf,
    };
    data.padding = padding;
    data.size = size;
    data.error_cost = error_cost;
    data.has_external_tokens = totals.has_external_tokens;
    data.depends_on_column = totals.depends_on_column;
    data.has_external_scanner_state_change = totals.has_external_scanner_state_change;
    data.fragile_left = totals.fragile_left;
    data.fragile_right = totals.fragile_right;
    data.parse_state = totals.parse_state;
    data.lookahead_bytes = lookahead_bytes;
}

pub(crate) fn ts_subtree_new_node(
    symbol: Symbol,
    children: impl Into<Children>,
    production_id: u32,
    language: &Language,
) -> Subtree {
    ts_subtree_new_node_with(symbol, children, production_id, language, |_| {})
}

/// Initialize a reduction's header while it is exclusively owned, before Arc
/// introduces the need for copy-on-write uniqueness checks.
#[inline]
pub(crate) fn ts_subtree_new_node_with(
    symbol: Symbol,
    children: impl Into<Children>,
    production_id: u32,
    language: &Language,
    initialize: impl FnOnce(&mut SubtreeHeapData),
) -> Subtree {
    let children = children.into();
    let metadata = ts_language_symbol_metadata(language, symbol);
    let fragile = symbol == BUILTIN_SYM_ERROR || symbol == BUILTIN_SYM_ERROR_REPEAT;
    // Finish the non-owning summary first, so the hot loop need not initialize
    // and repeatedly update an owning, droppable heap node. Keep the children
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
        flags: HeapFlags::default()
            .with_visible(metadata.visible)
            .with_named(metadata.named)
            .with_has_external_tokens(summary.has_external_tokens)
            .with_has_external_scanner_state_change(summary.has_external_scanner_state_change)
            .with_depends_on_column(summary.depends_on_column)
            .with_fragile_left(summary.fragile_left)
            .with_fragile_right(summary.fragile_right),
        padding: summary.padding,
        size: summary.size,
        lookahead_bytes: summary.lookahead_bytes,
        error_cost: summary.error_cost,
        parse_state: summary.parse_state,
        payload: SubtreePayload::Branch(summary.branch),
    };
    initialize(&mut result);
    Subtree::Heap(HeapSubtree::new(result, children))
}

pub(crate) fn ts_subtree_new_error_node(
    children: impl Into<Children>,
    extra: bool,
    language: &Language,
) -> Subtree {
    let mut result = ts_subtree_new_node(BUILTIN_SYM_ERROR, children, 0, language);
    result.heap_mut().unwrap().set_extra(extra);
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
        Subtree::Inline(data) => data.set_flag(MISSING, true),
        Subtree::Heap(data) => data.make_mut().set_is_missing(true),
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

fn release_heap(pool: &mut SubtreePool, mut data: HeapSubtree) {
    pool.tree_stack.clear();
    // Shared handles only need Arc's decrement. The initial unique handle can
    // be consumed directly, without a round trip through the worklist.
    if data.strong_count() > 1 {
        return;
    }
    loop {
        // Queuing only checks the strong count; prove uniqueness once here.
        // Weak observers may prevent pooling, in which case normal Arc Drop
        // still drains the tree iteratively.
        match data {
            HeapSubtree::Leaf(mut leaf) => {
                if let Some(node) = Arc::get_mut(&mut leaf)
                    && pool.free_trees.capacity() > 0
                    && pool.free_trees.len() < TS_MAX_TREE_POOL_SIZE
                {
                    // This already-borrowed header is unique. Avoid a second
                    // uniqueness check just to reset it and move it into the pool.
                    node.data = SubtreeHeapData::default();
                    pool.free_trees.push(leaf);
                }
            }
            mut branch => {
                if let Some((_, children)) = branch.get_mut() {
                    for child in children {
                        subtree_queue_release(&mut pool.tree_stack, std::mem::take(child));
                    }
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
        && data.strong_count() == 1
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
        Subtree::Inline(data) => data.set_flag(HAS_CHANGES, true),
        Subtree::Heap(data) => data.make_mut().set_has_changes(true),
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
                    data.update(|data| {
                        data.padding_bytes = padding.bytes as u8;
                        data.padding_columns = padding.extent.column as u8;
                        data.padding_rows_and_lookahead =
                            (data.padding_rows_and_lookahead & 0xf0) | padding.extent.row as u8;
                        data.size_bytes = size.bytes as u8;
                    });
                } else {
                    let data = data.fields();
                    let mut heap = ts_subtree_pool_allocate(pool);
                    Arc::get_mut(&mut heap)
                        .expect("pooled heaps are unique")
                        .data = SubtreeHeapData {
                        padding,
                        size,
                        lookahead_bytes,
                        symbol: data.symbol as Symbol,
                        parse_state: data.parse_state,
                        flags: HeapFlags::default()
                            .with_visible(data.flags & VISIBLE != 0)
                            .with_named(data.flags & NAMED != 0)
                            .with_extra(data.flags & EXTRA != 0)
                            .with_is_missing(data.flags & MISSING != 0)
                            .with_is_keyword(data.flags & KEYWORD != 0),
                        payload: SubtreePayload::Leaf,
                        ..SubtreeHeapData::default()
                    };
                    *tree = Subtree::Heap(HeapSubtree::Leaf(heap));
                }
            }
            Subtree::Heap(data) => {
                let data = data.make_mut();
                data.padding = padding;
                data.size = size;
            }
            Subtree::Null => panic!("cannot edit a null subtree"),
        }
        ts_subtree_set_has_changes(tree);

        let mut child_right = length_zero();
        if let Subtree::Heap(data) = tree {
            for (i, child) in data.make_mut_with_children().1.iter_mut().enumerate() {
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
    let Subtree::Heap(data) = tree else {
        return None;
    };
    if data.has_external_tokens()
        && !data.has_children()
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
    #[inline]
    pub(crate) fn inline(data: InlineLeaf) -> Self {
        Self::Inline(PackedInlineLeaf::new(data))
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    /// Moves `tree` into this handle, which must be null. An assignment would
    /// drop the null handle first, and that drop is a call, since the compiler
    /// doesn't inline `Subtree`'s drop with five Arc types behind a heap handle.
    #[inline]
    pub(crate) fn fill(&mut self, tree: Subtree) {
        debug_assert!(self.is_null(), "only a null subtree is filled");
        std::mem::forget(std::mem::replace(self, tree));
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
            Some(data.make_mut())
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
            (Self::Heap(a), Self::Heap(b)) => std::ptr::eq(a.as_ptr(), b.as_ptr()),
            _ => false,
        }
    }
}

#[inline]
pub(crate) fn ts_subtree_visible(tree: &Subtree) -> bool {
    match tree {
        Subtree::Inline(d) => d.fields().flags & VISIBLE != 0,
        Subtree::Heap(d) => d.visible(),
        Subtree::Null => false,
    }
}

#[inline]
pub(crate) fn ts_subtree_named(tree: &Subtree) -> bool {
    match tree {
        Subtree::Inline(d) => d.fields().flags & NAMED != 0,
        Subtree::Heap(d) => d.named(),
        Subtree::Null => false,
    }
}

#[inline]
pub(crate) fn ts_subtree_extra(tree: &Subtree) -> bool {
    match tree {
        Subtree::Inline(d) => d.fields().flags & EXTRA != 0,
        Subtree::Heap(d) => d.extra(),
        Subtree::Null => false,
    }
}

pub(crate) fn ts_subtree_has_changes(tree: &Subtree) -> bool {
    match tree {
        Subtree::Inline(d) => d.fields().flags & HAS_CHANGES != 0,
        Subtree::Heap(d) => d.has_changes(),
        Subtree::Null => false,
    }
}

pub(crate) fn ts_subtree_missing(tree: &Subtree) -> bool {
    match tree {
        Subtree::Inline(d) => d.fields().flags & MISSING != 0,
        Subtree::Heap(d) => d.is_missing(),
        Subtree::Null => false,
    }
}

pub(crate) fn ts_subtree_is_keyword(tree: &Subtree) -> bool {
    match tree {
        Subtree::Inline(d) => d.fields().flags & KEYWORD != 0,
        Subtree::Heap(d) => d.is_keyword(),
        Subtree::Null => false,
    }
}

#[inline]
pub(crate) fn ts_subtree_symbol(tree: &Subtree) -> Symbol {
    match tree {
        Subtree::Inline(d) => d.fields().symbol as Symbol,
        Subtree::Heap(d) => d.symbol,
        Subtree::Null => 0,
    }
}

pub(crate) fn ts_subtree_parse_state(tree: &Subtree) -> StateId {
    match tree {
        Subtree::Inline(d) => d.fields().parse_state,
        Subtree::Heap(d) => d.parse_state,
        Subtree::Null => 0,
    }
}

pub(crate) fn ts_subtree_lookahead_bytes(tree: &Subtree) -> u32 {
    match tree {
        Subtree::Inline(d) => (d.fields().padding_rows_and_lookahead >> 4) as u32,
        Subtree::Heap(d) => d.lookahead_bytes,
        Subtree::Null => 0,
    }
}

pub(crate) fn ts_subtree_fragile_left(tree: &Subtree) -> bool {
    tree.heap().is_some_and(|d| d.fragile_left())
}

pub(crate) fn ts_subtree_fragile_right(tree: &Subtree) -> bool {
    tree.heap().is_some_and(|d| d.fragile_right())
}

pub(crate) fn ts_subtree_has_external_tokens(tree: &Subtree) -> bool {
    tree.heap().is_some_and(|d| d.has_external_tokens())
}

pub(crate) fn ts_subtree_has_external_scanner_state_change(tree: &Subtree) -> bool {
    tree.heap()
        .is_some_and(|d| d.has_external_scanner_state_change())
}

pub(crate) fn ts_subtree_depends_on_column(tree: &Subtree) -> bool {
    tree.heap().is_some_and(|d| d.depends_on_column())
}

pub(crate) fn ts_subtree_visible_child_count(tree: &Subtree) -> u32 {
    tree.branch().map_or(0, |d| d.visible_child_count)
}

pub(crate) fn ts_subtree_named_child_count(tree: &Subtree) -> u32 {
    tree.branch().map_or(0, |d| d.named_child_count)
}

#[inline]
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
    match tree.heap().map(|data| &data.payload) {
        Some(SubtreePayload::Branch(data)) => u32::from(data.repeat_depth),
        // C does not check child_count here: repeat_depth overlaps bytes 16..18
        // of the external scanner state's inline buffer in SubtreeHeapData's
        // union. Preserve that value because it affects repetition balancing
        // (and its progress checkpoints), even though this is a leaf.
        Some(SubtreePayload::External(ExternalScannerState::Inline { data, .. })) => {
            u32::from(u16::from_ne_bytes([data[16], data[17]]))
        }
        // A long scanner state's pointer, and an error leaf's lookahead_char,
        // do not overlap repeat_depth. The leaf constructor zeroes this region.
        _ => 0,
    }
}

#[inline]
pub(crate) fn ts_subtree_children(tree: &Subtree) -> &[Subtree] {
    match tree {
        Subtree::Heap(data) => data.children(),
        _ => &[],
    }
}
#[inline]
pub(crate) fn ts_subtree_child_count(tree: &Subtree) -> u32 {
    match tree {
        Subtree::Heap(data) => data.child_count() as u32,
        _ => 0,
    }
}
pub(crate) fn ts_subtree_set_extra(tree: &mut Subtree, extra: bool) {
    match tree {
        Subtree::Inline(d) => d.set_flag(EXTRA, extra),
        Subtree::Heap(d) => d.make_mut().set_extra(extra),
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
#[inline]
pub(crate) fn ts_subtree_padding(tree: &Subtree) -> Length {
    match tree {
        Subtree::Inline(d) => Length {
            bytes: d.fields().padding_bytes as u32,
            extent: Point {
                row: (d.fields().padding_rows_and_lookahead & 15) as u32,
                column: d.fields().padding_columns as u32,
            },
        },
        Subtree::Heap(d) => d.padding,
        Subtree::Null => length_zero(),
    }
}
#[inline]
pub(crate) fn ts_subtree_size(tree: &Subtree) -> Length {
    match tree {
        Subtree::Inline(d) => Length {
            bytes: d.fields().size_bytes as u32,
            extent: Point {
                row: 0,
                column: d.fields().size_bytes as u32,
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
    ts_subtree_symbol(tree) == tree_sitter_language::BUILTIN_SYM_ERROR
}
pub(crate) fn ts_subtree_is_eof(tree: &Subtree) -> bool {
    ts_subtree_symbol(tree) == tree_sitter_language::BUILTIN_SYM_END
}
pub(crate) fn ts_subtree_from_mut(tree: MutableSubtree) -> Subtree {
    tree
}
// Unlike the C reinterpretation, Rust transfers ownership; subsequent heap mutation
// still goes through HeapSubtree::make_mut. This function needs no unsafe code.
pub(crate) fn ts_subtree_to_mut_unsafe(tree: Subtree) -> MutableSubtree {
    tree
}
#[cfg(test)]
mod layout_tests {
    use super::*;
    #[test]
    fn compact_handles() {
        assert_eq!(std::mem::size_of::<InlineLeaf>(), 8);
        assert_eq!(std::mem::size_of::<PackedInlineLeaf>(), 8);
        #[cfg(target_pointer_width = "64")]
        {
            assert_eq!(std::mem::size_of::<Subtree>(), 16);
            assert_eq!(std::mem::size_of::<Option<Subtree>>(), 16);
            // A node is its header and the child slots it uses.
            let header = std::mem::size_of::<SubtreeHeapData>();
            assert_eq!(header, 72);
            assert_eq!(std::mem::size_of::<LeafNode>(), header);
            assert_eq!(std::mem::size_of::<HeapNode<[Subtree; 3]>>(), header + 48);
        }
    }
    #[test]
    fn packed_inline_fields_round_trip_and_update_independently() {
        for value in 0..=u16::MAX {
            let byte = value as u8;
            let fields = InlineLeaf {
                parse_state: value,
                symbol: byte,
                flags: byte.wrapping_add(1),
                padding_bytes: byte.wrapping_add(2),
                padding_columns: byte.wrapping_add(3),
                size_bytes: byte.wrapping_add(4),
                padding_rows_and_lookahead: byte.wrapping_add(5),
            };
            let packed = PackedInlineLeaf::new(fields);
            assert_eq!(packed.fields(), fields);
            for flag in [VISIBLE, NAMED, EXTRA, HAS_CHANGES, MISSING, KEYWORD] {
                for enabled in [false, true] {
                    let mut actual = packed;
                    actual.set_flag(flag, enabled);
                    let mut expected = fields;
                    if enabled {
                        expected.flags |= flag;
                    } else {
                        expected.flags &= !flag;
                    }
                    assert_eq!(actual.fields(), expected);
                }
            }
            let mut actual = packed;
            actual.update(|data| data.parse_state = !value);
            assert_eq!(
                actual.fields(),
                InlineLeaf {
                    parse_state: !value,
                    ..fields
                }
            );
        }
    }

    #[test]
    fn heap_mutation_is_shallow_copy_on_write() {
        let child = SubtreeHeapData::default().with_children(Children::new());
        let original = SubtreeHeapData {
            payload: SubtreePayload::Branch(BranchData::default()),
            ..SubtreeHeapData::default()
        }
        .with_children(vec![child]);
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
                let leaf = Arc::new(LeafNode::default());
                let weak = Arc::downgrade(&leaf);
                let mut tree = Subtree::Heap(HeapSubtree::Leaf(leaf));
                for _ in 0..50_000 {
                    tree = SubtreeHeapData {
                        payload: SubtreePayload::Branch(BranchData::default()),
                        ..SubtreeHeapData::default()
                    }
                    .with_children(vec![tree]);
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
                let shared_leaf = SubtreeHeapData::default().with_children(Children::new());
                let mut tree = shared_leaf.clone();
                let mut observers = Vec::new();
                for _ in 0..20_000 {
                    let data = Arc::new(HeapNode {
                        data: SubtreeHeapData {
                            payload: SubtreePayload::Branch(BranchData::default()),
                            ..SubtreeHeapData::default()
                        },
                        children: [tree],
                    });
                    observers.push(Arc::downgrade(&data));
                    tree = Subtree::Heap(HeapSubtree::One(data));
                }
                let mut pool = ts_subtree_pool_new(32);
                ts_subtree_release(&mut pool, tree);
                assert!(pool.tree_stack.is_empty());
                assert!(pool.free_trees.is_empty());
                assert!(observers.iter().all(|weak| weak.upgrade().is_none()));
                let Subtree::Heap(data) = shared_leaf else {
                    unreachable!()
                };
                assert_eq!(data.strong_count(), 1);
            })
            .unwrap()
            .join()
            .unwrap();
    }

    #[test]
    fn small_leaf_extents() {
        let tree = Subtree::inline(InlineLeaf {
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
        Subtree::inline(InlineLeaf {
            symbol,
            flags: if extra { EXTRA } else { 0 },
            ..InlineLeaf::default()
        })
    }

    #[test]
    fn release_fast_paths_preserve_scratch_and_shared_ownership() {
        let shared = Arc::new(LeafNode::default());
        let mut pool = ts_subtree_pool_new(32);
        pool.tree_stack
            .push(Subtree::Heap(HeapSubtree::Leaf(shared.clone())));
        ts_subtree_release(&mut pool, Subtree::Null);
        ts_subtree_release(&mut pool, leaf(1, false));
        assert_eq!(pool.tree_stack.len(), 1);
        assert_eq!(Arc::strong_count(&shared), 2);

        ts_subtree_release(&mut pool, Subtree::Heap(HeapSubtree::Leaf(shared.clone())));
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
            SubtreeHeapData::default().with_children(Children::new()),
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
        Arc::get_mut(&mut allocation).unwrap().data.symbol = 12;
        ts_subtree_pool_free(&mut pool, allocation);
        assert_eq!(pool.free_trees.len(), 1);
        let reused = ts_subtree_pool_allocate(&mut pool);
        assert_eq!(Arc::as_ptr(&reused), address);
        assert_eq!(reused.data.symbol, 0);
        assert!(pool.free_trees.is_empty());

        // A still-live owner prevents the allocation from entering the pool.
        ts_subtree_pool_free(&mut pool, reused.clone());
        assert!(pool.free_trees.is_empty());
        ts_subtree_pool_free(&mut pool, reused);
        for _ in 0..40 {
            ts_subtree_pool_free(&mut pool, Arc::new(LeafNode::default()));
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
        let child = SubtreeHeapData {
            flags: HeapFlags::default().with_has_external_tokens(true),
            payload: SubtreePayload::External(state),
            ..SubtreeHeapData::default()
        }
        .with_children(Children::new());
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

        let parent = SubtreeHeapData {
            payload: SubtreePayload::Branch(BranchData::default()),
            ..SubtreeHeapData::default()
        }
        .with_children(vec![child]);
        let copied_parent = ts_subtree_clone(&parent);
        assert!(!copied_parent.ptr_eq(&parent));
        assert!(ts_subtree_children(&copied_parent)[0].ptr_eq(&ts_subtree_children(&parent)[0]));
        let mut pool = ts_subtree_pool_new(0);
        let address = if let Subtree::Heap(data) = &parent {
            data.as_ptr()
        } else {
            unreachable!()
        };
        let unique = ts_subtree_make_mut(&mut pool, parent);
        assert_eq!(
            match &unique {
                Subtree::Heap(data) => data.as_ptr(),
                _ => unreachable!(),
            },
            address
        );
    }

    #[test]
    fn empty_reductions_are_leaves_for_header_accessors() {
        let tree = SubtreeHeapData {
            symbol: 73,
            parse_state: 12,
            payload: SubtreePayload::Branch(BranchData {
                production_id: 4,
                dynamic_precedence: 10,
                ..BranchData::default()
            }),
            ..SubtreeHeapData::default()
        }
        .with_children(Children::new());
        assert_eq!(ts_subtree_leaf_symbol(&tree), 73);
        assert_eq!(ts_subtree_leaf_parse_state(&tree), 12);
        assert_eq!(ts_subtree_production_id(&tree), 0);
        assert_eq!(ts_subtree_dynamic_precedence(&tree), 0);
    }
}

#[cfg(test)]
mod operation_tests {
    use super::*;

    fn leaf(symbol: u8, size: u8) -> Subtree {
        Subtree::inline(InlineLeaf {
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
        SubtreeHeapData {
            symbol,
            size,
            flags: HeapFlags::default().with_visible(true).with_named(true),
            payload: SubtreePayload::Branch(BranchData::default()),
            ..SubtreeHeapData::default()
        }
        .with_children(children)
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
            HeapSubtree::strong_count(match &left {
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
            HeapSubtree::strong_count(match &child {
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
        data.set_has_external_tokens(true);
        data.payload = SubtreePayload::External(ExternalScannerState::default());
        let mut internal = branch(2, vec![external.clone(), leaf(4, 1)]);
        internal.heap_mut().unwrap().set_has_external_tokens(true);
        let mut root = branch(1, vec![external.clone(), internal.clone(), leaf(5, 1)]);
        root.heap_mut().unwrap().set_has_external_tokens(true);
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
            SubtreeHeapData {
                flags: HeapFlags::default().with_has_external_tokens(true),
                payload: SubtreePayload::External(ts_external_scanner_state_init(bytes)),
                ..SubtreeHeapData::default()
            }
            .with_children(Children::new())
        };
        let token = external(&[1, 2, 3]);
        let retained = token.clone();
        assert!(ts_subtree_external_scanner_state_eq(&token, &retained));
        assert!(ts_subtree_external_scanner_state_eq(
            &token,
            &external(&[1, 2, 3])
        ));
        assert!(!ts_subtree_external_scanner_state_eq(
            &token,
            &external(&[1, 2])
        ));
        assert!(!ts_subtree_external_scanner_state_eq(
            &token,
            &external(&[1, 2, 4])
        ));
        assert!(!ts_subtree_external_scanner_state_eq(
            &token,
            &Subtree::Null
        ));
        assert!(ts_subtree_external_scanner_state_eq(
            &external(&[]),
            &Subtree::Null
        ));
        assert!(ts_subtree_external_scanner_state_eq(
            &Subtree::Null,
            &Subtree::Null
        ));
        assert!(ts_subtree_external_scanner_state_eq(
            &leaf(1, 2),
            &leaf(3, 4)
        ));
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
    use tree_sitter_language::{LanguageTables, SymbolMetadata};

    /// ts_subtree_compress as C writes it, summarizing all three nodes of
    /// every step.
    fn compress_like_c(tree: &mut Subtree, count: u32, language: &Language) {
        let mut stack = Vec::new();
        let symbol = tree.heap().unwrap().symbol;
        let mut current = std::mem::take(tree);
        for _ in 0..count {
            let rotatable = |tree: &Subtree| {
                matches!(tree, Subtree::Heap(data)
                    if data.strong_count() == 1 && data.children().len() >= 2 && data.symbol == symbol)
            };
            if !(rotatable(&current)
                && rotatable(&ts_subtree_children(&current)[0])
                && rotatable(&ts_subtree_children(&ts_subtree_children(&current)[0])[0]))
            {
                break;
            }
            let mut child = std::mem::take(&mut unique_heap_mut(&mut current).1[0]);
            let mut grandchild = std::mem::take(&mut unique_heap_mut(&mut child).1[0]);
            let last = std::mem::take(unique_heap_mut(&mut grandchild).1.last_mut().unwrap());
            unique_heap_mut(&mut child).1[0] = last;
            *unique_heap_mut(&mut grandchild).1.last_mut().unwrap() = child;
            stack.push(current);
            current = grandchild;
        }
        while let Some(mut parent) = stack.pop() {
            unique_heap_mut(&mut parent).1[0] = current;
            let child = &mut unique_heap_mut(&mut parent).1[0];
            let grandchild = unique_heap_mut(child).1.last_mut().unwrap();
            let (data, children) = unique_heap_mut(grandchild);
            summarize_children(data, children, language);
            let (data, children) = unique_heap_mut(child);
            summarize_children(data, children, language);
            let (data, children) = unique_heap_mut(&mut parent);
            summarize_children(data, children, language);
            current = parent;
        }
        *tree = current;
    }

    /// A left-deep repetition with leaves of varied sizes, rows and column
    /// dependence, from a seed.
    fn repetition(seed: u64, length: usize, language: &Language) -> Subtree {
        let mut state = seed;
        let mut next = move || {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (state >> 33) as u32
        };
        let leaf = |next: &mut dyn FnMut() -> u32| {
            // Rows are rare, so that sizes on one line, where a summary's
            // column dependence turns on its previous size, are common.
            let rows = u32::from(next().is_multiple_of(12));
            let bytes = 1 + next() % 300;
            SubtreeHeapData {
                symbol: 1,
                padding: Length {
                    bytes: next() % 4,
                    extent: Point {
                        row: u32::from(next().is_multiple_of(12)),
                        column: next() % 4,
                    },
                },
                size: Length {
                    bytes,
                    extent: Point {
                        row: rows,
                        column: bytes,
                    },
                },
                flags: HeapFlags::default()
                    .with_depends_on_column(next().is_multiple_of(3))
                    .with_fragile_left(next().is_multiple_of(7))
                    .with_fragile_right(next().is_multiple_of(7)),
                payload: SubtreePayload::Leaf,
                ..SubtreeHeapData::default()
            }
            .with_children(Children::new())
        };
        let first = leaf(&mut next);
        let second = leaf(&mut next);
        let mut tree = ts_subtree_new_node(2, vec![first, second], 0, language);
        for _ in 1..length {
            let mut children = vec![tree, leaf(&mut next)];
            if next() % 4 == 0 {
                children.push(leaf(&mut next));
            }
            tree = ts_subtree_new_node(2, children, 0, language);
        }
        tree
    }

    #[test]
    fn compression_matches_summarizing_every_node_as_c_does() {
        let language = language();
        for seed in 0..200 {
            for length in [3, 4, 5, 7, 9, 16, 33] {
                let mut fast = repetition(seed, length, &language);
                let mut reference = repetition(seed, length, &language);
                assert_eq!(format!("{fast:?}"), format!("{reference:?}"));
                let delta = ts_subtree_repeat_depth(&ts_subtree_children(&fast)[0])
                    - ts_subtree_repeat_depth(ts_subtree_children(&fast).last().unwrap());
                let mut i = delta / 2;
                let mut stack = Vec::new();
                while i > 0 {
                    ts_subtree_compress(&mut fast, i, &language, &mut stack);
                    compress_like_c(&mut reference, i, &language);
                    i /= 2;
                }
                assert_eq!(
                    format!("{fast:?}"),
                    format!("{reference:?}"),
                    "seed {seed}, length {length}"
                );
            }
        }
    }

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
    fn external_leaf_repeat_depth_preserves_c_union_overlap() {
        let language = language();
        let mut pool = ts_subtree_pool_new(0);
        let mut bytes = [0xff; 25];
        bytes[16..18].copy_from_slice(&0x1234_u16.to_ne_bytes());
        for length in [0, 16, 17, 18, 24, 25] {
            let tree = ts_subtree_new_leaf_with(
                &mut pool,
                1,
                length_zero(),
                length_zero(),
                0,
                1,
                true,
                false,
                false,
                &language,
                |data| {
                    data.payload =
                        SubtreePayload::External(ts_external_scanner_state_init(&bytes[..length]))
                },
            );
            let expected = match length {
                17 => u16::from_ne_bytes([bytes[16], 0]),
                18 | 24 => 0x1234,
                _ => 0,
            };
            assert_eq!(ts_subtree_child_count(&tree), 0);
            assert_eq!(ts_subtree_repeat_depth(&tree), u32::from(expected));
            assert_eq!(
                ts_subtree_repeat_depth(&ts_subtree_clone(&tree)),
                u32::from(expected)
            );
            // Other branch accessors do check child_count in C.
            assert_eq!(ts_subtree_visible_descendant_count(&tree), 0);
            assert_eq!(ts_subtree_dynamic_precedence(&tree), 0);
            assert_eq!(ts_subtree_production_id(&tree), 0);
        }
    }

    #[test]
    fn repetition_summary_uses_external_leaf_repeat_depth() {
        let language = language();
        let mut pool = ts_subtree_pool_new(0);
        for depth in [0_u16, 2, u16::MAX] {
            let mut bytes = [0; 18];
            bytes[16..18].copy_from_slice(&depth.to_ne_bytes());
            let token = ts_subtree_new_leaf_with(
                &mut pool,
                1,
                length_zero(),
                length_zero(),
                0,
                1,
                true,
                false,
                false,
                &language,
                |data| {
                    data.payload = SubtreePayload::External(ts_external_scanner_state_init(&bytes))
                },
            );
            // Symbol 2 is invisible and unnamed, so its left-recursive parent
            // inherits the larger depth of its first and last children.
            let left = ts_subtree_new_node(2, vec![token.clone()], 0, &language);
            let mut parent = ts_subtree_new_node(2, vec![left, token], 0, &language);
            assert_eq!(
                ts_subtree_repeat_depth(&parent),
                u32::from(depth.wrapping_add(1))
            );
            ts_subtree_summarize_children(&mut parent, &language);
            assert_eq!(
                ts_subtree_repeat_depth(&parent),
                u32::from(depth.wrapping_add(1))
            );
        }
    }

    #[test]
    fn heap_leaf_initialization_precedes_sharing_for_fresh_and_pooled_headers() {
        let language = language();
        let mut pool = ts_subtree_pool_new(32);
        let inline = ts_subtree_new_leaf_with(
            &mut pool,
            1,
            length_zero(),
            length_zero(),
            0,
            7,
            false,
            false,
            false,
            &language,
            |_| panic!("inline leaves do not construct a heap header"),
        );
        assert!(matches!(inline, Subtree::Inline(_)));

        for length in [0, 24, 25, 1024] {
            let bytes: Vec<_> = (0..length).map(|i| i as u8).collect();
            let mut initialized = false;
            let tree = ts_subtree_new_leaf_with(
                &mut pool,
                1,
                length_zero(),
                length_zero(),
                0,
                7,
                true,
                true,
                false,
                &language,
                |header| {
                    assert!(header.has_external_tokens());
                    header.payload =
                        SubtreePayload::External(ts_external_scanner_state_init(&bytes));
                    header.set_has_external_scanner_state_change(true);
                    initialized = true;
                },
            );
            assert!(initialized);
            assert!(pool.free_trees.is_empty());
            let Subtree::Heap(header) = &tree else {
                panic!("external tokens are heap leaves")
            };
            assert_eq!(header.strong_count(), 1);
            assert!(header.children().is_empty());
            assert!(header.has_external_scanner_state_change() && header.depends_on_column());
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
        let inline = Subtree::inline(InlineLeaf {
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
        data.set_extra(true);
        data.set_has_external_tokens(true);
        data.set_has_external_scanner_state_change(true);
        data.set_depends_on_column(true);
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
                        flags: HeapFlags::default()
                            .with_visible(metadata.visible)
                            .with_named(metadata.named)
                            .with_fragile_left(fragile)
                            .with_fragile_right(fragile),
                        payload: SubtreePayload::Branch(BranchData {
                            production_id,
                            ..BranchData::default()
                        }),
                        ..SubtreeHeapData::default()
                    };
                    summarize_children(&mut expected, &children[..count], &language);
                    let fresh = ts_subtree_new_node(
                        symbol,
                        children[..count].to_vec(),
                        production_id as u32,
                        &language,
                    );
                    // Debug compares every header/payload field and the child
                    // contents, not the Arc allocation's address.
                    assert_eq!(
                        format!(
                            "{:?} {:?}",
                            fresh.heap().unwrap(),
                            ts_subtree_children(&fresh)
                        ),
                        format!("{expected:?} {:?}", &children[..count]),
                        "count={count}, symbol={symbol}, production_id={production_id}",
                    );
                }
            }
        }
    }

    #[test]
    fn unary_summary_matches_general_for_all_child_representations() {
        let mut children = vec![Subtree::Null];
        for flags in [
            0,
            VISIBLE,
            NAMED,
            VISIBLE | NAMED,
            EXTRA | VISIBLE,
            MISSING | VISIBLE,
        ] {
            children.push(Subtree::inline(InlineLeaf {
                parse_state: 23,
                symbol: 1,
                flags,
                padding_bytes: 11,
                padding_columns: 3,
                size_bytes: 7,
                padding_rows_and_lookahead: 0xe2,
            }));
        }
        for symbol in [0, 1, 2, BUILTIN_SYM_ERROR, BUILTIN_SYM_ERROR_REPEAT] {
            for flags in 0..32 {
                for has_children in [false, true] {
                    children.push(
                        SubtreeHeapData {
                            symbol,
                            parse_state: 27,
                            flags: HeapFlags::default()
                                .with_visible(flags & 1 != 0)
                                .with_named(flags & 2 != 0)
                                .with_extra(flags & 4 != 0)
                                .with_is_missing(flags & 8 != 0)
                                .with_fragile_left(flags & 16 != 0)
                                .with_fragile_right(flags & 16 == 0)
                                .with_depends_on_column(true)
                                .with_has_external_tokens(true)
                                .with_has_external_scanner_state_change(true),
                            padding: Length {
                                bytes: u32::MAX - 3,
                                extent: Point { row: 2, column: 4 },
                            },
                            size: Length {
                                bytes: 17,
                                extent: Point { row: 1, column: 3 },
                            },
                            lookahead_bytes: u32::MAX - 7,
                            error_cost: 97,
                            payload: SubtreePayload::Branch(BranchData {
                                visible_child_count: 3,
                                named_child_count: 2,
                                visible_descendant_count: u32::MAX,
                                dynamic_precedence: -11,
                                repeat_depth: 9,
                                production_id: 1,
                                first_leaf: FirstLeaf {
                                    symbol: 2,
                                    parse_state: 13,
                                },
                            }),
                        }
                        .with_children(if has_children {
                            vec![Subtree::inline(InlineLeaf::default())]
                        } else {
                            Vec::new()
                        }),
                    );
                }
            }
        }
        for child in children {
            for old_rows in [0, 2] {
                for parent_symbol in [3, BUILTIN_SYM_ERROR, BUILTIN_SYM_ERROR_REPEAT] {
                    for production_id in [0, 1] {
                        let mut optimized = ChildSummary {
                            symbol: parent_symbol,
                            parse_state: 31,
                            padding: length_zero(),
                            size: Length {
                                bytes: 53,
                                extent: Point {
                                    row: old_rows,
                                    column: 7,
                                },
                            },
                            branch: BranchData {
                                production_id,
                                repeat_depth: 5,
                                ..BranchData::default()
                            },
                            lookahead_bytes: 0,
                            error_cost: 0,
                            has_external_tokens: false,
                            has_external_scanner_state_change: false,
                            depends_on_column: false,
                            fragile_left: true,
                            fragile_right: false,
                            visible: false,
                            named: false,
                        };
                        let mut general = optimized.clone();
                        // Repeat after the first pass has changed rows/flags.
                        for _ in 0..2 {
                            summarize_child_slice(
                                &mut optimized,
                                std::slice::from_ref(&child),
                                &language(),
                            );
                            summarize_child_slice_general(
                                &mut general,
                                std::slice::from_ref(&child),
                                &language(),
                            );
                            assert_eq!(format!("{optimized:?}"), format!("{general:?}"));
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn summaries_combine_missing_errors_hidden_branches_and_external_extras() {
        let missing = Subtree::inline(InlineLeaf {
            symbol: 1,
            parse_state: 7,
            flags: VISIBLE | NAMED | MISSING,
            padding_bytes: 2,
            padding_columns: 2,
            size_bytes: 3,
            padding_rows_and_lookahead: 7 << 4,
        });
        let external = SubtreeHeapData {
            symbol: 2,
            flags: HeapFlags::default()
                .with_visible(true)
                .with_extra(true)
                .with_has_external_tokens(true)
                .with_has_external_scanner_state_change(true)
                .with_depends_on_column(true),
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
            payload: SubtreePayload::External(ExternalScannerState::default()),
            ..SubtreeHeapData::default()
        }
        .with_children(Children::new());
        let hidden = SubtreeHeapData {
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
            payload: SubtreePayload::Branch(BranchData {
                visible_child_count: 2,
                named_child_count: 1,
                visible_descendant_count: 3,
                dynamic_precedence: -4,
                ..BranchData::default()
            }),
            ..SubtreeHeapData::default()
        }
        .with_children(vec![missing.clone()]);
        let error = SubtreeHeapData {
            symbol: BUILTIN_SYM_ERROR,
            flags: HeapFlags::default().with_visible(true).with_named(true),
            size: Length {
                bytes: 1,
                extent: Point { row: 0, column: 1 },
            },
            error_cost: 11,
            payload: SubtreePayload::Error(0),
            ..SubtreeHeapData::default()
        }
        .with_children(Children::new());
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
            data.has_external_tokens()
                && data.has_external_scanner_state_change()
                && data.depends_on_column()
        );
        assert!(data.fragile_left() && data.fragile_right());
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
        let child = SubtreeHeapData {
            symbol: 2,
            parse_state: 23,
            flags: HeapFlags::default().with_depends_on_column(true),
            payload: SubtreePayload::Branch(BranchData {
                dynamic_precedence: 99,
                ..BranchData::default()
            }),
            ..SubtreeHeapData::default()
        }
        .with_children(Children::new());
        let mut data = SubtreeHeapData {
            symbol: 3,
            size: Length {
                bytes: 3,
                extent: Point { row: 3, column: 0 },
            },
            parse_state: 9,
            flags: HeapFlags::default()
                .with_fragile_left(true)
                .with_has_external_tokens(true)
                .with_has_external_scanner_state_change(true),
            payload: SubtreePayload::Branch(BranchData::default()),
            ..SubtreeHeapData::default()
        };
        let children = [child];
        summarize_children(&mut data, &children, &language());
        assert!(!data.depends_on_column());
        assert!(!data.has_external_tokens() && !data.has_external_scanner_state_change());
        assert_eq!(data.size, length_zero());
        assert!(data.fragile_left());
        assert_eq!(data.parse_state, 9);
        summarize_children(&mut data, &children, &language());
        assert!(data.depends_on_column());
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
            Subtree::inline(InlineLeaf {
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

#[cfg(test)]
mod children_tests {
    use super::*;

    fn leaf(symbol: u8) -> Subtree {
        Subtree::inline(InlineLeaf {
            symbol,
            ..InlineLeaf::default()
        })
    }

    fn symbols(children: &[Subtree]) -> Vec<Symbol> {
        children.iter().map(ts_subtree_symbol).collect()
    }

    #[test]
    fn up_to_three_children_are_kept_inline() {
        for count in 0..=5u8 {
            let children = Children::from((1..=count).map(leaf).collect::<Vec<_>>());
            assert_eq!(
                matches!(children, Children::Inline { .. }),
                usize::from(count) <= INLINE_CHILD_CAPACITY
            );
            assert_eq!(
                symbols(&children),
                (1..=Symbol::from(count)).collect::<Vec<_>>()
            );
            let borrowed = Children::from(&*children);
            assert_eq!(symbols(&borrowed), symbols(&children));
        }
    }

    #[test]
    fn pushing_past_the_inline_capacity_keeps_order() {
        let mut children = Children::with_capacity(2);
        for symbol in 1..=5 {
            children.push(leaf(symbol));
        }
        assert!(matches!(children, Children::Heap(_)));
        assert_eq!(symbols(&children), [1, 2, 3, 4, 5]);
        children.as_mut_slice().reverse();
        assert_eq!(symbols(&children.clone()), [5, 4, 3, 2, 1]);
        assert_eq!(symbols(&children.into_vec()), [5, 4, 3, 2, 1]);
    }

    #[test]
    fn moving_children_out_leaves_the_list_empty() {
        for count in [2u8, 4] {
            let mut children = Children::from((1..=count).map(leaf).collect::<Vec<_>>());
            let mut out = vec![leaf(9)];
            children.append_to(&mut out);
            assert!(children.is_empty());
            assert_eq!(
                symbols(&out),
                [9].into_iter()
                    .chain(1..=Symbol::from(count))
                    .collect::<Vec<_>>()
            );
            let owned = Children::from((1..=count).map(leaf).collect::<Vec<_>>());
            assert_eq!(
                owned
                    .into_iter()
                    .map(|tree| ts_subtree_symbol(&tree))
                    .collect::<Vec<_>>(),
                (1..=Symbol::from(count)).collect::<Vec<_>>()
            );
        }
    }

    #[test]
    fn dropping_a_deep_tree_of_inline_children_does_not_recurse() {
        let mut tree = leaf(1);
        for _ in 0..200_000 {
            tree = SubtreeHeapData {
                payload: SubtreePayload::Branch(BranchData::default()),
                ..SubtreeHeapData::default()
            }
            .with_children(vec![tree, leaf(2)]);
        }
        let shared = tree.clone();
        drop(tree);
        // The second owner frees the whole chain.
        drop(shared);
    }
}
