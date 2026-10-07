use crate::{length::*, types::*, unicode::*};
use std::fmt::Write;

const BYTE_ORDER_MARK: i32 = 0xfeff;
const DEFAULT_RANGE: Range = Range {
    start_point: crate::point::POINT_ZERO,
    end_point: crate::point::POINT_MAX,
    start_byte: 0,
    end_byte: u32::MAX,
};

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct ColumnData {
    pub value: u32,
    pub valid: bool,
}
#[derive(Debug, Default)]
pub(crate) struct LexerState {
    pub lookahead: i32,
    pub result_symbol: Symbol,
    pub current_position: Length,
    pub token_start_position: Length,
    pub token_end_position: Length,
    pub included_ranges: Vec<Range>,
    pub encoding: InputEncoding,
    pub current_included_range_index: u32,
    // End of the current nonempty range, or zero at EOF. Avoid looking up the
    // range in its Vec on every character; refresh only on jumps and reads.
    current_range_end: u32,
    pub chunk_start: u32,
    pub chunk_size: u32,
    pub lookahead_size: u32,
    pub did_get_column: bool,
    pub column_data: ColumnData,
    pub debug_buffer: String,
}
/// The generated lexers see this transient adapter. No client callback or chunk
/// reference is ever stored in the persistent parser.
pub(crate) struct Lexer<'a> {
    pub state: &'a mut LexerState,
    pub input: &'a mut dyn Input,
    pub logger: Option<&'a mut crate::parser::Logger>,
}
impl ts_port_tables::Lexer for Lexer<'_> {
    fn lookahead(&self) -> i32 {
        self.state.lookahead
    }
    fn result_symbol(&self) -> Symbol {
        self.state.result_symbol
    }
    fn set_result_symbol(&mut self, symbol: Symbol) {
        self.state.result_symbol = symbol;
    }
    fn advance(&mut self, skip: bool) {
        ts_lexer__advance(self, skip);
    }
    fn mark_end(&mut self) {
        ts_lexer_mark_end(self.state);
    }
    fn get_column(&mut self) -> u32 {
        ts_lexer__get_column(self)
    }
    fn is_at_included_range_start(&self) -> bool {
        ts_lexer__is_at_included_range_start(self.state)
    }
    fn eof(&self) -> bool {
        ts_lexer__eof(self.state)
    }
    fn log(&mut self, args: std::fmt::Arguments<'_>) {
        ts_lexer__log(self, args);
    }
}

pub(crate) fn ts_lexer__set_column_data(lexer: &mut LexerState, value: u32) {
    lexer.column_data.valid = true;
    lexer.column_data.value = value;
}

pub(crate) fn ts_lexer__increment_column_data(lexer: &mut LexerState) {
    if lexer.column_data.valid {
        lexer.column_data.value = lexer.column_data.value.wrapping_add(1);
    }
}

pub(crate) fn ts_lexer__invalidate_column_data(lexer: &mut LexerState) {
    lexer.column_data.valid = false;
    lexer.column_data.value = 0;
}

pub(crate) fn ts_lexer__eof(lexer: &LexerState) -> bool {
    lexer.current_included_range_index == lexer.included_ranges.len() as u32
}

pub(crate) fn ts_lexer__clear_chunk(lexer: &mut LexerState) {
    // A zero size also represents C's null chunk pointer. The provider retains
    // ownership of its old chunk, but no lexer operation uses it until read.
    lexer.chunk_size = 0;
    lexer.chunk_start = 0;
}

#[inline(never)]
pub(crate) fn ts_lexer__get_chunk(lexer: &mut Lexer<'_>) {
    lexer.state.chunk_start = lexer.state.current_position.bytes;
    lexer.input.read(
        lexer.state.current_position.bytes,
        lexer.state.current_position.extent,
    );
    lexer.state.chunk_size = lexer.input.chunk().len() as u32;
    if lexer.state.chunk_size == 0 {
        lexer.state.current_included_range_index = lexer.state.included_ranges.len() as u32;
        lexer.state.current_range_end = 0;
    }
}

#[inline(always)]
pub(crate) fn ts_lexer__get_lookahead(lexer: &mut Lexer<'_>) {
    let position_in_chunk = lexer
        .state
        .current_position
        .bytes
        .wrapping_sub(lexer.state.chunk_start);
    let size = lexer.state.chunk_size.wrapping_sub(position_in_chunk);
    if size == 0 {
        lexer.state.lookahead_size = 1;
        lexer.state.lookahead = 0;
        return;
    }

    let chunk =
        &lexer.input.chunk()[position_in_chunk as usize..lexer.state.chunk_size as usize];
    if lexer.state.encoding == InputEncoding::Utf8 && chunk[0].is_ascii() {
        lexer.state.lookahead_size = 1;
        lexer.state.lookahead = i32::from(chunk[0]);
        return;
    }

    (lexer.state.lookahead_size, lexer.state.lookahead) =
        decode_lookahead(lexer.state.encoding, chunk);
    if lexer.state.lookahead == DECODE_ERROR {
        finish_lookahead_error(lexer, size);
    }
}

// Keep Unicode validation and chunk-boundary retries out of the ASCII path.
// Direct calls avoid selecting and calling a decoder function pointer.
#[inline(never)]
fn decode_lookahead(encoding: InputEncoding, chunk: &[u8]) -> (u32, i32) {
    match encoding {
        InputEncoding::Utf8 => ts_decode_utf8(chunk),
        InputEncoding::Utf16Le => ts_decode_utf16_le(chunk),
        InputEncoding::Utf16Be => ts_decode_utf16_be(chunk),
    }
}

#[cold]
#[inline(never)]
fn finish_lookahead_error(lexer: &mut Lexer<'_>, size: u32) {
    // Retry every decoding error near a chunk boundary, not just incomplete
    // sequences: the read callback may supply more bytes at this position.
    if size < 4 {
        ts_lexer__get_chunk(lexer);
        (lexer.state.lookahead_size, lexer.state.lookahead) = decode_lookahead(
            lexer.state.encoding,
            &lexer.input.chunk()[..lexer.state.chunk_size as usize],
        );
    }
    if lexer.state.lookahead == DECODE_ERROR {
        lexer.state.lookahead_size = 1;
    }
}

pub(crate) fn ts_lexer_goto(lexer: &mut LexerState, position: Length) {
    if position.bytes != lexer.current_position.bytes {
        ts_lexer__invalidate_column_data(lexer);
    }
    lexer.current_position = position;

    let mut found_included_range = false;
    for (i, range) in lexer.included_ranges.iter().enumerate() {
        if range.end_byte > lexer.current_position.bytes && range.end_byte > range.start_byte {
            if range.start_byte >= lexer.current_position.bytes {
                lexer.current_position = Length {
                    bytes: range.start_byte,
                    extent: range.start_point,
                };
            }
            lexer.current_included_range_index = i as u32;
            lexer.current_range_end = range.end_byte;
            found_included_range = true;
            break;
        }
    }

    if found_included_range {
        if lexer.chunk_size != 0
            && (lexer.current_position.bytes < lexer.chunk_start
                || lexer.current_position.bytes >= lexer.chunk_start.wrapping_add(lexer.chunk_size))
        {
            ts_lexer__clear_chunk(lexer);
        }
        lexer.lookahead_size = 0;
        lexer.lookahead = 0;
    } else {
        lexer.current_included_range_index = lexer.included_ranges.len() as u32;
        lexer.current_range_end = 0;
        // Initialization and set_included_ranges always install at least one
        // range, even when the caller supplies an empty list.
        let last = lexer
            .included_ranges
            .last()
            .expect("lexer has an included range");
        lexer.current_position = Length {
            bytes: last.end_byte,
            extent: last.end_point,
        };
        ts_lexer__clear_chunk(lexer);
        lexer.lookahead_size = 1;
        lexer.lookahead = 0;
    }
}

#[inline(always)]
pub(crate) fn ts_lexer__do_advance(lexer: &mut Lexer<'_>, skip: bool) {
    let state = &mut *lexer.state;
    if state.lookahead_size != 0 {
        if state.lookahead == i32::from(b'\n') {
            state.current_position.extent.row = state.current_position.extent.row.wrapping_add(1);
            state.current_position.extent.column = 0;
            ts_lexer__set_column_data(state, 0);
        } else {
            let is_bom = state.current_position.bytes == 0 && state.lookahead == BYTE_ORDER_MARK;
            if !is_bom {
                ts_lexer__increment_column_data(state);
            }
            state.current_position.extent.column = state
                .current_position
                .extent
                .column
                .wrapping_add(state.lookahead_size);
        }
        state.current_position.bytes = state
            .current_position
            .bytes
            .wrapping_add(state.lookahead_size);
    }

    if state.current_position.bytes >= state.current_range_end && !advance_range(state) {
        if skip {
            state.token_start_position = state.current_position;
        }
        ts_lexer__clear_chunk(state);
        state.lookahead = 0;
        state.lookahead_size = 1;
        return;
    }

    if skip {
        state.token_start_position = state.current_position;
    }
    if state.current_position.bytes < state.chunk_start
        || state.current_position.bytes >= state.chunk_start.wrapping_add(state.chunk_size)
    {
        ts_lexer__get_chunk(lexer);
    }
    ts_lexer__get_lookahead(lexer);
}

// A range transition is rare compared with advancing within a range. Keep the
// C loop (including empty ranges and their end positions) on this slow path.
#[inline(never)]
fn advance_range(state: &mut LexerState) -> bool {
    while let Some(range) = state
        .included_ranges
        .get(state.current_included_range_index as usize)
    {
        if state.current_position.bytes < range.end_byte && range.end_byte != range.start_byte {
            state.current_range_end = range.end_byte;
            return true;
        }
        state.current_included_range_index += 1;
        if let Some(next) = state
            .included_ranges
            .get(state.current_included_range_index as usize)
        {
            state.current_position = Length {
                bytes: next.start_byte,
                extent: next.start_point,
            };
        }
    }
    state.current_range_end = 0;
    false
}

#[inline]
pub(crate) fn ts_lexer__advance(lexer: &mut Lexer<'_>, skip: bool) {
    if lexer.state.chunk_size == 0 {
        return;
    }
    if lexer.logger.is_some() {
        log_advance(lexer, skip);
    }
    ts_lexer__do_advance(lexer, skip);
}

// Keep formatting and logger dispatch out of the per-character hot path.
#[cold]
#[inline(never)]
fn log_advance(lexer: &mut Lexer<'_>, skip: bool) {
    let message = if skip { "skip" } else { "consume" };
    let character = lexer.state.lookahead;
    if (32..127).contains(&character) {
        ts_lexer__log(
            lexer,
            format_args!("{message} character:'{}'", character as u8 as char),
        );
    } else {
        ts_lexer__log(lexer, format_args!("{message} character:{character}"));
    }
}

pub(crate) fn ts_lexer__mark_end(lexer: &mut LexerState) {
    if !ts_lexer__eof(lexer) {
        let index = lexer.current_included_range_index as usize;
        if index > 0 && lexer.current_position.bytes == lexer.included_ranges[index].start_byte {
            let previous = &lexer.included_ranges[index - 1];
            lexer.token_end_position = Length {
                bytes: previous.end_byte,
                extent: previous.end_point,
            };
            return;
        }
    }
    lexer.token_end_position = lexer.current_position;
}

pub(crate) fn ts_lexer__get_column(lexer: &mut Lexer<'_>) -> u32 {
    lexer.state.did_get_column = true;
    if !lexer.state.column_data.valid {
        let goal_byte = lexer.state.current_position.bytes;
        let start_of_col = Length {
            bytes: goal_byte.wrapping_sub(lexer.state.current_position.extent.column),
            extent: Point {
                row: lexer.state.current_position.extent.row,
                column: 0,
            },
        };
        ts_lexer_goto(lexer.state, start_of_col);
        ts_lexer__set_column_data(lexer.state, 0);
        ts_lexer__get_chunk(lexer);
        if !ts_lexer__eof(lexer.state) {
            ts_lexer__get_lookahead(lexer);
            while lexer.state.current_position.bytes < goal_byte
                && !ts_lexer__eof(lexer.state)
                && lexer.state.chunk_size != 0
            {
                ts_lexer__do_advance(lexer, false);
                if ts_lexer__eof(lexer.state) {
                    break;
                }
            }
        }
    }
    lexer.state.column_data.value
}

pub(crate) fn ts_lexer__is_at_included_range_start(lexer: &LexerState) -> bool {
    lexer
        .included_ranges
        .get(lexer.current_included_range_index as usize)
        .is_some_and(|range| lexer.current_position.bytes == range.start_byte)
}

pub(crate) fn ts_lexer__log(lexer: &mut Lexer<'_>, args: std::fmt::Arguments<'_>) {
    if let Some(logger) = &mut lexer.logger {
        let buffer = &mut lexer.state.debug_buffer;
        buffer.clear();
        buffer.write_fmt(args).expect("formatting into a String");
        let mut end = buffer
            .len()
            .min(ts_port_tables::SERIALIZATION_BUFFER_SIZE - 1);
        // C's fixed buffer reserves a byte for NUL. The Rust logger takes &str,
        // so keep a complete UTF-8 prefix if truncation splits a code point.
        while !buffer.is_char_boundary(end) {
            end -= 1;
        }
        if let Some(nul) = buffer[..end].find('\0') {
            end = nul;
        }
        buffer.truncate(end);
        logger(crate::api::LogType::Lex, buffer);
    }
}

pub(crate) fn ts_lexer_init() -> LexerState {
    let mut lexer = LexerState::default();
    ts_lexer_set_included_ranges(&mut lexer, &[]);
    lexer
}

pub(crate) fn ts_lexer_delete(lexer: &mut LexerState) {
    lexer.included_ranges = Vec::new();
}

pub(crate) fn ts_lexer_set_input(lexer: &mut LexerState, encoding: InputEncoding) {
    lexer.encoding = encoding;
    ts_lexer__clear_chunk(lexer);
    ts_lexer_goto(lexer, lexer.current_position);
}

pub(crate) fn ts_lexer_reset(lexer: &mut LexerState, position: Length) {
    if position.bytes != lexer.current_position.bytes {
        ts_lexer_goto(lexer, position);
    }
}

pub(crate) fn ts_lexer_start(lexer: &mut Lexer<'_>) {
    lexer.state.token_start_position = lexer.state.current_position;
    lexer.state.token_end_position = LENGTH_UNDEFINED;
    lexer.state.result_symbol = 0;
    lexer.state.did_get_column = false;
    if !ts_lexer__eof(lexer.state) {
        if lexer.state.chunk_size == 0 {
            ts_lexer__get_chunk(lexer);
        }
        if lexer.state.lookahead_size == 0 {
            ts_lexer__get_lookahead(lexer);
        }
        if lexer.state.current_position.bytes == 0 {
            if lexer.state.lookahead == BYTE_ORDER_MARK {
                ts_lexer__advance(lexer, true);
            }
            ts_lexer__set_column_data(lexer.state, 0);
        }
    }
}

pub(crate) fn ts_lexer_finish(lexer: &mut LexerState, lookahead_end_byte: &mut u32) {
    if length_is_undefined(lexer.token_end_position) {
        ts_lexer__mark_end(lexer);
    }
    if lexer.token_end_position.bytes < lexer.token_start_position.bytes {
        lexer.token_start_position = lexer.token_end_position;
    }

    let mut current_lookahead_end_byte = lexer.current_position.bytes.wrapping_add(1);
    if lexer.lookahead == DECODE_ERROR {
        // Invalid decoding may have inspected up to four following bytes.
        current_lookahead_end_byte = current_lookahead_end_byte.wrapping_add(4);
    }
    if current_lookahead_end_byte > *lookahead_end_byte {
        *lookahead_end_byte = current_lookahead_end_byte;
    }
}

pub(crate) fn ts_lexer_mark_end(lexer: &mut LexerState) {
    ts_lexer__mark_end(lexer);
}

pub(crate) fn ts_lexer_set_included_ranges(lexer: &mut LexerState, ranges: &[Range]) -> bool {
    let ranges = if ranges.is_empty() {
        std::slice::from_ref(&DEFAULT_RANGE)
    } else {
        let mut previous_byte = 0;
        for range in ranges {
            if range.start_byte < previous_byte || range.end_byte < range.start_byte {
                return false;
            }
            previous_byte = range.end_byte;
        }
        ranges
    };
    lexer.included_ranges.clear();
    lexer.included_ranges.extend_from_slice(ranges);
    ts_lexer_goto(lexer, lexer.current_position);
    true
}

pub(crate) fn ts_lexer_included_ranges(lexer: &LexerState) -> &[Range] {
    &lexer.included_ranges
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::point::point_new;
    use std::collections::VecDeque;
    use ts_port_tables::Lexer as _;

    struct RecordingInput<'a> {
        source: &'a [u8],
        chunk: &'a [u8],
        read_sizes: VecDeque<usize>,
        reads: Vec<(u32, Point)>,
    }

    impl<'a> RecordingInput<'a> {
        fn new(source: &'a [u8], read_sizes: &[usize]) -> Self {
            Self {
                source,
                chunk: &[],
                read_sizes: read_sizes.iter().copied().collect(),
                reads: Vec::new(),
            }
        }
    }

    impl Input for RecordingInput<'_> {
        fn read(&mut self, byte: u32, point: Point) {
            self.reads.push((byte, point));
            let tail = &self.source[(byte as usize).min(self.source.len())..];
            let size = self
                .read_sizes
                .pop_front()
                .unwrap_or(tail.len())
                .min(tail.len());
            self.chunk = &tail[..size];
        }
        fn chunk(&self) -> &[u8] {
            self.chunk
        }
    }

    fn position(bytes: u32, row: u32, column: u32) -> Length {
        Length {
            bytes,
            extent: point_new(row, column),
        }
    }

    fn range(start_byte: u32, end_byte: u32, row: u32) -> Range {
        Range {
            start_byte,
            end_byte,
            start_point: point_new(row, 0),
            end_point: point_new(row, end_byte - start_byte),
        }
    }

    #[test]
    fn bom_columns_byte_points_and_embedded_nul() {
        let mut state = ts_lexer_init();
        let mut input = RecordingInput::new("\u{feff}aé\n\t\0β".as_bytes(), &[]);
        let mut lexer = Lexer {
            state: &mut state,
            input: &mut input,
            logger: None,
        };
        ts_lexer_start(&mut lexer);
        assert_eq!(lexer.state.current_position, position(3, 0, 3));
        assert_eq!(lexer.state.token_start_position, position(3, 0, 3));
        assert_eq!(lexer.lookahead(), 'a' as i32);
        assert_eq!(lexer.get_column(), 0);
        lexer.advance(false);
        assert_eq!(lexer.state.current_position, position(4, 0, 4));
        assert_eq!(lexer.get_column(), 1);
        lexer.advance(false);
        assert_eq!(lexer.state.current_position, position(6, 0, 6));
        assert_eq!(lexer.get_column(), 2);
        lexer.advance(false);
        assert_eq!(lexer.state.current_position, position(7, 1, 0));
        assert_eq!(lexer.get_column(), 0);
        lexer.advance(true);
        assert_eq!(lexer.state.token_start_position, position(8, 1, 1));
        assert_eq!(lexer.lookahead(), 0);
        assert!(!lexer.eof());
        assert_eq!(lexer.get_column(), 1);
        lexer.advance(false);
        assert_eq!(lexer.lookahead(), 'β' as i32);
        assert_eq!(lexer.get_column(), 2);
        lexer.advance(false);
        assert!(lexer.eof());
        assert_eq!(lexer.lookahead(), 0);
        assert_eq!(lexer.get_column(), 3);
        assert_eq!(lexer.state.current_position, position(11, 1, 4));
        lexer.advance(true);
        assert_eq!(lexer.state.current_position, position(11, 1, 4));
        assert_eq!(lexer.state.token_start_position, position(8, 1, 1));
        assert_eq!(input.reads, [(0, point_new(0, 0)), (11, point_new(1, 4))]);
    }

    #[test]
    fn partial_character_retries_at_current_position() {
        let mut state = ts_lexer_init();
        let mut input = RecordingInput::new("a€b".as_bytes(), &[2, 3, 1]);
        let mut lexer = Lexer {
            state: &mut state,
            input: &mut input,
            logger: None,
        };
        ts_lexer_start(&mut lexer);
        assert_eq!(lexer.lookahead(), 'a' as i32);
        lexer.advance(false);
        assert_eq!(lexer.lookahead(), '€' as i32);
        assert_eq!(lexer.state.lookahead_size, 3);
        assert_eq!(lexer.state.current_position, position(1, 0, 1));
        lexer.advance(false);
        assert_eq!(lexer.lookahead(), 'b' as i32);
        assert_eq!(lexer.get_column(), 2);
        lexer.advance(false);
        assert!(lexer.eof());
        assert_eq!(
            input.reads,
            [0, 1, 4, 5].map(|byte| (byte, point_new(0, byte)))
        );
    }

    #[test]
    fn invalid_characters_advance_one_byte_and_extend_lookahead() {
        let mut state = ts_lexer_init();
        let mut input = RecordingInput::new(b"\xe2\x82x\xffz", &[]);
        let mut lexer = Lexer {
            state: &mut state,
            input: &mut input,
            logger: None,
        };
        ts_lexer_start(&mut lexer);
        let mut lookahead_end = 0;
        for (byte, lookahead) in [
            DECODE_ERROR,
            DECODE_ERROR,
            'x' as i32,
            DECODE_ERROR,
            'z' as i32,
        ]
        .into_iter()
        .enumerate()
        {
            assert_eq!(lexer.state.current_position.bytes, byte as u32);
            assert_eq!(lexer.lookahead(), lookahead);
            assert_eq!(lexer.state.lookahead_size, 1);
            ts_lexer_finish(lexer.state, &mut lookahead_end);
            if byte == 0 {
                assert_eq!(lookahead_end, 5);
            }
            lexer.advance(false);
        }
        assert_eq!(lookahead_end, 8);
        ts_lexer_finish(lexer.state, &mut lookahead_end);
        assert_eq!(lookahead_end, 8);
        assert!(lexer.eof());
        assert_eq!(
            input.reads,
            [0, 3, 5].map(|byte| (byte, point_new(0, byte)))
        );
    }

    #[test]
    fn empty_input_starts_and_finishes_at_zero() {
        let mut state = ts_lexer_init();
        let mut input = RecordingInput::new(b"", &[]);
        let mut lexer = Lexer {
            state: &mut state,
            input: &mut input,
            logger: None,
        };
        ts_lexer_start(&mut lexer);
        assert!(lexer.eof());
        assert_eq!(lexer.lookahead(), 0);
        assert_eq!(lexer.get_column(), 0);
        lexer.advance(false);
        let mut lookahead_end = 0;
        ts_lexer_finish(lexer.state, &mut lookahead_end);
        assert_eq!(lookahead_end, 1);
        assert_eq!(lexer.state.token_end_position, length_zero());
        assert_eq!(input.reads, [(0, point_new(0, 0))]);
    }

    #[test]
    fn lazy_column_replays_the_line_without_changing_token_start() {
        let mut state = ts_lexer_init();
        let mut input = RecordingInput::new("\u{feff}aé𐐀\nxy".as_bytes(), &[]);
        ts_lexer_reset(&mut state, position(10, 0, 10));
        let mut lexer = Lexer {
            state: &mut state,
            input: &mut input,
            logger: None,
        };
        ts_lexer_start(&mut lexer);
        assert!(!lexer.state.column_data.valid);
        assert_eq!(lexer.get_column(), 3);
        assert!(lexer.state.did_get_column);
        assert_eq!(lexer.state.current_position, position(10, 0, 10));
        assert_eq!(lexer.state.token_start_position, position(10, 0, 10));
        assert_eq!(lexer.lookahead(), '\n' as i32);
        // Reset compares bytes alone, retaining lookahead, point and column.
        ts_lexer_reset(lexer.state, position(10, 99, 99));
        assert_eq!(lexer.state.current_position, position(10, 0, 10));
        assert_eq!(lexer.get_column(), 3);
        ts_lexer_reset(lexer.state, position(13, 1, 2));
        ts_lexer_start(&mut lexer);
        assert!(lexer.eof());
        assert!(!lexer.state.column_data.valid);
        assert_eq!(lexer.get_column(), 2);
        assert!(lexer.eof());
        assert_eq!(lexer.state.current_position, position(13, 1, 2));
        assert_eq!(
            input.reads,
            [
                (10, point_new(0, 10)),
                (0, point_new(0, 0)),
                (13, point_new(1, 2)),
                (11, point_new(1, 0)),
                (13, point_new(1, 2)),
            ]
        );
    }

    #[test]
    fn included_range_boundaries_relocate_token_ends_and_empty_tokens() {
        let mut state = ts_lexer_init();
        assert!(ts_lexer_set_included_ranges(
            &mut state,
            &[range(0, 2, 0), range(4, 6, 1), range(9, 11, 3),]
        ));
        let mut input = RecordingInput::new(b"ab--cd---ef.", &[]);
        let mut lexer = Lexer {
            state: &mut state,
            input: &mut input,
            logger: None,
        };
        ts_lexer_start(&mut lexer);
        assert!(lexer.is_at_included_range_start());
        lexer.advance(false);
        assert!(!lexer.is_at_included_range_start());
        lexer.advance(false);
        assert_eq!(lexer.lookahead(), 'c' as i32);
        assert!(lexer.is_at_included_range_start());
        assert_eq!(lexer.state.current_position, position(4, 1, 0));
        // Crossing ranges does not invalidate or reset the cached column.
        assert_eq!(lexer.get_column(), 2);
        lexer.mark_end();
        assert_eq!(lexer.state.token_end_position, position(2, 0, 2));
        ts_lexer_start(&mut lexer);
        assert!(!lexer.state.did_get_column);
        let mut lookahead_end = 0;
        ts_lexer_finish(lexer.state, &mut lookahead_end);
        assert_eq!(lexer.state.token_start_position, position(2, 0, 2));
        assert_eq!(lexer.state.token_end_position, position(2, 0, 2));
        assert_eq!(lookahead_end, 5);
        lexer.advance(true);
        assert_eq!(lexer.state.token_start_position, position(5, 1, 1));
        lexer.advance(false);
        assert_eq!(lexer.state.current_position, position(9, 3, 0));
        lexer.mark_end();
        assert_eq!(lexer.state.token_end_position, position(6, 1, 2));
        assert_eq!(lexer.get_column(), 4);
        lexer.advance(false);
        lexer.advance(false);
        assert!(lexer.eof());
        assert!(!lexer.is_at_included_range_start());
        lexer.mark_end();
        assert_eq!(lexer.state.token_end_position, position(11, 3, 2));
        // EOF at the range's end does not read the trailing source byte.
        assert_eq!(input.reads, [(0, point_new(0, 0))]);
    }

    #[test]
    fn zero_width_ranges_are_skipped_but_remain_token_end_boundaries() {
        let mut state = ts_lexer_init();
        assert!(ts_lexer_set_included_ranges(
            &mut state,
            &[
                range(0, 1, 0),
                range(3, 3, 1),
                range(5, 6, 2),
                range(7, 7, 3),
            ]
        ));
        let mut input = RecordingInput::new(b"a----b--", &[]);
        let mut lexer = Lexer {
            state: &mut state,
            input: &mut input,
            logger: None,
        };
        ts_lexer_start(&mut lexer);
        lexer.advance(false);
        assert_eq!(lexer.state.current_included_range_index, 2);
        assert_eq!(lexer.state.current_position, position(5, 2, 0));
        lexer.mark_end();
        assert_eq!(lexer.state.token_end_position, position(3, 1, 0));
        lexer.advance(false);
        assert!(lexer.eof());
        assert_eq!(lexer.state.current_position, position(7, 3, 0));
        ts_lexer_reset(lexer.state, position(3, 99, 99));
        assert_eq!(lexer.state.current_position, position(5, 2, 0));
        ts_lexer_reset(lexer.state, position(100, 99, 99));
        assert!(lexer.eof());
        assert_eq!(lexer.state.current_position, position(7, 3, 0));
    }

    #[test]
    fn included_range_validation_is_atomic_and_uses_bytes_only() {
        let mut state = ts_lexer_init();
        assert_eq!(ts_lexer_included_ranges(&state), &[DEFAULT_RANGE]);
        let initial = range(3, 8, 0);
        assert!(ts_lexer_set_included_ranges(&mut state, &[initial]));
        assert_eq!(state.current_position, position(3, 0, 0));
        assert!(!ts_lexer_set_included_ranges(
            &mut state,
            &[range(2, 8, 0), range(7, 10, 1),]
        ));
        let backwards = Range {
            start_byte: 10,
            end_byte: 5,
            ..initial
        };
        assert!(!ts_lexer_set_included_ranges(&mut state, &[backwards]));
        assert_eq!(ts_lexer_included_ranges(&state), &[initial]);
        assert_eq!(state.current_position, position(3, 0, 0));
        let reversed_points = Range {
            start_point: point_new(9, 9),
            end_point: point_new(1, 1),
            ..initial
        };
        assert!(ts_lexer_set_included_ranges(&mut state, &[reversed_points]));
        assert_eq!(state.current_position, position(3, 9, 9));
        assert!(ts_lexer_set_included_ranges(&mut state, &[]));
        assert_eq!(ts_lexer_included_ranges(&state), &[DEFAULT_RANGE]);
    }

    #[test]
    fn changing_input_invalidates_chunk_but_not_cached_column() {
        let mut state = ts_lexer_init();
        let mut first = RecordingInput::new(b"ab", &[]);
        {
            let mut lexer = Lexer {
                state: &mut state,
                input: &mut first,
                logger: None,
            };
            ts_lexer_start(&mut lexer);
            lexer.advance(false);
            assert_eq!(lexer.get_column(), 1);
            assert_eq!(lexer.lookahead(), 'b' as i32);
        }
        ts_lexer_set_input(&mut state, InputEncoding::Utf8);
        assert_eq!(state.chunk_size, 0);
        assert_eq!(state.lookahead_size, 0);
        assert!(state.column_data.valid);
        let mut second = RecordingInput::new(b"ac", &[]);
        let mut lexer = Lexer {
            state: &mut state,
            input: &mut second,
            logger: None,
        };
        ts_lexer_start(&mut lexer);
        assert_eq!(lexer.get_column(), 1);
        assert_eq!(lexer.lookahead(), 'c' as i32);
        assert_eq!(second.reads, [(1, point_new(0, 1))]);
    }

    #[test]
    fn column_and_lookahead_arithmetic_wraps_like_u32() {
        let mut state = ts_lexer_init();
        ts_lexer__increment_column_data(&mut state);
        assert!(!state.column_data.valid);
        assert_eq!(state.column_data.value, 0);
        ts_lexer__set_column_data(&mut state, u32::MAX);
        ts_lexer__increment_column_data(&mut state);
        assert_eq!(state.column_data.value, 0);
        ts_lexer__invalidate_column_data(&mut state);
        assert!(!state.column_data.valid);
        state.current_position = LENGTH_MAX;
        state.lookahead = DECODE_ERROR;
        let mut end = 0;
        ts_lexer_finish(&mut state, &mut end);
        assert_eq!(end, 4);
        state.lookahead = 0;
        ts_lexer_finish(&mut state, &mut end);
        assert_eq!(end, 4);
    }

    #[test]
    fn cached_range_end_follows_jumps_range_changes_and_early_input_eof() {
        let mut state = ts_lexer_init();
        let mut input = RecordingInput::new(b"abc", &[1, 0]);
        let mut lexer = Lexer {
            state: &mut state,
            input: &mut input,
            logger: None,
        };
        assert_eq!(lexer.state.current_range_end, u32::MAX);
        ts_lexer_start(&mut lexer);
        lexer.advance(false);
        // The callback can report EOF before the included range ends.
        assert!(lexer.eof());
        assert_eq!(lexer.state.current_range_end, 0);
        ts_lexer_goto(lexer.state, length_zero());
        assert_eq!(lexer.state.current_range_end, u32::MAX);
        ts_lexer_start(&mut lexer);
        assert_eq!(lexer.lookahead(), i32::from(b'a'));

        assert!(ts_lexer_set_included_ranges(
            lexer.state,
            &[range(0, 1, 0), range(1, 1, 0), range(2, 3, 1)],
        ));
        assert_eq!(lexer.state.current_range_end, 1);
        ts_lexer_start(&mut lexer);
        lexer.advance(false);
        assert_eq!(lexer.state.current_range_end, 3);
        assert_eq!(lexer.state.current_position, position(2, 1, 0));
        assert_eq!(lexer.lookahead(), i32::from(b'c'));
        lexer.advance(false);
        assert!(lexer.eof());
        assert_eq!(lexer.state.current_range_end, 0);
        ts_lexer_goto(lexer.state, length_zero());
        assert_eq!(lexer.state.current_range_end, 1);
    }

    #[test]
    fn utf16_ascii_code_units_keep_their_width_and_retry_partial_chunks() {
        for encoding in [InputEncoding::Utf16Le, InputEncoding::Utf16Be] {
            let mut state = ts_lexer_init();
            ts_lexer_set_input(&mut state, encoding);
            let bytes = match encoding {
                InputEncoding::Utf16Le => [b'a', 0, b'\n', 0, 0, 1],
                InputEncoding::Utf16Be => [0, b'a', 0, b'\n', 1, 0],
                InputEncoding::Utf8 => unreachable!(),
            };
            let mut input = RecordingInput::new(&bytes, &[1, 4]);
            let mut lexer = Lexer {
                state: &mut state,
                input: &mut input,
                logger: None,
            };
            ts_lexer_start(&mut lexer);
            assert_eq!(lexer.lookahead(), i32::from(b'a'));
            assert_eq!(lexer.state.lookahead_size, 2);
            lexer.advance(false);
            assert_eq!(lexer.lookahead(), i32::from(b'\n'));
            assert_eq!(lexer.state.current_position, position(2, 0, 2));
            lexer.advance(false);
            assert_eq!(lexer.lookahead(), 0x100);
            assert_eq!(lexer.state.current_position, position(4, 1, 0));
            assert_eq!(lexer.get_column(), 0);
            lexer.advance(false);
            assert!(lexer.eof());
            assert_eq!(lexer.state.current_position, position(6, 1, 2));
            assert_eq!(lexer.get_column(), 1);
            assert_eq!(
                input.reads,
                [
                    (0, point_new(0, 0)),
                    (0, point_new(0, 0)),
                    (4, point_new(1, 0)),
                    (6, point_new(1, 2)),
                ]
            );
        }
    }

    #[test]
    fn logger_receives_c_messages_and_bounded_text() {
        use std::sync::{Arc, Mutex};
        let messages = Arc::new(Mutex::new(Vec::new()));
        let captured = Arc::clone(&messages);
        let mut logger: crate::parser::Logger = Box::new(move |kind, text| {
            assert_eq!(kind, crate::api::LogType::Lex);
            captured.lock().unwrap().push(text.to_owned());
        });
        let mut state = ts_lexer_init();
        let mut input = RecordingInput::new(b"a\n", &[]);
        let mut lexer = Lexer {
            state: &mut state,
            input: &mut input,
            logger: Some(&mut logger),
        };
        ts_lexer_start(&mut lexer);
        lexer.advance(false);
        lexer.advance(true);
        lexer.advance(false);
        lexer.log(format_args!("custom {}", 42));
        lexer.log(format_args!("{}", "x".repeat(1025)));
        lexer.log(format_args!("{}é", "x".repeat(1022)));
        lexer.log(format_args!("before\0after"));
        assert_eq!(
            *messages.lock().unwrap(),
            [
                "consume character:'a'".to_owned(),
                "skip character:10".to_owned(),
                "custom 42".to_owned(),
                "x".repeat(1023),
                "x".repeat(1022),
                "before".to_owned(),
            ]
        );
    }
}
