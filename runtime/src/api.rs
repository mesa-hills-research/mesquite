//! Binding-compatible, safe public API. Runtime algorithms live in sibling modules.
use crate::{
    Language, LanguageMetadata, Node, Parser, Tree, TreeCursor,
    language::*,
    node::*,
    parser::*,
    tree::*,
    tree_cursor::*,
    types::{CallbackInput, SymbolType},
};
use std::{
    fmt,
    hash::{Hash, Hasher},
    num::NonZeroU16,
    ops::Deref,
    str,
};
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
        ts_language_name(self)
    }
    pub fn version(&self) -> usize {
        ts_language_version(self) as usize
    }
    pub fn abi_version(&self) -> usize {
        ts_language_abi_version(self) as usize
    }
    pub fn metadata(&self) -> Option<LanguageMetadata> {
        ts_language_metadata(self)
    }
    pub fn node_kind_count(&self) -> usize {
        ts_language_symbol_count(self) as usize
    }
    pub fn parse_state_count(&self) -> usize {
        ts_language_state_count(self) as usize
    }
    pub fn supertypes(&self) -> &[u16] {
        ts_language_supertypes(self)
    }
    pub fn subtypes_for_supertype(&self, supertype: u16) -> &[u16] {
        ts_language_subtypes(self, supertype)
    }
    pub fn node_kind_for_id(&self, id: u16) -> Option<&'static str> {
        ts_language_symbol_name(self, id)
    }
    pub fn id_for_node_kind(&self, kind: &str, named: bool) -> u16 {
        ts_language_symbol_for_name(self, kind.as_bytes(), named)
    }
    pub fn node_kind_is_named(&self, id: u16) -> bool {
        ts_language_symbol_type(self, id) == SymbolType::Regular
    }
    pub fn node_kind_is_visible(&self, id: u16) -> bool {
        matches!(
            ts_language_symbol_type(self, id),
            SymbolType::Regular | SymbolType::Anonymous
        )
    }
    pub fn node_kind_is_supertype(&self, id: u16) -> bool {
        ts_language_symbol_type(self, id) == SymbolType::Supertype
    }
    pub fn field_count(&self) -> usize {
        ts_language_field_count(self) as usize
    }
    pub fn field_name_for_id(&self, field_id: u16) -> Option<&'static str> {
        ts_language_field_name_for_id(self, field_id)
    }
    pub fn field_id_for_name(&self, field_name: impl AsRef<[u8]>) -> Option<FieldId> {
        FieldId::new(ts_language_field_id_for_name(self, field_name.as_ref()))
    }
    pub fn next_state(&self, state: u16, id: u16) -> u16 {
        ts_language_next_state(self, state, id)
    }
}

impl Parser {
    pub fn new() -> Self {
        ts_parser_new()
    }
    pub fn set_language(&mut self, language: &Language) -> Result<(), LanguageError> {
        let version = language.abi_version();
        if (MIN_COMPATIBLE_LANGUAGE_VERSION..=LANGUAGE_VERSION).contains(&version) {
            ts_parser_set_language(self, Some(language));
            Ok(())
        } else {
            Err(LanguageError { version })
        }
    }
    pub fn language(&self) -> Option<LanguageRef<'_>> {
        ts_parser_language(self).map(LanguageRef)
    }
    pub fn parse(&mut self, text: impl AsRef<[u8]>, old_tree: Option<&Tree>) -> Option<Tree> {
        let bytes = text.as_ref();
        self.parse_with_options(
            &mut |i, _| bytes.get(i..).unwrap_or_default(),
            old_tree,
            None,
        )
    }
    pub fn parse_with<T: AsRef<[u8]>, F: FnMut(usize, Point) -> T>(
        &mut self,
        callback: &mut F,
        old_tree: Option<&Tree>,
    ) -> Option<Tree> {
        self.parse_with_options(callback, old_tree, None)
    }
    pub fn parse_with_options<T: AsRef<[u8]>, F: FnMut(usize, Point) -> T>(
        &mut self,
        callback: &mut F,
        old_tree: Option<&Tree>,
        options: Option<ParseOptions<'_>>,
    ) -> Option<Tree> {
        // Keep the returned chunk alive until the next read. Input and progress
        // callbacks are borrowed only for this parse, including on cancellation.
        let mut input = CallbackInput {
            callback,
            chunk: None,
        };
        ts_parser_parse_with_options(self, old_tree, &mut input, options.unwrap_or_default())
    }
    pub fn reset(&mut self) {
        ts_parser_reset(self);
    }
    pub fn timeout_micros(&self) -> u64 {
        ts_parser_timeout_micros(self)
    }
    pub fn set_timeout_micros(&mut self, timeout_micros: u64) {
        ts_parser_set_timeout_micros(self, timeout_micros);
    }
    pub fn set_included_ranges(&mut self, ranges: &[Range]) -> Result<(), IncludedRangesError> {
        let ts_ranges: Vec<_> = ranges.iter().copied().map(Into::into).collect();
        if ts_parser_set_included_ranges(self, &ts_ranges) {
            Ok(())
        } else {
            // Diagnose using public-width coordinates, just as the binding does
            // after the runtime has validated the converted u32 coordinates.
            let mut prev_end_byte = 0;
            for (i, range) in ranges.iter().enumerate() {
                if range.start_byte < prev_end_byte || range.end_byte < range.start_byte {
                    return Err(IncludedRangesError(i));
                }
                prev_end_byte = range.end_byte;
            }
            Err(IncludedRangesError(0))
        }
    }
    pub fn included_ranges(&self) -> Vec<Range> {
        ts_parser_included_ranges(self)
            .iter()
            .copied()
            .map(Into::into)
            .collect()
    }
}

impl Tree {
    pub fn root_node(&self) -> Node<'_> {
        ts_tree_root_node(self)
    }
    pub fn root_node_with_offset(&self, offset_bytes: usize, offset_extent: Point) -> Node<'_> {
        ts_tree_root_node_with_offset(self, offset_bytes as u32, offset_extent.into())
    }
    pub fn language(&self) -> LanguageRef<'_> {
        LanguageRef(ts_tree_language(self))
    }
    pub fn edit(&mut self, edit: &InputEdit) {
        ts_tree_edit(self, &edit.into());
    }
    pub fn walk(&self) -> TreeCursor<'_> {
        self.root_node().walk()
    }
    pub fn changed_ranges(&self, other: &Self) -> impl ExactSizeIterator<Item = Range> + use<> {
        ts_tree_get_changed_ranges(self, other)
            .into_iter()
            .map(Into::into)
    }
    pub fn included_ranges(&self) -> Vec<Range> {
        ts_tree_included_ranges(self)
            .into_iter()
            .map(Into::into)
            .collect()
    }
}

impl<'tree> Node<'tree> {
    pub fn id(&self) -> usize {
        std::ptr::from_ref(self.subtree) as usize
    }
    #[inline]
    pub fn kind_id(&self) -> u16 {
        ts_node_symbol(*self)
    }
    #[inline]
    pub fn grammar_id(&self) -> u16 {
        ts_node_grammar_symbol(*self)
    }
    #[inline]
    pub fn kind(&self) -> &'static str {
        ts_node_type(*self)
    }
    #[inline]
    pub fn grammar_name(&self) -> &'static str {
        ts_node_grammar_type(*self)
    }
    #[inline]
    pub fn language(&self) -> LanguageRef<'_> {
        LanguageRef(ts_node_language(*self))
    }
    #[inline]
    pub fn is_named(&self) -> bool {
        ts_node_is_named(*self)
    }
    #[inline]
    pub fn is_extra(&self) -> bool {
        ts_node_is_extra(*self)
    }
    #[inline]
    pub fn has_changes(&self) -> bool {
        ts_node_has_changes(*self)
    }
    #[inline]
    pub fn has_error(&self) -> bool {
        ts_node_has_error(*self)
    }
    #[inline]
    pub fn is_error(&self) -> bool {
        ts_node_is_error(*self)
    }
    #[inline]
    pub fn parse_state(&self) -> u16 {
        ts_node_parse_state(*self)
    }
    #[inline]
    pub fn next_parse_state(&self) -> u16 {
        ts_node_next_parse_state(*self)
    }
    #[inline]
    pub fn is_missing(&self) -> bool {
        ts_node_is_missing(*self)
    }
    #[inline]
    pub fn start_byte(&self) -> usize {
        ts_node_start_byte(*self) as usize
    }
    #[inline]
    pub fn end_byte(&self) -> usize {
        ts_node_end_byte(*self) as usize
    }
    #[inline]
    pub fn byte_range(&self) -> core::ops::Range<usize> {
        self.start_byte()..self.end_byte()
    }
    #[inline]
    pub fn range(&self) -> Range {
        Range {
            start_byte: self.start_byte(),
            end_byte: self.end_byte(),
            start_point: self.start_position(),
            end_point: self.end_position(),
        }
    }
    #[inline]
    pub fn start_position(&self) -> Point {
        ts_node_start_point(*self).into()
    }
    #[inline]
    pub fn end_position(&self) -> Point {
        ts_node_end_point(*self).into()
    }
    pub fn child(&self, i: usize) -> Option<Self> {
        ts_node_child(*self, i as u32)
    }
    #[inline]
    pub fn child_count(&self) -> usize {
        ts_node_child_count(*self) as usize
    }
    pub fn named_child(&self, i: usize) -> Option<Self> {
        ts_node_named_child(*self, i as u32)
    }
    #[inline]
    pub fn named_child_count(&self) -> usize {
        ts_node_named_child_count(*self) as usize
    }
    pub fn child_by_field_name(&self, field_name: impl AsRef<[u8]>) -> Option<Self> {
        ts_node_child_by_field_name(*self, field_name.as_ref())
    }
    pub fn child_by_field_id(&self, field_id: u16) -> Option<Self> {
        ts_node_child_by_field_id(*self, field_id)
    }
    pub fn field_name_for_child(&self, child_index: u32) -> Option<&'static str> {
        ts_node_field_name_for_child(*self, child_index)
    }
    pub fn field_name_for_named_child(&self, named_child_index: u32) -> Option<&'static str> {
        ts_node_field_name_for_named_child(*self, named_child_index)
    }
    pub fn children<'cursor>(
        &self,
        cursor: &'cursor mut TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<Item = Node<'tree>> + 'cursor + use<'cursor, 'tree> {
        cursor.reset(*self);
        cursor.goto_first_child();
        Children {
            cursor,
            remaining: self.child_count(),
            named: false,
        }
    }
    pub fn named_children<'cursor>(
        &self,
        cursor: &'cursor mut TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<Item = Node<'tree>> + 'cursor + use<'cursor, 'tree> {
        cursor.reset(*self);
        cursor.goto_first_child();
        Children {
            cursor,
            remaining: self.named_child_count(),
            named: true,
        }
    }
    pub fn children_by_field_name<'cursor>(
        &self,
        field_name: &str,
        cursor: &'cursor mut TreeCursor<'tree>,
    ) -> impl Iterator<Item = Node<'tree>> + 'cursor + use<'cursor, 'tree> {
        let field_id = self.language().field_id_for_name(field_name);
        let done = field_id.is_none();
        if !done {
            cursor.reset(*self);
            cursor.goto_first_child();
        }
        FieldChildren {
            cursor,
            // An unknown field never advances the cursor. The placeholder is
            // unobservable because this iterator is already done.
            field_id: field_id.unwrap_or(NonZeroU16::MIN),
            done,
        }
    }
    pub fn children_by_field_id<'cursor>(
        &self,
        field_id: FieldId,
        cursor: &'cursor mut TreeCursor<'tree>,
    ) -> impl Iterator<Item = Node<'tree>> + 'cursor + use<'cursor, 'tree> {
        cursor.reset(*self);
        cursor.goto_first_child();
        FieldChildren {
            cursor,
            field_id,
            done: false,
        }
    }
    pub fn parent(&self) -> Option<Self> {
        ts_node_parent(*self)
    }
    pub fn child_with_descendant(&self, descendant: Self) -> Option<Self> {
        ts_node_child_with_descendant(*self, descendant)
    }
    pub fn next_sibling(&self) -> Option<Self> {
        ts_node_next_sibling(*self)
    }
    pub fn prev_sibling(&self) -> Option<Self> {
        ts_node_prev_sibling(*self)
    }
    pub fn next_named_sibling(&self) -> Option<Self> {
        ts_node_next_named_sibling(*self)
    }
    pub fn prev_named_sibling(&self) -> Option<Self> {
        ts_node_prev_named_sibling(*self)
    }
    pub fn first_child_for_byte(&self, byte: usize) -> Option<Self> {
        ts_node_first_child_for_byte(*self, byte as u32)
    }
    pub fn first_named_child_for_byte(&self, byte: usize) -> Option<Self> {
        ts_node_first_named_child_for_byte(*self, byte as u32)
    }
    pub fn descendant_count(&self) -> usize {
        ts_node_descendant_count(*self) as usize
    }
    pub fn descendant_for_byte_range(&self, start: usize, end: usize) -> Option<Self> {
        ts_node_descendant_for_byte_range(*self, start as u32, end as u32)
    }
    pub fn named_descendant_for_byte_range(&self, start: usize, end: usize) -> Option<Self> {
        ts_node_named_descendant_for_byte_range(*self, start as u32, end as u32)
    }
    pub fn descendant_for_point_range(&self, start: Point, end: Point) -> Option<Self> {
        ts_node_descendant_for_point_range(*self, start.into(), end.into())
    }
    pub fn named_descendant_for_point_range(&self, start: Point, end: Point) -> Option<Self> {
        ts_node_named_descendant_for_point_range(*self, start.into(), end.into())
    }
    pub fn to_sexp(&self) -> String {
        ts_node_string(*self)
    }
    pub fn utf8_text<'a>(&self, source: &'a [u8]) -> Result<&'a str, str::Utf8Error> {
        str::from_utf8(&source[self.start_byte()..self.end_byte()])
    }
    pub fn utf16_text<'a>(&self, source: &'a [u16]) -> &'a [u16] {
        &source[self.start_byte() / 2..self.end_byte() / 2]
    }
    pub fn walk(&self) -> TreeCursor<'tree> {
        ts_tree_cursor_new(*self)
    }
    pub fn edit(&mut self, edit: &InputEdit) {
        ts_node_edit(self, &edit.into());
    }
}

impl<'cursor> TreeCursor<'cursor> {
    #[inline]
    pub fn node(&self) -> Node<'cursor> {
        ts_tree_cursor_current_node(self)
    }
    #[inline]
    pub fn field_id(&self) -> Option<FieldId> {
        FieldId::new(ts_tree_cursor_current_field_id(self))
    }
    #[inline]
    pub fn field_name(&self) -> Option<&'static str> {
        ts_tree_cursor_current_field_name(self)
    }
    #[inline]
    pub fn depth(&self) -> u32 {
        ts_tree_cursor_current_depth(self)
    }
    #[inline]
    pub fn descendant_index(&self) -> usize {
        ts_tree_cursor_current_descendant_index(self) as usize
    }
    #[inline]
    pub fn goto_first_child(&mut self) -> bool {
        ts_tree_cursor_goto_first_child(self)
    }
    pub fn goto_last_child(&mut self) -> bool {
        ts_tree_cursor_goto_last_child(self)
    }
    #[inline]
    pub fn goto_parent(&mut self) -> bool {
        ts_tree_cursor_goto_parent(self)
    }
    #[inline]
    pub fn goto_next_sibling(&mut self) -> bool {
        ts_tree_cursor_goto_next_sibling(self)
    }
    pub fn goto_descendant(&mut self, descendant_index: usize) {
        ts_tree_cursor_goto_descendant(self, descendant_index as u32);
    }
    pub fn goto_previous_sibling(&mut self) -> bool {
        ts_tree_cursor_goto_previous_sibling(self)
    }
    pub fn goto_first_child_for_byte(&mut self, index: usize) -> Option<usize> {
        ts_tree_cursor_goto_first_child_for_byte(self, index as u32).map(|i| i as usize)
    }
    pub fn goto_first_child_for_point(&mut self, point: Point) -> Option<usize> {
        ts_tree_cursor_goto_first_child_for_point(self, point.into()).map(|i| i as usize)
    }
    pub fn reset(&mut self, node: Node<'cursor>) {
        ts_tree_cursor_reset(self, node);
    }
    pub fn reset_to(&mut self, cursor: &Self) {
        ts_tree_cursor_reset_to(self, cursor);
    }
}

impl ParseState {
    pub const fn current_byte_offset(&self) -> usize {
        self.current_byte_offset as usize
    }
    pub const fn has_error(&self) -> bool {
        self.has_error
    }
}

impl<'a> ParseOptions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn progress_callback<F: FnMut(&ParseState) -> bool>(self, callback: &'a mut F) -> Self {
        Self {
            progress_callback: Some(callback),
        }
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for Tree {
    fn clone(&self) -> Self {
        ts_tree_copy(self)
    }
}

impl Clone for TreeCursor<'_> {
    fn clone(&self) -> Self {
        ts_tree_cursor_copy(self)
    }
}

impl fmt::Debug for Node<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{Node {} {} - {}}}",
            self.kind(),
            self.start_position(),
            self.end_position()
        )
    }
}

impl PartialEq for Node<'_> {
    fn eq(&self, other: &Self) -> bool {
        // The Rust binding compares only the subtree slot, not ts_node_eq's
        // additional tree identity check.
        std::ptr::eq(self.subtree, other.subtree)
    }
}

impl Hash for Node<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Preserve the binding's pointer and four u32 context hash inputs.
        std::ptr::from_ref(self.subtree).hash(state);
        self.position.bytes.hash(state);
        self.position.extent.row.hash(state);
        self.position.extent.column.hash(state);
        u32::from(self.alias).hash(state);
    }
}

impl fmt::Display for Node<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sexp = self.to_sexp();
        if sexp.is_empty() {
            Ok(())
        } else if !f.alternate() {
            f.write_str(&sexp)
        } else {
            f.write_str(&format_sexp(&sexp, f.width().unwrap_or(0)))
        }
    }
}

// Match the binding's alternate Node display, including quoted unexpected
// tokens and the special MISSING/UNEXPECTED S-expression forms.
fn format_sexp(sexp: &str, initial_indent_level: usize) -> String {
    let mut indent_level = initial_indent_level;
    let mut formatted = String::new();
    let mut has_field = false;
    let mut chars = sexp.chars().peekable();
    let mut token = String::with_capacity(sexp.len());
    let mut quote = '\0';
    let mut saw_paren = false;
    let mut did_last = false;

    let mut fetch_next_str = |next: &mut String| {
        next.clear();
        while let Some(c) = chars.next() {
            if c == '\'' || c == '"' {
                quote = c;
            } else if c == ' ' || (c == ')' && quote != '\0') {
                if let Some(&next_c) = chars.peek()
                    && next_c == quote
                {
                    next.push(c);
                    next.push(next_c);
                    chars.next();
                    quote = '\0';
                    continue;
                }
                break;
            }
            if c == ')' {
                saw_paren = true;
                break;
            }
            next.push(c);
        }

        if chars.peek().is_none() && next.is_empty() {
            if saw_paren {
                saw_paren = false;
                return Some(());
            }
            if !did_last {
                did_last = true;
                return Some(());
            }
            return None;
        }
        Some(())
    };

    while fetch_next_str(&mut token).is_some() {
        if token.is_empty() && indent_level > 0 {
            indent_level -= 1;
            formatted.push(')');
        } else if token.starts_with('(') {
            if has_field {
                has_field = false;
            } else {
                if indent_level > 0 {
                    formatted.push('\n');
                    for _ in 0..indent_level {
                        formatted.push_str("  ");
                    }
                }
                indent_level += 1;
            }
            formatted.push_str(&token);
            if token.starts_with("(MISSING") || token.starts_with("(UNEXPECTED") {
                fetch_next_str(&mut token).unwrap();
                if token.is_empty() {
                    while indent_level > 0 {
                        indent_level -= 1;
                        formatted.push(')');
                    }
                } else {
                    formatted.push(' ');
                    formatted.push_str(&token);
                }
            }
        } else if token.ends_with(':') {
            formatted.push('\n');
            for _ in 0..indent_level {
                formatted.push_str("  ");
            }
            formatted.push_str(&token);
            formatted.push(' ');
            has_field = true;
            indent_level += 1;
        }
    }
    formatted
}

impl fmt::Display for LanguageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Incompatible language version {}. Expected minimum {}, maximum {}",
            self.version, MIN_COMPATIBLE_LANGUAGE_VERSION, LANGUAGE_VERSION,
        )
    }
}

impl fmt::Display for IncludedRangesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Incorrect range by index: {}", self.0)
    }
}

impl Eq for Node<'_> {}
impl std::error::Error for LanguageError {}
impl std::error::Error for IncludedRangesError {}

impl<'tree> Iterator for Children<'_, 'tree> {
    type Item = Node<'tree>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        if self.named {
            while !self.cursor.node().is_named() {
                if !self.cursor.goto_next_sibling() {
                    break;
                }
            }
        }
        let result = self.cursor.node();
        self.cursor.goto_next_sibling();
        self.remaining -= 1;
        Some(result)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }

    // The binding uses (0..child_count).map(...) with cursor movement inside
    // the mapping closure. Preserve Map's specialized consumers: skipped
    // indices do not execute that closure and therefore do not move the cursor.
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.remaining = self.remaining.saturating_sub(n);
        self.next()
    }

    fn count(self) -> usize {
        self.remaining
    }

    fn last(mut self) -> Option<Self::Item> {
        self.remaining = usize::from(self.remaining != 0);
        self.next()
    }
}

impl<'tree> Iterator for FieldChildren<'_, 'tree> {
    type Item = Node<'tree>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        while self.cursor.field_id() != Some(self.field_id) {
            if !self.cursor.goto_next_sibling() {
                return None;
            }
        }
        let result = self.cursor.node();
        if !self.cursor.goto_next_sibling() {
            self.done = true;
        }
        Some(result)
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

    #[test]
    fn range_and_edit_conversions_match_binding() {
        let range = Range {
            start_byte: 3,
            end_byte: 29,
            start_point: Point::new(1, 2),
            end_point: Point::new(4, 5),
        };
        assert_eq!(Range::from(crate::types::Range::from(range)), range);
        let edit = InputEdit {
            start_byte: range.start_byte,
            old_end_byte: range.end_byte,
            new_end_byte: 40,
            start_position: range.start_point,
            old_end_position: range.end_point,
            new_end_position: Point::new(6, 7),
        };
        let internal = crate::types::InputEdit::from(&edit);
        assert_eq!(internal.start_byte, 3);
        assert_eq!(internal.old_end_byte, 29);
        assert_eq!(internal.new_end_byte, 40);
        assert_eq!(Point::from(internal.start_point), edit.start_position);
        assert_eq!(Point::from(internal.old_end_point), edit.old_end_position);
        assert_eq!(Point::from(internal.new_end_point), edit.new_end_position);
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn coordinate_conversion_truncates_to_c_width() {
        let point = Point::new((1usize << 32) + 2, (1usize << 32) + 17);
        assert_eq!(
            Point::from(crate::point::Point::from(point)),
            Point::new(2, 17)
        );
        let range = Range {
            start_byte: (1usize << 32) + 7,
            end_byte: (1usize << 32) + 9,
            start_point: point,
            end_point: point,
        };
        let internal = crate::types::Range::from(range);
        assert_eq!(internal.start_byte, 7);
        assert_eq!(internal.end_byte, 9);
    }

    #[test]
    fn parse_options_borrow_and_replace_progress_callbacks() {
        use std::{cell::Cell, rc::Rc};
        const STATE: ParseState = ParseState {
            current_byte_offset: 17,
            has_error: true,
        };
        const OFFSET: usize = STATE.current_byte_offset();
        const ERROR: bool = STATE.has_error();
        assert_eq!((OFFSET, ERROR), (17, true));
        assert!(ParseOptions::new().progress_callback.is_none());

        let count = Rc::new(Cell::new(0));
        let mut unused = |_: &ParseState| panic!("replaced callback must not be called");
        let mut callback = |state: &ParseState| {
            count.set(count.get() + 1);
            assert_eq!(state.current_byte_offset(), 17);
            state.has_error()
        };
        let mut options = ParseOptions::new()
            .progress_callback(&mut unused)
            .progress_callback(&mut callback);
        assert!(options.progress_callback.as_mut().unwrap()(&STATE));
        assert_eq!(count.get(), 1);
    }

    #[test]
    fn callback_input_retains_owned_chunks_until_the_next_read() {
        use crate::types::Input;
        use std::{cell::Cell, rc::Rc};
        let calls = Rc::new(Cell::new(0));
        let mut callback = |byte: usize, point: Point| {
            calls.set(calls.get() + 1);
            assert_eq!(point, Point::new(2, byte));
            if byte < 8 {
                vec![byte as u8, 42]
            } else {
                Vec::new()
            }
        };
        let mut input = CallbackInput {
            callback: &mut callback,
            chunk: None,
        };
        assert!(input.chunk().is_empty());
        input.read(3, Point::new(2, 3).into());
        assert_eq!(input.chunk(), &[3, 42]);
        assert_eq!(input.chunk(), &[3, 42]);
        assert_eq!(calls.get(), 1);
        input.read(8, Point::new(2, 8).into());
        assert!(input.chunk().is_empty());
        assert_eq!(calls.get(), 2);
    }

    #[test]
    fn errors_match_the_official_binding_messages() {
        assert_eq!(
            IncludedRangesError(3).to_string(),
            "Incorrect range by index: 3"
        );
        assert_eq!(
            LanguageError { version: 12 }.to_string(),
            "Incompatible language version 12. Expected minimum 13, maximum 15",
        );
    }

    #[test]
    fn alternate_sexp_formatting_matches_binding() {
        for (input, indent, expected) in [
            ("", 0, ""),
            ("(identifier)", 0, "(identifier)"),
            ("(root (left) (right))", 0, "(root\n  (left)\n  (right))"),
            (
                "(root name: (identifier) body: (block (return_statement)))",
                0,
                "(root\n  name: (identifier)\n  body: (block\n    (return_statement)))",
            ),
            (
                "(root (MISSING identifier) (UNEXPECTED ' '))",
                0,
                "(root\n  (MISSING identifier)\n  (UNEXPECTED ' '))",
            ),
            ("(ERROR (UNEXPECTED ')'))", 0, "(ERROR\n  (UNEXPECTED ')'))"),
            (
                "(root (left) (right))",
                2,
                "\n    (root\n      (left)\n      (right))",
            ),
            // Retain the reference formatter's final-empty-token behavior.
            ("(identifier)", 2, "\n    (identifier))"),
            ("(MISSING)", 2, "\n    (MISSING)))"),
        ] {
            assert_eq!(
                format_sexp(input, indent),
                expected,
                "{input:?}, indent {indent}"
            );
        }
    }

    #[test]
    fn children_count_and_exhaustion_do_not_visit_the_cursor() {
        // No tree is needed: these operations must not ask the cursor for a node.
        let mut cursor = TreeCursor {
            tree: None,
            stack: Vec::new(),
            root_alias_symbol: 0,
        };
        let children = Children {
            cursor: &mut cursor,
            remaining: 4,
            named: false,
        };
        assert_eq!(children.size_hint(), (4, Some(4)));
        assert_eq!(children.len(), 4);
        assert_eq!(children.count(), 4);
        let mut children = Children {
            cursor: &mut cursor,
            remaining: 4,
            named: true,
        };
        assert!(children.nth(4).is_none());
        assert_eq!(children.len(), 0);
        assert!(children.next().is_none());
        assert!(children.last().is_none());
    }
}
