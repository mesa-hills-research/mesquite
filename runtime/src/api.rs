//! Binding-compatible, safe public API. Runtime algorithms live in sibling modules.
use crate::{Language, LanguageMetadata, Node, Parser, Tree, TreeCursor};
use std::{
    fmt,
    hash::{Hash, Hasher},
    num::NonZeroU16,
    ops::{Deref, Range as ByteRange},
    str,
};
// Literal-only panic shim: the standard formatted todo macro is not const-safe.
// Typed arms preserve the official opaque iterator signatures while leaving each
// translator's function body exactly one unit-tagged stub invocation.
macro_rules! todo {
    ("api: Tree::changed_ranges") => {
        pending::<ChangedRanges>("api: Tree::changed_ranges")
    };
    ("api: Node::children") => {
        pending::<Children<'_, '_>>("api: Node::children")
    };
    ("api: Node::named_children") => {
        pending::<Children<'_, '_>>("api: Node::named_children")
    };
    ("api: Node::children_by_field_name") => {
        pending::<FieldChildren<'_, '_>>("api: Node::children_by_field_name")
    };
    ("api: Node::children_by_field_id") => {
        pending::<FieldChildren<'_, '_>>("api: Node::children_by_field_id")
    };
    ($message:literal) => {
        panic!(concat!("not yet implemented: ", $message))
    };
}
fn pending<T>(message: &str) -> T {
    panic!("not yet implemented: {message}")
}

pub const LANGUAGE_VERSION: usize = 15;
pub const MIN_COMPATIBLE_LANGUAGE_VERSION: usize = 13;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point {
    pub row: usize,
    pub column: usize,
}
impl Point {
    pub const fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.row, self.column)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Range {
    pub start_byte: usize,
    pub end_byte: usize,
    pub start_point: Point,
    pub end_point: Point,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InputEdit {
    pub start_byte: usize,
    pub old_end_byte: usize,
    pub new_end_byte: usize,
    pub start_position: Point,
    pub old_end_position: Point,
    pub new_end_position: Point,
}
#[derive(Debug, PartialEq, Eq)]
pub struct LanguageError {
    pub(crate) version: usize,
}
#[derive(Debug, PartialEq, Eq)]
pub struct IncludedRangesError(pub usize);
#[derive(Debug, PartialEq, Eq)]
pub enum LogType {
    Parse,
    Lex,
}
#[derive(Debug)]
pub struct LanguageRef<'a>(pub(crate) &'a Language);
impl Deref for LanguageRef<'_> {
    type Target = Language;
    fn deref(&self) -> &Language {
        self.0
    }
}
pub type ParseProgressCallback<'a> = &'a mut dyn FnMut(&ParseState) -> bool;
#[derive(Default)]
pub struct ParseOptions<'a> {
    pub progress_callback: Option<ParseProgressCallback<'a>>,
}
#[derive(Debug, Default)]
pub struct ParseState {
    pub(crate) current_byte_offset: u32,
    pub(crate) has_error: bool,
}
type FieldId = NonZeroU16;

/// Cursor-backed iteration, retaining the binding's exclusive cursor borrow.
pub struct Children<'cursor, 'tree> {
    pub(crate) cursor: &'cursor mut TreeCursor<'tree>,
    pub(crate) remaining: usize,
    pub(crate) named: bool,
}
pub struct FieldChildren<'cursor, 'tree> {
    pub(crate) cursor: &'cursor mut TreeCursor<'tree>,
    pub(crate) field_id: FieldId,
    pub(crate) done: bool,
}
pub type ChangedRanges = std::vec::IntoIter<Range>;

impl Language {
    pub fn name(&self) -> Option<&'static str> {
        todo!("api: Language::name")
    }
    pub fn version(&self) -> usize {
        todo!("api: Language::version")
    }
    pub fn abi_version(&self) -> usize {
        todo!("api: Language::abi_version")
    }
    pub fn metadata(&self) -> Option<LanguageMetadata> {
        todo!("api: Language::metadata")
    }
    pub fn node_kind_count(&self) -> usize {
        todo!("api: Language::node_kind_count")
    }
    pub fn parse_state_count(&self) -> usize {
        todo!("api: Language::parse_state_count")
    }
    pub fn supertypes(&self) -> &[u16] {
        todo!("api: Language::supertypes")
    }
    pub fn subtypes_for_supertype(&self, supertype: u16) -> &[u16] {
        todo!("api: Language::subtypes_for_supertype")
    }
    pub fn node_kind_for_id(&self, id: u16) -> Option<&'static str> {
        todo!("api: Language::node_kind_for_id")
    }
    pub fn id_for_node_kind(&self, kind: &str, named: bool) -> u16 {
        todo!("api: Language::id_for_node_kind")
    }
    pub fn node_kind_is_named(&self, id: u16) -> bool {
        todo!("api: Language::node_kind_is_named")
    }
    pub fn node_kind_is_visible(&self, id: u16) -> bool {
        todo!("api: Language::node_kind_is_visible")
    }
    pub fn node_kind_is_supertype(&self, id: u16) -> bool {
        todo!("api: Language::node_kind_is_supertype")
    }
    pub fn field_count(&self) -> usize {
        todo!("api: Language::field_count")
    }
    pub fn field_name_for_id(&self, field_id: u16) -> Option<&'static str> {
        todo!("api: Language::field_name_for_id")
    }
    pub fn field_id_for_name(&self, field_name: impl AsRef<[u8]>) -> Option<FieldId> {
        todo!("api: Language::field_id_for_name")
    }
    pub fn next_state(&self, state: u16, id: u16) -> u16 {
        todo!("api: Language::next_state")
    }
}

impl Parser {
    pub fn new() -> Self {
        todo!("api: Parser::new")
    }
    pub fn set_language(&mut self, language: &Language) -> Result<(), LanguageError> {
        todo!("api: Parser::set_language")
    }
    pub fn language(&self) -> Option<LanguageRef<'_>> {
        todo!("api: Parser::language")
    }
    pub fn parse(&mut self, text: impl AsRef<[u8]>, old_tree: Option<&Tree>) -> Option<Tree> {
        todo!("api: Parser::parse")
    }
    pub fn parse_with<T: AsRef<[u8]>, F: FnMut(usize, Point) -> T>(
        &mut self,
        callback: &mut F,
        old_tree: Option<&Tree>,
    ) -> Option<Tree> {
        todo!("api: Parser::parse_with")
    }
    pub fn parse_with_options<T: AsRef<[u8]>, F: FnMut(usize, Point) -> T>(
        &mut self,
        callback: &mut F,
        old_tree: Option<&Tree>,
        options: Option<ParseOptions<'_>>,
    ) -> Option<Tree> {
        todo!("api: Parser::parse_with_options")
    }
    pub fn reset(&mut self) {
        todo!("api: Parser::reset")
    }
    pub fn timeout_micros(&self) -> u64 {
        todo!("api: Parser::timeout_micros")
    }
    pub fn set_timeout_micros(&mut self, timeout_micros: u64) {
        todo!("api: Parser::set_timeout_micros")
    }
    pub fn set_included_ranges(&mut self, ranges: &[Range]) -> Result<(), IncludedRangesError> {
        todo!("api: Parser::set_included_ranges")
    }
    pub fn included_ranges(&self) -> Vec<Range> {
        todo!("api: Parser::included_ranges")
    }
}

impl Tree {
    pub fn root_node(&self) -> Node<'_> {
        todo!("api: Tree::root_node")
    }
    pub fn root_node_with_offset(&self, offset_bytes: usize, offset_extent: Point) -> Node<'_> {
        todo!("api: Tree::root_node_with_offset")
    }
    pub fn language(&self) -> LanguageRef<'_> {
        todo!("api: Tree::language")
    }
    pub fn edit(&mut self, edit: &InputEdit) {
        todo!("api: Tree::edit")
    }
    pub fn walk(&self) -> TreeCursor<'_> {
        todo!("api: Tree::walk")
    }
    pub fn changed_ranges(&self, other: &Self) -> impl ExactSizeIterator<Item = Range> {
        todo!("api: Tree::changed_ranges")
    }
    pub fn included_ranges(&self) -> Vec<Range> {
        todo!("api: Tree::included_ranges")
    }
}

impl<'tree> Node<'tree> {
    pub fn id(&self) -> usize {
        todo!("api: Node::id")
    }
    pub fn kind_id(&self) -> u16 {
        todo!("api: Node::kind_id")
    }
    pub fn grammar_id(&self) -> u16 {
        todo!("api: Node::grammar_id")
    }
    pub fn kind(&self) -> &'static str {
        todo!("api: Node::kind")
    }
    pub fn grammar_name(&self) -> &'static str {
        todo!("api: Node::grammar_name")
    }
    pub fn language(&self) -> LanguageRef<'_> {
        todo!("api: Node::language")
    }
    pub fn is_named(&self) -> bool {
        todo!("api: Node::is_named")
    }
    pub fn is_extra(&self) -> bool {
        todo!("api: Node::is_extra")
    }
    pub fn has_changes(&self) -> bool {
        todo!("api: Node::has_changes")
    }
    pub fn has_error(&self) -> bool {
        todo!("api: Node::has_error")
    }
    pub fn is_error(&self) -> bool {
        todo!("api: Node::is_error")
    }
    pub fn parse_state(&self) -> u16 {
        todo!("api: Node::parse_state")
    }
    pub fn next_parse_state(&self) -> u16 {
        todo!("api: Node::next_parse_state")
    }
    pub fn is_missing(&self) -> bool {
        todo!("api: Node::is_missing")
    }
    pub fn start_byte(&self) -> usize {
        todo!("api: Node::start_byte")
    }
    pub fn end_byte(&self) -> usize {
        todo!("api: Node::end_byte")
    }
    pub fn byte_range(&self) -> core::ops::Range<usize> {
        todo!("api: Node::byte_range")
    }
    pub fn range(&self) -> Range {
        todo!("api: Node::range")
    }
    pub fn start_position(&self) -> Point {
        todo!("api: Node::start_position")
    }
    pub fn end_position(&self) -> Point {
        todo!("api: Node::end_position")
    }
    pub fn child(&self, i: usize) -> Option<Self> {
        todo!("api: Node::child")
    }
    pub fn child_count(&self) -> usize {
        todo!("api: Node::child_count")
    }
    pub fn named_child(&self, i: usize) -> Option<Self> {
        todo!("api: Node::named_child")
    }
    pub fn named_child_count(&self) -> usize {
        todo!("api: Node::named_child_count")
    }
    pub fn child_by_field_name(&self, field_name: impl AsRef<[u8]>) -> Option<Self> {
        todo!("api: Node::child_by_field_name")
    }
    pub fn child_by_field_id(&self, field_id: u16) -> Option<Self> {
        todo!("api: Node::child_by_field_id")
    }
    pub fn field_name_for_child(&self, child_index: u32) -> Option<&'static str> {
        todo!("api: Node::field_name_for_child")
    }
    pub fn field_name_for_named_child(&self, named_child_index: u32) -> Option<&'static str> {
        todo!("api: Node::field_name_for_named_child")
    }
    pub fn children<'cursor>(
        &self,
        cursor: &'cursor mut TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<Item = Node<'tree>> + 'cursor {
        todo!("api: Node::children")
    }
    pub fn named_children<'cursor>(
        &self,
        cursor: &'cursor mut TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<Item = Node<'tree>> + 'cursor {
        todo!("api: Node::named_children")
    }
    pub fn children_by_field_name<'cursor>(
        &self,
        field_name: &str,
        cursor: &'cursor mut TreeCursor<'tree>,
    ) -> impl Iterator<Item = Node<'tree>> + 'cursor {
        todo!("api: Node::children_by_field_name")
    }
    pub fn children_by_field_id<'cursor>(
        &self,
        field_id: FieldId,
        cursor: &'cursor mut TreeCursor<'tree>,
    ) -> impl Iterator<Item = Node<'tree>> + 'cursor {
        todo!("api: Node::children_by_field_id")
    }
    pub fn parent(&self) -> Option<Self> {
        todo!("api: Node::parent")
    }
    pub fn child_with_descendant(&self, descendant: Self) -> Option<Self> {
        todo!("api: Node::child_with_descendant")
    }
    pub fn next_sibling(&self) -> Option<Self> {
        todo!("api: Node::next_sibling")
    }
    pub fn prev_sibling(&self) -> Option<Self> {
        todo!("api: Node::prev_sibling")
    }
    pub fn next_named_sibling(&self) -> Option<Self> {
        todo!("api: Node::next_named_sibling")
    }
    pub fn prev_named_sibling(&self) -> Option<Self> {
        todo!("api: Node::prev_named_sibling")
    }
    pub fn first_child_for_byte(&self, byte: usize) -> Option<Self> {
        todo!("api: Node::first_child_for_byte")
    }
    pub fn first_named_child_for_byte(&self, byte: usize) -> Option<Self> {
        todo!("api: Node::first_named_child_for_byte")
    }
    pub fn descendant_count(&self) -> usize {
        todo!("api: Node::descendant_count")
    }
    pub fn descendant_for_byte_range(&self, start: usize, end: usize) -> Option<Self> {
        todo!("api: Node::descendant_for_byte_range")
    }
    pub fn named_descendant_for_byte_range(&self, start: usize, end: usize) -> Option<Self> {
        todo!("api: Node::named_descendant_for_byte_range")
    }
    pub fn descendant_for_point_range(&self, start: Point, end: Point) -> Option<Self> {
        todo!("api: Node::descendant_for_point_range")
    }
    pub fn named_descendant_for_point_range(&self, start: Point, end: Point) -> Option<Self> {
        todo!("api: Node::named_descendant_for_point_range")
    }
    pub fn to_sexp(&self) -> String {
        todo!("api: Node::to_sexp")
    }
    pub fn utf8_text<'a>(&self, source: &'a [u8]) -> Result<&'a str, str::Utf8Error> {
        todo!("api: Node::utf8_text")
    }
    pub fn utf16_text<'a>(&self, source: &'a [u16]) -> &'a [u16] {
        todo!("api: Node::utf16_text")
    }
    pub fn walk(&self) -> TreeCursor<'tree> {
        todo!("api: Node::walk")
    }
    pub fn edit(&mut self, edit: &InputEdit) {
        todo!("api: Node::edit")
    }
}

impl<'cursor> TreeCursor<'cursor> {
    pub fn node(&self) -> Node<'cursor> {
        todo!("api: TreeCursor::node")
    }
    pub fn field_id(&self) -> Option<FieldId> {
        todo!("api: TreeCursor::field_id")
    }
    pub fn field_name(&self) -> Option<&'static str> {
        todo!("api: TreeCursor::field_name")
    }
    pub fn depth(&self) -> u32 {
        todo!("api: TreeCursor::depth")
    }
    pub fn descendant_index(&self) -> usize {
        todo!("api: TreeCursor::descendant_index")
    }
    pub fn goto_first_child(&mut self) -> bool {
        todo!("api: TreeCursor::goto_first_child")
    }
    pub fn goto_last_child(&mut self) -> bool {
        todo!("api: TreeCursor::goto_last_child")
    }
    pub fn goto_parent(&mut self) -> bool {
        todo!("api: TreeCursor::goto_parent")
    }
    pub fn goto_next_sibling(&mut self) -> bool {
        todo!("api: TreeCursor::goto_next_sibling")
    }
    pub fn goto_descendant(&mut self, descendant_index: usize) {
        todo!("api: TreeCursor::goto_descendant")
    }
    pub fn goto_previous_sibling(&mut self) -> bool {
        todo!("api: TreeCursor::goto_previous_sibling")
    }
    pub fn goto_first_child_for_byte(&mut self, index: usize) -> Option<usize> {
        todo!("api: TreeCursor::goto_first_child_for_byte")
    }
    pub fn goto_first_child_for_point(&mut self, point: Point) -> Option<usize> {
        todo!("api: TreeCursor::goto_first_child_for_point")
    }
    pub fn reset(&mut self, node: Node<'cursor>) {
        todo!("api: TreeCursor::reset")
    }
    pub fn reset_to(&mut self, cursor: &Self) {
        todo!("api: TreeCursor::reset_to")
    }
}

impl ParseState {
    pub const fn current_byte_offset(&self) -> usize {
        todo!("api: ParseState::current_byte_offset")
    }
    pub const fn has_error(&self) -> bool {
        todo!("api: ParseState::has_error")
    }
}

impl<'a> ParseOptions<'a> {
    pub fn new() -> Self {
        todo!("api: ParseOptions::new")
    }
    pub fn progress_callback<F: FnMut(&ParseState) -> bool>(self, callback: &'a mut F) -> Self {
        todo!("api: ParseOptions::progress_callback")
    }
}

impl Default for Parser {
    fn default() -> Self {
        todo!("api: Parser::default")
    }
}

impl Clone for Tree {
    fn clone(&self) -> Self {
        todo!("api: Tree::clone")
    }
}

impl Clone for TreeCursor<'_> {
    fn clone(&self) -> Self {
        todo!("api: TreeCursor::clone")
    }
}

impl fmt::Debug for Node<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("api: Node::fmt")
    }
}

impl PartialEq for Node<'_> {
    fn eq(&self, other: &Self) -> bool {
        todo!("api: Node::eq")
    }
}

impl Hash for Node<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        todo!("api: Node::hash")
    }
}

impl fmt::Display for Node<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("api: Node::fmt_display")
    }
}

impl fmt::Display for LanguageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("api: LanguageError::fmt")
    }
}

impl fmt::Display for IncludedRangesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("api: IncludedRangesError::fmt")
    }
}

impl Eq for Node<'_> {}
impl std::error::Error for LanguageError {}
impl std::error::Error for IncludedRangesError {}

impl<'tree> Iterator for Children<'_, 'tree> {
    type Item = Node<'tree>;
    fn next(&mut self) -> Option<Self::Item> {
        todo!("api: Children::next")
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        todo!("api: Children::size_hint")
    }
}

impl<'tree> Iterator for FieldChildren<'_, 'tree> {
    type Item = Node<'tree>;
    fn next(&mut self) -> Option<Self::Item> {
        todo!("api: FieldChildren::next")
    }
}
impl ExactSizeIterator for Children<'_, '_> {}

impl From<Point> for crate::point::Point {
    fn from(value: Point) -> Self {
        Self {
            row: value.row as u32,
            column: value.column as u32,
        }
    }
}
impl From<crate::point::Point> for Point {
    fn from(value: crate::point::Point) -> Self {
        Self {
            row: value.row as usize,
            column: value.column as usize,
        }
    }
}
impl From<Range> for crate::types::Range {
    fn from(value: Range) -> Self {
        Self {
            start_point: value.start_point.into(),
            end_point: value.end_point.into(),
            start_byte: value.start_byte as u32,
            end_byte: value.end_byte as u32,
        }
    }
}
impl From<crate::types::Range> for Range {
    fn from(value: crate::types::Range) -> Self {
        Self {
            start_point: value.start_point.into(),
            end_point: value.end_point.into(),
            start_byte: value.start_byte as usize,
            end_byte: value.end_byte as usize,
        }
    }
}
impl From<&InputEdit> for crate::types::InputEdit {
    fn from(value: &InputEdit) -> Self {
        Self {
            start_byte: value.start_byte as u32,
            old_end_byte: value.old_end_byte as u32,
            new_end_byte: value.new_end_byte as u32,
            start_point: value.start_position.into(),
            old_end_point: value.old_end_position.into(),
            new_end_point: value.new_end_position.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn send<T: Send>() {}
    fn send_sync_clone<T: Send + Sync + Clone>() {}
    #[test]
    fn public_thread_safety() {
        send::<Parser>();
        send_sync_clone::<Tree>();
        send_sync_clone::<Language>();
        send_sync_clone::<Node<'static>>();
    }
    #[test]
    fn point_conversion_matches_binding() {
        let p = Point::new(2, 17);
        assert_eq!(Point::from(crate::point::Point::from(p)), p);
        assert_eq!(p.to_string(), "(2, 17)");
    }
}
