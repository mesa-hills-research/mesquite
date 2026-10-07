//! Official 0.25.10 Rust query binding surface; implementation unit query-api.
// std::todo!("label") uses formatting, which is not const-compatible. This
// temporary equivalent keeps the official const signatures on tagged stubs.
macro_rules! todo {
    ($label:literal) => {
        ::core::panic!(concat!("not yet implemented: ", $label))
    };
}
use crate::{Language, Node, Point, query::*};
use std::{fmt, iter, marker::PhantomData, ops, sync::Mutex};
use streaming_iterator::{StreamingIterator, StreamingIteratorMut};

/// A set of patterns compiled for a language.
#[derive(Debug)]
pub struct Query {
    pub(crate) compiled: Box<CompiledQuery>,
    capture_names: CaptureNames,
    capture_quantifiers: Box<[Box<[CaptureQuantifier]>]>,
    text_predicates: Box<[Box<[TextPredicateCapture]>]>,
    property_settings: Box<[Box<[QueryProperty]>]>,
    property_predicates: Box<[PatternPropertyPredicates]>,
    general_predicates: Box<[Box<[QueryPredicate]>]>,
}

type PatternPropertyPredicates = Box<[(QueryProperty, bool)]>;

/// Stable owned strings plus a private lifetime-erased view, as required by
/// capture_names() -> &[&str]. Never return the internal 'static lifetime.
#[derive(Debug)]
struct CaptureNames {
    views: Box<[&'static str]>,
    storage: Box<[Box<str>]>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CaptureQuantifier {
    Zero,
    ZeroOrOne,
    ZeroOrMore,
    One,
    OneOrMore,
}

/// Persistent settings and storage. All traversal borrows live in the iterators.
pub struct QueryCursor {
    config: CursorConfig,
    snapshots: CaptureSnapshots,
    removals: Mutex<Vec<u32>>,
}

/// Binding-only immutable result storage, not the engine's match-limited pool.
/// Reset only under an exclusive cursor borrow, before starting a new execution.
/// See PORTING.md for the narrowly scoped lifetime-erasure implementation contract.
#[derive(Default)]
struct CaptureSnapshots {
    lists: Mutex<Vec<Box<[QueryCapture<'static>]>>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct QueryProperty {
    pub key: Box<str>,
    pub value: Option<Box<str>>,
    pub capture_id: Option<usize>,
}
#[derive(Debug, PartialEq, Eq)]
pub enum QueryPredicateArg {
    Capture(u32),
    String(Box<str>),
}
#[derive(Debug, PartialEq, Eq)]
pub struct QueryPredicate {
    pub operator: Box<str>,
    pub args: Box<[QueryPredicateArg]>,
}

pub struct QueryMatch<'cursor, 'tree> {
    pub pattern_index: usize,
    pub captures: &'cursor [QueryCapture<'tree>],
    id: u32,
    /// remove() queues an id; advance drains the queue before touching the engine.
    removals: &'cursor Mutex<Vec<u32>>,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct QueryCapture<'tree> {
    pub node: Node<'tree>,
    pub index: u32,
}

pub trait TextProvider<I: AsRef<[u8]>> {
    type I: Iterator<Item = I>;
    fn text(&mut self, node: Node<'_>) -> Self::I;
}

/// Matches are streamed in the runtime's order and filtered by text predicates.
pub struct QueryMatches<'query, 'tree: 'query, T: TextProvider<I>, I: AsRef<[u8]>> {
    execution: QueryExecution<'query, 'tree>,
    query: &'query Query,
    text_provider: T,
    buffer1: Vec<u8>,
    buffer2: Vec<u8>,
    current_match: Option<QueryMatch<'query, 'tree>>,
    snapshots: &'query CaptureSnapshots,
    removals: &'query Mutex<Vec<u32>>,
    _phantom: PhantomData<I>,
}

/// Each item is a match and the index of its next capture (not the capture id).
pub struct QueryCaptures<'query, 'tree: 'query, T: TextProvider<I>, I: AsRef<[u8]>> {
    execution: QueryExecution<'query, 'tree>,
    query: &'query Query,
    text_provider: T,
    buffer1: Vec<u8>,
    buffer2: Vec<u8>,
    current_match: Option<(QueryMatch<'query, 'tree>, usize)>,
    snapshots: &'query CaptureSnapshots,
    removals: &'query Mutex<Vec<u32>>,
    _phantom: PhantomData<I>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct QueryError {
    pub row: usize,
    pub column: usize,
    pub offset: usize,
    pub message: String,
    pub kind: QueryErrorKind,
}
#[derive(Debug, PartialEq, Eq)]
pub enum QueryErrorKind {
    Syntax,
    NodeType,
    Field,
    Capture,
    Predicate,
    Structure,
    Language,
}

/// Booleans have the binding's order: positive, then match_all_nodes.
#[derive(Debug)]
enum TextPredicateCapture {
    EqString(u32, Box<str>, bool, bool),
    EqCapture(u32, u32, bool, bool),
    MatchString(u32, regex::bytes::Regex, bool, bool),
    AnyString(u32, Box<[Box<str>]>, bool),
}

pub type QueryProgressCallback<'a> = &'a mut dyn FnMut(&QueryCursorState) -> bool;
#[derive(Default)]
pub struct QueryCursorOptions<'a> {
    pub progress_callback: Option<QueryProgressCallback<'a>>,
}
#[derive(Debug, Default)]
pub struct QueryCursorState {
    pub(crate) current_byte_offset: u32,
}

/// Concrete iterator backing nodes_for_capture_index (keeps the API fully safe).
pub struct NodesForCaptureIndex<'a, 'tree> {
    captures: std::slice::Iter<'a, QueryCapture<'tree>>,
    capture_index: u32,
}

// Binding helpers. Lifetime erasure is restricted to these storage adapters;
// query compilation and execution must remain entirely safe Rust.
impl CaptureNames {
    fn new(names: Vec<Box<str>>) -> Self {
        todo!("query-api: CaptureNames::new")
    }
}
impl CaptureSnapshots {
    fn store<'cursor, 'tree: 'cursor>(
        &'cursor self,
        captures: CaptureList<'tree>,
    ) -> &'cursor [QueryCapture<'tree>] {
        todo!("query-api: CaptureSnapshots::store")
    }
    fn reset(&mut self) {
        todo!("query-api: CaptureSnapshots::reset")
    }
}
fn drain_removals(execution: &mut QueryExecution<'_, '_>, removals: &Mutex<Vec<u32>>) {
    todo!("query-api: drain_removals")
}

impl Query {
    pub fn new(language: &Language, source: &str) -> Result<Self, QueryError> {
        todo!("query-api: Query::new")
    }
    /// Counterpart of binding Query::from_raw_parts, without FFI or raw pointers.
    fn from_compiled(compiled: CompiledQuery, source: &str) -> Result<Self, QueryError> {
        todo!("query-api: Query::from_raw_parts")
    }
    pub fn pattern_count(&self) -> usize {
        todo!("query-api: Query::pattern_count")
    }
    pub const fn capture_names(&self) -> &[&str] {
        todo!("query-api: Query::capture_names")
    }
    pub const fn capture_quantifiers(&self, index: usize) -> &[CaptureQuantifier] {
        todo!("query-api: Query::capture_quantifiers")
    }
    pub fn start_byte_for_pattern(&self, pattern_index: usize) -> usize {
        todo!("query-api: Query::start_byte_for_pattern")
    }
    pub fn end_byte_for_pattern(&self, pattern_index: usize) -> usize {
        todo!("query-api: Query::end_byte_for_pattern")
    }
    pub const fn general_predicates(&self, index: usize) -> &[QueryPredicate] {
        todo!("query-api: Query::general_predicates")
    }
    pub const fn property_settings(&self, index: usize) -> &[QueryProperty] {
        todo!("query-api: Query::property_settings")
    }
    pub const fn property_predicates(&self, index: usize) -> &[(QueryProperty, bool)] {
        todo!("query-api: Query::property_predicates")
    }
    pub fn disable_capture(&mut self, name: &str) {
        todo!("query-api: Query::disable_capture")
    }
    pub fn disable_pattern(&mut self, index: usize) {
        todo!("query-api: Query::disable_pattern")
    }
    pub fn is_pattern_rooted(&self, index: usize) -> bool {
        todo!("query-api: Query::is_pattern_rooted")
    }
    pub fn is_pattern_non_local(&self, index: usize) -> bool {
        todo!("query-api: Query::is_pattern_non_local")
    }
    pub fn is_pattern_guaranteed_at_step(&self, byte_offset: usize) -> bool {
        todo!("query-api: Query::is_pattern_guaranteed_at_step")
    }
    pub fn capture_index_for_name(&self, name: &str) -> Option<u32> {
        todo!("query-api: Query::capture_index_for_name")
    }
    fn parse_property(
        row: usize,
        function_name: &str,
        capture_names: &[&str],
        string_values: &[&str],
        args: &[PredicateStep],
    ) -> Result<QueryProperty, QueryError> {
        todo!("query-api: Query::parse_property")
    }
}
impl PartialEq for Query {
    fn eq(&self, other: &Self) -> bool {
        todo!("query-api: Query::eq")
    }
}
impl QueryCursor {
    pub fn new() -> Self {
        todo!("query-api: QueryCursor::new")
    }
    pub fn set_match_limit(&mut self, limit: u32) {
        todo!("query-api: QueryCursor::set_match_limit")
    }
    pub fn match_limit(&self) -> u32 {
        todo!("query-api: QueryCursor::match_limit")
    }
    pub fn did_exceed_match_limit(&self) -> bool {
        todo!("query-api: QueryCursor::did_exceed_match_limit")
    }
    pub fn set_byte_range(&mut self, range: ops::Range<usize>) -> &mut Self {
        todo!("query-api: QueryCursor::set_byte_range")
    }
    pub fn set_point_range(&mut self, range: ops::Range<Point>) -> &mut Self {
        todo!("query-api: QueryCursor::set_point_range")
    }
    pub fn set_max_start_depth(&mut self, max_start_depth: Option<u32>) -> &mut Self {
        todo!("query-api: QueryCursor::set_max_start_depth")
    }
    #[deprecated(
        since = "0.25.0",
        note = "Use matches_with_options or captures_with_options"
    )]
    pub fn set_timeout_micros(&mut self, timeout: u64) {
        todo!("query-api: QueryCursor::set_timeout_micros")
    }
    #[deprecated(
        since = "0.25.0",
        note = "Use matches_with_options or captures_with_options"
    )]
    pub fn timeout_micros(&self) -> u64 {
        todo!("query-api: QueryCursor::timeout_micros")
    }
    pub fn matches<'query, 'cursor: 'query, 'tree: 'query, T: TextProvider<I>, I: AsRef<[u8]>>(
        &'cursor mut self,
        query: &'query Query,
        node: Node<'tree>,
        text_provider: T,
    ) -> QueryMatches<'query, 'tree, T, I> {
        todo!("query-api: QueryCursor::matches")
    }
    pub fn matches_with_options<
        'query,
        'cursor: 'query,
        'tree: 'query,
        T: TextProvider<I>,
        I: AsRef<[u8]>,
    >(
        &'cursor mut self,
        query: &'query Query,
        node: Node<'tree>,
        text_provider: T,
        options: QueryCursorOptions<'query>,
    ) -> QueryMatches<'query, 'tree, T, I> {
        todo!("query-api: QueryCursor::matches_with_options")
    }
    pub fn captures<'query, 'cursor: 'query, 'tree: 'query, T: TextProvider<I>, I: AsRef<[u8]>>(
        &'cursor mut self,
        query: &'query Query,
        node: Node<'tree>,
        text_provider: T,
    ) -> QueryCaptures<'query, 'tree, T, I> {
        todo!("query-api: QueryCursor::captures")
    }
    pub fn captures_with_options<
        'query,
        'cursor: 'query,
        'tree: 'query,
        T: TextProvider<I>,
        I: AsRef<[u8]>,
    >(
        &'cursor mut self,
        query: &'query Query,
        node: Node<'tree>,
        text_provider: T,
        options: QueryCursorOptions<'query>,
    ) -> QueryCaptures<'query, 'tree, T, I> {
        todo!("query-api: QueryCursor::captures_with_options")
    }
}
impl Default for QueryCursor {
    fn default() -> Self {
        todo!("query-api: QueryCursor::default")
    }
}

impl<'cursor, 'tree: 'cursor> QueryMatch<'cursor, 'tree> {
    pub const fn id(&self) -> u32 {
        todo!("query-api: QueryMatch::id")
    }
    pub fn remove(&self) {
        todo!("query-api: QueryMatch::remove")
    }
    pub fn nodes_for_capture_index(&self, capture_ix: u32) -> NodesForCaptureIndex<'_, 'tree> {
        todo!("query-api: QueryMatch::nodes_for_capture_index")
    }
    fn new(
        result: QueryMatchData<'tree>,
        snapshots: &'cursor CaptureSnapshots,
        removals: &'cursor Mutex<Vec<u32>>,
    ) -> Self {
        todo!("query-api: QueryMatch::new")
    }
    pub fn satisfies_text_predicates<I: AsRef<[u8]>>(
        &self,
        query: &Query,
        buffer1: &mut Vec<u8>,
        buffer2: &mut Vec<u8>,
        text_provider: &mut impl TextProvider<I>,
    ) -> bool {
        todo!("query-api: QueryMatch::satisfies_text_predicates")
    }
}
impl<'tree> Iterator for NodesForCaptureIndex<'_, 'tree> {
    type Item = Node<'tree>;
    fn next(&mut self) -> Option<Self::Item> {
        todo!("query-api: nodes_for_capture_index iterator")
    }
}
impl fmt::Debug for QueryMatch<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("query-api: QueryMatch::fmt")
    }
}
impl QueryProperty {
    pub fn new(key: &str, value: Option<&str>, capture_id: Option<usize>) -> Self {
        todo!("query-api: QueryProperty::new")
    }
}
impl QueryCursorState {
    pub const fn current_byte_offset(&self) -> usize {
        todo!("query-api: QueryCursorState::current_byte_offset")
    }
}
impl<'a> QueryCursorOptions<'a> {
    pub fn new() -> Self {
        todo!("query-api: QueryCursorOptions::new")
    }
    pub fn progress_callback<F: FnMut(&QueryCursorState) -> bool>(
        self,
        callback: &'a mut F,
    ) -> Self {
        todo!("query-api: QueryCursorOptions::progress_callback")
    }
}

impl<'query, 'tree: 'query, T: TextProvider<I>, I: AsRef<[u8]>> StreamingIterator
    for QueryMatches<'query, 'tree, T, I>
{
    type Item = QueryMatch<'query, 'tree>;
    fn advance(&mut self) {
        todo!("query-api: QueryMatches::advance")
    }
    fn get(&self) -> Option<&Self::Item> {
        todo!("query-api: QueryMatches::get")
    }
}
impl<'query, 'tree: 'query, T: TextProvider<I>, I: AsRef<[u8]>> StreamingIteratorMut
    for QueryMatches<'query, 'tree, T, I>
{
    fn get_mut(&mut self) -> Option<&mut Self::Item> {
        todo!("query-api: QueryMatches::get_mut")
    }
}
impl<'query, 'tree: 'query, T: TextProvider<I>, I: AsRef<[u8]>> StreamingIterator
    for QueryCaptures<'query, 'tree, T, I>
{
    type Item = (QueryMatch<'query, 'tree>, usize);
    fn advance(&mut self) {
        todo!("query-api: QueryCaptures::advance")
    }
    fn get(&self) -> Option<&Self::Item> {
        todo!("query-api: QueryCaptures::get")
    }
}
impl<'query, 'tree: 'query, T: TextProvider<I>, I: AsRef<[u8]>> StreamingIteratorMut
    for QueryCaptures<'query, 'tree, T, I>
{
    fn get_mut(&mut self) -> Option<&mut Self::Item> {
        todo!("query-api: QueryCaptures::get_mut")
    }
}
impl<T: TextProvider<I>, I: AsRef<[u8]>> QueryMatches<'_, '_, T, I> {
    pub fn set_byte_range(&mut self, range: ops::Range<usize>) {
        todo!("query-api: QueryMatches::set_byte_range")
    }
    pub fn set_point_range(&mut self, range: ops::Range<Point>) {
        todo!("query-api: QueryMatches::set_point_range")
    }
}
impl<T: TextProvider<I>, I: AsRef<[u8]>> QueryCaptures<'_, '_, T, I> {
    pub fn set_byte_range(&mut self, range: ops::Range<usize>) {
        todo!("query-api: QueryCaptures::set_byte_range")
    }
    pub fn set_point_range(&mut self, range: ops::Range<Point>) {
        todo!("query-api: QueryCaptures::set_point_range")
    }
}
impl<F, R, I> TextProvider<I> for F
where
    F: FnMut(Node<'_>) -> R,
    R: Iterator<Item = I>,
    I: AsRef<[u8]>,
{
    type I = R;
    fn text(&mut self, node: Node<'_>) -> Self::I {
        todo!("query-api: TextProvider::text (closure)")
    }
}
impl<'a> TextProvider<&'a [u8]> for &'a [u8] {
    type I = iter::Once<&'a [u8]>;
    fn text(&mut self, node: Node<'_>) -> Self::I {
        todo!("query-api: TextProvider::text (bytes)")
    }
}
impl fmt::Display for QueryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("query-api: QueryError::fmt")
    }
}
impl std::error::Error for QueryError {}
const fn predicate_error(row: usize, message: String) -> QueryError {
    todo!("query-api: predicate_error")
}

/// Text chunks are concatenated only when a node spans multiple provider chunks.
struct NodeText<'a, T> {
    buffer: &'a mut Vec<u8>,
    first_chunk: Option<T>,
}
impl<'a, T: AsRef<[u8]>> NodeText<'a, T> {
    fn new(buffer: &'a mut Vec<u8>) -> Self {
        todo!("query-api: NodeText::new")
    }
    fn get_text(&mut self, chunks: &mut impl Iterator<Item = T>) -> &[u8] {
        todo!("query-api: NodeText::get_text")
    }
}
