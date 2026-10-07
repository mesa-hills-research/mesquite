//! The HTML external scanner, translated from `src/scanner.c` and `src/tag.h`.

use ts_port_tables::{ExternalScanner, Lexer, SERIALIZATION_BUFFER_SIZE};

// External token indices, in the order of the C TokenType enum.
const START_TAG_NAME: usize = 0;
const SCRIPT_START_TAG_NAME: usize = 1;
const STYLE_START_TAG_NAME: usize = 2;
const END_TAG_NAME: usize = 3;
const ERRONEOUS_END_TAG_NAME: usize = 4;
const SELF_CLOSING_TAG_DELIMITER: usize = 5;
const IMPLICIT_END_TAG: usize = 6;
const RAW_TEXT: usize = 7;
const COMMENT: usize = 8;

// Keep the serialized discriminants and the name map in the same order as tag.h.
// The two sentinels have no name-map entry. The array also permits decoding the
// discriminants without an unsafe integer-to-enum conversion.
macro_rules! tag_types {
    ($($variant:ident $(: $name:literal)?),* $(,)?) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        enum TagType {
            $($variant),*
        }

        impl TagType {
            fn from_byte(byte: u8) -> Self {
                const TYPES: &[TagType] = &[$(TagType::$variant),*];
                TYPES[usize::from(byte)]
            }

            fn name(self) -> &'static [u8] {
                match self {
                    $($(Self::$variant => $name,)?)*
                    _ => &[],
                }
            }

            fn for_name(name: &[u8]) -> Self {
                // A byte-slice match lets the compiler dispatch by length and
                // bytes instead of searching all 126 entries for every tag.
                match name {
                    $($($name => Self::$variant,)?)*
                    _ => Self::Custom,
                }
            }
        }

        #[cfg(test)]
        const TAG_TYPES_BY_TAG_NAME: &[(&[u8], TagType)] = &[
            $($(($name, TagType::$variant),)?)*
        ];
    };
}

tag_types! {
    Area: b"AREA",
    Base: b"BASE",
    Basefont: b"BASEFONT",
    Bgsound: b"BGSOUND",
    Br: b"BR",
    Col: b"COL",
    Command: b"COMMAND",
    Embed: b"EMBED",
    Frame: b"FRAME",
    Hr: b"HR",
    Image: b"IMAGE",
    Img: b"IMG",
    Input: b"INPUT",
    Isindex: b"ISINDEX",
    Keygen: b"KEYGEN",
    Link: b"LINK",
    Menuitem: b"MENUITEM",
    Meta: b"META",
    Nextid: b"NEXTID",
    Param: b"PARAM",
    Source: b"SOURCE",
    Track: b"TRACK",
    Wbr: b"WBR",
    EndOfVoidTags,

    A: b"A",
    Abbr: b"ABBR",
    Address: b"ADDRESS",
    Article: b"ARTICLE",
    Aside: b"ASIDE",
    Audio: b"AUDIO",
    B: b"B",
    Bdi: b"BDI",
    Bdo: b"BDO",
    Blockquote: b"BLOCKQUOTE",
    Body: b"BODY",
    Button: b"BUTTON",
    Canvas: b"CANVAS",
    Caption: b"CAPTION",
    Cite: b"CITE",
    Code: b"CODE",
    Colgroup: b"COLGROUP",
    Data: b"DATA",
    Datalist: b"DATALIST",
    Dd: b"DD",
    Del: b"DEL",
    Details: b"DETAILS",
    Dfn: b"DFN",
    Dialog: b"DIALOG",
    Div: b"DIV",
    Dl: b"DL",
    Dt: b"DT",
    Em: b"EM",
    Fieldset: b"FIELDSET",
    Figcaption: b"FIGCAPTION",
    Figure: b"FIGURE",
    Footer: b"FOOTER",
    Form: b"FORM",
    H1: b"H1",
    H2: b"H2",
    H3: b"H3",
    H4: b"H4",
    H5: b"H5",
    H6: b"H6",
    Head: b"HEAD",
    Header: b"HEADER",
    Hgroup: b"HGROUP",
    Html: b"HTML",
    I: b"I",
    Iframe: b"IFRAME",
    Ins: b"INS",
    Kbd: b"KBD",
    Label: b"LABEL",
    Legend: b"LEGEND",
    Li: b"LI",
    Main: b"MAIN",
    Map: b"MAP",
    Mark: b"MARK",
    Math: b"MATH",
    Menu: b"MENU",
    Meter: b"METER",
    Nav: b"NAV",
    Noscript: b"NOSCRIPT",
    Object: b"OBJECT",
    Ol: b"OL",
    Optgroup: b"OPTGROUP",
    Option: b"OPTION",
    Output: b"OUTPUT",
    P: b"P",
    Picture: b"PICTURE",
    Pre: b"PRE",
    Progress: b"PROGRESS",
    Q: b"Q",
    Rb: b"RB",
    Rp: b"RP",
    Rt: b"RT",
    Rtc: b"RTC",
    Ruby: b"RUBY",
    S: b"S",
    Samp: b"SAMP",
    Script: b"SCRIPT",
    Section: b"SECTION",
    Select: b"SELECT",
    Slot: b"SLOT",
    Small: b"SMALL",
    Span: b"SPAN",
    Strong: b"STRONG",
    Style: b"STYLE",
    Sub: b"SUB",
    Summary: b"SUMMARY",
    Sup: b"SUP",
    Svg: b"SVG",
    Table: b"TABLE",
    Tbody: b"TBODY",
    Td: b"TD",
    Template: b"TEMPLATE",
    Textarea: b"TEXTAREA",
    Tfoot: b"TFOOT",
    Th: b"TH",
    Thead: b"THEAD",
    Time: b"TIME",
    Title: b"TITLE",
    Tr: b"TR",
    U: b"U",
    Ul: b"UL",
    Var: b"VAR",
    Video: b"VIDEO",

    Custom: b"CUSTOM",
    End,
}

const TAG_TYPES_NOT_ALLOWED_IN_PARAGRAPHS: [TagType; 26] = {
    use TagType::*;
    [
        Address, Article, Aside, Blockquote, Details, Div, Dl, Fieldset, Figcaption, Figure,
        Footer, Form, H1, H2, H3, H4, H5, H6, Header, Hr, Main, Nav, Ol, P, Pre, Section,
    ]
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Tag<'a> {
    kind: TagType,
    custom_tag_name: &'a [u8],
}

impl Default for Tag<'_> {
    fn default() -> Self {
        Self {
            kind: TagType::End,
            custom_tag_name: &[],
        }
    }
}

impl<'a> Tag<'a> {
    fn for_name(name: &'a [u8]) -> Self {
        let kind = TagType::for_name(name);
        Self {
            kind,
            custom_tag_name: if kind == TagType::Custom { name } else { &[] },
        }
    }

    fn is_void(&self) -> bool {
        (self.kind as u8) < TagType::EndOfVoidTags as u8
    }

    fn can_contain(&self, other: &Self) -> bool {
        use TagType::*;
        let child = other.kind;
        match self.kind {
            Li => child != Li,
            Dt | Dd => child != Dt && child != Dd,
            P => !TAG_TYPES_NOT_ALLOWED_IN_PARAGRAPHS.contains(&child),
            Colgroup => child == Col,
            Rb | Rt | Rp => child != Rb && child != Rt && child != Rp,
            Optgroup => child != Optgroup,
            Tr => child != Tr,
            Td | Th => child != Td && child != Th && child != Tr,
            _ => true,
        }
    }
}

/// Builtin tags occupy just one byte each. Custom names share a byte arena;
/// their ends form a separate stack, since only CUSTOM entries need a name.
/// All three buffers retain their capacity across scanner-state restores.
#[derive(Default, Debug)]
struct TagStack {
    kinds: Vec<u8>,
    names: Vec<u8>,
    name_ends: Vec<usize>,
    snapshot: Snapshot,
}

// A scanner is often restored to its current state (including after failed
// external scans). Keep an exact snapshot, invalidated on every stack mutation,
// so those restores need not decode custom names and rebuild the three buffers.
// `canonical` additionally means serializing the live stack yields these bytes.
// Restored snapshots need not be canonical: strncpy can zero-pad custom names,
// and END_ placeholders can fit where a truncated custom tag did not.
struct Snapshot {
    bytes: [u8; SERIALIZATION_BUFFER_SIZE],
    len: usize,
    canonical: bool,
}

impl Default for Snapshot {
    fn default() -> Self {
        Self {
            bytes: [0; SERIALIZATION_BUFFER_SIZE],
            len: 0,
            canonical: false,
        }
    }
}

impl std::fmt::Debug for Snapshot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Snapshot")
            .field(&&self.bytes[..self.len])
            .finish()
    }
}

impl Snapshot {
    fn save(&mut self, buffer: &[u8], canonical: bool) {
        self.bytes[..buffer.len()].copy_from_slice(buffer);
        self.len = buffer.len();
        self.canonical = canonical;
    }

    fn matches(&self, buffer: &[u8]) -> bool {
        self.len != 0 && self.bytes[..self.len] == *buffer
    }
}

// Snapshot caching does not change the semantic identity of a tag stack.
impl PartialEq for TagStack {
    fn eq(&self, other: &Self) -> bool {
        self.kinds == other.kinds && self.names == other.names && self.name_ends == other.name_ends
    }
}

impl Eq for TagStack {}

impl TagStack {
    fn len(&self) -> usize {
        self.kinds.len()
    }

    fn is_empty(&self) -> bool {
        self.kinds.is_empty()
    }

    fn clear(&mut self) {
        self.snapshot.len = 0;
        self.kinds.clear();
        self.names.clear();
        self.name_ends.clear();
    }

    fn last(&self) -> Option<Tag<'_>> {
        let kind = TagType::from_byte(*self.kinds.last()?);
        let custom_tag_name = if kind == TagType::Custom {
            let start = self.name_ends.iter().rev().nth(1).copied().unwrap_or(0);
            &self.names[start..]
        } else {
            &[]
        };
        Some(Tag {
            kind,
            custom_tag_name,
        })
    }

    fn last_matches_name(&self, name: &[u8]) -> bool {
        self.last().is_some_and(|tag| {
            if tag.kind == TagType::Custom {
                tag.custom_tag_name == name && TagType::for_name(name) == TagType::Custom
            } else {
                // Sentinels have no spelling, and cannot match the empty
                // custom name produced when an implicit scan reaches EOF.
                !name.is_empty() && tag.kind.name() == name
            }
        })
    }

    fn push(&mut self, tag: Tag<'_>) {
        self.snapshot.len = 0;
        self.kinds.push(tag.kind as u8);
        if tag.kind == TagType::Custom {
            self.names.extend_from_slice(tag.custom_tag_name);
            self.name_ends.push(self.names.len());
        }
    }

    fn pop(&mut self) {
        self.snapshot.len = 0;
        if self.kinds.pop() == Some(TagType::Custom as u8) {
            self.name_ends.pop();
            self.names
                .truncate(self.name_ends.last().copied().unwrap_or(0));
        }
    }

    fn iter(&self) -> impl Iterator<Item = Tag<'_>> {
        let mut ends = self.name_ends.iter();
        let mut start = 0;
        self.kinds.iter().map(move |&kind| {
            let kind = TagType::from_byte(kind);
            let custom_tag_name = if kind == TagType::Custom {
                let end = *ends.next().expect("custom tag has a name");
                let name = &self.names[start..end];
                start = end;
                name
            } else {
                &[]
            };
            Tag {
                kind,
                custom_tag_name,
            }
        })
    }
}

#[cfg(test)]
impl<'a> FromIterator<Tag<'a>> for TagStack {
    fn from_iter<T: IntoIterator<Item = Tag<'a>>>(tags: T) -> Self {
        let mut stack = Self::default();
        for tag in tags {
            stack.push(tag);
        }
        stack
    }
}

// The reference uses the default C locale, not Unicode character classes.
// iswspace includes vertical tab, unlike Rust's u8::is_ascii_whitespace.
fn is_space(c: i32) -> bool {
    matches!(c, 0x09..=0x0d | 0x20)
}

// Classification and ASCII uppercasing for tag names in the C locale.
const TAG_NAME_CHARS: [u8; 128] = {
    let mut chars = [0; 128];
    let mut c = 0;
    while c < chars.len() {
        chars[c] = match c {
            0x61..=0x7a => c as u8 - (b'a' - b'A'),
            0x30..=0x39 | 0x41..=0x5a | 0x2d | 0x3a => c as u8,
            _ => 0,
        };
        c += 1;
    }
    chars
};

fn to_upper(c: i32) -> i32 {
    if (i32::from(b'a')..=i32::from(b'z')).contains(&c) {
        c - i32::from(b'a' - b'A')
    } else {
        c
    }
}

fn scan_tag_name(lexer: &mut dyn Lexer, tag_name: &mut Vec<u8>, mut c: i32) {
    tag_name.clear();
    loop {
        let byte = TAG_NAME_CHARS.get(c as usize).copied().unwrap_or(0);
        if byte == 0 {
            break;
        }
        tag_name.push(byte);
        lexer.advance(false);
        c = lexer.lookahead();
    }
}

fn scan_comment(lexer: &mut dyn Lexer) -> bool {
    if lexer.lookahead() != i32::from(b'-') {
        return false;
    }
    lexer.advance(false);
    if lexer.lookahead() != i32::from(b'-') {
        return false;
    }
    lexer.advance(false);

    let mut dashes = 0u32;
    loop {
        match lexer.lookahead() {
            0 => return false,
            0x2d => dashes = dashes.wrapping_add(1),
            0x3e if dashes >= 2 => {
                lexer.set_result_symbol(COMMENT as u16);
                lexer.advance(false);
                lexer.mark_end();
                return true;
            }
            _ => dashes = 0,
        }
        lexer.advance(false);
    }
}

/// The scanner's tag stack. Owned vectors also implement C's free/destroy steps.
#[derive(Default)]
pub(crate) struct Scanner {
    tags: TagStack,
    tag_name: Vec<u8>,
}

impl Scanner {
    fn scan_raw_text(&self, lexer: &mut dyn Lexer) -> bool {
        let Some(parent) = self.tags.last() else {
            return false;
        };

        lexer.mark_end();
        let end_delimiter: &[u8] = if parent.kind == TagType::Script {
            b"</SCRIPT"
        } else {
            b"</STYLE"
        };

        let mut delimiter_index = 0;
        let mut c = lexer.lookahead();
        while c != 0 {
            if to_upper(c) == i32::from(end_delimiter[delimiter_index]) {
                delimiter_index += 1;
                if delimiter_index == end_delimiter.len() {
                    break;
                }
                lexer.advance(false);
                c = lexer.lookahead();
            } else {
                // Consume the mismatch, even when it is '<', rather than
                // reconsidering it as a new delimiter. Once reset, only '<'
                // can start another match. All intermediate mark_end calls
                // would be overwritten, so mark just the end of this run.
                delimiter_index = 0;
                loop {
                    lexer.advance(false);
                    c = lexer.lookahead();
                    if c == 0 || c == i32::from(b'<') {
                        break;
                    }
                }
                lexer.mark_end();
            }
        }

        lexer.set_result_symbol(RAW_TEXT as u16);
        true
    }

    fn scan_implicit_end_tag(&mut self, lexer: &mut dyn Lexer, mut c: i32) -> bool {
        let is_closing_tag = c == i32::from(b'/');
        if is_closing_tag {
            lexer.advance(false);
            c = lexer.lookahead();
        } else if self.tags.last().is_some_and(|tag| tag.is_void()) {
            self.tags.pop();
            lexer.set_result_symbol(IMPLICIT_END_TAG as u16);
            return true;
        }

        scan_tag_name(lexer, &mut self.tag_name, c);
        if self.tag_name.is_empty() && !lexer.eof() {
            return false;
        }
        if is_closing_tag {
            // Most closing tags match their parent. Compare its spelling
            // directly instead of searching the full builtin name map.
            if self.tags.last_matches_name(&self.tag_name) {
                return false;
            }

            let next_tag = Tag::for_name(&self.tag_name);
            // Recovery deliberately compares only types here, not custom names.
            // Only one stack entry is removed, even if the match is much deeper.
            if self
                .tags
                .kinds
                .iter()
                .rev()
                .any(|&kind| kind == next_tag.kind as u8)
            {
                self.tags.pop();
                lexer.set_result_symbol(IMPLICIT_END_TAG as u16);
                return true;
            }
        } else if self.tags.last().is_some_and(|parent| {
            !parent.can_contain(&Tag::for_name(&self.tag_name))
                || (matches!(parent.kind, TagType::Html | TagType::Head | TagType::Body)
                    && lexer.eof())
        }) {
            self.tags.pop();
            lexer.set_result_symbol(IMPLICIT_END_TAG as u16);
            return true;
        }
        false
    }

    fn scan_start_tag_name(&mut self, lexer: &mut dyn Lexer, c: i32) -> bool {
        scan_tag_name(lexer, &mut self.tag_name, c);
        if self.tag_name.is_empty() {
            return false;
        }

        let tag = Tag::for_name(&self.tag_name);
        let kind = tag.kind;
        self.tags.push(tag);
        lexer.set_result_symbol(match kind {
            TagType::Script => SCRIPT_START_TAG_NAME as u16,
            TagType::Style => STYLE_START_TAG_NAME as u16,
            _ => START_TAG_NAME as u16,
        });
        true
    }

    fn scan_end_tag_name(&mut self, lexer: &mut dyn Lexer, c: i32) -> bool {
        scan_tag_name(lexer, &mut self.tag_name, c);
        if self.tag_name.is_empty() {
            return false;
        }

        if self.tags.last_matches_name(&self.tag_name) {
            self.tags.pop();
            lexer.set_result_symbol(END_TAG_NAME as u16);
        } else {
            lexer.set_result_symbol(ERRONEOUS_END_TAG_NAME as u16);
        }
        true
    }

    fn scan_self_closing_tag_delimiter(&mut self, lexer: &mut dyn Lexer) -> bool {
        lexer.advance(false);
        if lexer.lookahead() == i32::from(b'>') {
            lexer.advance(false);
            if !self.tags.is_empty() {
                self.tags.pop();
                lexer.set_result_symbol(SELF_CLOSING_TAG_DELIMITER as u16);
            }
            // C succeeds without setting the result when the tag stack is empty.
            return true;
        }
        false
    }
}

impl ExternalScanner for Scanner {
    fn scan(&mut self, lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> bool {
        if valid_symbols[RAW_TEXT] && !valid_symbols[START_TAG_NAME] && !valid_symbols[END_TAG_NAME]
        {
            return self.scan_raw_text(lexer);
        }

        let mut c = lexer.lookahead();
        while is_space(c) {
            lexer.advance(true);
            c = lexer.lookahead();
        }

        match c {
            0x3c => {
                lexer.mark_end();
                lexer.advance(false);
                c = lexer.lookahead();
                if c == i32::from(b'!') {
                    lexer.advance(false);
                    return scan_comment(lexer);
                }
                if valid_symbols[IMPLICIT_END_TAG] {
                    return self.scan_implicit_end_tag(lexer, c);
                }
            }
            0 => {
                if valid_symbols[IMPLICIT_END_TAG] {
                    return self.scan_implicit_end_tag(lexer, c);
                }
            }
            0x2f => {
                if valid_symbols[SELF_CLOSING_TAG_DELIMITER] {
                    return self.scan_self_closing_tag_delimiter(lexer);
                }
            }
            _ => {
                if (valid_symbols[START_TAG_NAME] || valid_symbols[END_TAG_NAME])
                    && !valid_symbols[RAW_TEXT]
                {
                    return if valid_symbols[START_TAG_NAME] {
                        self.scan_start_tag_name(lexer, c)
                    } else {
                        self.scan_end_tag_name(lexer, c)
                    };
                }
            }
        }
        false
    }

    fn serialize(&mut self, buffer: &mut [u8]) -> usize {
        let snapshot = &self.tags.snapshot;
        if snapshot.len != 0 && snapshot.canonical {
            buffer[..snapshot.len].copy_from_slice(&snapshot.bytes[..snapshot.len]);
            return snapshot.len;
        }
        let tag_count = self.tags.len().min(usize::from(u16::MAX)) as u16;
        let mut serialized_tag_count = 0u16;
        // C uses memcpy for both uint16_t counts, so the format is native-endian.
        buffer[2..4].copy_from_slice(&tag_count.to_ne_bytes());
        let mut size = 4;

        if self.tags.name_ends.is_empty() {
            // No custom names: the stack is already in wire format. C leaves
            // the last buffer byte unused even when another tag would fit.
            let count = usize::from(tag_count).min(SERIALIZATION_BUFFER_SIZE - 5);
            buffer[4..4 + count].copy_from_slice(&self.tags.kinds[..count]);
            buffer[..2].copy_from_slice(&(count as u16).to_ne_bytes());
            if count == self.tags.len() {
                self.tags.snapshot.save(&buffer[..4 + count], true);
            }
            return 4 + count;
        }

        let mut lossless = true;
        for tag in self.tags.iter().take(usize::from(tag_count)) {
            if tag.kind == TagType::Custom {
                let name_length = tag.custom_tag_name.len().min(usize::from(u8::MAX));
                if size + 2 + name_length >= SERIALIZATION_BUFFER_SIZE {
                    break;
                }
                buffer[size] = tag.kind as u8;
                buffer[size + 1] = name_length as u8;
                size += 2;

                // Match strncpy's zero padding, including names restored by
                // deserialize that contain an embedded NUL.
                let name = &tag.custom_tag_name[..name_length];
                let copy_length = name.iter().position(|&c| c == 0).unwrap_or(name_length);
                lossless &= copy_length == tag.custom_tag_name.len();
                buffer[size..size + copy_length].copy_from_slice(&name[..copy_length]);
                buffer[size + copy_length..size + name_length].fill(0);
                size += name_length;
            } else {
                if size + 1 >= SERIALIZATION_BUFFER_SIZE {
                    break;
                }
                buffer[size] = tag.kind as u8;
                size += 1;
            }
            serialized_tag_count += 1;
        }
        buffer[..2].copy_from_slice(&serialized_tag_count.to_ne_bytes());
        if lossless && usize::from(serialized_tag_count) == self.tags.len() {
            self.tags.snapshot.save(&buffer[..size], true);
        }
        size
    }

    fn deserialize(&mut self, buffer: &[u8]) {
        if !self.tags.snapshot.matches(buffer) {
            self.restore(buffer);
        }
    }
}

impl Scanner {
    // Keep the rebuilding path's register/stack setup out of cache-hit restores.
    #[inline(never)]
    fn restore(&mut self, buffer: &[u8]) {
        self.tags.clear();
        if buffer.is_empty() {
            return;
        }

        let serialized_tag_count = usize::from(u16::from_ne_bytes([buffer[0], buffer[1]]));
        let tag_count = usize::from(u16::from_ne_bytes([buffer[2], buffer[3]]));
        let mut size = 4;
        self.tags.kinds.reserve(tag_count);
        if tag_count > 0 {
            // Every custom tag needs a length byte in addition to its type,
            // even for an empty name. Equal counts/bytes imply all builtins.
            if buffer.len() == 4 + serialized_tag_count {
                self.tags.kinds.extend_from_slice(&buffer[4..]);
                self.tags.kinds.resize(tag_count, TagType::End as u8);
                self.tags.snapshot.save(buffer, false);
                return;
            }
            let mut remaining = serialized_tag_count;
            while remaining > 0 {
                // Builtins already have exactly the representation we need.
                // Copy whole runs rather than constructing a Tag/Vec per byte.
                let builtin_count = buffer[size..]
                    .iter()
                    .take(remaining)
                    .position(|&kind| kind == TagType::Custom as u8)
                    .unwrap_or(remaining);
                self.tags
                    .kinds
                    .extend_from_slice(&buffer[size..size + builtin_count]);
                size += builtin_count;
                remaining -= builtin_count;
                if remaining == 0 {
                    break;
                }
                let name_length = usize::from(buffer[size + 1]);
                size += 2;
                self.tags.push(Tag {
                    kind: TagType::Custom,
                    custom_tag_name: &buffer[size..size + name_length],
                });
                size += name_length;
                remaining -= 1;
            }
            // The stack depth survives buffer exhaustion. Missing entries are
            // END_ tags (tag_new), not zero-valued AREA tags.
            self.tags.kinds.resize(tag_count, TagType::End as u8);
        }
        self.tags.snapshot.save(buffer, false);
    }
}

/// Creates a scanner (C's `tree_sitter_html_external_scanner_create`).
pub(crate) fn create() -> Box<dyn ExternalScanner> {
    Box::<Scanner>::default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    #[derive(Debug, PartialEq, Eq)]
    enum Call {
        Advance(usize, bool),
        MarkEnd(usize),
        Result(u16),
        Eof(usize),
    }

    struct TestLexer {
        input: Vec<i32>,
        position: usize,
        end: Option<usize>,
        symbol: u16,
        calls: RefCell<Vec<Call>>,
    }

    impl TestLexer {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().map(|c| c as i32).collect(),
                position: 0,
                end: None,
                symbol: u16::MAX,
                calls: RefCell::default(),
            }
        }
    }

    impl Lexer for TestLexer {
        fn lookahead(&self) -> i32 {
            self.input.get(self.position).copied().unwrap_or(0)
        }

        fn result_symbol(&self) -> u16 {
            self.symbol
        }

        fn set_result_symbol(&mut self, symbol: u16) {
            self.symbol = symbol;
            self.calls.borrow_mut().push(Call::Result(symbol));
        }

        fn advance(&mut self, skip: bool) {
            self.calls
                .borrow_mut()
                .push(Call::Advance(self.position, skip));
            if self.position < self.input.len() {
                self.position += 1;
            }
        }

        fn mark_end(&mut self) {
            self.end = Some(self.position);
            self.calls.borrow_mut().push(Call::MarkEnd(self.position));
        }

        fn eof(&self) -> bool {
            self.calls.borrow_mut().push(Call::Eof(self.position));
            self.position == self.input.len()
        }

        fn get_column(&mut self) -> u32 {
            panic!("the HTML scanner does not call get_column")
        }

        fn is_at_included_range_start(&self) -> bool {
            panic!("the HTML scanner does not check included ranges")
        }
    }

    fn scan(scanner: &mut Scanner, input: &str, symbols: &[usize]) -> (bool, TestLexer) {
        let mut lexer = TestLexer::new(input);
        let mut valid = [false; 9];
        for &symbol in symbols {
            valid[symbol] = true;
        }
        let accepted = scanner.scan(&mut lexer, &valid);
        (accepted, lexer)
    }

    fn with_tags(names: &[&str]) -> Scanner {
        Scanner {
            tags: names
                .iter()
                .map(|name| Tag::for_name(name.as_bytes()))
                .collect(),
            ..Scanner::default()
        }
    }

    fn serialized(scanner: &mut dyn ExternalScanner) -> Vec<u8> {
        let mut buffer = [0xa5; SERIALIZATION_BUFFER_SIZE];
        let size = scanner.serialize(&mut buffer);
        assert!(buffer[size..].iter().all(|&byte| byte == 0xa5));
        buffer[..size].to_vec()
    }

    fn header(serialized_count: u16, total_count: u16) -> Vec<u8> {
        [serialized_count.to_ne_bytes(), total_count.to_ne_bytes()].concat()
    }

    #[test]
    fn tag_map_and_serialized_discriminants() {
        assert_eq!(TAG_TYPES_BY_TAG_NAME.len(), 126);
        for (i, &(name, kind)) in TAG_TYPES_BY_TAG_NAME.iter().enumerate() {
            let id = if i < 23 { i } else { i + 1 } as u8;
            assert_eq!(kind as u8, id);
            assert_eq!(TagType::from_byte(id), kind);
            let tag = Tag::for_name(name);
            assert_eq!(tag.kind, kind);
            assert_eq!(tag.is_void(), i < 23);
            if kind == TagType::Custom {
                assert_eq!(tag.custom_tag_name, b"CUSTOM");
            } else {
                assert!(tag.custom_tag_name.is_empty());
            }
        }
        assert_eq!(Tag::default().kind as u8, 127);
        for name in [b"".as_slice(), b"END_", b"div", b"DIV\0", b"X-A"] {
            assert_eq!(Tag::for_name(name).kind, TagType::Custom);
        }
    }

    #[test]
    fn parent_name_comparison_matches_tag_classification() {
        let mut parents: Vec<_> = TAG_TYPES_BY_TAG_NAME
            .iter()
            .map(|&(name, _)| Tag::for_name(name))
            .collect();
        parents.extend([
            Tag::default(),
            Tag {
                kind: TagType::EndOfVoidTags,
                custom_tag_name: &[],
            },
            Tag::for_name(b""),
            Tag::for_name(b"X-A"),
            Tag::for_name(b"DIV\0"),
            // Snapshots can restore a custom tag whose bytes spell a builtin.
            // The C comparison still requires the tag types to match.
            Tag {
                kind: TagType::Custom,
                custom_tag_name: b"DIV",
            },
        ]);
        let mut names: Vec<_> = TAG_TYPES_BY_TAG_NAME
            .iter()
            .map(|&(name, _)| name)
            .collect();
        names.extend([b"".as_slice(), b"X-A", b"X-B", b"DIV\0", b"div", b"END_"]);
        for parent in parents {
            let stack = TagStack::from_iter([parent]);
            for &name in &names {
                assert_eq!(
                    stack.last_matches_name(name),
                    parent == Tag::for_name(name),
                    "parent={parent:?}, name={name:?}",
                );
            }
        }
        for name in names {
            assert!(!TagStack::default().last_matches_name(name));
        }
    }

    #[test]
    fn serialization_layout_and_reset() {
        let mut scanner = with_tags(&["HTML", "SCRIPT", "X-A"]);
        let mut expected = header(3, 3);
        expected.extend_from_slice(&[66, 99, 126, 3, b'X', b'-', b'A']);
        assert_eq!(serialized(&mut scanner), expected);

        let mut restored = with_tags(&["OLD"]);
        restored.deserialize(&expected);
        assert_eq!(restored.tags, scanner.tags);
        assert_eq!(serialized(&mut restored), expected);
        restored.deserialize(&[]);
        assert!(restored.tags.is_empty());
        assert_eq!(serialized(&mut restored), header(0, 0));
        assert_eq!(serialized(create().as_mut()), header(0, 0));
    }

    #[test]
    fn serialization_limits_preserve_depth_with_end_sentinels() {
        let mut scanner = Scanner {
            tags: std::iter::repeat_n(Tag::for_name(b"DIV"), usize::from(u16::MAX) + 1).collect(),
            ..Scanner::default()
        };
        let bytes = serialized(&mut scanner);
        assert_eq!(bytes.len(), 1023);
        assert_eq!(bytes[..4], header(1019, u16::MAX));
        assert!(bytes[4..].iter().all(|&byte| byte == 48));
        assert_eq!(scanner.tags.len(), usize::from(u16::MAX) + 1);

        let mut restored = Scanner::default();
        restored.deserialize(&bytes);
        assert_eq!(restored.tags.len(), usize::from(u16::MAX));
        assert_eq!(restored.tags.kinds[..1019], scanner.tags.kinds[..1019]);
        assert!(
            restored
                .tags
                .iter()
                .skip(1019)
                .all(|tag| tag == Tag::default())
        );
        assert_eq!(serialized(&mut restored), bytes);
    }

    #[test]
    fn custom_name_truncation_and_strict_buffer_boundary() {
        for (builtin_count, fits) in [(762, true), (763, false)] {
            let mut scanner = Scanner {
                tags: std::iter::repeat_n(Tag::for_name(b"DIV"), builtin_count).collect(),
                ..Scanner::default()
            };
            scanner.tags.push(Tag::for_name(&[b'X'; 300]));
            let bytes = serialized(&mut scanner);
            let expected_count = builtin_count + usize::from(fits);
            assert_eq!(
                bytes[..4],
                header(expected_count as u16, (builtin_count + 1) as u16)
            );
            let mut restored = Scanner::default();
            restored.deserialize(&bytes);
            let last = restored.tags.last().unwrap();
            if fits {
                assert_eq!(bytes.len(), 1023);
                assert_eq!(&bytes[766..768], &[126, 255]);
                assert_eq!(last.custom_tag_name, vec![b'X'; 255]);
            } else {
                assert_eq!(bytes.len(), 767);
                assert_eq!(last, Tag::default());
            }
        }
    }

    #[test]
    fn serialization_uses_strncpy_not_memcpy_for_custom_names() {
        let mut bytes = header(1, 1);
        bytes.extend_from_slice(&[126, 4, b'X', 0, b'Y', b'Z']);
        let mut scanner = Scanner::default();
        scanner.deserialize(&bytes);
        assert_eq!(scanner.tags.last().unwrap().custom_tag_name, b"X\0YZ");
        assert_eq!(&serialized(&mut scanner)[4..], &[126, 4, b'X', 0, 0, 0]);
    }

    #[test]
    fn custom_name_arena_preserves_stack_order_across_pops_and_restores() {
        let mut bytes = header(7, 7);
        bytes.extend_from_slice(&[
            TagType::Div as u8,
            TagType::Custom as u8,
            3,
            b'X',
            b'-',
            b'A',
            TagType::P as u8,
            TagType::Custom as u8,
            0,
            TagType::Custom as u8,
            3,
            b'X',
            b'-',
            b'B',
            TagType::Script as u8,
            TagType::Html as u8,
        ]);
        let expected = [
            Tag::for_name(b"DIV"),
            Tag::for_name(b"X-A"),
            Tag::for_name(b"P"),
            Tag::for_name(b""),
            Tag::for_name(b"X-B"),
            Tag::for_name(b"SCRIPT"),
            Tag::for_name(b"HTML"),
        ];
        let mut scanner = Scanner::default();
        scanner.deserialize(&bytes);
        assert_eq!(serialized(&mut scanner), bytes);
        assert_eq!(scanner.tags.iter().collect::<Vec<_>>(), expected);
        let capacities = (
            scanner.tags.kinds.capacity(),
            scanner.tags.names.capacity(),
            scanner.tags.name_ends.capacity(),
        );
        for _ in 0..3 {
            for remaining in (0..expected.len()).rev() {
                assert_eq!(scanner.tags.last(), Some(expected[remaining]));
                scanner.tags.pop();
                assert_eq!(
                    scanner.tags.iter().collect::<Vec<_>>(),
                    expected[..remaining]
                );
            }
            assert!(scanner.tags.names.is_empty());
            assert!(scanner.tags.name_ends.is_empty());
            scanner.deserialize(&bytes);
            assert_eq!(serialized(&mut scanner), bytes);
            assert_eq!(
                (
                    scanner.tags.kinds.capacity(),
                    scanner.tags.names.capacity(),
                    scanner.tags.name_ends.capacity(),
                ),
                capacities,
            );
        }

        // Replacing custom tags with a builtin-only snapshot must clear the
        // name arena too, including when the depth is filled with END_ tags.
        let mut truncated = header(1, 3);
        truncated.push(TagType::Div as u8);
        scanner.deserialize(&truncated);
        assert!(scanner.tags.names.is_empty());
        assert!(scanner.tags.name_ends.is_empty());
        assert_eq!(
            scanner.tags.iter().collect::<Vec<_>>(),
            [Tag::for_name(b"DIV"), Tag::default(), Tag::default()],
        );
        scanner.deserialize(&bytes);
        assert_eq!(serialized(&mut scanner), bytes);
        scanner.deserialize(&[]);
        assert_eq!(scanner.tags, TagStack::default());
    }

    #[test]
    fn snapshot_cache_tracks_mutations_and_backtracking() {
        let mut scanner = with_tags(&["HTML", "DIV", "X-ONE"]);
        let original = serialized(&mut scanner);
        assert!(scanner.tags.snapshot.matches(&original));
        assert!(scanner.tags.snapshot.canonical);
        for _ in 0..3 {
            scanner.deserialize(&original);
            assert!(scanner.tags.snapshot.canonical); // Cache hit, not a rebuild.
            assert!(!scan(&mut scanner, "</x-one>", &[IMPLICIT_END_TAG]).0);
            assert_eq!(serialized(&mut scanner), original);
        }

        assert!(scan(&mut scanner, "span>", &[START_TAG_NAME]).0);
        assert_eq!(scanner.tags.snapshot.len, 0);
        let deeper = serialized(&mut scanner);
        assert!(scanner.tags.snapshot.canonical);
        assert_ne!(deeper, original);
        assert!(scan(&mut scanner, "span>", &[END_TAG_NAME]).0);
        assert_eq!(scanner.tags.snapshot.len, 0);
        scanner.deserialize(&deeper);
        assert_eq!(scanner.tags.last(), Some(Tag::for_name(b"SPAN")));
        assert_eq!(serialized(&mut scanner), deeper);
        scanner.deserialize(&original);
        assert_eq!(scanner.tags.last(), Some(Tag::for_name(b"X-ONE")));
        assert_eq!(serialized(&mut scanner), original);

        // Equal-sized snapshots can differ in either a builtin or a name byte.
        let mut other = original.clone();
        other[5] = TagType::P as u8;
        *other.last_mut().unwrap() = b'X';
        scanner.deserialize(&other);
        assert_eq!(scanner.tags, with_tags(&["HTML", "P", "X-ONX"]).tags);
        assert_eq!(serialized(&mut scanner), other);
        scanner.deserialize(&original);
        assert_eq!(scanner.tags, with_tags(&["HTML", "DIV", "X-ONE"]).tags);
        scanner.deserialize(&[]);
        assert!(scanner.tags.is_empty());
        assert_eq!(scanner.tags.snapshot.len, 0);
        assert_eq!(serialized(&mut scanner), header(0, 0));
    }

    #[test]
    fn snapshot_cache_does_not_skip_lossy_restores_or_canonicalization() {
        let long_name = [b'X'; 300];
        let mut scanner = Scanner::default();
        scanner.tags.push(Tag::for_name(&long_name));
        let truncated_name = serialized(&mut scanner);
        assert_eq!(scanner.tags.snapshot.len, 0);
        assert_eq!(scanner.tags.last().unwrap().custom_tag_name.len(), 300);
        scanner.deserialize(&truncated_name);
        assert_eq!(scanner.tags.last().unwrap().custom_tag_name.len(), 255);
        assert_eq!(serialized(&mut scanner), truncated_name);
        assert!(scanner.tags.snapshot.canonical);

        // The original last tag cannot fit, but its restored END_ sentinel can.
        scanner.tags = std::iter::repeat_n(Tag::for_name(b"DIV"), 763).collect();
        scanner.tags.push(Tag::for_name(&long_name));
        let truncated_stack = serialized(&mut scanner);
        assert_eq!(scanner.tags.snapshot.len, 0);
        scanner.deserialize(&truncated_stack);
        assert!(scanner.tags.snapshot.matches(&truncated_stack));
        assert!(!scanner.tags.snapshot.canonical);
        let canonical = serialized(&mut scanner);
        assert_eq!(canonical[..4], header(764, 764));
        assert_eq!(canonical.last(), Some(&(TagType::End as u8)));
        assert_ne!(canonical, truncated_stack);
        assert_eq!(serialized(&mut scanner), canonical);

        let mut embedded_nul = header(1, 1);
        embedded_nul.extend_from_slice(&[126, 4, b'X', 0, b'Y', b'Z']);
        scanner.deserialize(&embedded_nul);
        assert!(scanner.tags.snapshot.matches(&embedded_nul));
        for _ in 0..2 {
            assert_eq!(&serialized(&mut scanner)[4..], &[126, 4, b'X', 0, 0, 0]);
            assert_eq!(scanner.tags.last().unwrap().custom_tag_name, b"X\0YZ");
            scanner.deserialize(&embedded_nul);
        }
    }

    #[test]
    fn tag_name_classification_table_matches_c_locale() {
        for c in -1..=256 {
            let expected = if matches!(c, 0x30..=0x39 | 0x41..=0x5a | 0x61..=0x7a | 0x2d | 0x3a) {
                to_upper(c) as u8
            } else {
                0
            };
            assert_eq!(
                TAG_NAME_CHARS.get(c as usize).copied().unwrap_or(0),
                expected
            );
        }
    }

    #[test]
    fn scanning_reuses_names_without_truncating_live_custom_tags() {
        let long_name = "x".repeat(300);
        let mut scanner = with_tags(&["DIV"]);
        assert!(scan(&mut scanner, &long_name, &[START_TAG_NAME]).0);
        assert_eq!(
            scanner.tags.last().unwrap().custom_tag_name,
            vec![b'X'; 300]
        );
        let capacity = scanner.tag_name.capacity();
        let (_, wrong_end) = scan(&mut scanner, &long_name[..255], &[END_TAG_NAME]);
        assert_eq!(wrong_end.symbol, ERRONEOUS_END_TAG_NAME as u16);
        assert_eq!(scanner.tags.len(), 2);
        let (_, right_end) = scan(&mut scanner, &long_name, &[END_TAG_NAME]);
        assert_eq!(right_end.symbol, END_TAG_NAME as u16);
        assert_eq!(scanner.tags, with_tags(&["DIV"]).tags);
        assert!(scanner.tags.names.is_empty());
        assert_eq!(scanner.tag_name.capacity(), capacity);
    }

    #[test]
    fn start_names_are_case_insensitive_and_use_c_locale_classes() {
        for (input, name, symbol, position) in [
            ("sCrIpT>", "SCRIPT", SCRIPT_START_TAG_NAME, 6),
            ("style ", "STYLE", STYLE_START_TAG_NAME, 5),
            ("DiV>", "DIV", START_TAG_NAME, 3),
            ("x-Ab:9>", "X-AB:9", START_TAG_NAME, 6),
            ("0-d>", "0-D", START_TAG_NAME, 3),
            ("div_foo", "DIV", START_TAG_NAME, 3),
            ("p\u{e9}>", "P", START_TAG_NAME, 1),
        ] {
            let mut scanner = Scanner::default();
            let (accepted, lexer) = scan(&mut scanner, input, &[START_TAG_NAME]);
            assert!(accepted, "{input:?}");
            assert_eq!(lexer.symbol, symbol as u16);
            assert_eq!(lexer.position, position);
            assert_eq!(lexer.end, None);
            assert_eq!(scanner.tags, with_tags(&[name]).tags);
        }
        for input in ["", "_x", "\u{e9}>", "\u{2003}div>", "\0div>"] {
            let mut scanner = Scanner::default();
            assert!(!scan(&mut scanner, input, &[START_TAG_NAME]).0, "{input:?}");
            assert!(scanner.tags.is_empty());
        }
        let (_, lexer) = scan(
            &mut Scanner::default(),
            " \t\n\x0b\x0c\rbr>",
            &[START_TAG_NAME],
        );
        for i in 0..6 {
            assert_eq!(lexer.calls.borrow()[i], Call::Advance(i, true));
        }
    }

    #[test]
    fn end_names_match_only_the_top_tag_and_custom_names_exactly() {
        let mut scanner = with_tags(&["DIV", "X-ONE"]);
        for name in ["div>", "x-two>"] {
            let (accepted, lexer) = scan(&mut scanner, name, &[END_TAG_NAME]);
            assert!(accepted);
            assert_eq!(lexer.symbol, ERRONEOUS_END_TAG_NAME as u16);
            assert_eq!(scanner.tags.len(), 2);
        }
        let (accepted, lexer) = scan(&mut scanner, "x-oNe>", &[END_TAG_NAME]);
        assert!(accepted);
        assert_eq!(lexer.symbol, END_TAG_NAME as u16);
        assert_eq!(scanner.tags, with_tags(&["DIV"]).tags);
        assert!(!scan(&mut scanner, ">", &[END_TAG_NAME]).0);
    }

    #[test]
    fn all_containment_rules() {
        for &(parent_name, parent_kind) in TAG_TYPES_BY_TAG_NAME {
            let parent = Tag::for_name(parent_name);
            for &(child_name, child_kind) in TAG_TYPES_BY_TAG_NAME {
                let forbidden = match parent_kind {
                    TagType::Li => child_name == b"LI",
                    TagType::Dt | TagType::Dd => {
                        matches!(child_name, b"DT" | b"DD")
                    }
                    TagType::P => TAG_TYPES_NOT_ALLOWED_IN_PARAGRAPHS.contains(&child_kind),
                    TagType::Colgroup => child_name != b"COL",
                    TagType::Rb | TagType::Rt | TagType::Rp => {
                        matches!(child_name, b"RB" | b"RT" | b"RP")
                    }
                    TagType::Optgroup => child_name == b"OPTGROUP",
                    TagType::Tr => child_name == b"TR",
                    TagType::Td | TagType::Th => matches!(child_name, b"TD" | b"TH" | b"TR"),
                    _ => false,
                };
                assert_eq!(parent.can_contain(&Tag::for_name(child_name)), !forbidden);
            }
        }
    }

    #[test]
    fn implicit_closes_are_zero_width_and_pop_only_one_entry() {
        let mut scanner = with_tags(&["UL", "LI"]);
        let (accepted, lexer) = scan(&mut scanner, "<li>", &[IMPLICIT_END_TAG]);
        assert!(accepted);
        assert_eq!(lexer.symbol, IMPLICIT_END_TAG as u16);
        assert_eq!(lexer.end, Some(0));
        assert_eq!(lexer.position, 3);
        assert_eq!(scanner.tags, with_tags(&["UL"]).tags);
        assert_eq!(
            lexer.calls.into_inner(),
            [
                Call::MarkEnd(0),
                Call::Advance(0, false),
                Call::Advance(1, false),
                Call::Advance(2, false),
                Call::Result(IMPLICIT_END_TAG as u16),
            ]
        );

        let mut scanner = with_tags(&["DIV", "SPAN", "P"]);
        assert!(scan(&mut scanner, "</div>", &[IMPLICIT_END_TAG]).0);
        assert_eq!(scanner.tags, with_tags(&["DIV", "SPAN"]).tags);
        assert!(scan(&mut scanner, "</div>", &[IMPLICIT_END_TAG]).0);
        assert_eq!(scanner.tags, with_tags(&["DIV"]).tags);
        assert!(!scan(&mut scanner, "</div>", &[IMPLICIT_END_TAG]).0);
        assert!(!scan(&mut scanner, "</other>", &[IMPLICIT_END_TAG]).0);
    }

    #[test]
    fn implicit_custom_recovery_compares_type_not_name() {
        let mut scanner = with_tags(&["X-ONE", "DIV"]);
        assert!(scan(&mut scanner, "</x-two>", &[IMPLICIT_END_TAG]).0);
        assert_eq!(scanner.tags, with_tags(&["X-ONE"]).tags);
        assert!(!scan(&mut scanner, "</x-one>", &[IMPLICIT_END_TAG]).0);
        assert!(scan(&mut scanner, "</x-two>", &[IMPLICIT_END_TAG]).0);
        assert!(scanner.tags.is_empty());
    }

    #[test]
    fn void_tags_and_eof_implicit_closes() {
        let mut scanner = with_tags(&["BR"]);
        let (accepted, lexer) = scan(&mut scanner, "<?>", &[IMPLICIT_END_TAG]);
        assert!(accepted);
        assert_eq!(lexer.position, 1);
        assert_eq!(lexer.end, Some(0));
        assert!(scanner.tags.is_empty());
        assert_eq!(
            lexer.calls.into_inner(),
            [
                Call::MarkEnd(0),
                Call::Advance(0, false),
                Call::Result(IMPLICIT_END_TAG as u16),
            ]
        );

        for (parent, closes) in [
            ("HTML", true),
            ("HEAD", true),
            ("BODY", true),
            ("COLGROUP", true),
            ("BR", true),
            ("DIV", false),
            ("P", false),
            ("X-ONE", false),
        ] {
            let mut scanner = with_tags(&[parent]);
            assert_eq!(
                scan(&mut scanner, "", &[IMPLICIT_END_TAG]).0,
                closes,
                "{parent}"
            );
        }
        let (_, lexer) = scan(&mut with_tags(&["HTML"]), "", &[IMPLICIT_END_TAG]);
        assert_eq!(
            lexer.calls.into_inner(),
            [
                Call::Eof(0),
                Call::Eof(0),
                Call::Result(IMPLICIT_END_TAG as u16)
            ]
        );
        // An embedded NUL has zero lookahead but is not EOF.
        assert!(!scan(&mut with_tags(&["HTML"]), "\0x", &[IMPLICIT_END_TAG]).0);
        assert!(!scan(&mut with_tags(&["BR"]), "</div>", &[IMPLICIT_END_TAG]).0);
    }

    #[test]
    fn self_closing_delimiters_preserve_empty_stack_result() {
        let mut scanner = with_tags(&["DIV"]);
        let (accepted, lexer) = scan(&mut scanner, "/>", &[SELF_CLOSING_TAG_DELIMITER]);
        assert!(accepted);
        assert_eq!(lexer.symbol, SELF_CLOSING_TAG_DELIMITER as u16);
        assert!(scanner.tags.is_empty());
        let (accepted, lexer) = scan(&mut scanner, "/>", &[SELF_CLOSING_TAG_DELIMITER]);
        assert!(accepted);
        assert_eq!(lexer.symbol, u16::MAX);
        assert_eq!(lexer.position, 2);
        assert_eq!(lexer.end, None);
        let (accepted, lexer) = scan(&mut scanner, "/ >", &[SELF_CLOSING_TAG_DELIMITER]);
        assert!(!accepted);
        assert_eq!(lexer.position, 1);
    }

    #[test]
    fn comments_ignore_valid_symbols_and_preserve_callback_order() {
        let (accepted, lexer) = scan(&mut Scanner::default(), "<!---->", &[]);
        assert!(accepted);
        assert_eq!(
            lexer.calls.into_inner(),
            [
                Call::MarkEnd(0),
                Call::Advance(0, false),
                Call::Advance(1, false),
                Call::Advance(2, false),
                Call::Advance(3, false),
                Call::Advance(4, false),
                Call::Advance(5, false),
                Call::Result(COMMENT as u16),
                Call::Advance(6, false),
                Call::MarkEnd(7),
            ]
        );
        for input in ["<!-->", "<!--->", "<!-x", "<!x", "<!--a--x>", "<!--a\0-->"] {
            assert!(
                !scan(&mut Scanner::default(), input, &[COMMENT]).0,
                "{input:?}"
            );
        }
        for input in ["<!--a--->", "<!--a->b-->", "<!--a-->extra"] {
            assert!(
                scan(&mut Scanner::default(), input, &[COMMENT]).0,
                "{input:?}"
            );
        }
    }

    #[test]
    fn raw_text_delimiter_lookahead_and_partial_matches() {
        for (input, position, end) in [
            ("", 0, 0),
            ("abc", 3, 3),
            ("</sCrIpT>", 7, 0),
            ("a</SCRIPT>", 8, 1),
            ("a</SCRIPTING>", 8, 1),
            ("a</SCR", 6, 1),
            ("a<", 2, 1),
            ("<</SCRIPT>", 10, 10),
            ("abc\0more", 3, 3),
        ] {
            let mut scanner = with_tags(&["SCRIPT"]);
            let (accepted, lexer) = scan(&mut scanner, input, &[RAW_TEXT]);
            assert!(accepted, "{input:?}");
            assert_eq!(lexer.symbol, RAW_TEXT as u16);
            assert_eq!(lexer.position, position, "{input:?}");
            assert_eq!(lexer.end, Some(end), "{input:?}");
            assert_eq!(scanner.tags.len(), 1);
        }
        for parent in ["STYLE", "DIV", "X-A"] {
            let (_, lexer) = scan(&mut with_tags(&[parent]), " </sTyLe>", &[RAW_TEXT]);
            assert_eq!(lexer.position, 7);
            assert_eq!(lexer.end, Some(1));
            assert_eq!(lexer.calls.borrow()[1], Call::Advance(0, false));
        }
        let (accepted, lexer) = scan(&mut Scanner::default(), "abc", &[RAW_TEXT]);
        assert!(!accepted);
        assert!(lexer.calls.into_inner().is_empty());
    }

    #[test]
    fn raw_text_run_boundaries_match_character_at_a_time_scanning() {
        // A direct model of C's loop, including the consumed mismatching '<'.
        for (parent, delimiter) in [("SCRIPT", b"</SCRIPT".as_slice()), ("STYLE", b"</STYLE")] {
            for prefix_len in 0..delimiter.len() {
                let prefix = std::str::from_utf8(&delimiter[..prefix_len]).unwrap();
                for mismatch in ["", "x", "<", ">", "\0", "é"] {
                    for tail in ["", "abc", "</sCrIpT>", "</sTyLe>", "<<</SCRIPT>"] {
                        let input = format!("abc{prefix}{mismatch}{tail}");
                        let mut reference = TestLexer::new(&input);
                        reference.mark_end();
                        let mut index = 0;
                        while reference.lookahead() != 0 {
                            if to_upper(reference.lookahead()) == i32::from(delimiter[index]) {
                                index += 1;
                                if index == delimiter.len() {
                                    break;
                                }
                                reference.advance(false);
                            } else {
                                index = 0;
                                reference.advance(false);
                                reference.mark_end();
                            }
                        }
                        let (accepted, lexer) =
                            scan(&mut with_tags(&[parent]), &input, &[RAW_TEXT]);
                        assert!(accepted);
                        assert_eq!(
                            (lexer.position, lexer.end),
                            (reference.position, reference.end),
                            "{parent}: {input:?}"
                        );
                        let advances = |lexer: &TestLexer| {
                            lexer
                                .calls
                                .borrow()
                                .iter()
                                .filter_map(|call| match call {
                                    Call::Advance(position, skip) => Some((*position, *skip)),
                                    _ => None,
                                })
                                .collect::<Vec<_>>()
                        };
                        assert_eq!(advances(&lexer), advances(&reference));
                    }
                }
            }
        }
    }

    #[test]
    fn raw_text_is_disabled_when_start_or_end_names_are_valid() {
        for symbols in [vec![RAW_TEXT, START_TAG_NAME], vec![RAW_TEXT, END_TAG_NAME]] {
            let mut scanner = with_tags(&["SCRIPT"]);
            let (accepted, lexer) = scan(&mut scanner, "abc", &symbols);
            assert!(!accepted);
            assert_eq!(lexer.position, 0);
        }
        let mut scanner = with_tags(&["DIV"]);
        let (accepted, lexer) = scan(&mut scanner, "div>", &[START_TAG_NAME, END_TAG_NAME]);
        assert!(accepted);
        assert_eq!(lexer.symbol, START_TAG_NAME as u16);
        assert_eq!(scanner.tags.len(), 2);
    }
}
