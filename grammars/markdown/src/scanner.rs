//! The Markdown block scanner, translated from tree-sitter-markdown/src/scanner.c.

use std::sync::OnceLock;

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

pub(crate) struct Scanner {
    // A header and four native-endian blocks: all snapshots small enough for
    // the runtime's short external-token state stay inline here as well.
    snapshot: [u8; HEADER_SIZE + 4 * INLINE_BLOCKS],
    spilled_blocks: Vec<[u8; 4]>,
    // 0..=4 is the inline block count; 5 means use spilled_blocks.len().
    inline_count: u8,
    simulate: bool,
    modes: &'static ModeSymbols,
    // The hottest row is held directly to avoid a dependent modes-table load
    // on every text scan. The complete scanner still fits in 64 bytes.
    text_mode: &'static [bool; 47],
}

impl Default for Scanner {
    fn default() -> Self {
        Self {
            snapshot: [0; HEADER_SIZE + 4 * INLINE_BLOCKS],
            spilled_blocks: Vec::new(),
            inline_count: 0,
            simulate: false,
            modes: &DEFAULT_MODES,
            text_mode: &TEXT_FLAGS,
        }
    }
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

// All 47 real flags fit below this sentinel. Common text, fenced-code and
// block-line states, plus fixed paragraph-interruption lookahead, each get a
// specialized dispatcher. Other token sets share the dynamic implementation.
const DYNAMIC_SYMBOLS: u64 = u64::MAX;
const TEXT_SYMBOLS: u64 = (1 << LINE_ENDING) | (1 << SOFT_LINE_ENDING) | (1 << TOKEN_EOF);
const CODE_SYMBOLS: u64 = (1 << LINE_ENDING) | (1 << BLOCK_CLOSE)
    | (1 << FENCED_CODE_BLOCK_END_BACKTICK);
const BLOCK_LINE_SYMBOLS: u64 = (1 << LINE_ENDING) | (1 << BLOCK_CLOSE);

const fn flags_for(mask: u64) -> [bool; 47] {
    let mut flags = [false; 47];
    let mut i = 0;
    while i < flags.len() {
        flags[i] = mask & (1 << i) != 0;
        i += 1;
    }
    flags
}

static TEXT_FLAGS: [bool; 47] = flags_for(TEXT_SYMBOLS);
static CODE_FLAGS: [bool; 47] = flags_for(CODE_SYMBOLS);
static BLOCK_LINE_FLAGS: [bool; 47] = flags_for(BLOCK_LINE_SYMBOLS);

struct ModeSymbols {
    text: &'static [bool; 47],
    code: &'static [bool; 47],
    block_line: &'static [bool; 47],
}

static DEFAULT_MODES: ModeSymbols = ModeSymbols {
    text: &TEXT_FLAGS,
    code: &CODE_FLAGS,
    block_line: &BLOCK_LINE_FLAGS,
};
static GRAMMAR_MODES: OnceLock<ModeSymbols> = OnceLock::new();

impl ModeSymbols {
    fn new() -> Self {
        let rows = crate::language()
            .external_scanner
            .as_ref()
            .unwrap()
            .states
            .as_chunks::<47>()
            .0;
        let find = |flags: &'static [bool; 47]| {
            rows.iter().find(|row| *row == flags).unwrap_or(flags)
        };
        Self {
            text: find(&TEXT_FLAGS),
            code: find(&CODE_FLAGS),
            block_line: find(&BLOCK_LINE_FLAGS),
        }
    }
}

const PARAGRAPH_SYMBOLS: u64 = {
    let mut flags = 0;
    let mut i = 0;
    while i < PARAGRAPH_INTERRUPT_SYMBOLS.len() {
        flags |= (PARAGRAPH_INTERRUPT_SYMBOLS[i] as u64) << i;
        i += 1;
    }
    flags
};

/// Fixed modes retain the same control flow while exposing token validity as
/// compile-time constants. The dynamic mode reads the caller's actual flags.
#[derive(Clone, Copy)]
struct Symbols<'a, const FLAGS: u64>(&'a [bool; 47]);

impl<const FLAGS: u64> Symbols<'_, FLAGS> {
    #[inline]
    fn contains(self, symbol: usize) -> bool {
        if FLAGS == DYNAMIC_SYMBOLS {
            self.0[symbol]
        } else {
            FLAGS & (1 << symbol) != 0
        }
    }
}

impl ExternalScanner for Scanner {
    fn scan(&mut self, lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> bool {
        self.simulate = false;
        // C's token array has exactly 47 entries. Validate that prefix once so
        // the dynamic helpers also avoid carrying a slice length and checking
        // each constant token index. Short caller-provided slices are invalid.
        let Some(valid_symbols) = valid_symbols.first_chunk::<47>() else {
            return false;
        };
        // Specialize the common line-oriented states, rather than skipping
        // invalid handlers: stars, underscores and fences still perform their
        // original advances and marks even when no token can be returned.
        if std::ptr::eq(self.text_mode, valid_symbols) {
            self.scan_content::<TEXT_SYMBOLS>(lexer)
        } else if std::ptr::eq(self.modes.code, valid_symbols) {
            self.scan_mode::<CODE_SYMBOLS>(lexer, Symbols(valid_symbols))
        } else if std::ptr::eq(self.modes.block_line, valid_symbols) {
            self.scan_mode::<BLOCK_LINE_SYMBOLS>(lexer, Symbols(valid_symbols))
        } else {
            self.scan_mode::<DYNAMIC_SYMBOLS>(lexer, Symbols(valid_symbols))
        }
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
    // These references come from the grammar's immutable static table, so
    // pointer equality proves each token set without caching arbitrary data.
    // Other rows (including duplicates) safely use dynamic dispatch.
    let modes = GRAMMAR_MODES.get_or_init(ModeSymbols::new);
    Box::new(Scanner {
        modes,
        text_mode: modes.text,
        ..Scanner::default()
    })
}

/// A scan-local copy of C's `TSLexer.lookahead` field. All tests of the current
/// character (including column accounting and recursive paragraph lookahead)
/// share one trait call. The plain table-header run keeps this cache in a local
/// instead, refreshing it on exit before returning to the scanner's helpers.
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

// A conservative low-six-bit filter: false positives (including ASCII aliases
// such as 'j' for '*') go through full-code-point dispatch. A register bit test
// avoids a character-table load in the common text rejection path.
const TEXT_ACTIVE: u64 = {
    let chars = [0, b'\t', b'\n', b'\r', b' ', b'*', b'_', b'`', b'~'];
    let mut mask = 0;
    let mut i = 0;
    while i < chars.len() {
        mask |= 1u64 << (chars[i] % 64);
        i += 1;
    }
    mask
};

// Use seven bits for header runs: ASCII letters must not interrupt this hot
// loop. Unicode aliases may end a run, but the outer loop compares full code
// points, consumes each alias normally, and resumes the run.
const TABLE_HEADER_BOUNDARY: [bool; 128] = {
    let mut boundary = [false; 128];
    boundary[0] = true;
    boundary[b'\t' as usize] = true;
    boundary[b'\r' as usize] = true;
    boundary[b'\n' as usize] = true;
    boundary[b'|' as usize] = true;
    boundary[b'\\' as usize] = true;
    boundary
};

impl Scanner {
    // Keep ordinary text rejection in the external entry: it needs no cached
    // lexer on the stack, validity slice, or delimiter-dispatch frame. C's star,
    // underscore and fence attempts still advance even when their tokens are
    // invalid, so those characters must always reach the full scanner.
    #[inline(always)]
    fn scan_content<const FLAGS: u64>(&mut self, lexer: &mut dyn Lexer) -> bool {
        let mut lookahead = lexer.lookahead();
        if self.snapshot[STATE] & STATE_MATCHING == 0 && lookahead != 0 {
            if lookahead == i32::from(b' ') {
                self.snapshot[COLUMN] = self.snapshot[COLUMN].wrapping_add(1) % 4;
                self.snapshot[INDENTATION] = self.snapshot[INDENTATION].wrapping_add(1);
                lexer.advance(false);
                lookahead = lexer.lookahead();
                // C checks EOF only before indentation. After a space, both
                // actual EOF and an embedded NUL take ordinary text rejection.
                if lookahead == 0 {
                    return false;
                }
            }
            if TEXT_ACTIVE & 1u64.wrapping_shl(lookahead as u32) == 0 {
                return false;
            }
        }
        self.scan_content_slow::<FLAGS>(lexer, lookahead)
    }

    #[inline(never)]
    fn scan_content_slow<const FLAGS: u64>(&mut self, lexer: &mut dyn Lexer, lookahead: i32) -> bool {
        self.scan_inner::<FLAGS>(
            &mut ScanLexer { inner: lexer, lookahead },
            Symbols(&flags_for(FLAGS)),
        )
    }

    // Choose the other modes before calling the lexer. Their fixed entries
    // need not keep a dynamic validity slice live across lexer calls.
    #[inline(never)]
    fn scan_mode<const FLAGS: u64>(
        &mut self,
        lexer: &mut dyn Lexer,
        valid_symbols: Symbols<'_, FLAGS>,
    ) -> bool {
        self.scan_inner(&mut ScanLexer::new(lexer), valid_symbols)
    }

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

    // Most scanner calls are not at whitespace. Keep their dispatch small:
    // only set up the column/indentation loop when there is a run to consume.
    #[inline]
    fn consume_indentation(&mut self, lexer: &mut ScanLexer<'_>) {
        if is_space(lexer.lookahead()) {
            self.consume_indentation_run(lexer);
        }
    }

    #[inline(never)]
    fn consume_indentation_run(&mut self, lexer: &mut ScanLexer<'_>) {
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

    // Even an invalid fence consumes its delimiters and marks the end in C.
    // Keep these required loops out of the common text entry's stack frame.
    #[inline(never)]
    fn parse_fenced_code_block<const FLAGS: u64>(
        &mut self,
        delimiter: u8,
        lexer: &mut ScanLexer<'_>,
        valid_symbols: Symbols<'_, FLAGS>,
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
        if valid_symbols.contains(end)
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
        if valid_symbols.contains(start) && level >= 3 {
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

    #[inline(never)]
    fn parse_star<const FLAGS: u64>(
        &mut self,
        lexer: &mut ScanLexer<'_>,
        valid_symbols: Symbols<'_, FLAGS>,
    ) -> bool {
        self.advance(lexer);
        self.mark_end(lexer);
        let mut star_count = 1usize;
        let mut extra_indentation = 0u8;
        loop {
            if lexer.lookahead() == i32::from(b'*') {
                if star_count == 1 && extra_indentation >= 1 && valid_symbols.contains(LIST_MARKER_STAR) {
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
        if valid_symbols.contains(THEMATIC_BREAK) && thematic_break && self.snapshot[INDENTATION] < 4 {
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
        if valid_symbols.contains(symbol) && list_marker_star {
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

    #[inline(never)]
    fn parse_thematic_break_underscore<const FLAGS: u64>(
        &mut self,
        lexer: &mut ScanLexer<'_>,
        valid_symbols: Symbols<'_, FLAGS>,
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
        if underscore_count >= 3 && is_line_end(lexer.lookahead()) && valid_symbols.contains(THEMATIC_BREAK)
        {
            lexer.set_result_symbol(THEMATIC_BREAK as u16);
            self.mark_end(lexer);
            self.snapshot[INDENTATION] = 0;
            return true;
        }
        false
    }

    fn parse_block_quote<const FLAGS: u64>(
        &mut self,
        lexer: &mut ScanLexer<'_>,
        valid_symbols: Symbols<'_, FLAGS>,
    ) -> bool {
        if valid_symbols.contains(BLOCK_QUOTE_START) {
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

    fn parse_atx_heading<const FLAGS: u64>(
        &mut self,
        lexer: &mut ScanLexer<'_>,
        valid_symbols: Symbols<'_, FLAGS>,
    ) -> bool {
        if valid_symbols.contains(ATX_H1_MARKER) && self.snapshot[INDENTATION] <= 3 {
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

    fn parse_setext_underline<const FLAGS: u64>(
        &mut self,
        lexer: &mut ScanLexer<'_>,
        valid_symbols: Symbols<'_, FLAGS>,
    ) -> bool {
        if valid_symbols.contains(SETEXT_H1_UNDERLINE)
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

    fn parse_plus<const FLAGS: u64>(
        &mut self,
        lexer: &mut ScanLexer<'_>,
        valid_symbols: Symbols<'_, FLAGS>,
    ) -> bool {
        if self.snapshot[INDENTATION] <= 3
            && (valid_symbols.contains(LIST_MARKER_PLUS)
                || valid_symbols.contains(LIST_MARKER_PLUS_DONT_INTERRUPT)
                || valid_symbols.contains(PLUS_METADATA))
        {
            self.advance(lexer);
            if valid_symbols.contains(PLUS_METADATA) && lexer.lookahead() == i32::from(b'+') {
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
                if extra_indentation >= 1 && valid_symbols.contains(symbol) {
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

    fn parse_ordered_list_marker<const FLAGS: u64>(
        &mut self,
        lexer: &mut ScanLexer<'_>,
        valid_symbols: Symbols<'_, FLAGS>,
    ) -> bool {
        if self.snapshot[INDENTATION] <= 3
            && (valid_symbols.contains(LIST_MARKER_PARENTHESIS)
                || valid_symbols.contains(LIST_MARKER_DOT)
                || valid_symbols.contains(LIST_MARKER_PARENTHESIS_DONT_INTERRUPT)
                || valid_symbols.contains(LIST_MARKER_DOT_DONT_INTERRUPT))
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
                    if extra_indentation >= 1 && valid_symbols.contains(symbol) {
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

    fn parse_minus<const FLAGS: u64>(
        &mut self,
        lexer: &mut ScanLexer<'_>,
        valid_symbols: Symbols<'_, FLAGS>,
    ) -> bool {
        if self.snapshot[INDENTATION] <= 3
            && (valid_symbols.contains(LIST_MARKER_MINUS)
                || valid_symbols.contains(LIST_MARKER_MINUS_DONT_INTERRUPT)
                || valid_symbols.contains(SETEXT_H2_UNDERLINE)
                || valid_symbols.contains(THEMATIC_BREAK)
                || valid_symbols.contains(MINUS_METADATA))
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
            if valid_symbols.contains(SETEXT_H2_UNDERLINE) && underline {
                lexer.set_result_symbol(SETEXT_H2_UNDERLINE as u16);
                self.mark_end(lexer);
                self.snapshot[INDENTATION] = 0;
                success = true;
            } else if valid_symbols.contains(THEMATIC_BREAK) && thematic_break {
                lexer.set_result_symbol(THEMATIC_BREAK as u16);
                self.mark_end(lexer);
                self.snapshot[INDENTATION] = 0;
                success = true;
            } else if valid_symbols.contains(symbol) && list_marker_minus {
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
                && valid_symbols.contains(MINUS_METADATA)
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

    fn parse_html_block<const FLAGS: u64>(
        &mut self,
        lexer: &mut ScanLexer<'_>,
        valid_symbols: Symbols<'_, FLAGS>,
    ) -> bool {
        if !(valid_symbols.contains(HTML_BLOCK_1_START)
            || valid_symbols.contains(HTML_BLOCK_1_END)
            || valid_symbols.contains(HTML_BLOCK_2_START)
            || valid_symbols.contains(HTML_BLOCK_3_START)
            || valid_symbols.contains(HTML_BLOCK_4_START)
            || valid_symbols.contains(HTML_BLOCK_5_START)
            || valid_symbols.contains(HTML_BLOCK_6_START)
            || valid_symbols.contains(HTML_BLOCK_7_START))
        {
            return false;
        }
        self.advance(lexer);
        if lexer.lookahead() == i32::from(b'?') && valid_symbols.contains(HTML_BLOCK_3_START) {
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
                if lexer.lookahead() == i32::from(b'-') && valid_symbols.contains(HTML_BLOCK_2_START) {
                    self.advance(lexer);
                    lexer.set_result_symbol(HTML_BLOCK_2_START as u16);
                    if !self.simulate {
                        self.push_block(Block::ANONYMOUS);
                    }
                    return true;
                }
            } else if (i32::from(b'A')..=i32::from(b'Z')).contains(&lexer.lookahead())
                && valid_symbols.contains(HTML_BLOCK_4_START)
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
                    && valid_symbols.contains(HTML_BLOCK_5_START)
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
                            if valid_symbols.contains(HTML_BLOCK_1_END) {
                                lexer.set_result_symbol(HTML_BLOCK_1_END as u16);
                                return true;
                            }
                        } else if valid_symbols.contains(HTML_BLOCK_1_START) {
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
                    if name == tag && valid_symbols.contains(HTML_BLOCK_6_START) {
                        lexer.set_result_symbol(HTML_BLOCK_6_START as u16);
                        if !self.simulate {
                            self.push_block(Block::ANONYMOUS);
                        }
                        return true;
                    }
                }
            }
        }
        if !valid_symbols.contains(HTML_BLOCK_7_START) {
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

    // Isolate the tight run from the table's counters and cached lexer. Return
    // the boundary character rather than retaining a mutable cache reference
    // across the loop. Only the caller publishes it to the cache on exit.
    #[inline(never)]
    fn consume_plain_header(lexer: &mut dyn Lexer, column: u8) -> (u8, i32) {
        let mut column = u32::from(column);
        loop {
            column = column.wrapping_add(1);
            lexer.advance(false);
            let c = lexer.lookahead();
            if TABLE_HEADER_BOUNDARY[c as usize & 127] {
                return ((column % 4) as u8, c);
            }
        }
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
        loop {
            match lexer.lookahead() {
                0x0a | 0x0d => break,
                0 if lexer.eof() => break,
                0x7c => {
                    cell_count += 1;
                    ending_pipe = true;
                    self.advance(lexer);
                }
                0x20 | 0x09 => {
                    self.advance(lexer);
                }
                0x5c => {
                    ending_pipe = false;
                    self.advance(lexer);
                    if is_punctuation(lexer.lookahead()) {
                        self.advance(lexer);
                    }
                }
                _ => {
                    // Plain header text (including spaces once a cell has
                    // content) needs neither escape/pipe tests nor tab expansion.
                    // Keep its column in a register until the run ends. Wrapping
                    // u32 addition followed by modulo four is equivalent to C's
                    // per-character modulo four, even for very long lines.
                    ending_pipe = false;
                    let (column, lookahead) =
                        Self::consume_plain_header(lexer.inner, self.snapshot[COLUMN]);
                    self.snapshot[COLUMN] = column;
                    lexer.lookahead = lookahead;
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

    fn scan_inner<const FLAGS: u64>(
        &mut self,
        lexer: &mut ScanLexer<'_>,
        valid_symbols: Symbols<'_, FLAGS>,
    ) -> bool {
        if valid_symbols.contains(TRIGGER_ERROR) {
            lexer.set_result_symbol(ERROR as u16);
            return true;
        }
        if valid_symbols.contains(CLOSE_BLOCK) {
            self.snapshot[STATE] |= STATE_CLOSE_BLOCK;
            lexer.set_result_symbol(CLOSE_BLOCK as u16);
            return true;
        }
        if lexer.eof() {
            if valid_symbols.contains(TOKEN_EOF) {
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
            // Within text, a single space is much more common than a run.
            // Perform the same first advance inline, leaving tabs and longer
            // runs to the shared helper without adding setup to other modes.
            if matches!(FLAGS, TEXT_SYMBOLS | CODE_SYMBOLS | BLOCK_LINE_SYMBOLS)
                && lexer.lookahead() == i32::from(b' ') {
                self.snapshot[INDENTATION] =
                    self.snapshot[INDENTATION].wrapping_add(self.advance(lexer));
            }
            self.consume_indentation(lexer);
            if valid_symbols.contains(INDENTED_CHUNK_START)
                && !valid_symbols.contains(NO_INDENTED_CHUNK)
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
                    if valid_symbols.contains(BLANK_LINE_START) {
                        lexer.set_result_symbol(BLANK_LINE_START as u16);
                        return true;
                    }
                }
                0x60 => return self.parse_fenced_code_block::<FLAGS>(b'`', lexer, valid_symbols),
                0x7e => return self.parse_fenced_code_block::<FLAGS>(b'~', lexer, valid_symbols),
                0x2a => return self.parse_star::<FLAGS>(lexer, valid_symbols),
                0x5f => return self.parse_thematic_break_underscore::<FLAGS>(lexer, valid_symbols),
                0x3e => return self.parse_block_quote::<FLAGS>(lexer, valid_symbols),
                0x23 => return self.parse_atx_heading::<FLAGS>(lexer, valid_symbols),
                0x3d => return self.parse_setext_underline::<FLAGS>(lexer, valid_symbols),
                0x2b => return self.parse_plus::<FLAGS>(lexer, valid_symbols),
                0x30..=0x39 => return self.parse_ordered_list_marker::<FLAGS>(lexer, valid_symbols),
                0x2d => return self.parse_minus::<FLAGS>(lexer, valid_symbols),
                0x3c => return self.parse_html_block::<FLAGS>(lexer, valid_symbols),
                _ => {}
            }
            if !is_line_end(lexer.lookahead()) && valid_symbols.contains(PIPE_TABLE_START) {
                return self.parse_pipe_table(lexer);
            }
        } else if self.match_continuation(lexer) {
            return true;
        }
        if (valid_symbols.contains(LINE_ENDING)
            || valid_symbols.contains(SOFT_LINE_ENDING)
            || valid_symbols.contains(PIPE_TABLE_LINE_ENDING))
            && is_line_end(lexer.lookahead())
        {
            return self.parse_line_ending(lexer, valid_symbols);
        }
        false
    }

    #[inline(never)]
    fn match_continuation(&mut self, lexer: &mut ScanLexer<'_>) -> bool {
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
        false
    }

    #[inline(never)]
    fn parse_line_ending<const FLAGS: u64>(
        &mut self,
        lexer: &mut ScanLexer<'_>,
        valid_symbols: Symbols<'_, FLAGS>,
    ) -> bool {
        self.consume_newline(lexer);
        self.snapshot[INDENTATION] = 0;
        self.snapshot[COLUMN] = 0;
        if self.snapshot[STATE] & STATE_CLOSE_BLOCK == 0
            && (valid_symbols.contains(SOFT_LINE_ENDING) || valid_symbols.contains(PIPE_TABLE_LINE_ENDING))
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
            if !lexer.eof() && !self.scan_inner::<PARAGRAPH_SYMBOLS>(lexer, Symbols(&PARAGRAPH_INTERRUPT_SYMBOLS)) {
                // C restores matched_temp here, then immediately resets it.
                self.snapshot[MATCHED] = 0;
                self.snapshot[INDENTATION] = 0;
                self.snapshot[COLUMN] = 0;
                if one_will_be_matched {
                    self.snapshot[STATE] |= STATE_MATCHING;
                } else {
                    self.snapshot[STATE] &= !STATE_MATCHING;
                }
                if valid_symbols.contains(PIPE_TABLE_LINE_ENDING) {
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
        if valid_symbols.contains(LINE_ENDING) {
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
    fn fixed_text_row_is_immutable_and_foreign_flags_remain_dynamic() {
        let rows = &crate::language().external_scanner.as_ref().unwrap().states;
        let text = rows.as_chunks::<47>().0.iter().find(|flags| **flags == TEXT_FLAGS).unwrap();
        let modes = GRAMMAR_MODES.get_or_init(ModeSymbols::new);
        let mut scanner = Scanner { modes, text_mode: modes.text, ..Scanner::default() };
        for _ in 0..2 {
            let mut lexer = TestLexer::new("word");
            assert!(!scanner.scan(&mut lexer, text));
            assert!(lexer.events.is_empty());
        }
        // A mutable array with the same initial bytes must not be cached by
        // address: changing it to request an error must be observed next time.
        let mut foreign = TEXT_FLAGS;
        assert!(!scanner.scan(&mut TestLexer::new("word"), &foreign));
        foreign[TRIGGER_ERROR] = true;
        let mut lexer = TestLexer::new("word");
        assert!(scanner.scan(&mut lexer, &foreign));
        assert_eq!(usize::from(lexer.symbol), ERROR);
        assert_eq!(lexer.events, vec![Event::Symbol(ERROR)]);
        // Factory lookup and a noncanonical view of the table are both safe.
        assert!(!create().scan(&mut TestLexer::new("word"), text));
        let interior = rows[1..].first_chunk::<47>().unwrap();
        let mut scanner = Scanner::default();
        let mut dynamic = Scanner::default();
        let mut actual = TestLexer::new("word");
        let mut expected = TestLexer::new("word");
        assert_eq!(
            scanner.scan(&mut actual, interior),
            dynamic.scan_inner::<DYNAMIC_SYMBOLS>(&mut ScanLexer::new(&mut expected), Symbols(interior)),
        );
        assert_eq!(actual.events, expected.events);
        assert_eq!(serialized(&mut scanner), serialized(&mut dynamic));
    }

    #[test]
    fn text_specialization_matches_dynamic_dispatch() {
        // Exercise the common text specialization and all token flags handled
        // by dynamic dispatch. Compare not only tokens but all state bytes and
        // lexer advances/marks, including unsuccessful speculative scans.
        let mut symbol_sets = vec![
            [false; 47],
            [true; 47],
            PARAGRAPH_INTERRUPT_SYMBOLS,
            valid(&[LINE_ENDING, SOFT_LINE_ENDING, TOKEN_EOF]),
        ];
        for symbol in 0..47 {
            symbol_sets.push(valid(&[symbol]));
            symbol_sets.push(valid(&[LINE_ENDING, SOFT_LINE_ENDING, symbol]));
        }
        for c in (0..=127).map(|c| char::from_u32(c).unwrap()).chain([
            'é', '\u{141}', '\u{161}', '\u{17c}', '\u{10ffff}',
        ]) {
            let input = format!("{c}|b\n-|-\n");
            for (state, indentation, column, blocks) in [
                (0, 0, 0, &[][..]),
                (STATE_WAS_SOFT_LINE_BREAK, 3, 2, &[][..]),
                (0, 4, 3, &[][..]),
                (0, 255, 1, &[][..]),
                (0, 4, 0, &[Block::INDENTED_CODE][..]),
                (STATE_MATCHING, 0, 2, &[Block::QUOTE][..]),
                (STATE_MATCHING, 4, 3, &[Block::INDENTED_CODE][..]),
                (
                    STATE_MATCHING | STATE_WAS_SOFT_LINE_BREAK | STATE_CLOSE_BLOCK,
                    2,
                    1,
                    &[Block::QUOTE, Block::FENCED_CODE][..],
                ),
            ] {
                let mut fixture = Scanner::with_blocks(blocks.iter().copied());
                fixture.snapshot[..HEADER_SIZE].copy_from_slice(&[
                    state, 0, indentation, column, 3,
                ]);
                let snapshot = serialized(&mut fixture);
                // The final rows have the exact identities selected by Default,
                // so this exercises fixed entries as well as foreign slices.
                for symbols in symbol_sets.iter().chain([&TEXT_FLAGS, &CODE_FLAGS, &BLOCK_LINE_FLAGS]) {
                    let mut fast = Scanner::default();
                    fast.deserialize(&snapshot);
                    fast.simulate = true; // scan must clear this even on rejection
                    let mut slow = Scanner::default();
                    slow.deserialize(&snapshot);
                    let mut fast_lexer = TestLexer::new(&input);
                    let mut slow_lexer = TestLexer::new(&input);
                    let result = fast.scan(&mut fast_lexer, symbols);
                    let expected = slow.scan_inner::<DYNAMIC_SYMBOLS>(
                        &mut ScanLexer::new(&mut slow_lexer), Symbols(symbols),
                    );
                    assert_eq!(result, expected, "{input:?}, {snapshot:?}");
                    assert_eq!(fast_lexer.events, slow_lexer.events, "{input:?}");
                    assert_eq!(serialized(&mut fast), serialized(&mut slow), "{input:?}");
                    assert_eq!(fast.simulate, slow.simulate, "{input:?}");
                    assert_eq!(fast_lexer.lookahead_calls, slow_lexer.lookahead_calls);
                    assert_eq!(fast_lexer.eof_calls, slow_lexer.eof_calls);
                }
            }
        }
    }

    #[test]
    fn fixed_symbol_modes_preserve_speculative_scans() {
        fn check<const FLAGS: u64>(input: &str, snapshot: &[u8], simulate: bool) {
            let mut fixed = Scanner::default();
            fixed.deserialize(snapshot);
            fixed.simulate = simulate;
            let mut dynamic = Scanner::default();
            dynamic.deserialize(snapshot);
            dynamic.simulate = simulate;
            let mut fixed_lexer = TestLexer::new(input);
            let mut dynamic_lexer = TestLexer::new(input);
            let flags: [bool; 47] = std::array::from_fn(|i| FLAGS & (1 << i) != 0);
            let actual = fixed.scan_inner::<FLAGS>(
                &mut ScanLexer::new(&mut fixed_lexer), Symbols(&flags),
            );
            let expected = dynamic.scan_inner::<DYNAMIC_SYMBOLS>(
                &mut ScanLexer::new(&mut dynamic_lexer), Symbols(&flags),
            );
            assert_eq!(actual, expected, "{input:?}, {snapshot:?}");
            assert_eq!(fixed_lexer.events, dynamic_lexer.events, "{input:?}");
            assert_eq!(fixed_lexer.lookahead_calls, dynamic_lexer.lookahead_calls);
            assert_eq!(fixed_lexer.eof_calls, dynamic_lexer.eof_calls);
            assert_eq!(serialized(&mut fixed), serialized(&mut dynamic), "{input:?}");
            assert_eq!(fixed.simulate, dynamic.simulate, "{input:?}");
        }
        let wrapped_fence = format!("{} \n", "`".repeat(260));
        for input in [
            "***\n", "*  ** text\n", "_ _ _\r\n", "~~~ info\n", "```info\n",
            "````info`\n", "- item\n", "+ item\n", "123. item\n", "# Heading\n",
            "<script>\n", "<custom a='b'>\n", "a|b\n-|-\n", "a\0|b\n-|-\n",
            "\ncontinued text\n", "\r\n> quote\n", "\n~~~ info\n", "\n* list\n",
            " text\n", "    text\n", " \ttext\n", " ",
            "\t text\n", "\t", "", "\0", "``` \n", "~~~\n", &wrapped_fence,
        ] {
            for state in 0..=(STATE_CLOSE_BLOCK | STATE_WAS_SOFT_LINE_BREAK | STATE_MATCHING) {
                for indentation in [0, 4] {
                    let mut fixture = Scanner::with_blocks([Block::QUOTE, Block::list_item(0)]);
                    fixture.snapshot[..HEADER_SIZE].copy_from_slice(&[state, 0, indentation, 3, 3]);
                    let snapshot = serialized(&mut fixture);
                    for simulate in [false, true] {
                        check::<TEXT_SYMBOLS>(input, &snapshot, simulate);
                        check::<CODE_SYMBOLS>(input, &snapshot, simulate);
                        check::<BLOCK_LINE_SYMBOLS>(input, &snapshot, simulate);
                        check::<PARAGRAPH_SYMBOLS>(input, &snapshot, simulate);
                    }
                }
            }
        }
    }

    #[test]
    fn symbol_array_prefix_is_checked_once_at_the_external_boundary() {
        let mut scanner = Scanner::default();
        for length in 0..47 {
            scanner.simulate = true;
            let mut lexer = TestLexer::new("text");
            assert!(!scanner.scan(&mut lexer, &TEXT_FLAGS[..length]));
            assert!(lexer.events.is_empty());
            assert_eq!(lexer.lookahead_calls.get(), 0);
            assert!(!scanner.simulate);
        }
        // Flags past the external-token array do not affect the scanner.
        let mut extended = [true; 48];
        extended[..47].copy_from_slice(&TEXT_FLAGS);
        let mut lexer = TestLexer::new("");
        assert!(scanner.scan(&mut lexer, &extended));
        assert_eq!(lexer.events, [Event::Symbol(TOKEN_EOF)]);
    }

    #[test]
    fn text_entry_filter_preserves_indentation_eof_and_unicode_aliases() {
        fn check(input: &str, indentation: u8, column: u8) {
            let mut fixture = Scanner::with_blocks([Block::QUOTE]);
            fixture.snapshot[..HEADER_SIZE].copy_from_slice(&[
                STATE_WAS_SOFT_LINE_BREAK, 0, indentation, column, 3,
            ]);
            let snapshot = serialized(&mut fixture);
            let mut fast = Scanner::default();
            fast.deserialize(&snapshot);
            fast.simulate = true; // the external entry must clear this
            let mut dynamic = Scanner::default();
            dynamic.deserialize(&snapshot);
            let mut actual = TestLexer::new(input);
            let mut expected = TestLexer::new(input);
            let result = fast.scan(&mut actual, &TEXT_FLAGS);
            let reference = dynamic.scan_inner::<DYNAMIC_SYMBOLS>(
                &mut ScanLexer::new(&mut expected), Symbols(&TEXT_FLAGS),
            );
            assert_eq!(result, reference, "{input:?}");
            assert_eq!(actual.events, expected.events, "{input:?}");
            assert_eq!(actual.position, expected.position, "{input:?}");
            assert_eq!(actual.lookahead_calls, expected.lookahead_calls, "{input:?}");
            assert_eq!(actual.eof_calls, expected.eof_calls, "{input:?}");
            assert_eq!(serialized(&mut fast), serialized(&mut dynamic), "{input:?}");
            assert_eq!(fast.simulate, dynamic.simulate, "{input:?}");
        }

        // The filter uses only six bits, but dispatch and tab expansion must
        // still use the complete character. Include every low-bit alias, not
        // just printable ASCII, and wrap indentation across the initial space.
        for base in [0, 128, 256, 0x10f000] {
            for low in 0..128 {
                let c = char::from_u32(base + low).unwrap();
                for prefix in ["", " ", "  ", "\t", " \t"] {
                    let input = format!("{prefix}{c}|b\n-|-\n");
                    for indentation in [0, 255] {
                        for column in [0, 3] {
                            check(&input, indentation, column);
                        }
                    }
                }
            }
        }
        for input in ["", " ", "  ", "\t", " \t", "\0", " \0", "  \0", " \t\0"] {
            check(input, 255, 3);
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
    fn indentation_runs_preserve_wrapping_state_and_lexer_events() {
        for spaces in [0, 1, 2, 3, 4, 255, 256, 257, 511] {
            for tabs in ["", "\t", " \t\t \t"] {
                let prefix = format!("{}{tabs}", " ".repeat(spaces));
                for column in 0..4 {
                    for indentation in [0u8, 3, 251, 255] {
                        let mut scanner = Scanner::default();
                        scanner.snapshot[COLUMN] = column;
                        scanner.snapshot[INDENTATION] = indentation;
                        let mut expected_column = column;
                        let mut expected_indentation = indentation;
                        for c in prefix.chars() {
                            let width = if c == '\t' { 4 - expected_column } else { 1 };
                            expected_column = (expected_column + width) % 4;
                            expected_indentation = expected_indentation.wrapping_add(width);
                        }
                        let mut lexer = TestLexer::new(&format!("{prefix}text"));
                        scanner.consume_indentation(&mut ScanLexer::new(&mut lexer));
                        assert_eq!(scanner.snapshot[COLUMN], expected_column);
                        assert_eq!(scanner.snapshot[INDENTATION], expected_indentation);
                        assert_eq!(lexer.position, prefix.len());
                        assert_eq!(
                            lexer.events,
                            (0..prefix.len()).map(Event::Advance).collect::<Vec<_>>()
                        );
                        assert_eq!(lexer.lookahead_calls.get(), prefix.len() + 1);
                    }
                }
            }
        }
    }

    #[test]
    fn table_header_runs_preserve_columns_at_eof_and_embedded_nul() {
        // No delimiter row: all of these attempts must fail after consuming the
        // complete header, with the exact column C leaves behind on failure.
        // Include long runs, trailing spaces/pipes, tabs, escapes, embedded NUL,
        // and wide characters whose low byte is punctuation in C.
        let alphabet = [
            'a', ' ', '\t', '|', '\\', '\0', 'é', '!', '\u{100}', '\u{109}',
            '\u{10a}', '\u{10d}', '\u{15c}', '\u{17c}', '\u{2009}',
        ];
        let mut seed = 0x3141_5926u32;
        for length in [0, 1, 2, 3, 4, 31, 255, 256, 257, 1025] {
            let mut text = "x".repeat(length);
            for _ in 0..64 {
                seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
                text.push(alphabet[seed as usize % alphabet.len()]);
                for column in 0..4 {
                    let expected_column = text.chars().fold(column, |column, c| {
                        if c == '\t' {
                            0
                        } else {
                            (column + 1) % 4
                        }
                    });
                    let mut scanner = Scanner::default();
                    scanner.snapshot[COLUMN] = column;
                    let mut lexer = TestLexer::new(&text);
                    assert!(!scanner.parse_pipe_table(&mut ScanLexer::new(&mut lexer)));
                    assert_eq!(scanner.snapshot[COLUMN], expected_column);
                    assert_eq!(lexer.position, text.chars().count());
                    assert_eq!(lexer.end, Some(0));
                    assert_eq!(lexer.symbol, u16::MAX);
                    assert!(!scanner.simulate);
                    assert_eq!(lexer.events[0], Event::Mark(0));
                    assert_eq!(
                        lexer.events[1..],
                        (0..lexer.position).map(Event::Advance).collect::<Vec<_>>()
                    );
                    assert_eq!(lexer.lookahead_calls.get(), lexer.position + 1);
                }
            }
        }
    }

    #[test]
    fn pipe_table_start_is_zero_width_and_checks_cell_counts() {
        for input in [
            "a | b\n- | -\n",
            "|a|\n|-|\n",
            "a\\|b | c\n- | -\n",
            "a|  b c \t\n-|-\n",
            "|a|\t \r\n|-|\n",
            "text \0 more text | cell\n- | -\n",
            "a\\\u{17c}|b\n- | -\n",
        ] {
            let mut scanner = Scanner::default();
            let mut lexer = TestLexer::new(input);
            assert!(
                scanner.scan(&mut lexer, &valid(&[PIPE_TABLE_START])),
                "{input}"
            );
            assert_eq!(lexer.end, Some(0));
            assert_eq!(lexer.position, input.chars().count() - 1);
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
