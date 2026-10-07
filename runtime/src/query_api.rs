//! Official 0.25.10 Rust query binding surface; implementation unit query-api.
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

/// Owned names; capture_names builds a safely borrowed view on demand.
#[derive(Debug)]
struct CaptureNames {
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
    removals: Mutex<Vec<u32>>,
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

#[derive(Clone)]
pub struct QueryMatch<'cursor, 'tree> {
    pub pattern_index: usize,
    pub captures: Vec<QueryCapture<'tree>>,
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

// Metadata and matches own their buffers: no lifetime erasure is necessary.
impl CaptureNames {
    fn new(names: Vec<Box<str>>) -> Self {
        Self {
            storage: names.into_boxed_slice(),
        }
    }
}
fn drain_removals(execution: &mut QueryExecution<'_, '_>, removals: &Mutex<Vec<u32>>) {
    for id in removals.lock().unwrap().drain(..) {
        ts_query_cursor_remove_match(execution, id);
    }
}

impl Query {
    pub fn new(language: &Language, source: &str) -> Result<Self, QueryError> {
        // Preserve the binding's u32 source-length truncation at the engine boundary.
        let bytes = source.as_bytes();
        let compiled = ts_query_new(language, &bytes[..bytes.len() as u32 as usize])
            .map_err(|error| compile_error(language, source, error))?;
        Self::from_compiled(compiled, source)
    }
    /// Counterpart of binding Query::from_raw_parts, without FFI or raw pointers.
    fn from_compiled(compiled: CompiledQuery, source: &str) -> Result<Self, QueryError> {
        let string_count = ts_query_string_count(&compiled);
        let capture_count = ts_query_capture_count(&compiled);
        let pattern_count = ts_query_pattern_count(&compiled) as usize;
        let mut capture_quantifiers_vec = Vec::with_capacity(pattern_count);
        let mut text_predicates_vec = Vec::with_capacity(pattern_count);
        let mut property_predicates_vec = Vec::with_capacity(pattern_count);
        let mut property_settings_vec = Vec::with_capacity(pattern_count);
        let mut general_predicates_vec = Vec::with_capacity(pattern_count);

        // Compilation only copies/decodes UTF-8 from the supplied &str.
        let capture_names = (0..capture_count)
            .map(|i| std::str::from_utf8(ts_query_capture_name_for_id(&compiled, i)).unwrap())
            .collect::<Vec<_>>();
        for i in 0..pattern_count {
            let quantifiers = (0..capture_count)
                .map(|j| ts_query_capture_quantifier_for_id(&compiled, i as u32, j))
                .collect::<Box<[_]>>();
            capture_quantifiers_vec.push(quantifiers);
        }
        let string_values = (0..string_count)
            .map(|i| std::str::from_utf8(ts_query_string_value_for_id(&compiled, i)).unwrap())
            .collect::<Vec<_>>();

        for i in 0..pattern_count {
            let predicate_steps = ts_query_predicates_for_pattern(&compiled, i as u32);
            let byte_offset = ts_query_start_byte_for_pattern(&compiled, i as u32);
            let row = source
                .char_indices()
                .take_while(|(i, _)| *i < byte_offset as usize)
                .filter(|(_, c)| *c == '\n')
                .count();

            use PredicateStepKind::{Capture, Done, String as Literal};

            let mut text_predicates = Vec::new();
            let mut property_predicates = Vec::new();
            let mut property_settings = Vec::new();
            let mut general_predicates = Vec::new();
            for p in predicate_steps.split(|s| s.kind == Done) {
                if p.is_empty() {
                    continue;
                }

                if p[0].kind != Literal {
                    return Err(predicate_error(
                        row,
                        format!(
                            "Expected predicate to start with a function name. Got @{}.",
                            capture_names[p[0].value_id as usize],
                        ),
                    ));
                }

                // Build a predicate for each of the known predicate function names.
                let operator_name = string_values[p[0].value_id as usize];
                match operator_name {
                    "eq?" | "not-eq?" | "any-eq?" | "any-not-eq?" => {
                        if p.len() != 3 {
                            return Err(predicate_error(
                                row,
                                format!(
                                    "Wrong number of arguments to #eq? predicate. Expected 2, got {}.",
                                    p.len() - 1
                                ),
                            ));
                        }
                        if p[1].kind != Capture {
                            return Err(predicate_error(
                                row,
                                format!(
                                    "First argument to #eq? predicate must be a capture name. Got literal \"{}\".",
                                    string_values[p[1].value_id as usize],
                                ),
                            ));
                        }

                        let is_positive = operator_name == "eq?" || operator_name == "any-eq?";
                        let match_all = match operator_name {
                            "eq?" | "not-eq?" => true,
                            "any-eq?" | "any-not-eq?" => false,
                            _ => unreachable!(),
                        };
                        text_predicates.push(if p[2].kind == Capture {
                            TextPredicateCapture::EqCapture(
                                p[1].value_id,
                                p[2].value_id,
                                is_positive,
                                match_all,
                            )
                        } else {
                            TextPredicateCapture::EqString(
                                p[1].value_id,
                                string_values[p[2].value_id as usize].to_string().into(),
                                is_positive,
                                match_all,
                            )
                        });
                    }

                    "match?" | "not-match?" | "any-match?" | "any-not-match?" => {
                        if p.len() != 3 {
                            return Err(predicate_error(
                                row,
                                format!(
                                    "Wrong number of arguments to #match? predicate. Expected 2, got {}.",
                                    p.len() - 1
                                ),
                            ));
                        }
                        if p[1].kind != Capture {
                            return Err(predicate_error(
                                row,
                                format!(
                                    "First argument to #match? predicate must be a capture name. Got literal \"{}\".",
                                    string_values[p[1].value_id as usize],
                                ),
                            ));
                        }
                        if p[2].kind == Capture {
                            return Err(predicate_error(
                                row,
                                format!(
                                    "Second argument to #match? predicate must be a literal. Got capture @{}.",
                                    capture_names[p[2].value_id as usize],
                                ),
                            ));
                        }

                        let is_positive =
                            operator_name == "match?" || operator_name == "any-match?";
                        let match_all = match operator_name {
                            "match?" | "not-match?" => true,
                            "any-match?" | "any-not-match?" => false,
                            _ => unreachable!(),
                        };
                        let regex = &string_values[p[2].value_id as usize];
                        text_predicates.push(TextPredicateCapture::MatchString(
                            p[1].value_id,
                            regex::bytes::Regex::new(regex).map_err(|_| {
                                predicate_error(row, format!("Invalid regex '{regex}'"))
                            })?,
                            is_positive,
                            match_all,
                        ));
                    }

                    "set!" => property_settings.push(Self::parse_property(
                        row,
                        operator_name,
                        &capture_names,
                        &string_values,
                        &p[1..],
                    )?),

                    "is?" | "is-not?" => property_predicates.push((
                        Self::parse_property(
                            row,
                            operator_name,
                            &capture_names,
                            &string_values,
                            &p[1..],
                        )?,
                        operator_name == "is?",
                    )),

                    "any-of?" | "not-any-of?" => {
                        if p.len() < 2 {
                            return Err(predicate_error(
                                row,
                                format!(
                                    "Wrong number of arguments to #any-of? predicate. Expected at least 1, got {}.",
                                    p.len() - 1
                                ),
                            ));
                        }
                        if p[1].kind != Capture {
                            return Err(predicate_error(
                                row,
                                format!(
                                    "First argument to #any-of? predicate must be a capture name. Got literal \"{}\".",
                                    string_values[p[1].value_id as usize],
                                ),
                            ));
                        }

                        let is_positive = operator_name == "any-of?";
                        let mut values = Vec::new();
                        for arg in &p[2..] {
                            if arg.kind == Capture {
                                return Err(predicate_error(
                                    row,
                                    format!(
                                        "Arguments to #any-of? predicate must be literals. Got capture @{}.",
                                        capture_names[arg.value_id as usize],
                                    ),
                                ));
                            }
                            values.push(string_values[arg.value_id as usize]);
                        }
                        text_predicates.push(TextPredicateCapture::AnyString(
                            p[1].value_id,
                            values
                                .iter()
                                .map(|x| (*x).to_string().into())
                                .collect::<Vec<_>>()
                                .into(),
                            is_positive,
                        ));
                    }

                    _ => general_predicates.push(QueryPredicate {
                        operator: operator_name.to_string().into(),
                        args: p[1..]
                            .iter()
                            .map(|a| {
                                if a.kind == Capture {
                                    QueryPredicateArg::Capture(a.value_id)
                                } else {
                                    QueryPredicateArg::String(
                                        string_values[a.value_id as usize].to_string().into(),
                                    )
                                }
                            })
                            .collect(),
                    }),
                }
            }

            text_predicates_vec.push(text_predicates.into());
            property_predicates_vec.push(property_predicates.into());
            property_settings_vec.push(property_settings.into());
            general_predicates_vec.push(general_predicates.into());
        }

        let capture_names =
            CaptureNames::new(capture_names.into_iter().map(Box::<str>::from).collect());
        Ok(Self {
            compiled: Box::new(compiled),
            capture_names,
            capture_quantifiers: capture_quantifiers_vec.into(),
            text_predicates: text_predicates_vec.into(),
            property_predicates: property_predicates_vec.into(),
            property_settings: property_settings_vec.into(),
            general_predicates: general_predicates_vec.into(),
        })
    }
    pub fn pattern_count(&self) -> usize {
        ts_query_pattern_count(&self.compiled) as usize
    }
    /// Return names borrowed from this query. The vector owns only the views.
    /// Unlike the FFI binding's self-referential slice this is entirely safe Rust.
    pub fn capture_names(&self) -> Vec<&str> {
        self.capture_names
            .storage
            .iter()
            .map(AsRef::as_ref)
            .collect()
    }
    pub const fn capture_quantifiers(&self, index: usize) -> &[CaptureQuantifier] {
        &self.capture_quantifiers[index]
    }
    pub fn start_byte_for_pattern(&self, pattern_index: usize) -> usize {
        assert!(
            pattern_index < self.text_predicates.len(),
            "Pattern index is {pattern_index} but the pattern count is {}",
            self.text_predicates.len(),
        );
        ts_query_start_byte_for_pattern(&self.compiled, pattern_index as u32) as usize
    }
    pub fn end_byte_for_pattern(&self, pattern_index: usize) -> usize {
        assert!(
            pattern_index < self.text_predicates.len(),
            "Pattern index is {pattern_index} but the pattern count is {}",
            self.text_predicates.len(),
        );
        ts_query_end_byte_for_pattern(&self.compiled, pattern_index as u32) as usize
    }
    pub const fn general_predicates(&self, index: usize) -> &[QueryPredicate] {
        &self.general_predicates[index]
    }
    pub const fn property_settings(&self, index: usize) -> &[QueryProperty] {
        &self.property_settings[index]
    }
    pub const fn property_predicates(&self, index: usize) -> &[(QueryProperty, bool)] {
        &self.property_predicates[index]
    }
    pub fn disable_capture(&mut self, name: &str) {
        let bytes = name.as_bytes();
        ts_query_disable_capture(&mut self.compiled, &bytes[..bytes.len() as u32 as usize]);
    }
    pub fn disable_pattern(&mut self, index: usize) {
        ts_query_disable_pattern(&mut self.compiled, index as u32);
    }
    pub fn is_pattern_rooted(&self, index: usize) -> bool {
        ts_query_is_pattern_rooted(&self.compiled, index as u32)
    }
    pub fn is_pattern_non_local(&self, index: usize) -> bool {
        ts_query_is_pattern_non_local(&self.compiled, index as u32)
    }
    pub fn is_pattern_guaranteed_at_step(&self, byte_offset: usize) -> bool {
        ts_query_is_pattern_guaranteed_at_step(&self.compiled, byte_offset as u32)
    }
    pub fn capture_index_for_name(&self, name: &str) -> Option<u32> {
        self.capture_names
            .storage
            .iter()
            .position(|n| n.as_ref() == name)
            .map(|ix| ix as u32)
    }
    fn parse_property(
        row: usize,
        function_name: &str,
        capture_names: &[&str],
        string_values: &[&str],
        args: &[PredicateStep],
    ) -> Result<QueryProperty, QueryError> {
        if args.is_empty() || args.len() > 3 {
            return Err(predicate_error(
                row,
                format!(
                    "Wrong number of arguments to {function_name} predicate. Expected 1 to 3, got {}.",
                    args.len(),
                ),
            ));
        }

        let mut capture_id = None;
        let mut key = None;
        let mut value = None;

        for arg in args {
            if arg.kind == PredicateStepKind::Capture {
                if capture_id.is_some() {
                    return Err(predicate_error(
                        row,
                        format!(
                            "Invalid arguments to {function_name} predicate. Unexpected second capture name @{}",
                            capture_names[arg.value_id as usize]
                        ),
                    ));
                }
                capture_id = Some(arg.value_id as usize);
            } else if key.is_none() {
                key = Some(&string_values[arg.value_id as usize]);
            } else if value.is_none() {
                value = Some(string_values[arg.value_id as usize]);
            } else {
                return Err(predicate_error(
                    row,
                    format!(
                        "Invalid arguments to {function_name} predicate. Unexpected third argument @{}",
                        string_values[arg.value_id as usize]
                    ),
                ));
            }
        }

        if let Some(key) = key {
            Ok(QueryProperty::new(key, value, capture_id))
        } else {
            Err(predicate_error(
                row,
                format!("Invalid arguments to {function_name} predicate. Missing key argument",),
            ))
        }
    }
}
impl PartialEq for Query {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(&*self.compiled, &*other.compiled)
    }
}
impl QueryCursor {
    pub fn new() -> Self {
        Self {
            config: ts_query_cursor_new(),
            removals: Mutex::default(),
        }
    }
    pub fn set_match_limit(&mut self, limit: u32) {
        ts_query_cursor_set_match_limit(&mut self.config, limit);
    }
    pub fn match_limit(&self) -> u32 {
        ts_query_cursor_match_limit(&self.config)
    }
    pub fn did_exceed_match_limit(&self) -> bool {
        ts_query_cursor_did_exceed_match_limit(&self.config)
    }
    pub fn set_byte_range(&mut self, range: ops::Range<usize>) -> &mut Self {
        ts_query_cursor_set_byte_range(&mut self.config, range.start as u32, range.end as u32);
        self
    }
    pub fn set_point_range(&mut self, range: ops::Range<Point>) -> &mut Self {
        ts_query_cursor_set_point_range(&mut self.config, range.start.into(), range.end.into());
        self
    }
    pub fn set_max_start_depth(&mut self, max_start_depth: Option<u32>) -> &mut Self {
        ts_query_cursor_set_max_start_depth(&mut self.config, max_start_depth.unwrap_or(u32::MAX));
        self
    }
    #[deprecated(
        since = "0.25.0",
        note = "Use matches_with_options or captures_with_options"
    )]
    pub fn set_timeout_micros(&mut self, timeout: u64) {
        ts_query_cursor_set_timeout_micros(&mut self.config, timeout);
    }
    #[deprecated(
        since = "0.25.0",
        note = "Use matches_with_options or captures_with_options"
    )]
    pub fn timeout_micros(&self) -> u64 {
        ts_query_cursor_timeout_micros(&self.config)
    }
    pub fn matches<'query, 'cursor: 'query, 'tree: 'query, T: TextProvider<I>, I: AsRef<[u8]>>(
        &'cursor mut self,
        query: &'query Query,
        node: Node<'tree>,
        text_provider: T,
    ) -> QueryMatches<'query, 'tree, T, I> {
        self.removals.get_mut().unwrap().clear();
        let execution = ts_query_cursor_exec(&mut self.config, &query.compiled, node);
        QueryMatches {
            execution,
            query,
            text_provider,
            buffer1: Vec::new(),
            buffer2: Vec::new(),
            current_match: None,
            removals: &self.removals,
            _phantom: PhantomData,
        }
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
        self.removals.get_mut().unwrap().clear();
        let execution =
            ts_query_cursor_exec_with_options(&mut self.config, &query.compiled, node, options);
        QueryMatches {
            execution,
            query,
            text_provider,
            buffer1: Vec::new(),
            buffer2: Vec::new(),
            current_match: None,
            removals: &self.removals,
            _phantom: PhantomData,
        }
    }
    pub fn captures<'query, 'cursor: 'query, 'tree: 'query, T: TextProvider<I>, I: AsRef<[u8]>>(
        &'cursor mut self,
        query: &'query Query,
        node: Node<'tree>,
        text_provider: T,
    ) -> QueryCaptures<'query, 'tree, T, I> {
        self.removals.get_mut().unwrap().clear();
        let execution = ts_query_cursor_exec(&mut self.config, &query.compiled, node);
        QueryCaptures {
            execution,
            query,
            text_provider,
            buffer1: Vec::new(),
            buffer2: Vec::new(),
            current_match: None,
            removals: &self.removals,
            _phantom: PhantomData,
        }
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
        self.removals.get_mut().unwrap().clear();
        let execution =
            ts_query_cursor_exec_with_options(&mut self.config, &query.compiled, node, options);
        QueryCaptures {
            execution,
            query,
            text_provider,
            buffer1: Vec::new(),
            buffer2: Vec::new(),
            current_match: None,
            removals: &self.removals,
            _phantom: PhantomData,
        }
    }
}
impl Default for QueryCursor {
    fn default() -> Self {
        Self::new()
    }
}

impl<'cursor, 'tree: 'cursor> QueryMatch<'cursor, 'tree> {
    pub const fn id(&self) -> u32 {
        self.id
    }
    pub fn remove(&self) {
        self.removals.lock().unwrap().push(self.id);
    }
    pub fn nodes_for_capture_index(&self, capture_ix: u32) -> NodesForCaptureIndex<'_, 'tree> {
        NodesForCaptureIndex {
            captures: self.captures.iter(),
            capture_index: capture_ix,
        }
    }
    fn new(result: QueryMatchData<'tree>, removals: &'cursor Mutex<Vec<u32>>) -> Self {
        Self {
            id: result.id,
            pattern_index: result.pattern_index as usize,
            captures: result.captures,
            removals,
        }
    }
    pub fn satisfies_text_predicates<I: AsRef<[u8]>>(
        &self,
        query: &Query,
        buffer1: &mut Vec<u8>,
        buffer2: &mut Vec<u8>,
        text_provider: &mut impl TextProvider<I>,
    ) -> bool {
        let mut node_text1 = NodeText::new(buffer1);
        let mut node_text2 = NodeText::new(buffer2);

        query.text_predicates[self.pattern_index]
            .iter()
            .all(|predicate| match predicate {
                TextPredicateCapture::EqCapture(i, j, is_positive, match_all_nodes) => {
                    let mut nodes_1 = self.nodes_for_capture_index(*i).peekable();
                    let mut nodes_2 = self.nodes_for_capture_index(*j).peekable();
                    while nodes_1.peek().is_some() && nodes_2.peek().is_some() {
                        let node1 = nodes_1.next().unwrap();
                        let node2 = nodes_2.next().unwrap();
                        let mut text1 = text_provider.text(node1);
                        let mut text2 = text_provider.text(node2);
                        let text1 = node_text1.get_text(&mut text1);
                        let text2 = node_text2.get_text(&mut text2);
                        let is_positive_match = text1 == text2;
                        if is_positive_match != *is_positive && *match_all_nodes {
                            return false;
                        }
                        if is_positive_match == *is_positive && !*match_all_nodes {
                            return true;
                        }
                    }
                    nodes_1.next().is_none() && nodes_2.next().is_none()
                }
                TextPredicateCapture::EqString(i, s, is_positive, match_all_nodes) => {
                    let nodes = self.nodes_for_capture_index(*i);
                    for node in nodes {
                        let mut text = text_provider.text(node);
                        let text = node_text1.get_text(&mut text);
                        let is_positive_match = text == s.as_bytes();
                        if is_positive_match != *is_positive && *match_all_nodes {
                            return false;
                        }
                        if is_positive_match == *is_positive && !*match_all_nodes {
                            return true;
                        }
                    }
                    true
                }
                TextPredicateCapture::MatchString(i, r, is_positive, match_all_nodes) => {
                    let nodes = self.nodes_for_capture_index(*i);
                    for node in nodes {
                        let mut text = text_provider.text(node);
                        let text = node_text1.get_text(&mut text);
                        let is_positive_match = r.is_match(text);
                        if is_positive_match != *is_positive && *match_all_nodes {
                            return false;
                        }
                        if is_positive_match == *is_positive && !*match_all_nodes {
                            return true;
                        }
                    }
                    true
                }
                TextPredicateCapture::AnyString(i, v, is_positive) => {
                    let nodes = self.nodes_for_capture_index(*i);
                    for node in nodes {
                        let mut text = text_provider.text(node);
                        let text = node_text1.get_text(&mut text);
                        if (v.iter().any(|s| text == s.as_bytes())) != *is_positive {
                            return false;
                        }
                    }
                    true
                }
            })
    }
}
impl<'tree> Iterator for NodesForCaptureIndex<'_, 'tree> {
    type Item = Node<'tree>;
    fn next(&mut self) -> Option<Self::Item> {
        self.captures
            .find_map(|capture| (capture.index == self.capture_index).then_some(capture.node))
    }
}
impl fmt::Debug for QueryMatch<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "QueryMatch {{ id: {}, pattern_index: {}, captures: {:?} }}",
            self.id, self.pattern_index, self.captures
        )
    }
}
impl QueryProperty {
    pub fn new(key: &str, value: Option<&str>, capture_id: Option<usize>) -> Self {
        Self {
            capture_id,
            key: key.into(),
            value: value.map(Into::into),
        }
    }
}
impl QueryCursorState {
    pub const fn current_byte_offset(&self) -> usize {
        self.current_byte_offset as usize
    }
}
impl<'a> QueryCursorOptions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn progress_callback<F: FnMut(&QueryCursorState) -> bool>(
        self,
        callback: &'a mut F,
    ) -> Self {
        Self {
            progress_callback: Some(callback),
        }
    }
}

impl<'query, 'tree: 'query, T: TextProvider<I>, I: AsRef<[u8]>> StreamingIterator
    for QueryMatches<'query, 'tree, T, I>
{
    type Item = QueryMatch<'query, 'tree>;
    fn advance(&mut self) {
        self.current_match = loop {
            drain_removals(&mut self.execution, self.removals);
            let Some(result) = ts_query_cursor_next_match(&mut self.execution) else {
                break None;
            };
            let candidate = QueryMatch::new(result, self.removals);
            if candidate.satisfies_text_predicates(
                self.query,
                &mut self.buffer1,
                &mut self.buffer2,
                &mut self.text_provider,
            ) {
                break Some(candidate);
            }
        };
    }
    fn get(&self) -> Option<&Self::Item> {
        self.current_match.as_ref()
    }
}
impl<'query, 'tree: 'query, T: TextProvider<I>, I: AsRef<[u8]>> StreamingIteratorMut
    for QueryMatches<'query, 'tree, T, I>
{
    fn get_mut(&mut self) -> Option<&mut Self::Item> {
        self.current_match.as_mut()
    }
}
impl<'query, 'tree: 'query, T: TextProvider<I>, I: AsRef<[u8]>> StreamingIterator
    for QueryCaptures<'query, 'tree, T, I>
{
    type Item = (QueryMatch<'query, 'tree>, usize);
    fn advance(&mut self) {
        self.current_match = loop {
            drain_removals(&mut self.execution, self.removals);
            let Some((result, capture_index)) = ts_query_cursor_next_capture(&mut self.execution)
            else {
                break None;
            };
            let candidate = QueryMatch::new(result, self.removals);
            if candidate.satisfies_text_predicates(
                self.query,
                &mut self.buffer1,
                &mut self.buffer2,
                &mut self.text_provider,
            ) {
                break Some((candidate, capture_index as usize));
            }
            candidate.remove();
        };
    }
    fn get(&self) -> Option<&Self::Item> {
        self.current_match.as_ref()
    }
}
impl<'query, 'tree: 'query, T: TextProvider<I>, I: AsRef<[u8]>> StreamingIteratorMut
    for QueryCaptures<'query, 'tree, T, I>
{
    fn get_mut(&mut self) -> Option<&mut Self::Item> {
        self.current_match.as_mut()
    }
}
impl<'query, 'tree: 'query, T: TextProvider<I>, I: AsRef<[u8]>> QueryMatches<'query, 'tree, T, I> {
    /// Advance and return an owned match, allowing captures to be iterated by
    /// value (`for capture in found.captures`) without moving out of a shared
    /// streaming reference. The current match is also retained for `get` and
    /// `get_mut`, just as with the borrowed streaming-trait API.
    ///
    /// Use [`StreamingIterator::next`] explicitly to borrow the current result
    /// instead, avoiding this convenience method's capture-vector clone.
    #[allow(clippy::should_implement_trait)] // The borrowed StreamingIterator API is also implemented.
    pub fn next(&mut self) -> Option<QueryMatch<'query, 'tree>> {
        StreamingIterator::next(self).cloned()
    }

    pub fn set_byte_range(&mut self, range: ops::Range<usize>) {
        ts_query_cursor_set_byte_range(self.execution.config, range.start as u32, range.end as u32);
    }
    pub fn set_point_range(&mut self, range: ops::Range<Point>) {
        ts_query_cursor_set_point_range(
            self.execution.config,
            range.start.into(),
            range.end.into(),
        );
    }
}
impl<T: TextProvider<I>, I: AsRef<[u8]>> QueryCaptures<'_, '_, T, I> {
    pub fn set_byte_range(&mut self, range: ops::Range<usize>) {
        ts_query_cursor_set_byte_range(self.execution.config, range.start as u32, range.end as u32);
    }
    pub fn set_point_range(&mut self, range: ops::Range<Point>) {
        ts_query_cursor_set_point_range(
            self.execution.config,
            range.start.into(),
            range.end.into(),
        );
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
        (self)(node)
    }
}
impl<'a> TextProvider<&'a [u8]> for &'a [u8] {
    type I = iter::Once<&'a [u8]>;
    fn text(&mut self, node: Node<'_>) -> Self::I {
        iter::once(&self[node.byte_range()])
    }
}
impl fmt::Display for QueryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self.kind {
            QueryErrorKind::Field => "Invalid field name ",
            QueryErrorKind::NodeType => "Invalid node type ",
            QueryErrorKind::Capture => "Invalid capture name ",
            QueryErrorKind::Predicate => "Invalid predicate: ",
            QueryErrorKind::Structure => "Impossible pattern:\n",
            QueryErrorKind::Syntax => "Invalid syntax:\n",
            QueryErrorKind::Language => "",
        };
        if msg.is_empty() {
            write!(f, "{}", self.message)
        } else {
            write!(
                f,
                "Query error at {}:{}. {}{}",
                self.row + 1,
                self.column + 1,
                msg,
                self.message
            )
        }
    }
}
impl std::error::Error for QueryError {}
const fn predicate_error(row: usize, message: String) -> QueryError {
    QueryError {
        kind: QueryErrorKind::Predicate,
        row,
        column: 0,
        offset: 0,
        message,
    }
}

/// Text chunks are concatenated only when a node spans multiple provider chunks.
struct NodeText<'a, T> {
    buffer: &'a mut Vec<u8>,
    first_chunk: Option<T>,
}
impl<'a, T: AsRef<[u8]>> NodeText<'a, T> {
    fn new(buffer: &'a mut Vec<u8>) -> Self {
        Self {
            buffer,
            first_chunk: None,
        }
    }
    fn get_text(&mut self, chunks: &mut impl Iterator<Item = T>) -> &[u8] {
        self.first_chunk = chunks.next();
        if let Some(next_chunk) = chunks.next() {
            self.buffer.clear();
            self.buffer
                .extend_from_slice(self.first_chunk.as_ref().unwrap().as_ref());
            self.buffer.extend_from_slice(next_chunk.as_ref());
            for chunk in chunks {
                self.buffer.extend_from_slice(chunk.as_ref());
            }
            self.buffer.as_slice()
        } else if let Some(ref first_chunk) = self.first_chunk {
            first_chunk.as_ref()
        } else {
            &[]
        }
    }
}

fn compile_error(language: &Language, source: &str, error: QueryCompileError) -> QueryError {
    if error.kind == QueryErrorCode::Language {
        return QueryError {
            row: 0,
            column: 0,
            offset: 0,
            message: crate::LanguageError {
                version: language.abi_version(),
            }
            .to_string(),
            kind: QueryErrorKind::Language,
        };
    }

    let offset = error.offset as usize;
    let mut line_start = 0;
    let mut row = 0;
    let mut line_containing_error = None;
    for line in source.lines() {
        let line_end = line_start + line.len() + 1;
        if line_end > offset {
            line_containing_error = Some(line);
            break;
        }
        line_start = line_end;
        row += 1;
    }
    let column = offset - line_start;

    let (message, kind) = match error.kind {
        // Error types that report names
        QueryErrorCode::NodeType | QueryErrorCode::Field | QueryErrorCode::Capture => {
            let suffix = source.split_at(offset).1;
            let in_quotes = offset > 0 && source.as_bytes()[offset - 1] == b'"';
            let mut backslashes = 0;
            let end_offset = suffix
                .find(|c| {
                    if in_quotes {
                        if c == '"' && backslashes % 2 == 0 {
                            true
                        } else if c == '\\' {
                            backslashes += 1;
                            false
                        } else {
                            backslashes = 0;
                            false
                        }
                    } else {
                        !char::is_alphanumeric(c) && c != '_' && c != '-'
                    }
                })
                .unwrap_or(suffix.len());
            let message = suffix.split_at(end_offset).0.to_string();
            let kind = match error.kind {
                QueryErrorCode::NodeType => QueryErrorKind::NodeType,
                QueryErrorCode::Field => QueryErrorKind::Field,
                QueryErrorCode::Capture => QueryErrorKind::Capture,
                _ => unreachable!(),
            };
            (message, kind)
        }

        // Error types that report positions
        _ => {
            let message = line_containing_error.map_or_else(
                || "Unexpected EOF".to_string(),
                |line| line.to_string() + "\n" + &" ".repeat(offset - line_start) + "^",
            );
            let kind = match error.kind {
                QueryErrorCode::Structure => QueryErrorKind::Structure,
                _ => QueryErrorKind::Syntax,
            };
            (message, kind)
        }
    };

    QueryError {
        row,
        column,
        offset,
        message,
        kind,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_chunks_are_lazy_and_concatenated_only_when_needed() {
        let mut buffer = b"unused".to_vec();
        {
            let mut text = NodeText::new(&mut buffer);
            assert_eq!(text.get_text(&mut iter::empty::<&[u8]>()), b"");
            let bytes = b"single";
            let actual = text.get_text(&mut iter::once(bytes.as_slice()));
            assert_eq!(actual.as_ptr(), bytes.as_ptr());
        }
        assert_eq!(buffer, b"unused");
        let mut text = NodeText::new(&mut buffer);
        assert_eq!(
            text.get_text(&mut [b"a".as_slice(), b"", b"bc"].into_iter()),
            b"abc"
        );
        assert_eq!(
            text.get_text(&mut [b"d".as_slice(), b"e"].into_iter()),
            b"de"
        );
    }

    #[test]
    fn properties_accept_capture_in_any_position() {
        let capture = PredicateStep {
            kind: PredicateStepKind::Capture,
            value_id: 0,
        };
        let key = PredicateStep {
            kind: PredicateStepKind::String,
            value_id: 0,
        };
        let value = PredicateStep {
            kind: PredicateStepKind::String,
            value_id: 1,
        };
        for args in [
            [capture, key, value],
            [key, capture, value],
            [key, value, capture],
        ] {
            assert_eq!(
                Query::parse_property(3, "set!", &["node"], &["key", "value"], &args).unwrap(),
                QueryProperty::new("key", Some("value"), Some(0))
            );
        }
    }

    #[test]
    fn property_errors_preserve_binding_diagnostics() {
        let capture = PredicateStep {
            kind: PredicateStepKind::Capture,
            value_id: 0,
        };
        let error = Query::parse_property(3, "is?", &["node"], &[], &[capture]).unwrap_err();
        assert_eq!(error.kind, QueryErrorKind::Predicate);
        assert_eq!((error.row, error.column, error.offset), (3, 0, 0));
        assert_eq!(
            error.to_string(),
            "Query error at 4:1. Invalid predicate: Invalid arguments to is? predicate. Missing key argument"
        );
        let error =
            Query::parse_property(0, "set!", &["node"], &[], &[capture, capture]).unwrap_err();
        assert_eq!(
            error.message,
            "Invalid arguments to set! predicate. Unexpected second capture name @node"
        );
    }

    #[test]
    fn names_remain_owned_across_moves() {
        let names = CaptureNames::new(vec!["one".into(), "two".into()]);
        let moved = Box::new(names);
        assert_eq!(
            moved
                .storage
                .iter()
                .map(AsRef::as_ref)
                .collect::<Vec<&str>>(),
            ["one", "two"]
        );
    }

    #[test]
    fn language_error_has_no_query_position_prefix() {
        let error = QueryError {
            row: 3,
            column: 4,
            offset: 8,
            message: "version error".into(),
            kind: QueryErrorKind::Language,
        };
        assert_eq!(error.to_string(), "version error");
    }
}

#[cfg(test)]
#[path = "query_api_tests.rs"]
mod predicate_tests;
