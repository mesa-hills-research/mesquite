//! The Markdown block scanner, translated from tree-sitter-markdown/src/scanner.c.

use ts_port_tables::{ExternalScanner, Lexer};

// Indices in the grammar's external-token array.
const LINE_ENDING: usize = 0;
const SOFT_LINE_ENDING: usize = 1;
const BLOCK_CLOSE: usize = 2;
const BLOCK_CONTINUATION: usize = 3;
const BLOCK_QUOTE_START: usize = 4;
const INDENTED_CHUNK_START: usize = 5;
const ATX_H1_MARKER: usize = 6;
// ATX_H2_MARKER = 7 (computed from ATX_H1_MARKER).
// ATX_H3_MARKER = 8 (computed from ATX_H1_MARKER).
// ATX_H4_MARKER = 9 (computed from ATX_H1_MARKER).
// ATX_H5_MARKER = 10 (computed from ATX_H1_MARKER).
// ATX_H6_MARKER = 11 (computed from ATX_H1_MARKER).
const SETEXT_H1_UNDERLINE: usize = 12;
const SETEXT_H2_UNDERLINE: usize = 13;
const THEMATIC_BREAK: usize = 14;
const LIST_MARKER_MINUS: usize = 15;
const LIST_MARKER_PLUS: usize = 16;
const LIST_MARKER_STAR: usize = 17;
const LIST_MARKER_PARENTHESIS: usize = 18;
const LIST_MARKER_DOT: usize = 19;
const LIST_MARKER_MINUS_DONT_INTERRUPT: usize = 20;
const LIST_MARKER_PLUS_DONT_INTERRUPT: usize = 21;
const LIST_MARKER_STAR_DONT_INTERRUPT: usize = 22;
const LIST_MARKER_PARENTHESIS_DONT_INTERRUPT: usize = 23;
const LIST_MARKER_DOT_DONT_INTERRUPT: usize = 24;
const FENCED_CODE_BLOCK_START_BACKTICK: usize = 25;
const FENCED_CODE_BLOCK_START_TILDE: usize = 26;
const BLANK_LINE_START: usize = 27;
const FENCED_CODE_BLOCK_END_BACKTICK: usize = 28;
const FENCED_CODE_BLOCK_END_TILDE: usize = 29;
const HTML_BLOCK_1_START: usize = 30;
const HTML_BLOCK_1_END: usize = 31;
const HTML_BLOCK_2_START: usize = 32;
const HTML_BLOCK_3_START: usize = 33;
const HTML_BLOCK_4_START: usize = 34;
const HTML_BLOCK_5_START: usize = 35;
const HTML_BLOCK_6_START: usize = 36;
const HTML_BLOCK_7_START: usize = 37;
const CLOSE_BLOCK: usize = 38;
const NO_INDENTED_CHUNK: usize = 39;
const ERROR: usize = 40;
const TRIGGER_ERROR: usize = 41;
const TOKEN_EOF: usize = 42;
const MINUS_METADATA: usize = 43;
const PLUS_METADATA: usize = 44;
const PIPE_TABLE_START: usize = 45;
const PIPE_TABLE_LINE_ENDING: usize = 46;

const PARAGRAPH_INTERRUPT_SYMBOLS: [bool; 47] = [
    false, // LINE_ENDING
    false, // SOFT_LINE_ENDING
    false, // BLOCK_CLOSE
    false, // BLOCK_CONTINUATION
    true,  // BLOCK_QUOTE_START
    false, // INDENTED_CHUNK_START
    true,  // ATX_H1_MARKER
    true,  // ATX_H2_MARKER
    true,  // ATX_H3_MARKER
    true,  // ATX_H4_MARKER
    true,  // ATX_H5_MARKER
    true,  // ATX_H6_MARKER
    true,  // SETEXT_H1_UNDERLINE
    true,  // SETEXT_H2_UNDERLINE
    true,  // THEMATIC_BREAK
    true,  // LIST_MARKER_MINUS
    true,  // LIST_MARKER_PLUS
    true,  // LIST_MARKER_STAR
    true,  // LIST_MARKER_PARENTHESIS
    true,  // LIST_MARKER_DOT
    false, // LIST_MARKER_MINUS_DONT_INTERRUPT
    false, // LIST_MARKER_PLUS_DONT_INTERRUPT
    false, // LIST_MARKER_STAR_DONT_INTERRUPT
    false, // LIST_MARKER_PARENTHESIS_DONT_INTERRUPT
    false, // LIST_MARKER_DOT_DONT_INTERRUPT
    true,  // FENCED_CODE_BLOCK_START_BACKTICK
    true,  // FENCED_CODE_BLOCK_START_TILDE
    true,  // BLANK_LINE_START
    false, // FENCED_CODE_BLOCK_END_BACKTICK
    false, // FENCED_CODE_BLOCK_END_TILDE
    true,  // HTML_BLOCK_1_START
    false, // HTML_BLOCK_1_END
    true,  // HTML_BLOCK_2_START
    true,  // HTML_BLOCK_3_START
    true,  // HTML_BLOCK_4_START
    true,  // HTML_BLOCK_5_START
    true,  // HTML_BLOCK_6_START
    false, // HTML_BLOCK_7_START
    false, // CLOSE_BLOCK
    false, // NO_INDENTED_CHUNK
    false, // ERROR
    false, // TRIGGER_ERROR
    false, // EOF
    false, // MINUS_METADATA
    false, // PLUS_METADATA
    true,  // PIPE_TABLE_START
    false, // PIPE_TABLE_LINE_ENDING
];

const HTML_TAG_NAMES_RULE_1: [&[u8]; 3] = [b"pre", b"script", b"style"];

const HTML_TAG_NAMES_RULE_7: [&[u8]; 62] = [
    b"address",
    b"article",
    b"aside",
    b"base",
    b"basefont",
    b"blockquote",
    b"body",
    b"caption",
    b"center",
    b"col",
    b"colgroup",
    b"dd",
    b"details",
    b"dialog",
    b"dir",
    b"div",
    b"dl",
    b"dt",
    b"fieldset",
    b"figcaption",
    b"figure",
    b"footer",
    b"form",
    b"frame",
    b"frameset",
    b"h1",
    b"h2",
    b"h3",
    b"h4",
    b"h5",
    b"h6",
    b"head",
    b"header",
    b"hr",
    b"html",
    b"iframe",
    b"legend",
    b"li",
    b"link",
    b"main",
    b"menu",
    b"menuitem",
    b"nav",
    b"noframes",
    b"ol",
    b"optgroup",
    b"option",
    b"p",
    b"param",
    b"section",
    b"source",
    b"summary",
    b"table",
    b"tbody",
    b"td",
    b"tfoot",
    b"th",
    b"thead",
    b"title",
    b"tr",
    b"track",
    b"ul",
];

const STATE_MATCHING: u8 = 1;
const STATE_WAS_SOFT_LINE_BREAK: u8 = 1 << 1;
const STATE_CLOSE_BLOCK: u8 = 1 << 4;

/// C's Block enum is serialized as native-endian, four-byte integers. Keep the
/// numeric representation: list-marker arithmetic can produce unnamed values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Block(u32);

impl Block {
    const QUOTE: Self = Self(0);
    const INDENTED_CODE: Self = Self(1);
    const LIST_ITEM: Self = Self(2);
    const LIST_ITEM_MAX_INDENTATION: Self = Self(17);
    const FENCED_CODE: Self = Self(18);
    const ANONYMOUS: Self = Self(19);

    fn list_item(extra_indentation: usize) -> Self {
        Self(Self::LIST_ITEM.0.wrapping_add(extra_indentation as u32))
    }

    fn is_list_item(self) -> bool {
        (Self::LIST_ITEM.0..=Self::LIST_ITEM_MAX_INDENTATION.0).contains(&self.0)
    }

    fn list_item_indentation(self) -> u8 {
        (self.0 - Self::LIST_ITEM.0 + 2) as u8
    }
}

const STATE: usize = 0;
const MATCHED: usize = 1;
const INDENTATION: usize = 2;
const COLUMN: usize = 3;
const FENCE_LENGTH: usize = 4;
const HEADER_SIZE: usize = 5;
const INLINE_BLOCKS: usize = 4;

#[derive(Default)]
pub(crate) struct Scanner {
    // A header and four native-endian blocks: all snapshots small enough for
    // the runtime's short external-token state stay inline here as well.
    snapshot: [u8; HEADER_SIZE + 4 * INLINE_BLOCKS],
    spilled_blocks: Vec<[u8; 4]>,
    // 0..=4 is the inline block count; 5 means use spilled_blocks.len().
    inline_count: u8,
    simulate: bool,
}

// The reference uses the default C locale for the wide-character predicates.
fn is_alpha(c: i32) -> bool {
    matches!(c, 0x41..=0x5a | 0x61..=0x7a)
}

fn is_digit(c: i32) -> bool {
    matches!(c, 0x30..=0x39)
}

fn is_alnum(c: i32) -> bool {
    is_alpha(c) || is_digit(c)
}

fn is_space(c: i32) -> bool {
    matches!(c, 0x20 | 0x09)
}

fn is_line_end(c: i32) -> bool {
    matches!(c, 0x0a | 0x0d)
}

fn is_punctuation(c: i32) -> bool {
    // The C function takes char, including truncation of non-ASCII lookahead.
    matches!(c as u8, b'!'..=b'/' | b':'..=b'@' | b'['..=b'`' | b'{'..=b'~')
}

impl ExternalScanner for Scanner {
    fn scan(&mut self, lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> bool {
        self.simulate = false;
        self.scan_inner(&mut ScanLexer::new(lexer), valid_symbols)
    }

    fn serialize(&mut self, buffer: &mut [u8]) -> usize {
        // Fixed-size copies keep these frequent, shallow snapshots inlined;
        // a variable-length copy would call memcpy even for the five-byte state.
        if self.inline_count == 0 {
            buffer[..5].copy_from_slice(&self.snapshot[..5]);
            return 5;
        }
        if self.inline_count == 1 {
            buffer[..9].copy_from_slice(&self.snapshot[..9]);
            return 9;
        }
        match self.inline_count {
            2 => {
                buffer[..13].copy_from_slice(&self.snapshot[..13]);
                13
            }
            3 => {
                buffer[..17].copy_from_slice(&self.snapshot[..17]);
                17
            }
            4 => {
                buffer[..21].copy_from_slice(&self.snapshot[..21]);
                21
            }
            _ => self.serialize_spilled(buffer),
        }
    }

    fn deserialize(&mut self, buffer: &[u8]) {
        // simulate is neither serialized nor reset by C's deserialize.
        if buffer.len() == 5 {
            self.snapshot[..5].copy_from_slice(buffer);
            self.inline_count = 0;
            return;
        }
        if buffer.len() == 9 {
            self.snapshot[..9].copy_from_slice(buffer);
            self.inline_count = 1;
            return;
        }
        match buffer.len() {
            0 => {
                self.snapshot[..HEADER_SIZE].fill(0);
                self.inline_count = 0;
            }
            13 => {
                self.snapshot[..13].copy_from_slice(buffer);
                self.inline_count = 2;
            }
            17 => {
                self.snapshot[..17].copy_from_slice(buffer);
                self.inline_count = 3;
            }
            21 => {
                self.snapshot[..21].copy_from_slice(buffer);
                self.inline_count = 4;
            }
            _ => self.deserialize_spilled(buffer),
        }
    }
}

pub(crate) fn create() -> Box<dyn ExternalScanner> {
    Box::<Scanner>::default()
}

/// A scan-local copy of C's `TSLexer.lookahead` field. Only `advance` changes
/// the input position, so all tests of the current character (including the
/// column accounting and recursive paragraph lookahead) share one trait call.
/// Keep this concrete in the helpers so cached reads do not use dynamic dispatch.
struct ScanLexer<'a> {
    inner: &'a mut dyn Lexer,
    lookahead: i32,
}

impl<'a> ScanLexer<'a> {
    fn new(inner: &'a mut dyn Lexer) -> Self {
        Self {
            lookahead: inner.lookahead(),
            inner,
        }
    }

    fn lookahead(&self) -> i32 {
        self.lookahead
    }

    fn advance(&mut self, skip: bool) {
        self.inner.advance(skip);
        self.lookahead = self.inner.lookahead();
    }

    fn mark_end(&mut self) {
        self.inner.mark_end();
    }

    fn set_result_symbol(&mut self, symbol: u16) {
        self.inner.set_result_symbol(symbol);
    }

    fn eof(&self) -> bool {
        // EOF always has zero lookahead, but an embedded NUL is not EOF.
        self.lookahead == 0 && self.inner.eof()
    }
}

impl Scanner {
    #[cold]
    fn serialize_spilled(&self, buffer: &mut [u8]) -> usize {
        buffer[..HEADER_SIZE].copy_from_slice(&self.snapshot[..HEADER_SIZE]);
        let blocks = self.spilled_blocks.as_flattened();
        let size = HEADER_SIZE + blocks.len();
        buffer[HEADER_SIZE..size].copy_from_slice(blocks);
        size
    }

    #[cold]
    fn deserialize_spilled(&mut self, buffer: &[u8]) {
        let (_, blocks) = buffer.split_at(HEADER_SIZE);
        let blocks = blocks.as_chunks::<4>().0;
        if blocks.len() <= INLINE_BLOCKS {
            let size = HEADER_SIZE + 4 * blocks.len();
            self.snapshot[..size].copy_from_slice(&buffer[..size]);
            self.inline_count = blocks.len() as u8;
        } else {
            self.snapshot
                .copy_from_slice(&buffer[..HEADER_SIZE + 4 * INLINE_BLOCKS]);
            self.spilled_blocks.clear();
            self.spilled_blocks.extend_from_slice(blocks);
            self.inline_count = INLINE_BLOCKS as u8 + 1;
        }
    }

    fn block_count(&self) -> usize {
        if self.inline_count <= INLINE_BLOCKS as u8 {
            usize::from(self.inline_count)
        } else {
            self.spilled_blocks.len()
        }
    }

    fn block_at(&self, index: usize) -> Block {
        let blocks = if self.inline_count <= INLINE_BLOCKS as u8 {
            &self.snapshot[HEADER_SIZE..].as_chunks::<4>().0[..usize::from(self.inline_count)]
        } else {
            &self.spilled_blocks
        };
        Block(u32::from_ne_bytes(blocks[index]))
    }

    fn push_block(&mut self, block: Block) {
        if self.inline_count < INLINE_BLOCKS as u8 {
            self.snapshot[HEADER_SIZE..].as_chunks_mut::<4>().0[usize::from(self.inline_count)] =
                block.0.to_ne_bytes();
            self.inline_count += 1;
        } else {
            if self.inline_count == INLINE_BLOCKS as u8 {
                self.spilled_blocks.clear();
                self.spilled_blocks
                    .extend_from_slice(self.snapshot[HEADER_SIZE..].as_chunks::<4>().0);
                self.inline_count += 1;
            }
            self.spilled_blocks.push(block.0.to_ne_bytes());
        }
    }

    fn pop_block(&mut self) {
        // The inline prefix is retained while spilled: these immutable entries
        // are still correct when the stack becomes shallow again.
        if self.inline_count <= INLINE_BLOCKS as u8 {
            self.inline_count = self.inline_count.saturating_sub(1);
        } else {
            self.spilled_blocks.pop();
            if self.spilled_blocks.len() == INLINE_BLOCKS {
                self.inline_count = INLINE_BLOCKS as u8;
            }
        }
    }

    #[cfg(test)]
    fn with_blocks(blocks: impl IntoIterator<Item = Block>) -> Self {
        let mut scanner = Self::default();
        for block in blocks {
            scanner.push_block(block);
        }
        scanner
    }

    #[cfg(test)]
    fn blocks(&self) -> Vec<Block> {
        (0..self.block_count()).map(|i| self.block_at(i)).collect()
    }

    fn mark_end(&self, lexer: &mut ScanLexer<'_>) {
        if !self.simulate {
            lexer.mark_end();
        }
    }

    /// Advance while tracking a column modulo four for tab expansion.
    fn advance(&mut self, lexer: &mut ScanLexer<'_>) -> u8 {
        let size = if lexer.lookahead() == i32::from(b'\t') {
            let size = 4 - self.snapshot[COLUMN];
            self.snapshot[COLUMN] = 0;
            size
        } else {
            self.snapshot[COLUMN] = ((u16::from(self.snapshot[COLUMN]) + 1) % 4) as u8;
            1
        };
        lexer.advance(false);
        size
    }

    fn consume_indentation(&mut self, lexer: &mut ScanLexer<'_>) {
        while is_space(lexer.lookahead()) {
            self.snapshot[INDENTATION] =
                self.snapshot[INDENTATION].wrapping_add(self.advance(lexer));
        }
    }

    fn consume_newline(&mut self, lexer: &mut ScanLexer<'_>) {
        if lexer.lookahead() == i32::from(b'\r') {
            self.advance(lexer);
            if lexer.lookahead() == i32::from(b'\n') {
                self.advance(lexer);
            }
        } else {
            self.advance(lexer);
        }
    }

    fn match_block(&mut self, lexer: &mut ScanLexer<'_>, block: Block) -> bool {
        match block {
            Block::INDENTED_CODE => {
                while self.snapshot[INDENTATION] < 4 {
                    if is_space(lexer.lookahead()) {
                        self.snapshot[INDENTATION] =
                            self.snapshot[INDENTATION].wrapping_add(self.advance(lexer));
                    } else {
                        break;
                    }
                }
                if self.snapshot[INDENTATION] >= 4 && !is_line_end(lexer.lookahead()) {
                    self.snapshot[INDENTATION] -= 4;
                    return true;
                }
            }
            block if block.is_list_item() => {
                while self.snapshot[INDENTATION] < block.list_item_indentation() {
                    if is_space(lexer.lookahead()) {
                        self.snapshot[INDENTATION] =
                            self.snapshot[INDENTATION].wrapping_add(self.advance(lexer));
                    } else {
                        break;
                    }
                }
                if self.snapshot[INDENTATION] >= block.list_item_indentation() {
                    self.snapshot[INDENTATION] -= block.list_item_indentation();
                    return true;
                }
                if is_line_end(lexer.lookahead()) {
                    self.snapshot[INDENTATION] = 0;
                    return true;
                }
            }
            Block::QUOTE => {
                self.consume_indentation(lexer);
                if lexer.lookahead() == i32::from(b'>') {
                    self.advance(lexer);
                    self.snapshot[INDENTATION] = 0;
                    if is_space(lexer.lookahead()) {
                        self.snapshot[INDENTATION] =
                            self.snapshot[INDENTATION].wrapping_add(self.advance(lexer) - 1);
                    }
                    return true;
                }
            }
            Block::FENCED_CODE | Block::ANONYMOUS => return true,
            _ => {}
        }
        false
    }

    fn parse_fenced_code_block(
        &mut self,
        delimiter: u8,
        lexer: &mut ScanLexer<'_>,
        valid_symbols: &[bool],
    ) -> bool {
        let mut level = 0u8;
        while lexer.lookahead() == i32::from(delimiter) {
            self.advance(lexer);
            level = level.wrapping_add(1);
        }
        self.mark_end(lexer);
        let (start, end) = if delimiter == b'`' {
            (
                FENCED_CODE_BLOCK_START_BACKTICK,
                FENCED_CODE_BLOCK_END_BACKTICK,
            )
        } else {
            (FENCED_CODE_BLOCK_START_TILDE, FENCED_CODE_BLOCK_END_TILDE)
        };
        if valid_symbols[end]
            && self.snapshot[INDENTATION] < 4
            && level >= self.snapshot[FENCE_LENGTH]
        {
            while is_space(lexer.lookahead()) {
                self.advance(lexer);
            }
            if is_line_end(lexer.lookahead()) {
                self.snapshot[FENCE_LENGTH] = 0;
                lexer.set_result_symbol(end as u16);
                return true;
            }
        }
        if valid_symbols[start] && level >= 3 {
            let mut info_string_has_backtick = false;
            if delimiter == b'`' {
                while !is_line_end(lexer.lookahead()) && !lexer.eof() {
                    if lexer.lookahead() == i32::from(b'`') {
                        info_string_has_backtick = true;
                        break;
                    }
                    self.advance(lexer);
                }
            }
            if !info_string_has_backtick {
                lexer.set_result_symbol(start as u16);
                if !self.simulate {
                    self.push_block(Block::FENCED_CODE);
                }
                self.snapshot[FENCE_LENGTH] = level;
                self.snapshot[INDENTATION] = 0;
                return true;
            }
        }
        false
    }

    /// After discounting the required space after a marker, choose the list
    /// content's indentation, leaving any indented-code whitespace unused.
    fn list_extra_indentation(&mut self, extra_indentation: u8) -> u8 {
        let mut extra = extra_indentation - 1;
        if extra <= 3 {
            extra = extra.wrapping_add(self.snapshot[INDENTATION]);
            self.snapshot[INDENTATION] = 0;
        } else {
            std::mem::swap(&mut self.snapshot[INDENTATION], &mut extra);
        }
        extra
    }

    fn parse_star(&mut self, lexer: &mut ScanLexer<'_>, valid_symbols: &[bool]) -> bool {
        self.advance(lexer);
        self.mark_end(lexer);
        let mut star_count = 1usize;
        let mut extra_indentation = 0u8;
        loop {
            if lexer.lookahead() == i32::from(b'*') {
                if star_count == 1 && extra_indentation >= 1 && valid_symbols[LIST_MARKER_STAR] {
                    self.mark_end(lexer);
                }
                star_count += 1;
                self.advance(lexer);
            } else if is_space(lexer.lookahead()) {
                if star_count == 1 {
                    extra_indentation = extra_indentation.wrapping_add(self.advance(lexer));
                } else {
                    self.advance(lexer);
                }
            } else {
                break;
            }
        }
        let line_end = is_line_end(lexer.lookahead());
        let mut dont_interrupt = false;
        if star_count == 1 && line_end {
            extra_indentation = 1;
            dont_interrupt = usize::from(self.snapshot[MATCHED]) == self.block_count();
        }
        let thematic_break = star_count >= 3 && line_end;
        let list_marker_star = star_count >= 1 && extra_indentation >= 1;
        if valid_symbols[THEMATIC_BREAK] && thematic_break && self.snapshot[INDENTATION] < 4 {
            lexer.set_result_symbol(THEMATIC_BREAK as u16);
            self.mark_end(lexer);
            self.snapshot[INDENTATION] = 0;
            return true;
        }
        let symbol = if dont_interrupt {
            LIST_MARKER_STAR_DONT_INTERRUPT
        } else {
            LIST_MARKER_STAR
        };
        if valid_symbols[symbol] && list_marker_star {
            if star_count == 1 {
                self.mark_end(lexer);
            }
            extra_indentation = self.list_extra_indentation(extra_indentation);
            if !self.simulate {
                self.push_block(Block::list_item(usize::from(extra_indentation)));
            }
            lexer.set_result_symbol(symbol as u16);
            return true;
        }
        false
    }

    fn parse_thematic_break_underscore(
        &mut self,
        lexer: &mut ScanLexer<'_>,
        valid_symbols: &[bool],
    ) -> bool {
        self.advance(lexer);
        self.mark_end(lexer);
        let mut underscore_count = 1usize;
        loop {
            if lexer.lookahead() == i32::from(b'_') {
                underscore_count += 1;
                self.advance(lexer);
            } else if is_space(lexer.lookahead()) {
                self.advance(lexer);
            } else {
                break;
            }
        }
        if underscore_count >= 3 && is_line_end(lexer.lookahead()) && valid_symbols[THEMATIC_BREAK]
        {
            lexer.set_result_symbol(THEMATIC_BREAK as u16);
            self.mark_end(lexer);
            self.snapshot[INDENTATION] = 0;
            return true;
        }
        false
    }

    fn parse_block_quote(&mut self, lexer: &mut ScanLexer<'_>, valid_symbols: &[bool]) -> bool {
        if valid_symbols[BLOCK_QUOTE_START] {
            self.advance(lexer);
            self.snapshot[INDENTATION] = 0;
            if is_space(lexer.lookahead()) {
                self.snapshot[INDENTATION] =
                    self.snapshot[INDENTATION].wrapping_add(self.advance(lexer) - 1);
            }
            lexer.set_result_symbol(BLOCK_QUOTE_START as u16);
            if !self.simulate {
                self.push_block(Block::QUOTE);
            }
            return true;
        }
        false
    }

    fn parse_atx_heading(&mut self, lexer: &mut ScanLexer<'_>, valid_symbols: &[bool]) -> bool {
        if valid_symbols[ATX_H1_MARKER] && self.snapshot[INDENTATION] <= 3 {
            self.mark_end(lexer);
            let mut level = 0u16;
            while lexer.lookahead() == i32::from(b'#') && level <= 6 {
                self.advance(lexer);
                level += 1;
            }
            if level <= 6 && (is_space(lexer.lookahead()) || is_line_end(lexer.lookahead())) {
                lexer.set_result_symbol(ATX_H1_MARKER as u16 + level - 1);
                self.snapshot[INDENTATION] = 0;
                self.mark_end(lexer);
                return true;
            }
        }
        false
    }

    fn parse_setext_underline(
        &mut self,
        lexer: &mut ScanLexer<'_>,
        valid_symbols: &[bool],
    ) -> bool {
        if valid_symbols[SETEXT_H1_UNDERLINE]
            && usize::from(self.snapshot[MATCHED]) == self.block_count()
        {
            self.mark_end(lexer);
            while lexer.lookahead() == i32::from(b'=') {
                self.advance(lexer);
            }
            while is_space(lexer.lookahead()) {
                self.advance(lexer);
            }
            if is_line_end(lexer.lookahead()) {
                lexer.set_result_symbol(SETEXT_H1_UNDERLINE as u16);
                self.mark_end(lexer);
                return true;
            }
        }
        false
    }

    /// The metadata branches for `+++` and `---` have identical line scanning.
    /// Called at the newline after the opening marker (never at EOF).
    fn parse_metadata_body(
        &mut self,
        lexer: &mut ScanLexer<'_>,
        delimiter: u8,
        symbol: usize,
    ) -> bool {
        loop {
            self.consume_newline(lexer);
            let mut count = 0usize;
            while lexer.lookahead() == i32::from(delimiter) {
                count += 1;
                self.advance(lexer);
            }
            if count == 3 {
                while is_space(lexer.lookahead()) {
                    self.advance(lexer);
                }
                if is_line_end(lexer.lookahead()) {
                    self.consume_newline(lexer);
                    self.mark_end(lexer);
                    lexer.set_result_symbol(symbol as u16);
                    return true;
                }
            }
            while !is_line_end(lexer.lookahead()) && !lexer.eof() {
                self.advance(lexer);
            }
            if lexer.eof() {
                break;
            }
        }
        false
    }

    fn parse_plus(&mut self, lexer: &mut ScanLexer<'_>, valid_symbols: &[bool]) -> bool {
        if self.snapshot[INDENTATION] <= 3
            && (valid_symbols[LIST_MARKER_PLUS]
                || valid_symbols[LIST_MARKER_PLUS_DONT_INTERRUPT]
                || valid_symbols[PLUS_METADATA])
        {
            self.advance(lexer);
            if valid_symbols[PLUS_METADATA] && lexer.lookahead() == i32::from(b'+') {
                self.advance(lexer);
                if lexer.lookahead() != i32::from(b'+') {
                    return false;
                }
                self.advance(lexer);
                while is_space(lexer.lookahead()) {
                    self.advance(lexer);
                }
                if !is_line_end(lexer.lookahead()) {
                    return false;
                }
                return self.parse_metadata_body(lexer, b'+', PLUS_METADATA);
            } else {
                let mut extra_indentation = 0u8;
                while is_space(lexer.lookahead()) {
                    extra_indentation = extra_indentation.wrapping_add(self.advance(lexer));
                }
                let mut dont_interrupt = false;
                if is_line_end(lexer.lookahead()) {
                    extra_indentation = 1;
                    dont_interrupt = true;
                }
                dont_interrupt =
                    dont_interrupt && usize::from(self.snapshot[MATCHED]) == self.block_count();
                let symbol = if dont_interrupt {
                    LIST_MARKER_PLUS_DONT_INTERRUPT
                } else {
                    LIST_MARKER_PLUS
                };
                if extra_indentation >= 1 && valid_symbols[symbol] {
                    lexer.set_result_symbol(symbol as u16);
                    extra_indentation = self.list_extra_indentation(extra_indentation);
                    if !self.simulate {
                        self.push_block(Block::list_item(usize::from(extra_indentation)));
                    }
                    return true;
                }
            }
        }
        false
    }

    fn parse_ordered_list_marker(
        &mut self,
        lexer: &mut ScanLexer<'_>,
        valid_symbols: &[bool],
    ) -> bool {
        if self.snapshot[INDENTATION] <= 3
            && (valid_symbols[LIST_MARKER_PARENTHESIS]
                || valid_symbols[LIST_MARKER_DOT]
                || valid_symbols[LIST_MARKER_PARENTHESIS_DONT_INTERRUPT]
                || valid_symbols[LIST_MARKER_DOT_DONT_INTERRUPT])
        {
            let mut digits = 1usize;
            // Deliberately tests isdigit, not whether the first digit is '1'.
            let mut dont_interrupt = !is_digit(lexer.lookahead());
            self.advance(lexer);
            while is_digit(lexer.lookahead()) {
                dont_interrupt = true;
                digits += 1;
                self.advance(lexer);
            }
            if (1..=9).contains(&digits) {
                let mut dot = false;
                let mut parenthesis = false;
                if lexer.lookahead() == i32::from(b'.') {
                    self.advance(lexer);
                    dot = true;
                } else if lexer.lookahead() == i32::from(b')') {
                    self.advance(lexer);
                    parenthesis = true;
                }
                if dot || parenthesis {
                    let mut extra_indentation = 0u8;
                    while is_space(lexer.lookahead()) {
                        extra_indentation = extra_indentation.wrapping_add(self.advance(lexer));
                    }
                    if is_line_end(lexer.lookahead()) {
                        extra_indentation = 1;
                        dont_interrupt = true;
                    }
                    dont_interrupt =
                        dont_interrupt && usize::from(self.snapshot[MATCHED]) == self.block_count();
                    let symbol = if dot {
                        if dont_interrupt {
                            LIST_MARKER_DOT_DONT_INTERRUPT
                        } else {
                            LIST_MARKER_DOT
                        }
                    } else if dont_interrupt {
                        LIST_MARKER_PARENTHESIS_DONT_INTERRUPT
                    } else {
                        LIST_MARKER_PARENTHESIS
                    };
                    if extra_indentation >= 1 && valid_symbols[symbol] {
                        // C emits the ordinary symbol even when validity was
                        // checked using the DONT_INTERRUPT variant.
                        lexer.set_result_symbol(if dot {
                            LIST_MARKER_DOT
                        } else {
                            LIST_MARKER_PARENTHESIS
                        } as u16);
                        extra_indentation = self.list_extra_indentation(extra_indentation);
                        if !self.simulate {
                            self.push_block(Block::list_item(
                                usize::from(extra_indentation) + digits,
                            ));
                        }
                        return true;
                    }
                }
            }
        }
        false
    }

    fn parse_minus(&mut self, lexer: &mut ScanLexer<'_>, valid_symbols: &[bool]) -> bool {
        if self.snapshot[INDENTATION] <= 3
            && (valid_symbols[LIST_MARKER_MINUS]
                || valid_symbols[LIST_MARKER_MINUS_DONT_INTERRUPT]
                || valid_symbols[SETEXT_H2_UNDERLINE]
                || valid_symbols[THEMATIC_BREAK]
                || valid_symbols[MINUS_METADATA])
        {
            self.mark_end(lexer);
            let mut whitespace_after_minus = false;
            let mut minus_after_whitespace = false;
            let mut minus_count = 0usize;
            let mut extra_indentation = 0u8;
            loop {
                if lexer.lookahead() == i32::from(b'-') {
                    if minus_count == 1 && extra_indentation >= 1 {
                        self.mark_end(lexer);
                    }
                    minus_count += 1;
                    self.advance(lexer);
                    minus_after_whitespace = whitespace_after_minus;
                } else if is_space(lexer.lookahead()) {
                    if minus_count == 1 {
                        extra_indentation = extra_indentation.wrapping_add(self.advance(lexer));
                    } else {
                        self.advance(lexer);
                    }
                    whitespace_after_minus = true;
                } else {
                    break;
                }
            }
            let line_end = is_line_end(lexer.lookahead());
            let mut dont_interrupt = false;
            if minus_count == 1 && line_end {
                extra_indentation = 1;
                dont_interrupt = true;
            }
            dont_interrupt =
                dont_interrupt && usize::from(self.snapshot[MATCHED]) == self.block_count();
            let thematic_break = minus_count >= 3 && line_end;
            let underline = minus_count >= 1
                && !minus_after_whitespace
                && line_end
                && usize::from(self.snapshot[MATCHED]) == self.block_count();
            let list_marker_minus = minus_count >= 1 && extra_indentation >= 1;
            let symbol = if dont_interrupt {
                LIST_MARKER_MINUS_DONT_INTERRUPT
            } else {
                LIST_MARKER_MINUS
            };
            let mut success = false;
            if valid_symbols[SETEXT_H2_UNDERLINE] && underline {
                lexer.set_result_symbol(SETEXT_H2_UNDERLINE as u16);
                self.mark_end(lexer);
                self.snapshot[INDENTATION] = 0;
                success = true;
            } else if valid_symbols[THEMATIC_BREAK] && thematic_break {
                lexer.set_result_symbol(THEMATIC_BREAK as u16);
                self.mark_end(lexer);
                self.snapshot[INDENTATION] = 0;
                success = true;
            } else if valid_symbols[symbol] && list_marker_minus {
                if minus_count == 1 {
                    self.mark_end(lexer);
                }
                extra_indentation = self.list_extra_indentation(extra_indentation);
                if !self.simulate {
                    self.push_block(Block::list_item(usize::from(extra_indentation)));
                }
                lexer.set_result_symbol(symbol as u16);
                return true;
            }
            if minus_count == 3
                && !minus_after_whitespace
                && line_end
                && valid_symbols[MINUS_METADATA]
                && self.parse_metadata_body(lexer, b'-', MINUS_METADATA)
            {
                return true;
            }
            if success {
                return true;
            }
        }
        false
    }

    fn parse_html_block(&mut self, lexer: &mut ScanLexer<'_>, valid_symbols: &[bool]) -> bool {
        if !(valid_symbols[HTML_BLOCK_1_START]
            || valid_symbols[HTML_BLOCK_1_END]
            || valid_symbols[HTML_BLOCK_2_START]
            || valid_symbols[HTML_BLOCK_3_START]
            || valid_symbols[HTML_BLOCK_4_START]
            || valid_symbols[HTML_BLOCK_5_START]
            || valid_symbols[HTML_BLOCK_6_START]
            || valid_symbols[HTML_BLOCK_7_START])
        {
            return false;
        }
        self.advance(lexer);
        if lexer.lookahead() == i32::from(b'?') && valid_symbols[HTML_BLOCK_3_START] {
            self.advance(lexer);
            lexer.set_result_symbol(HTML_BLOCK_3_START as u16);
            if !self.simulate {
                self.push_block(Block::ANONYMOUS);
            }
            return true;
        }
        if lexer.lookahead() == i32::from(b'!') {
            self.advance(lexer);
            if lexer.lookahead() == i32::from(b'-') {
                self.advance(lexer);
                if lexer.lookahead() == i32::from(b'-') && valid_symbols[HTML_BLOCK_2_START] {
                    self.advance(lexer);
                    lexer.set_result_symbol(HTML_BLOCK_2_START as u16);
                    if !self.simulate {
                        self.push_block(Block::ANONYMOUS);
                    }
                    return true;
                }
            } else if (i32::from(b'A')..=i32::from(b'Z')).contains(&lexer.lookahead())
                && valid_symbols[HTML_BLOCK_4_START]
            {
                self.advance(lexer);
                lexer.set_result_symbol(HTML_BLOCK_4_START as u16);
                if !self.simulate {
                    self.push_block(Block::ANONYMOUS);
                }
                return true;
            } else if lexer.lookahead() == i32::from(b'[') {
                self.advance(lexer);
                let mut matched = true;
                for &expected in b"CDATA" {
                    if lexer.lookahead() != i32::from(expected) {
                        matched = false;
                        break;
                    }
                    self.advance(lexer);
                }
                if matched
                    && lexer.lookahead() == i32::from(b'[')
                    && valid_symbols[HTML_BLOCK_5_START]
                {
                    self.advance(lexer);
                    lexer.set_result_symbol(HTML_BLOCK_5_START as u16);
                    if !self.simulate {
                        self.push_block(Block::ANONYMOUS);
                    }
                    return true;
                }
            }
        }
        let starting_slash = lexer.lookahead() == i32::from(b'/');
        if starting_slash {
            self.advance(lexer);
        }
        let mut name = [0u8; 10];
        let mut name_length = 0;
        while is_alpha(lexer.lookahead()) {
            if name_length < 10 {
                name[name_length] = (lexer.lookahead() as u8).to_ascii_lowercase();
                name_length += 1;
            } else {
                name_length = 12;
            }
            self.advance(lexer);
        }
        if name_length == 0 {
            return false;
        }
        let mut tag_closed = false;
        if name_length < 11 {
            let name = &name[..name_length];
            let next_symbol_valid = is_space(lexer.lookahead())
                || is_line_end(lexer.lookahead())
                || lexer.lookahead() == i32::from(b'>');
            if next_symbol_valid {
                for tag in HTML_TAG_NAMES_RULE_1 {
                    if name == tag {
                        if starting_slash {
                            if valid_symbols[HTML_BLOCK_1_END] {
                                lexer.set_result_symbol(HTML_BLOCK_1_END as u16);
                                return true;
                            }
                        } else if valid_symbols[HTML_BLOCK_1_START] {
                            lexer.set_result_symbol(HTML_BLOCK_1_START as u16);
                            if !self.simulate {
                                self.push_block(Block::ANONYMOUS);
                            }
                            return true;
                        }
                    }
                }
            }
            if !next_symbol_valid && lexer.lookahead() == i32::from(b'/') {
                self.advance(lexer);
                if lexer.lookahead() == i32::from(b'>') {
                    self.advance(lexer);
                    tag_closed = true;
                }
            }
            if next_symbol_valid || tag_closed {
                for tag in HTML_TAG_NAMES_RULE_7 {
                    if name == tag && valid_symbols[HTML_BLOCK_6_START] {
                        lexer.set_result_symbol(HTML_BLOCK_6_START as u16);
                        if !self.simulate {
                            self.push_block(Block::ANONYMOUS);
                        }
                        return true;
                    }
                }
            }
        }
        if !valid_symbols[HTML_BLOCK_7_START] {
            return false;
        }
        if !tag_closed {
            while is_alnum(lexer.lookahead()) || lexer.lookahead() == i32::from(b'-') {
                self.advance(lexer);
            }
            if !starting_slash {
                let mut had_whitespace = false;
                loop {
                    while is_space(lexer.lookahead()) {
                        had_whitespace = true;
                        self.advance(lexer);
                    }
                    if lexer.lookahead() == i32::from(b'/') {
                        self.advance(lexer);
                        break;
                    }
                    if lexer.lookahead() == i32::from(b'>') {
                        break;
                    }
                    if !had_whitespace {
                        return false;
                    }
                    if !is_alpha(lexer.lookahead())
                        && lexer.lookahead() != i32::from(b'_')
                        && lexer.lookahead() != i32::from(b':')
                    {
                        return false;
                    }
                    had_whitespace = false;
                    self.advance(lexer);
                    while is_alnum(lexer.lookahead())
                        || matches!(lexer.lookahead(), 0x5f | 0x2e | 0x3a | 0x2d)
                    {
                        self.advance(lexer);
                    }
                    while is_space(lexer.lookahead()) {
                        had_whitespace = true;
                        self.advance(lexer);
                    }
                    if lexer.lookahead() == i32::from(b'=') {
                        self.advance(lexer);
                        had_whitespace = false;
                        while is_space(lexer.lookahead()) {
                            self.advance(lexer);
                        }
                        if matches!(lexer.lookahead(), 0x27 | 0x22) {
                            let delimiter = lexer.lookahead();
                            self.advance(lexer);
                            while lexer.lookahead() != delimiter
                                && !is_line_end(lexer.lookahead())
                                && !lexer.eof()
                            {
                                self.advance(lexer);
                            }
                            if lexer.lookahead() != delimiter {
                                return false;
                            }
                            self.advance(lexer);
                        } else {
                            let mut had_one = false;
                            while !is_space(lexer.lookahead())
                                && !matches!(
                                    lexer.lookahead(),
                                    0x22 | 0x27 | 0x3d | 0x3c | 0x3e | 0x60
                                )
                                && !is_line_end(lexer.lookahead())
                                && !lexer.eof()
                            {
                                self.advance(lexer);
                                had_one = true;
                            }
                            if !had_one {
                                return false;
                            }
                        }
                    }
                }
            } else {
                while is_space(lexer.lookahead()) {
                    self.advance(lexer);
                }
            }
            if lexer.lookahead() != i32::from(b'>') {
                return false;
            }
            self.advance(lexer);
        }
        while is_space(lexer.lookahead()) {
            self.advance(lexer);
        }
        if is_line_end(lexer.lookahead()) {
            lexer.set_result_symbol(HTML_BLOCK_7_START as u16);
            if !self.simulate {
                self.push_block(Block::ANONYMOUS);
            }
            return true;
        }
        false
    }

    fn parse_pipe_table(&mut self, lexer: &mut ScanLexer<'_>) -> bool {
        // The table-start token is zero width. All subsequent work is lookahead.
        self.mark_end(lexer);
        let mut cell_count = 0usize;
        let mut starting_pipe = false;
        let mut ending_pipe = false;
        if lexer.lookahead() == i32::from(b'|') {
            starting_pipe = true;
            self.advance(lexer);
        }
        while !is_line_end(lexer.lookahead()) && !lexer.eof() {
            if lexer.lookahead() == i32::from(b'|') {
                cell_count += 1;
                ending_pipe = true;
                self.advance(lexer);
            } else {
                if !is_space(lexer.lookahead()) {
                    ending_pipe = false;
                }
                if lexer.lookahead() == i32::from(b'\\') {
                    self.advance(lexer);
                    if is_punctuation(lexer.lookahead()) {
                        self.advance(lexer);
                    }
                } else {
                    self.advance(lexer);
                }
            }
        }
        // C declares `empty = true` and never changes it, even for nonempty text.
        if cell_count == 0 && !(starting_pipe && ending_pipe) {
            return false;
        }
        if !ending_pipe {
            cell_count += 1;
        }
        if lexer.lookahead() == i32::from(b'\n') {
            self.advance(lexer);
        } else if lexer.lookahead() == i32::from(b'\r') {
            self.advance(lexer);
            if lexer.lookahead() == i32::from(b'\n') {
                self.advance(lexer);
            }
        } else {
            return false;
        }
        self.snapshot[INDENTATION] = 0;
        self.snapshot[COLUMN] = 0;
        self.consume_indentation(lexer);
        self.simulate = true;
        let mut matched_temp = 0u8;
        while matched_temp < self.block_count() as u8 {
            if self.match_block(lexer, self.block_at(usize::from(matched_temp))) {
                matched_temp = matched_temp.wrapping_add(1);
            } else {
                return false;
            }
        }
        let mut delimiter_cell_count = 0usize;
        if lexer.lookahead() == i32::from(b'|') {
            self.advance(lexer);
        }
        loop {
            while is_space(lexer.lookahead()) {
                self.advance(lexer);
            }
            if lexer.lookahead() == i32::from(b'|') {
                delimiter_cell_count += 1;
                self.advance(lexer);
                continue;
            }
            if lexer.lookahead() == i32::from(b':') {
                self.advance(lexer);
                if lexer.lookahead() != i32::from(b'-') {
                    return false;
                }
            }
            let mut had_one_minus = false;
            while lexer.lookahead() == i32::from(b'-') {
                had_one_minus = true;
                self.advance(lexer);
            }
            if had_one_minus {
                delimiter_cell_count += 1;
            }
            if lexer.lookahead() == i32::from(b':') {
                if !had_one_minus {
                    return false;
                }
                self.advance(lexer);
            }
            while is_space(lexer.lookahead()) {
                self.advance(lexer);
            }
            if lexer.lookahead() == i32::from(b'|') {
                if !had_one_minus {
                    delimiter_cell_count += 1;
                }
                self.advance(lexer);
                continue;
            }
            if !is_line_end(lexer.lookahead()) {
                return false;
            } else {
                break;
            }
        }
        if cell_count != delimiter_cell_count {
            return false;
        }
        lexer.set_result_symbol(PIPE_TABLE_START as u16);
        true
    }

    fn scan_inner(&mut self, lexer: &mut ScanLexer<'_>, valid_symbols: &[bool]) -> bool {
        if valid_symbols[TRIGGER_ERROR] {
            lexer.set_result_symbol(ERROR as u16);
            return true;
        }
        if valid_symbols[CLOSE_BLOCK] {
            self.snapshot[STATE] |= STATE_CLOSE_BLOCK;
            lexer.set_result_symbol(CLOSE_BLOCK as u16);
            return true;
        }
        if lexer.eof() {
            if valid_symbols[TOKEN_EOF] {
                lexer.set_result_symbol(TOKEN_EOF as u16);
                return true;
            }
            if self.inline_count != 0 {
                lexer.set_result_symbol(BLOCK_CLOSE as u16);
                if !self.simulate {
                    self.pop_block();
                }
                return true;
            }
            return false;
        }
        if self.snapshot[STATE] & STATE_MATCHING == 0 {
            self.consume_indentation(lexer);
            if valid_symbols[INDENTED_CHUNK_START]
                && !valid_symbols[NO_INDENTED_CHUNK]
                && self.snapshot[INDENTATION] >= 4
                && !is_line_end(lexer.lookahead())
            {
                lexer.set_result_symbol(INDENTED_CHUNK_START as u16);
                if !self.simulate {
                    self.push_block(Block::INDENTED_CODE);
                }
                self.snapshot[INDENTATION] -= 4;
                return true;
            }
            // Match the full code point, not its low byte.
            match lexer.lookahead() {
                0x0d | 0x0a => {
                    if valid_symbols[BLANK_LINE_START] {
                        lexer.set_result_symbol(BLANK_LINE_START as u16);
                        return true;
                    }
                }
                0x60 => return self.parse_fenced_code_block(b'`', lexer, valid_symbols),
                0x7e => return self.parse_fenced_code_block(b'~', lexer, valid_symbols),
                0x2a => return self.parse_star(lexer, valid_symbols),
                0x5f => return self.parse_thematic_break_underscore(lexer, valid_symbols),
                0x3e => return self.parse_block_quote(lexer, valid_symbols),
                0x23 => return self.parse_atx_heading(lexer, valid_symbols),
                0x3d => return self.parse_setext_underline(lexer, valid_symbols),
                0x2b => return self.parse_plus(lexer, valid_symbols),
                0x30..=0x39 => return self.parse_ordered_list_marker(lexer, valid_symbols),
                0x2d => return self.parse_minus(lexer, valid_symbols),
                0x3c => return self.parse_html_block(lexer, valid_symbols),
                _ => {}
            }
            if !is_line_end(lexer.lookahead()) && valid_symbols[PIPE_TABLE_START] {
                return self.parse_pipe_table(lexer);
            }
        } else {
            let mut partial_success = false;
            while self.snapshot[MATCHED] < self.block_count() as u8 {
                // C promotes the cast u8 to int before subtracting one.
                if i32::from(self.snapshot[MATCHED]) == i32::from(self.block_count() as u8) - 1
                    && self.snapshot[STATE] & STATE_CLOSE_BLOCK != 0
                {
                    if !partial_success {
                        self.snapshot[STATE] &= !STATE_CLOSE_BLOCK;
                    }
                    break;
                }
                if self.match_block(lexer, self.block_at(usize::from(self.snapshot[MATCHED]))) {
                    partial_success = true;
                    self.snapshot[MATCHED] = self.snapshot[MATCHED].wrapping_add(1);
                } else {
                    if self.snapshot[STATE] & STATE_WAS_SOFT_LINE_BREAK != 0 {
                        self.snapshot[STATE] &= !STATE_MATCHING;
                    }
                    break;
                }
            }
            if partial_success {
                if usize::from(self.snapshot[MATCHED]) == self.block_count() {
                    self.snapshot[STATE] &= !STATE_MATCHING;
                }
                lexer.set_result_symbol(BLOCK_CONTINUATION as u16);
                return true;
            }
            if self.snapshot[STATE] & STATE_WAS_SOFT_LINE_BREAK == 0 {
                lexer.set_result_symbol(BLOCK_CLOSE as u16);
                self.pop_block();
                if usize::from(self.snapshot[MATCHED]) == self.block_count() {
                    self.snapshot[STATE] &= !STATE_MATCHING;
                }
                return true;
            }
        }
        if (valid_symbols[LINE_ENDING]
            || valid_symbols[SOFT_LINE_ENDING]
            || valid_symbols[PIPE_TABLE_LINE_ENDING])
            && is_line_end(lexer.lookahead())
        {
            self.consume_newline(lexer);
            self.snapshot[INDENTATION] = 0;
            self.snapshot[COLUMN] = 0;
            if self.snapshot[STATE] & STATE_CLOSE_BLOCK == 0
                && (valid_symbols[SOFT_LINE_ENDING] || valid_symbols[PIPE_TABLE_LINE_ENDING])
            {
                // Unlike the helper, C calls mark_end even during simulation.
                lexer.mark_end();
                self.consume_indentation(lexer);
                self.simulate = true;
                let matched_temp = self.snapshot[MATCHED];
                self.snapshot[MATCHED] = 0;
                let mut one_will_be_matched = false;
                while self.snapshot[MATCHED] < self.block_count() as u8 {
                    if self.match_block(lexer, self.block_at(usize::from(self.snapshot[MATCHED]))) {
                        self.snapshot[MATCHED] = self.snapshot[MATCHED].wrapping_add(1);
                        one_will_be_matched = true;
                    } else {
                        break;
                    }
                }
                let all_will_be_matched = usize::from(self.snapshot[MATCHED]) == self.block_count();
                if !lexer.eof() && !self.scan_inner(lexer, &PARAGRAPH_INTERRUPT_SYMBOLS) {
                    // C restores matched_temp here, then immediately resets it.
                    self.snapshot[MATCHED] = 0;
                    self.snapshot[INDENTATION] = 0;
                    self.snapshot[COLUMN] = 0;
                    if one_will_be_matched {
                        self.snapshot[STATE] |= STATE_MATCHING;
                    } else {
                        self.snapshot[STATE] &= !STATE_MATCHING;
                    }
                    if valid_symbols[PIPE_TABLE_LINE_ENDING] {
                        if all_will_be_matched {
                            lexer.set_result_symbol(PIPE_TABLE_LINE_ENDING as u16);
                            return true;
                        }
                    } else {
                        lexer.set_result_symbol(SOFT_LINE_ENDING as u16);
                        self.snapshot[STATE] |= STATE_WAS_SOFT_LINE_BREAK;
                        return true;
                    }
                } else {
                    self.snapshot[MATCHED] = matched_temp;
                }
                self.snapshot[INDENTATION] = 0;
                self.snapshot[COLUMN] = 0;
            }
            if valid_symbols[LINE_ENDING] {
                self.snapshot[MATCHED] = 0;
                if self.inline_count != 0 {
                    self.snapshot[STATE] |= STATE_MATCHING;
                } else {
                    self.snapshot[STATE] &= !STATE_MATCHING;
                }
                self.snapshot[STATE] &= !STATE_WAS_SOFT_LINE_BREAK;
                lexer.set_result_symbol(LINE_ENDING as u16);
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    #[derive(Debug, PartialEq, Eq)]
    enum Event {
        Advance(usize),
        Mark(usize),
        Symbol(usize),
    }

    struct TestLexer {
        input: Vec<i32>,
        position: usize,
        end: Option<usize>,
        symbol: u16,
        events: Vec<Event>,
        lookahead_calls: Cell<usize>,
        eof_calls: Cell<usize>,
    }

    impl TestLexer {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().map(|c| c as i32).collect(),
                position: 0,
                end: None,
                symbol: u16::MAX,
                events: Vec::new(),
                lookahead_calls: Cell::new(0),
                eof_calls: Cell::new(0),
            }
        }
    }

    impl Lexer for TestLexer {
        fn lookahead(&self) -> i32 {
            self.lookahead_calls.set(self.lookahead_calls.get() + 1);
            self.input.get(self.position).copied().unwrap_or(0)
        }
        fn result_symbol(&self) -> u16 {
            self.symbol
        }
        fn set_result_symbol(&mut self, symbol: u16) {
            self.symbol = symbol;
            self.events.push(Event::Symbol(usize::from(symbol)));
        }
        fn advance(&mut self, skip: bool) {
            assert!(!skip, "the Markdown scanner never skips input");
            self.events.push(Event::Advance(self.position));
            if self.position < self.input.len() {
                self.position += 1;
            }
        }
        fn mark_end(&mut self) {
            self.end = Some(self.position);
            self.events.push(Event::Mark(self.position));
        }
        fn get_column(&mut self) -> u32 {
            panic!("the scanner maintains its own tab-expanded column")
        }
        fn is_at_included_range_start(&self) -> bool {
            panic!("not used by this scanner")
        }
        fn eof(&self) -> bool {
            self.eof_calls.set(self.eof_calls.get() + 1);
            self.position == self.input.len()
        }
    }

    fn valid(tokens: &[usize]) -> [bool; 47] {
        let mut result = [false; 47];
        for &token in tokens {
            result[token] = true;
        }
        result
    }

    fn serialized(scanner: &mut dyn ExternalScanner) -> Vec<u8> {
        let mut buffer = [0u8; ts_port_tables::SERIALIZATION_BUFFER_SIZE];
        let size = scanner.serialize(&mut buffer);
        buffer[..size].to_vec()
    }

    #[test]
    fn scan_lexer_caches_each_position_but_distinguishes_nul_from_eof() {
        let input = "x\t\0é\r\n";
        let mut lexer = TestLexer::new(input);
        {
            let mut cached = ScanLexer::new(&mut lexer);
            for c in input.chars() {
                assert_eq!(cached.lookahead(), c as i32);
                assert_eq!(cached.lookahead(), c as i32);
                assert!(!cached.eof());
                cached.mark_end();
                cached.set_result_symbol(PIPE_TABLE_START as u16);
                assert_eq!(cached.lookahead(), c as i32);
                cached.advance(false);
            }
            assert_eq!(cached.lookahead(), 0);
            assert!(cached.eof());
        }
        assert_eq!(lexer.lookahead_calls.get(), input.chars().count() + 1);
        // Only embedded NUL and actual EOF need the dynamic EOF query.
        assert_eq!(lexer.eof_calls.get(), 2);
    }

    #[test]
    fn table_and_paragraph_lookahead_share_the_scan_cache() {
        for (input, tokens, symbol) in [
            (
                "a\0b | c\r\n- | -\r\n",
                valid(&[PIPE_TABLE_START]),
                PIPE_TABLE_START,
            ),
            (
                "a\\ż | c\n- | -\n",
                valid(&[PIPE_TABLE_START]),
                PIPE_TABLE_START,
            ),
            (
                "\ncontinued text\n",
                valid(&[LINE_ENDING, SOFT_LINE_ENDING]),
                SOFT_LINE_ENDING,
            ),
        ] {
            let mut scanner = Scanner::default();
            let mut lexer = TestLexer::new(input);
            assert!(scanner.scan(&mut lexer, &tokens), "{input:?}");
            assert_eq!(usize::from(lexer.symbol), symbol);
            // Includes the recursively simulated scan after a newline. Reading
            // lookahead in the column tracker must not add another trait call.
            assert_eq!(lexer.lookahead_calls.get(), lexer.position + 1);
            assert_eq!(lexer.eof_calls.get(), usize::from(input.contains('\0')));
        }
    }

    #[test]
    fn inline_snapshots_spill_restore_and_pop_without_changing_wire_bytes() {
        assert!(std::mem::size_of::<Scanner>() <= 64);
        let mut scanner = Scanner::default();
        let mut capacity = 0;
        for count in [0, 1, 2, 3, 4, 5, 21, 254, 3, 0] {
            let mut expected = vec![STATE_MATCHING, 1, 255, 3, 255];
            for i in 0..count {
                expected.extend_from_slice(&(u32::MAX - i).to_ne_bytes());
            }
            scanner.simulate = true;
            scanner.deserialize(&expected);
            assert_eq!(scanner.block_count(), count as usize);
            assert_eq!(serialized(&mut scanner), expected);
            assert!(scanner.simulate);
            assert!(scanner.spilled_blocks.capacity() >= capacity);
            capacity = scanner.spilled_blocks.capacity();

            // The header remains mutable when the blocks are on the heap.
            scanner.snapshot[STATE] = STATE_CLOSE_BLOCK;
            expected[STATE] = STATE_CLOSE_BLOCK;
            for remaining in (0..count).rev() {
                scanner.pop_block();
                expected.truncate(HEADER_SIZE + 4 * remaining as usize);
                assert_eq!(serialized(&mut scanner), expected);
            }
            scanner.pop_block();
            assert_eq!(serialized(&mut scanner), expected);
            scanner.deserialize(&[]);
            assert_eq!(serialized(&mut scanner), [0; HEADER_SIZE]);
            assert_eq!(scanner.spilled_blocks.capacity(), capacity);
            assert!(scanner.simulate);
        }
    }

    #[test]
    fn snapshot_restoration_ignores_incomplete_trailing_blocks() {
        for count in 0..=6 {
            for trailing in 1..=3 {
                let mut scanner = Scanner::with_blocks((0..count).map(Block));
                let expected = serialized(&mut scanner);
                let mut input = expected.clone();
                input.extend(std::iter::repeat_n(0xff, trailing));
                scanner.deserialize(&input);
                assert_eq!(serialized(&mut scanner), expected);
                for _ in 0..count {
                    scanner.pop_block();
                }
                assert_eq!(scanner.inline_count, 0);
            }
        }
    }

    #[test]
    fn pushing_after_pop_preserves_the_inline_prefix_and_reuses_spill_storage() {
        let mut scanner = Scanner::default();
        let mut expected = vec![0; HEADER_SIZE];
        for round in 0..3 {
            for i in 0..12 {
                let block = Block(0x12345600 + round * 12 + i);
                scanner.push_block(block);
                expected.extend_from_slice(&block.0.to_ne_bytes());
                assert_eq!(scanner.block_at(i as usize), block);
                assert_eq!(serialized(&mut scanner), expected);
                if i < INLINE_BLOCKS as u32 && round == 0 {
                    assert_eq!(scanner.spilled_blocks.capacity(), 0);
                }
            }
            for count in (0..12).rev() {
                scanner.pop_block();
                expected.truncate(HEADER_SIZE + 4 * count);
                assert_eq!(serialized(&mut scanner), expected);
            }
        }
    }

    #[test]
    fn serialization_has_five_state_bytes_and_native_u32_blocks() {
        let mut scanner = Scanner::with_blocks((0..=20).map(Block));
        scanner.snapshot[..5].copy_from_slice(&[STATE_MATCHING | STATE_CLOSE_BLOCK, 3, 255, 2, 4]);
        scanner.simulate = true;
        let mut expected = vec![17, 3, 255, 2, 4];
        for block in 0u32..=20 {
            expected.extend_from_slice(&block.to_ne_bytes());
        }
        assert_eq!(serialized(&mut scanner), expected);
        let mut restored = Scanner::default();
        restored.deserialize(&expected);
        assert_eq!(serialized(&mut restored), expected);
        assert!(!restored.simulate);
        scanner.deserialize(&[]);
        assert_eq!(serialized(&mut scanner), [0; 5]);
        assert!(scanner.simulate);
        assert_eq!(serialized(create().as_mut()), [0; 5]);
    }

    #[test]
    fn quote_splits_a_tab_and_preserves_callback_order() {
        let mut scanner = Scanner::default();
        let mut lexer = TestLexer::new(">\ttext");
        assert!(scanner.scan(&mut lexer, &valid(&[BLOCK_QUOTE_START])));
        assert_eq!(scanner.snapshot[INDENTATION], 2);
        assert_eq!(scanner.snapshot[COLUMN], 0);
        assert_eq!(scanner.blocks(), [Block::QUOTE]);
        assert_eq!(
            lexer.events,
            [
                Event::Advance(0),
                Event::Advance(1),
                Event::Symbol(BLOCK_QUOTE_START)
            ]
        );
    }

    #[test]
    fn star_list_mark_is_before_the_second_star_not_after_lookahead() {
        let mut scanner = Scanner::default();
        let mut lexer = TestLexer::new("* * x");
        assert!(scanner.scan(&mut lexer, &valid(&[LIST_MARKER_STAR, THEMATIC_BREAK])));
        assert_eq!(
            lexer.events,
            [
                Event::Advance(0),
                Event::Mark(1),
                Event::Advance(1),
                Event::Mark(2),
                Event::Advance(2),
                Event::Advance(3),
                Event::Symbol(LIST_MARKER_STAR),
            ]
        );
        assert_eq!(scanner.blocks(), [Block::LIST_ITEM]);
    }

    #[test]
    fn ordered_markers_preserve_c_interrupt_quirks() {
        for (input, allowed, emitted, block) in [
            ("2. text", LIST_MARKER_DOT, LIST_MARKER_DOT, Block(3)),
            (
                "12. text",
                LIST_MARKER_DOT_DONT_INTERRUPT,
                LIST_MARKER_DOT,
                Block(4),
            ),
            (
                "12) text",
                LIST_MARKER_PARENTHESIS_DONT_INTERRUPT,
                LIST_MARKER_PARENTHESIS,
                Block(4),
            ),
        ] {
            let mut scanner = Scanner::default();
            let mut lexer = TestLexer::new(input);
            assert!(scanner.scan(&mut lexer, &valid(&[allowed])));
            assert_eq!(usize::from(lexer.symbol), emitted);
            assert_eq!(scanner.blocks(), [block]);
            assert_eq!(lexer.end, None);
        }
    }

    #[test]
    fn fenced_blocks_mark_only_the_delimiter_and_closing_takes_precedence() {
        let mut scanner = Scanner::default();
        let mut lexer = TestLexer::new("````rust\n");
        assert!(scanner.scan(&mut lexer, &valid(&[FENCED_CODE_BLOCK_START_BACKTICK])));
        assert_eq!(lexer.end, Some(4));
        assert_eq!(lexer.position, 8);
        assert_eq!(scanner.snapshot[FENCE_LENGTH], 4);
        assert_eq!(scanner.blocks(), [Block::FENCED_CODE]);

        let mut lexer = TestLexer::new("`````   \r\n");
        assert!(scanner.scan(
            &mut lexer,
            &valid(&[
                FENCED_CODE_BLOCK_START_BACKTICK,
                FENCED_CODE_BLOCK_END_BACKTICK
            ])
        ));
        assert_eq!(usize::from(lexer.symbol), FENCED_CODE_BLOCK_END_BACKTICK);
        assert_eq!(lexer.end, Some(5));
        assert_eq!(lexer.position, 8);
        assert_eq!(scanner.snapshot[FENCE_LENGTH], 0);
        // Ending the delimiter does not itself pop the block.
        assert_eq!(scanner.blocks(), [Block::FENCED_CODE]);
    }

    #[test]
    fn byte_sized_counters_wrap() {
        let mut scanner = Scanner::default();
        let mut lexer = TestLexer::new(&format!("{}\n", "`".repeat(259)));
        assert!(scanner.scan(&mut lexer, &valid(&[FENCED_CODE_BLOCK_START_BACKTICK])));
        assert_eq!(scanner.snapshot[FENCE_LENGTH], 3);
        assert_eq!(lexer.end, Some(259));
        scanner.deserialize(&[]);
        let mut lexer = TestLexer::new(&format!("{}text", " ".repeat(260)));
        assert!(scanner.scan(&mut lexer, &valid(&[INDENTED_CHUNK_START])));
        assert_eq!(scanner.snapshot[INDENTATION], 0);
        assert_eq!(scanner.blocks(), [Block::INDENTED_CODE]);
        scanner.deserialize(&[]);
        let mut lexer = TestLexer::new(&format!("+{}text", " ".repeat(256)));
        assert!(!scanner.scan(&mut lexer, &valid(&[LIST_MARKER_PLUS])));
    }

    #[test]
    fn metadata_requires_a_newline_after_the_closing_marker() {
        for (marker, symbol) in [("---", MINUS_METADATA), ("+++", PLUS_METADATA)] {
            let mut scanner = Scanner::default();
            let source = format!("{marker}\r\nkey = value\r\n{marker} \r\nafter");
            let mut lexer = TestLexer::new(&source);
            assert!(scanner.scan(&mut lexer, &valid(&[symbol])));
            assert_eq!(usize::from(lexer.symbol), symbol);
            assert_eq!(lexer.end, Some(source.len() - 5));
            assert!(scanner.blocks().is_empty());
            let mut scanner = Scanner::default();
            let mut lexer = TestLexer::new(&format!("{marker}\nkey = value\n{marker}"));
            assert!(!scanner.scan(&mut lexer, &valid(&[symbol])));
        }
        // A failed metadata lookahead keeps an already-recognized thematic break.
        let mut scanner = Scanner::default();
        let mut lexer = TestLexer::new("---\nbody\n---");
        assert!(scanner.scan(&mut lexer, &valid(&[MINUS_METADATA, THEMATIC_BREAK])));
        assert_eq!(usize::from(lexer.symbol), THEMATIC_BREAK);
        assert_eq!(lexer.end, Some(3));
        assert!(lexer.eof());
    }

    #[test]
    fn html_rules_and_c_locale_classification() {
        let symbols = valid(&[
            HTML_BLOCK_1_START,
            HTML_BLOCK_1_END,
            HTML_BLOCK_2_START,
            HTML_BLOCK_3_START,
            HTML_BLOCK_4_START,
            HTML_BLOCK_5_START,
            HTML_BLOCK_6_START,
            HTML_BLOCK_7_START,
        ]);
        for (input, symbol, position) in [
            ("<ScRiPt>\n", HTML_BLOCK_1_START, 7),
            ("</style>\n", HTML_BLOCK_1_END, 7),
            ("<!--text", HTML_BLOCK_2_START, 4),
            ("<?text", HTML_BLOCK_3_START, 2),
            ("<!DOCTYPE html>", HTML_BLOCK_4_START, 3),
            ("<![CDATA[text", HTML_BLOCK_5_START, 9),
            ("<div/>\n", HTML_BLOCK_6_START, 6),
            // C's first name loop consumes letters, so h1 is not rule 6.
            ("<h1>\n", HTML_BLOCK_7_START, 4),
            ("<custom a='x y'>\n", HTML_BLOCK_7_START, 16),
        ] {
            let mut scanner = Scanner::default();
            let mut lexer = TestLexer::new(input);
            assert!(scanner.scan(&mut lexer, &symbols), "{input}");
            assert_eq!(usize::from(lexer.symbol), symbol, "{input}");
            assert_eq!(lexer.position, position, "{input}");
            assert_eq!(lexer.end, None);
            assert_eq!(
                scanner.blocks().len(),
                usize::from(symbol != HTML_BLOCK_1_END)
            );
        }
        for input in ["<é>\n", "<custom a='unterminated\n", "<custom a=>\n"] {
            assert!(!Scanner::default().scan(&mut TestLexer::new(input), &symbols));
        }
        assert!(is_punctuation(0x17c)); // the C char cast retains the low '|'
        assert!(!is_alpha(0xe9));
        assert!(!is_digit(0x661));
    }

    #[test]
    fn pipe_table_start_is_zero_width_and_checks_cell_counts() {
        for input in ["a | b\n- | -\n", "|a|\n|-|\n", "a\\|b | c\n- | -\n"] {
            let mut scanner = Scanner::default();
            let mut lexer = TestLexer::new(input);
            assert!(
                scanner.scan(&mut lexer, &valid(&[PIPE_TABLE_START])),
                "{input}"
            );
            assert_eq!(lexer.end, Some(0));
            assert_eq!(lexer.position, input.len() - 1);
            assert_eq!(usize::from(lexer.symbol), PIPE_TABLE_START);
            assert!(scanner.simulate);
            assert!(scanner.blocks().is_empty());
        }
        for input in ["a | b\n-\n", "text\n---\n", "a | b\n- | -"] {
            assert!(
                !Scanner::default().scan(&mut TestLexer::new(input), &valid(&[PIPE_TABLE_START])),
                "{input}"
            );
        }
    }

    #[test]
    fn newline_simulation_keeps_the_newline_mark_and_does_not_push() {
        let mut scanner = Scanner::default();
        let mut lexer = TestLexer::new("\n# heading\n");
        assert!(scanner.scan(&mut lexer, &valid(&[LINE_ENDING, SOFT_LINE_ENDING])));
        assert_eq!(
            lexer.events,
            [
                Event::Advance(0),
                Event::Mark(1),
                Event::Advance(1),
                Event::Symbol(ATX_H1_MARKER),
                Event::Symbol(LINE_ENDING),
            ]
        );
        assert_eq!(scanner.snapshot[STATE], 0);

        let mut lexer = TestLexer::new("\n``` rust\n");
        assert!(scanner.scan(&mut lexer, &valid(&[LINE_ENDING, SOFT_LINE_ENDING])));
        assert_eq!(lexer.end, Some(1));
        assert_eq!(usize::from(lexer.symbol), LINE_ENDING);
        assert!(scanner.blocks().is_empty());
        // C's simulation does mutate the remembered delimiter length.
        assert_eq!(scanner.snapshot[FENCE_LENGTH], 3);

        let mut lexer = TestLexer::new("\ncontinued\n");
        assert!(scanner.scan(&mut lexer, &valid(&[LINE_ENDING, SOFT_LINE_ENDING])));
        assert_eq!(lexer.end, Some(1));
        assert_eq!(usize::from(lexer.symbol), SOFT_LINE_ENDING);
        assert_eq!(scanner.snapshot[STATE], STATE_WAS_SOFT_LINE_BREAK);
    }

    #[test]
    fn matching_yields_outer_continuation_before_requested_inner_close() {
        let mut scanner = Scanner::with_blocks([Block::QUOTE, Block::FENCED_CODE]);
        scanner.snapshot[STATE] = STATE_MATCHING | STATE_CLOSE_BLOCK;
        let mut lexer = TestLexer::new("> text");
        assert!(scanner.scan(&mut lexer, &valid(&[])));
        assert_eq!(usize::from(lexer.symbol), BLOCK_CONTINUATION);
        assert_eq!(scanner.snapshot[MATCHED], 1);
        assert_eq!(scanner.snapshot[STATE], STATE_MATCHING | STATE_CLOSE_BLOCK);
        assert!(scanner.scan(&mut lexer, &valid(&[])));
        assert_eq!(usize::from(lexer.symbol), BLOCK_CLOSE);
        assert_eq!(scanner.blocks(), [Block::QUOTE]);
        assert_eq!(scanner.snapshot[STATE], 0);
        assert_eq!(lexer.position, 2);
    }

    #[test]
    fn eof_and_error_requests_take_precedence_without_advancing() {
        let mut scanner = Scanner::with_blocks([Block::QUOTE]);
        let mut lexer = TestLexer::new("");
        assert!(scanner.scan(&mut lexer, &valid(&[TOKEN_EOF, CLOSE_BLOCK, TRIGGER_ERROR])));
        assert_eq!(lexer.events, [Event::Symbol(ERROR)]);
        assert_eq!(scanner.snapshot[STATE], 0);
        assert!(scanner.scan(&mut lexer, &valid(&[TOKEN_EOF, CLOSE_BLOCK])));
        assert_eq!(usize::from(lexer.symbol), CLOSE_BLOCK);
        assert_eq!(scanner.snapshot[STATE], STATE_CLOSE_BLOCK);
        assert!(scanner.scan(&mut lexer, &valid(&[TOKEN_EOF])));
        assert_eq!(usize::from(lexer.symbol), TOKEN_EOF);
        assert_eq!(scanner.blocks(), [Block::QUOTE]);
        assert!(scanner.scan(&mut lexer, &valid(&[])));
        assert_eq!(usize::from(lexer.symbol), BLOCK_CLOSE);
        assert!(scanner.blocks().is_empty());
        assert!(!scanner.scan(&mut lexer, &valid(&[])));
    }
}
