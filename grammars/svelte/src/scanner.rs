//! The Svelte external scanner, translated from `src/scanner.c` and `src/tag.h`.

use tree_sitter_language::{ExternalScanner, Lexer, SERIALIZATION_BUFFER_SIZE};

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
const SVELTE_RAW_TEXT: usize = 9;
const SVELTE_RAW_TEXT_EACH: usize = 10;
const SVELTE_RAW_TEXT_SNIPPET_ARGUMENTS: usize = 11;
const AT: usize = 12;
const HASH: usize = 13;
const SLASH: usize = 14;
const COLON: usize = 15;

// Preserve tag.h's discriminants, including its two unnamed sentinels. The
// explicit array decodes snapshot bytes without an unsafe enum conversion.
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

            fn for_name(name: &[u8]) -> Self {
                for &(tag_name, kind) in TAG_TYPES_BY_TAG_NAME {
                    if tag_name == name {
                        return kind;
                    }
                }
                Self::Custom
            }
        }

        const TAG_TYPES_BY_TAG_NAME: &[(&[u8], TagType)] = &[
            $($(($name, TagType::$variant),)?)*
        ];
    };
}

tag_types! {
    Area: b"area",
    Base: b"base",
    Basefont: b"basefont",
    Bgsound: b"bgsound",
    Br: b"br",
    Col: b"col",
    Command: b"command",
    Embed: b"embed",
    Frame: b"frame",
    Hr: b"hr",
    Image: b"image",
    Img: b"img",
    Input: b"input",
    Isindex: b"isindex",
    Keygen: b"keygen",
    Link: b"link",
    Menuitem: b"menuitem",
    Meta: b"meta",
    Nextid: b"nextid",
    Param: b"param",
    Source: b"source",
    Track: b"track",
    Wbr: b"wbr",
    EndOfVoidTags,
    A: b"a",
    Abbr: b"abbr",
    Address: b"address",
    Article: b"article",
    Aside: b"aside",
    Audio: b"audio",
    B: b"b",
    Bdi: b"bdi",
    Bdo: b"bdo",
    Blockquote: b"blockquote",
    Body: b"body",
    Button: b"button",
    Canvas: b"canvas",
    Caption: b"caption",
    Cite: b"cite",
    Code: b"code",
    Colgroup: b"colgroup",
    Data: b"data",
    Datalist: b"datalist",
    Dd: b"dd",
    Del: b"del",
    Details: b"details",
    Dfn: b"dfn",
    Dialog: b"dialog",
    Div: b"div",
    Dl: b"dl",
    Dt: b"dt",
    Em: b"em",
    Fieldset: b"fieldset",
    Figcaption: b"figcaption",
    Figure: b"figure",
    Footer: b"footer",
    Form: b"form",
    H1: b"h1",
    H2: b"h2",
    H3: b"h3",
    H4: b"h4",
    H5: b"h5",
    H6: b"h6",
    Head: b"head",
    Header: b"header",
    Hgroup: b"hgroup",
    Html: b"html",
    I: b"i",
    Iframe: b"iframe",
    Ins: b"ins",
    Kbd: b"kbd",
    Label: b"label",
    Legend: b"legend",
    Li: b"li",
    Main: b"main",
    Map: b"map",
    Mark: b"mark",
    Math: b"math",
    Menu: b"menu",
    Meter: b"meter",
    Nav: b"nav",
    Noscript: b"noscript",
    Object: b"object",
    Ol: b"ol",
    Optgroup: b"optgroup",
    Option: b"option",
    Output: b"output",
    P: b"p",
    Picture: b"picture",
    Pre: b"pre",
    Progress: b"progress",
    Q: b"q",
    Rb: b"rb",
    Rp: b"rp",
    Rt: b"rt",
    Rtc: b"rtc",
    Ruby: b"ruby",
    S: b"s",
    Samp: b"samp",
    Script: b"script",
    Section: b"section",
    Select: b"select",
    Slot: b"slot",
    Small: b"small",
    Span: b"span",
    Strong: b"strong",
    Style: b"style",
    Sub: b"sub",
    Summary: b"summary",
    Sup: b"sup",
    Svg: b"svg",
    Table: b"table",
    Tbody: b"tbody",
    Td: b"td",
    Template: b"template",
    Textarea: b"textarea",
    Tfoot: b"tfoot",
    Th: b"th",
    Thead: b"thead",
    Time: b"time",
    Title: b"title",
    Tr: b"tr",
    U: b"u",
    Ul: b"ul",
    Var: b"var",
    Video: b"video",
    Custom: b"custom",
    End,
}

const TAG_TYPES_NOT_ALLOWED_IN_PARAGRAPHS: [TagType; 26] = {
    use TagType::*;
    [
        Address, Article, Aside, Blockquote, Details, Div, Dl, Fieldset, Figcaption, Figure,
        Footer, Form, H1, H2, H3, H4, H5, H6, Header, Hr, Main, Nav, Ol, P, Pre, Section,
    ]
};

#[derive(Clone, Debug, PartialEq, Eq)]
struct Tag {
    kind: TagType,
    custom_tag_name: Vec<u8>,
}

impl Default for Tag {
    fn default() -> Self {
        Self {
            kind: TagType::End,
            custom_tag_name: Vec::new(),
        }
    }
}

impl Tag {
    fn for_name(name: Vec<u8>) -> Self {
        let kind = TagType::for_name(&name);
        Self {
            kind,
            custom_tag_name: if kind == TagType::Custom {
                name
            } else {
                Vec::new()
            },
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
            Rb | Rt | Rp => !matches!(child, Rb | Rt | Rp),
            Optgroup => child != Optgroup,
            Tr => child != Tr,
            Td | Th => !matches!(child, Td | Th | Tr),
            _ => true,
        }
    }
}

#[derive(Default)]
pub(crate) struct Scanner {
    tags: Vec<Tag>,
}

// Character classes as in C's default locale, not Unicode's.
fn is_space(c: i32) -> bool {
    matches!(c, 0x09..=0x0d | 0x20)
}

fn is_alnum(c: i32) -> bool {
    matches!(c, 0x30..=0x39 | 0x41..=0x5a | 0x61..=0x7a)
}

fn upper(c: i32) -> i32 {
    if (0x61..=0x7a).contains(&c) {
        c - 0x20
    } else {
        c
    }
}

fn advance(lexer: &mut dyn Lexer) {
    lexer.advance(false);
}

fn scan_tag_name(lexer: &mut dyn Lexer) -> Vec<u8> {
    let mut name = Vec::new();
    while is_alnum(lexer.lookahead()) || matches!(lexer.lookahead(), 0x2d | 0x3a | 0x2e) {
        // Svelte's built-in HTML names must be lowercase. Unlike HTML, do not
        // uppercase names: capitalized names identify custom Svelte components.
        name.push(lexer.lookahead() as u8);
        advance(lexer);
    }
    name
}

fn scan_comment(lexer: &mut dyn Lexer) -> bool {
    if lexer.lookahead() != i32::from(b'-') {
        return false;
    }
    advance(lexer);
    if lexer.lookahead() != i32::from(b'-') {
        return false;
    }
    advance(lexer);

    let mut dashes = 0u32;
    while lexer.lookahead() != 0 {
        match lexer.lookahead() {
            0x2d => dashes = dashes.wrapping_add(1),
            0x3e => {
                if dashes >= 2 {
                    lexer.set_result_symbol(COMMENT as u16);
                    advance(lexer);
                    lexer.mark_end();
                    return true;
                }
                dashes = 0;
            }
            _ => dashes = 0,
        }
        advance(lexer);
    }
    false
}

// Called after consuming the slash of a block comment.
fn scan_javascript_block_comment(lexer: &mut dyn Lexer) -> bool {
    if lexer.lookahead() != i32::from(b'*') {
        return false;
    }
    advance(lexer);
    while lexer.lookahead() != 0 {
        if lexer.lookahead() == i32::from(b'*') {
            advance(lexer);
            if lexer.lookahead() == i32::from(b'/') {
                advance(lexer);
                return true;
            }
        } else {
            advance(lexer);
        }
    }
    false
}

// Called after consuming the first slash of a line comment. At EOF without a
// newline this returns false, just as the C helper does.
fn scan_javascript_line_comment(lexer: &mut dyn Lexer) -> bool {
    if lexer.lookahead() != i32::from(b'/') {
        return false;
    }
    advance(lexer);
    while lexer.lookahead() != 0 {
        if matches!(lexer.lookahead(), 0x0a | 0x0d) {
            advance(lexer);
            return true;
        }
        advance(lexer);
    }
    false
}

fn scan_javascript_balanced_brace(lexer: &mut dyn Lexer) -> bool {
    if lexer.lookahead() != i32::from(b'{') {
        return false;
    }
    let mut brace_level = 0u8;
    advance(lexer);
    while lexer.lookahead() != 0 {
        match lexer.lookahead() {
            0x60 => {
                scan_javascript_template_string(lexer);
            }
            0x5c => {
                advance(lexer);
                advance(lexer);
            }
            delimiter @ (0x27 | 0x22) => {
                scan_javascript_quoted_string(lexer, delimiter);
            }
            0x7b => {
                brace_level = brace_level.wrapping_add(1);
                advance(lexer);
            }
            0x7d => {
                advance(lexer);
                if brace_level == 0 {
                    return true;
                }
                brace_level = brace_level.wrapping_sub(1);
            }
            _ => advance(lexer),
        }
    }
    false
}

fn scan_javascript_quoted_string(lexer: &mut dyn Lexer, delimiter: i32) -> bool {
    if lexer.lookahead() != delimiter {
        return false;
    }
    advance(lexer);
    while lexer.lookahead() != 0 {
        if lexer.lookahead() == i32::from(b'\\') {
            advance(lexer);
            advance(lexer);
        } else {
            if lexer.lookahead() == delimiter {
                advance(lexer);
                return true;
            }
            advance(lexer);
        }
    }
    false
}

fn scan_javascript_template_string(lexer: &mut dyn Lexer) -> bool {
    if lexer.lookahead() != i32::from(b'`') {
        return false;
    }
    advance(lexer);
    while lexer.lookahead() != 0 {
        match lexer.lookahead() {
            0x24 => {
                advance(lexer);
                if lexer.lookahead() == i32::from(b'{') {
                    scan_javascript_balanced_brace(lexer);
                }
            }
            0x5c => {
                advance(lexer);
                advance(lexer);
            }
            0x60 => {
                advance(lexer);
                return true;
            }
            _ => advance(lexer),
        }
    }
    false
}

fn scan_svelte_raw_text_snippet(lexer: &mut dyn Lexer) -> bool {
    while is_space(lexer.lookahead()) {
        lexer.advance(true);
    }
    lexer.set_result_symbol(SVELTE_RAW_TEXT_SNIPPET_ARGUMENTS as u16);
    let mut paren_level = 0u8;
    let mut advanced_once = false;
    while !lexer.eof() {
        match lexer.lookahead() {
            0x2f => {
                advance(lexer);
                if lexer.lookahead() == i32::from(b'*') {
                    scan_javascript_block_comment(lexer);
                } else if lexer.lookahead() == i32::from(b'/') {
                    scan_javascript_line_comment(lexer);
                }
            }
            // The C raw-text scanners advance only once for a backslash.
            0x5c => advance(lexer),
            delimiter @ (0x22 | 0x27) => {
                scan_javascript_quoted_string(lexer, delimiter);
            }
            0x60 => {
                scan_javascript_template_string(lexer);
            }
            0x29 => {
                if paren_level == 0 {
                    lexer.mark_end();
                    return advanced_once;
                }
                advance(lexer);
                paren_level = paren_level.wrapping_sub(1);
            }
            0x28 => {
                advance(lexer);
                paren_level = paren_level.wrapping_add(1);
            }
            _ => advance(lexer),
        }
        advanced_once = true;
    }
    false
}

fn scan_svelte_raw_text(lexer: &mut dyn Lexer, valid: &[bool]) -> bool {
    while is_space(lexer.lookahead()) {
        lexer.advance(true);
    }

    if (lexer.lookahead() == i32::from(b'@') && valid[AT])
        || (lexer.lookahead() == i32::from(b'#') && valid[HASH])
        || (lexer.lookahead() == i32::from(b':') && valid[COLON])
    {
        return false;
    }
    if matches!(lexer.lookahead(), 0x40 | 0x23 | 0x3a) {
        return false;
    }

    let mut advanced_once = false;
    if lexer.lookahead() == i32::from(b'/') && valid[SLASH] {
        advance(lexer);
        if lexer.lookahead() == i32::from(b'*') {
            // Deliberately returns before assigning result_symbol or mark_end.
            return scan_javascript_block_comment(lexer);
        }
        if lexer.lookahead() != i32::from(b'/') {
            return false;
        }
        advanced_once = true;
    }

    lexer.set_result_symbol(if valid[SVELTE_RAW_TEXT_EACH] {
        SVELTE_RAW_TEXT_EACH as u16
    } else {
        SVELTE_RAW_TEXT as u16
    });
    let mut brace_level = 0u8;
    while !lexer.eof() {
        match lexer.lookahead() {
            0x2f => {
                advance(lexer);
                advanced_once = true;
                if lexer.lookahead() == i32::from(b'*') {
                    scan_javascript_block_comment(lexer);
                } else if lexer.lookahead() == i32::from(b'/') {
                    scan_javascript_line_comment(lexer);
                }
            }
            0x5c => {
                advance(lexer);
                advanced_once = true;
            }
            delimiter @ (0x22 | 0x27) => {
                scan_javascript_quoted_string(lexer, delimiter);
                advanced_once = true;
            }
            0x60 => {
                scan_javascript_template_string(lexer);
                advanced_once = true;
            }
            0x7d => {
                if brace_level == 0 {
                    lexer.mark_end();
                    return advanced_once;
                }
                advance(lexer);
                brace_level = brace_level.wrapping_sub(1);
                advanced_once = true;
            }
            0x7b => {
                advance(lexer);
                brace_level = brace_level.wrapping_add(1);
                advanced_once = true;
            }
            0x61 if lexer.result_symbol() == SVELTE_RAW_TEXT_EACH as u16 => {
                lexer.mark_end();
                advance(lexer);
                advanced_once = true;
                if lexer.lookahead() == i32::from(b's') {
                    advance(lexer);
                    if is_space(lexer.lookahead()) {
                        return advanced_once;
                    }
                }
            }
            _ => {
                advance(lexer);
                advanced_once = true;
            }
        }
    }
    false
}

impl Scanner {
    fn scan_raw_text(&self, lexer: &mut dyn Lexer) -> bool {
        let Some(parent) = self.tags.last() else {
            return false;
        };
        lexer.mark_end();
        let delimiter: &[u8] = if parent.kind == TagType::Script {
            b"</SCRIPT"
        } else {
            b"</STYLE"
        };
        let mut index = 0;
        while lexer.lookahead() != 0 {
            // C applies towupper before narrowing to char. Non-ASCII code points
            // can therefore match delimiter bytes via their low byte.
            if upper(lexer.lookahead()) as u8 == delimiter[index] {
                index += 1;
                if index == delimiter.len() {
                    break;
                }
                advance(lexer);
            } else {
                index = 0;
                advance(lexer);
                lexer.mark_end();
            }
        }
        lexer.set_result_symbol(RAW_TEXT as u16);
        true
    }

    fn scan_implicit_end_tag(&mut self, lexer: &mut dyn Lexer) -> bool {
        let parent = self.tags.last();
        let closing = lexer.lookahead() == i32::from(b'/');
        if closing {
            advance(lexer);
        } else if parent.is_some_and(Tag::is_void) {
            self.tags.pop();
            lexer.set_result_symbol(IMPLICIT_END_TAG as u16);
            return true;
        }

        let name = scan_tag_name(lexer);
        if name.is_empty() && !lexer.eof() {
            return false;
        }
        let next = Tag::for_name(name);
        if closing {
            if self.tags.last() == Some(&next) {
                return false;
            }
            // This fallback intentionally compares types only, not custom names.
            for tag in self.tags.iter().rev() {
                if tag.kind == next.kind {
                    self.tags.pop();
                    lexer.set_result_symbol(IMPLICIT_END_TAG as u16);
                    return true;
                }
            }
        } else if parent.is_some_and(|tag| {
            !tag.can_contain(&next)
                || (matches!(tag.kind, TagType::Html | TagType::Head | TagType::Body)
                    && lexer.eof())
        }) {
            self.tags.pop();
            lexer.set_result_symbol(IMPLICIT_END_TAG as u16);
            return true;
        }
        false
    }

    fn scan_start_tag_name(&mut self, lexer: &mut dyn Lexer) -> bool {
        let name = scan_tag_name(lexer);
        if name.is_empty() {
            return false;
        }
        let tag = Tag::for_name(name);
        let kind = tag.kind;
        self.tags.push(tag);
        lexer.set_result_symbol(match kind {
            TagType::Script => SCRIPT_START_TAG_NAME,
            TagType::Style => STYLE_START_TAG_NAME,
            _ => START_TAG_NAME,
        } as u16);
        true
    }

    fn scan_end_tag_name(&mut self, lexer: &mut dyn Lexer) -> bool {
        let name = scan_tag_name(lexer);
        if name.is_empty() {
            return false;
        }
        let tag = Tag::for_name(name);
        if self.tags.last() == Some(&tag) {
            self.tags.pop();
            lexer.set_result_symbol(END_TAG_NAME as u16);
        } else {
            lexer.set_result_symbol(ERRONEOUS_END_TAG_NAME as u16);
        }
        true
    }

    fn scan_self_closing_tag_delimiter(&mut self, lexer: &mut dyn Lexer) -> bool {
        advance(lexer);
        if lexer.lookahead() == i32::from(b'>') {
            advance(lexer);
            if self.tags.pop().is_some() {
                lexer.set_result_symbol(SELF_CLOSING_TAG_DELIMITER as u16);
            }
            return true;
        }
        false
    }
}

impl ExternalScanner for Scanner {
    fn scan(&mut self, lexer: &mut dyn Lexer, valid: &[bool]) -> bool {
        if valid[RAW_TEXT] && !valid[START_TAG_NAME] && !valid[END_TAG_NAME] {
            return self.scan_raw_text(lexer);
        }
        if valid[SVELTE_RAW_TEXT_SNIPPET_ARGUMENTS] {
            return scan_svelte_raw_text_snippet(lexer);
        }
        if valid[SVELTE_RAW_TEXT] || valid[SVELTE_RAW_TEXT_EACH] {
            return scan_svelte_raw_text(lexer, valid);
        }
        while is_space(lexer.lookahead()) {
            lexer.advance(true);
        }
        match lexer.lookahead() {
            0x3c => {
                lexer.mark_end();
                advance(lexer);
                if lexer.lookahead() == i32::from(b'!') {
                    advance(lexer);
                    return scan_comment(lexer);
                }
                if valid[IMPLICIT_END_TAG] {
                    return self.scan_implicit_end_tag(lexer);
                }
            }
            0x7b | 0 => {
                if valid[IMPLICIT_END_TAG] {
                    return self.scan_implicit_end_tag(lexer);
                }
            }
            0x2f => {
                if valid[SELF_CLOSING_TAG_DELIMITER] {
                    return self.scan_self_closing_tag_delimiter(lexer);
                }
            }
            _ => {
                if (valid[START_TAG_NAME] || valid[END_TAG_NAME]) && !valid[RAW_TEXT] {
                    return if valid[START_TAG_NAME] {
                        self.scan_start_tag_name(lexer)
                    } else {
                        self.scan_end_tag_name(lexer)
                    };
                }
            }
        }
        false
    }

    fn serialize(&mut self, buffer: &mut [u8]) -> usize {
        // Two native-endian u16 counts precede the tags: the stored count and
        // the total count. Unstored tags are restored as End sentinels.
        let limit = buffer.len().min(SERIALIZATION_BUFFER_SIZE);
        if limit < 4 {
            return 0;
        }
        let tag_count = self.tags.len().min(usize::from(u16::MAX)) as u16;
        let mut serialized_count = 0u16;
        let mut size = 4;
        buffer[2..4].copy_from_slice(&tag_count.to_ne_bytes());
        for tag in &self.tags[..usize::from(tag_count)] {
            if tag.kind == TagType::Custom {
                let name_length = tag.custom_tag_name.len().min(usize::from(u8::MAX));
                if size + 2 + name_length >= limit {
                    break;
                }
                buffer[size] = tag.kind as u8;
                buffer[size + 1] = name_length as u8;
                size += 2;
                // C uses strncpy: zero-fill after the first NUL, if any.
                let name = &tag.custom_tag_name[..name_length];
                let copied = name.iter().position(|&c| c == 0).unwrap_or(name_length);
                buffer[size..size + copied].copy_from_slice(&name[..copied]);
                buffer[size + copied..size + name_length].fill(0);
                size += name_length;
            } else {
                if size + 1 >= limit {
                    break;
                }
                buffer[size] = tag.kind as u8;
                size += 1;
            }
            serialized_count += 1;
        }
        buffer[..2].copy_from_slice(&serialized_count.to_ne_bytes());
        size
    }

    fn deserialize(&mut self, buffer: &[u8]) {
        self.tags.clear();
        if buffer.is_empty() {
            return;
        }
        let serialized_count = u16::from_ne_bytes([buffer[0], buffer[1]]);
        let tag_count = u16::from_ne_bytes([buffer[2], buffer[3]]);
        self.tags.reserve(usize::from(tag_count));
        let mut size = 4;
        if tag_count > 0 {
            for _ in 0..serialized_count {
                let kind = TagType::from_byte(buffer[size]);
                size += 1;
                let mut tag = Tag {
                    kind,
                    ..Tag::default()
                };
                if kind == TagType::Custom {
                    let name_length = usize::from(buffer[size]);
                    size += 1;
                    tag.custom_tag_name
                        .extend_from_slice(&buffer[size..size + name_length]);
                    size += name_length;
                }
                self.tags.push(tag);
            }
            for _ in serialized_count..tag_count {
                self.tags.push(Tag::default());
            }
        }
    }
}

pub(crate) fn create() -> Box<dyn ExternalScanner> {
    Box::<Scanner>::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    enum Event {
        Advance(usize, bool),
        Mark(usize),
        Symbol(u16),
    }

    struct TestLexer {
        input: Vec<i32>,
        position: usize,
        end: Option<usize>,
        symbol: u16,
        events: Vec<Event>,
    }

    impl TestLexer {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().map(|c| c as i32).collect(),
                position: 0,
                end: None,
                symbol: u16::MAX,
                events: Vec::new(),
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
            self.events.push(Event::Symbol(symbol));
            self.symbol = symbol;
        }
        fn advance(&mut self, skip: bool) {
            self.events.push(Event::Advance(self.position, skip));
            if self.position < self.input.len() {
                self.position += 1;
            }
        }
        fn mark_end(&mut self) {
            self.events.push(Event::Mark(self.position));
            self.end = Some(self.position);
        }
        fn get_column(&mut self) -> u32 {
            panic!("the Svelte scanner does not request columns")
        }
        fn is_at_included_range_start(&self) -> bool {
            panic!("the Svelte scanner does not inspect included-range starts")
        }
        fn eof(&self) -> bool {
            self.position == self.input.len()
        }
    }

    fn valid(tokens: &[usize]) -> [bool; 16] {
        let mut result = [false; 16];
        for &token in tokens {
            result[token] = true;
        }
        result
    }

    fn scanner_with_tags(names: &[&str]) -> Scanner {
        Scanner {
            tags: names
                .iter()
                .map(|name| Tag::for_name(name.as_bytes().to_vec()))
                .collect(),
        }
    }

    #[test]
    fn tag_names_and_discriminants_match_tag_h() {
        assert_eq!(TAG_TYPES_BY_TAG_NAME.len(), 126);
        for (index, &(name, kind)) in TAG_TYPES_BY_TAG_NAME.iter().enumerate() {
            let discriminant = if index < 23 { index } else { index + 1 } as u8;
            assert_eq!(kind as u8, discriminant);
            assert_eq!(TagType::from_byte(discriminant), kind);
            assert_eq!(TagType::for_name(name), kind);
        }
        assert_eq!(TagType::End as u8, 127);
        assert_eq!(TagType::for_name(b"script"), TagType::Script);
        assert_eq!(TagType::for_name(b"Script"), TagType::Custom);
        assert!(!Tag::for_name(b"Br".to_vec()).is_void());
        assert!(Tag::for_name(b"br".to_vec()).is_void());
        assert_ne!(
            Tag::for_name(b"Foo".to_vec()),
            Tag::for_name(b"foo".to_vec())
        );
    }

    #[test]
    fn containment_uses_sveltes_rules_not_all_html_rules() {
        for (parent, child, allowed) in [
            ("li", "li", false),
            ("dt", "dd", false),
            ("dd", "dt", false),
            ("p", "div", false),
            ("p", "Div", true),
            ("p", "span", true),
            ("colgroup", "col", true),
            ("colgroup", "span", false),
            ("rb", "rt", false),
            ("rp", "rb", false),
            ("rt", "rp", false),
            ("optgroup", "optgroup", false),
            ("tr", "tr", false),
            ("td", "th", false),
            ("th", "tr", false),
            ("td", "td", false),
            ("option", "option", true),
            ("head", "body", true),
            ("select", "div", true),
            ("Li", "li", true),
        ] {
            let parent_tag = Tag::for_name(parent.as_bytes().to_vec());
            let child_tag = Tag::for_name(child.as_bytes().to_vec());
            assert_eq!(
                parent_tag.can_contain(&child_tag),
                allowed,
                "{parent} -> {child}"
            );
        }
    }

    #[test]
    fn snapshots_preserve_native_counts_custom_names_and_sentinels() {
        let mut scanner = scanner_with_tags(&["div", "Widget", "custom", "script"]);
        scanner.tags.push(Tag::default());
        let mut buffer = [0xcc; SERIALIZATION_BUFFER_SIZE];
        let size = scanner.serialize(&mut buffer);
        let mut expected = Vec::from(5u16.to_ne_bytes());
        expected.extend_from_slice(&5u16.to_ne_bytes());
        expected.push(TagType::Div as u8);
        expected.extend_from_slice(&[TagType::Custom as u8, 6]);
        expected.extend_from_slice(b"Widget");
        expected.extend_from_slice(&[TagType::Custom as u8, 6]);
        expected.extend_from_slice(b"custom");
        expected.extend_from_slice(&[TagType::Script as u8, TagType::End as u8]);
        assert_eq!(&buffer[..size], expected);
        assert_eq!(buffer[size], 0xcc);
        let mut restored = Scanner::default();
        restored.deserialize(&buffer[..size]);
        assert_eq!(restored.tags, scanner.tags);
        restored.deserialize(&[]);
        assert!(restored.tags.is_empty());
        assert_eq!(restored.serialize(&mut buffer), 4);
        assert_eq!(&buffer[..4], &[0, 0, 0, 0]);
    }

    #[test]
    fn snapshots_truncate_names_and_reconstruct_unstored_tags() {
        let mut scanner = Scanner {
            tags: vec![Tag::for_name(b"div".to_vec()); 1200],
        };
        let mut buffer = [0xcc; SERIALIZATION_BUFFER_SIZE];
        assert_eq!(scanner.serialize(&mut buffer), 1023);
        assert_eq!(&buffer[..2], &1019u16.to_ne_bytes());
        assert_eq!(&buffer[2..4], &1200u16.to_ne_bytes());
        assert_eq!(buffer[1023], 0xcc);
        scanner.deserialize(&buffer[..1023]);
        assert_eq!(scanner.tags.len(), 1200);
        assert!(
            scanner.tags[..1019]
                .iter()
                .all(|tag| tag.kind == TagType::Div)
        );
        assert!(
            scanner.tags[1019..]
                .iter()
                .all(|tag| tag.kind == TagType::End)
        );

        scanner.tags = vec![Tag::for_name(vec![b'X'; 300]); 4];
        let size = scanner.serialize(&mut buffer);
        assert_eq!(size, 4 + 3 * (2 + 255));
        assert_eq!(&buffer[..2], &3u16.to_ne_bytes());
        assert_eq!(&buffer[2..4], &4u16.to_ne_bytes());
        scanner.deserialize(&buffer[..size]);
        for tag in &scanner.tags[..3] {
            assert_eq!(tag.custom_tag_name, vec![b'X'; 255]);
        }
        assert_eq!(scanner.tags[3], Tag::default());

        // Mirrors strncpy's zero fill, even for a name from a supplied snapshot.
        scanner.tags = vec![Tag::for_name(b"X\0YZ".to_vec())];
        assert_eq!(scanner.serialize(&mut buffer), 10);
        assert_eq!(&buffer[6..10], b"X\0\0\0");
    }

    #[test]
    fn tag_stack_and_name_tokens_are_case_sensitive() {
        let mut scanner = Scanner::default();
        for (name, symbol) in [
            ("script", SCRIPT_START_TAG_NAME),
            ("style", STYLE_START_TAG_NAME),
            ("Script", START_TAG_NAME),
            ("svelte:component", START_TAG_NAME),
            ("Foo.Bar-1", START_TAG_NAME),
        ] {
            let mut lexer = TestLexer::new(&format!("{name}>"));
            assert!(scanner.scan(&mut lexer, &valid(&[START_TAG_NAME])));
            assert_eq!(lexer.symbol, symbol as u16);
            assert_eq!(lexer.position, name.len());
        }
        let mut lexer = TestLexer::new("foo.bar-1>");
        assert!(scanner.scan(&mut lexer, &valid(&[END_TAG_NAME])));
        assert_eq!(lexer.symbol, ERRONEOUS_END_TAG_NAME as u16);
        assert_eq!(scanner.tags.len(), 5);
        let mut lexer = TestLexer::new("Foo.Bar-1>");
        assert!(scanner.scan(&mut lexer, &valid(&[END_TAG_NAME])));
        assert_eq!(lexer.symbol, END_TAG_NAME as u16);
        assert_eq!(scanner.tags.len(), 4);
        let mut lexer = TestLexer::new("/>");
        assert!(scanner.scan(&mut lexer, &valid(&[SELF_CLOSING_TAG_DELIMITER])));
        assert_eq!(lexer.symbol, SELF_CLOSING_TAG_DELIMITER as u16);
        assert_eq!(scanner.tags.len(), 3);

        scanner.tags.clear();
        let mut lexer = TestLexer::new("/>");
        assert!(scanner.scan(&mut lexer, &valid(&[SELF_CLOSING_TAG_DELIMITER])));
        assert_eq!(lexer.symbol, u16::MAX); // C still succeeds without assigning a token.
    }

    #[test]
    fn implicit_end_tags_keep_the_mark_before_lookahead() {
        let mut scanner = scanner_with_tags(&["p"]);
        let mut lexer = TestLexer::new("<div>");
        assert!(scanner.scan(&mut lexer, &valid(&[IMPLICIT_END_TAG])));
        assert_eq!(lexer.position, 4);
        assert_eq!(lexer.end, Some(0));
        assert_eq!(lexer.symbol, IMPLICIT_END_TAG as u16);
        assert!(scanner.tags.is_empty());

        let mut scanner = scanner_with_tags(&["Widget"]);
        let mut lexer = TestLexer::new("</Other>");
        assert!(scanner.scan(&mut lexer, &valid(&[IMPLICIT_END_TAG])));
        assert!(scanner.tags.is_empty()); // Recovery compares Custom type, not names.

        let mut scanner = scanner_with_tags(&["Widget"]);
        let mut lexer = TestLexer::new("</Widget>");
        assert!(!scanner.scan(&mut lexer, &valid(&[IMPLICIT_END_TAG])));
        assert_eq!(scanner.tags.len(), 1);

        let mut scanner = scanner_with_tags(&["br"]);
        let mut lexer = TestLexer::new("{expression}");
        assert!(scanner.scan(&mut lexer, &valid(&[IMPLICIT_END_TAG])));
        assert_eq!(lexer.events, [Event::Symbol(IMPLICIT_END_TAG as u16)]);

        for name in ["html", "head", "body"] {
            let mut scanner = scanner_with_tags(&[name]);
            let mut lexer = TestLexer::new("");
            assert!(scanner.scan(&mut lexer, &valid(&[IMPLICIT_END_TAG])));
            assert!(scanner.tags.is_empty());
        }
    }

    #[test]
    fn html_comment_callback_order_does_not_depend_on_valid_comment() {
        let mut scanner = Scanner::default();
        let mut lexer = TestLexer::new("<!---->");
        assert!(scanner.scan(&mut lexer, &valid(&[])));
        assert_eq!(
            lexer.events,
            [
                Event::Mark(0),
                Event::Advance(0, false),
                Event::Advance(1, false),
                Event::Advance(2, false),
                Event::Advance(3, false),
                Event::Advance(4, false),
                Event::Advance(5, false),
                Event::Symbol(COMMENT as u16),
                Event::Advance(6, false),
                Event::Mark(7),
            ]
        );
    }

    #[test]
    fn raw_script_text_retains_delimiter_prefix_quirks() {
        let mut scanner = scanner_with_tags(&["script"]);
        for (input, end, position) in [
            ("x</ScRiPt>", 1, 8),
            ("x</SCR", 1, 6), // The partial prefix at EOF is not marked as content.
            ("x<</script>", 11, 11), // No restart on the second '<'.
            ("x\u{13c}/SCRIPT>", 1, 8), // towupper then char narrowing.
        ] {
            let mut lexer = TestLexer::new(input);
            assert!(scanner.scan(&mut lexer, &valid(&[RAW_TEXT])));
            assert_eq!(lexer.symbol, RAW_TEXT as u16);
            assert_eq!(lexer.end, Some(end), "{input}");
            assert_eq!(lexer.position, position, "{input}");
        }
        let mut lexer = TestLexer::new("");
        assert!(scanner.scan(&mut lexer, &valid(&[RAW_TEXT])));
        assert_eq!(
            lexer.events,
            [Event::Mark(0), Event::Symbol(RAW_TEXT as u16)]
        );
    }

    #[test]
    fn svelte_text_balances_around_comments_strings_and_templates() {
        for text in [
            "object({x: '\\'}', y: \"}\"})",
            "before /* } { */ + after // }\n more",
            "`text ${ {x: `nested ${'}'}`} } end`",
            "nul\0inside",
        ] {
            let mut scanner = Scanner::default();
            let mut lexer = TestLexer::new(&format!(" \t{text}}}suffix"));
            assert!(
                scanner.scan(&mut lexer, &valid(&[SVELTE_RAW_TEXT])),
                "{text}"
            );
            assert_eq!(lexer.end, Some(2 + text.len()), "{text}");
            assert_eq!(lexer.lookahead(), i32::from(b'}'));
            assert_eq!(
                &lexer.events[..3],
                &[
                    Event::Advance(0, true),
                    Event::Advance(1, true),
                    Event::Symbol(SVELTE_RAW_TEXT as u16),
                ]
            );
        }
        let mut scanner = Scanner::default();
        let mut lexer = TestLexer::new("}");
        assert!(!scanner.scan(&mut lexer, &valid(&[SVELTE_RAW_TEXT])));
        assert_eq!(lexer.end, Some(0));
        let mut lexer = TestLexer::new("\\}tail");
        assert!(scanner.scan(&mut lexer, &valid(&[SVELTE_RAW_TEXT])));
        assert_eq!(lexer.end, Some(1)); // Raw-text backslashes do not escape braces.
    }

    #[test]
    fn special_sigils_and_slash_paths_keep_the_c_early_returns() {
        let mut scanner = Scanner::default();
        for text in ["@html x}", "#if x}", ":else}"] {
            let mut lexer = TestLexer::new(text);
            assert!(!scanner.scan(&mut lexer, &valid(&[SVELTE_RAW_TEXT])));
            assert!(lexer.events.is_empty());
        }
        let mut lexer = TestLexer::new("/* } */ trailing}");
        assert!(scanner.scan(&mut lexer, &valid(&[SVELTE_RAW_TEXT, SLASH])));
        assert_eq!(lexer.position, 7);
        assert_eq!(lexer.symbol, u16::MAX);
        assert_eq!(lexer.end, None);
        let mut lexer = TestLexer::new("//}\nrest}");
        assert!(scanner.scan(&mut lexer, &valid(&[SVELTE_RAW_TEXT, SLASH])));
        assert_eq!(lexer.end, Some(2)); // Already consumed the first slash.
        let mut lexer = TestLexer::new("//}\nrest}");
        assert!(scanner.scan(&mut lexer, &valid(&[SVELTE_RAW_TEXT])));
        assert_eq!(lexer.end, Some(8));
        let mut lexer = TestLexer::new("/if}");
        assert!(!scanner.scan(&mut lexer, &valid(&[SVELTE_RAW_TEXT, SLASH])));
        assert_eq!(lexer.position, 1);
        assert_eq!(lexer.symbol, u16::MAX);
    }

    #[test]
    fn each_as_boundary_does_not_require_a_word_boundary_or_balanced_braces() {
        let mut scanner = Scanner::default();
        for (text, end, position) in [
            ("as value}", 0, 2),
            ("has value}", 1, 3),
            ("{as value}}", 1, 3),
            ("items as item}", 6, 8),
            ("last}", 4, 4),
        ] {
            let mut lexer = TestLexer::new(text);
            assert!(scanner.scan(&mut lexer, &valid(&[SVELTE_RAW_TEXT_EACH])));
            assert_eq!(lexer.end, Some(end), "{text}");
            assert_eq!(lexer.position, position, "{text}");
            assert_eq!(lexer.symbol, SVELTE_RAW_TEXT_EACH as u16);
        }
    }

    #[test]
    fn snippet_arguments_balance_parentheses_and_skip_javascript_contexts() {
        let mut scanner = Scanner::default();
        for text in [
            "fn(a, b)",
            "')' + \"(\"",
            "/* ) */ x",
            "// )\nx",
            "`text ${')'}`",
        ] {
            let mut lexer = TestLexer::new(&format!(" {text})tail"));
            assert!(scanner.scan(&mut lexer, &valid(&[SVELTE_RAW_TEXT_SNIPPET_ARGUMENTS])));
            assert_eq!(lexer.end, Some(1 + text.len()), "{text}");
            assert_eq!(lexer.lookahead(), i32::from(b')'));
        }
        let mut lexer = TestLexer::new(" )");
        assert!(!scanner.scan(&mut lexer, &valid(&[SVELTE_RAW_TEXT_SNIPPET_ARGUMENTS])));
        assert_eq!(lexer.end, Some(1));
        let mut lexer = TestLexer::new("\\)tail");
        assert!(scanner.scan(&mut lexer, &valid(&[SVELTE_RAW_TEXT_SNIPPET_ARGUMENTS])));
        assert_eq!(lexer.end, Some(1));
    }

    #[test]
    fn nesting_counters_wrap_at_u8_width() {
        let mut scanner = Scanner::default();
        let mut lexer = TestLexer::new(&format!("{}}}}}", "{".repeat(256)));
        assert!(scanner.scan(&mut lexer, &valid(&[SVELTE_RAW_TEXT])));
        assert_eq!(lexer.end, Some(256));
        let mut lexer = TestLexer::new(&format!("{}))", "(".repeat(256)));
        assert!(scanner.scan(&mut lexer, &valid(&[SVELTE_RAW_TEXT_SNIPPET_ARGUMENTS])));
        assert_eq!(lexer.end, Some(256));
        let mut lexer = TestLexer::new(&format!("{}}}}}", "{".repeat(257)));
        assert!(scan_javascript_balanced_brace(&mut lexer));
        assert_eq!(lexer.position, 258);
    }

    #[test]
    fn classification_and_helper_eof_behavior_match_the_c_locale() {
        assert!(is_space(0x0b));
        assert!(!is_space(0xa0));
        assert!(!is_alnum(0xe9));
        let mut lexer = TestLexer::new("divé");
        assert_eq!(scan_tag_name(&mut lexer), b"div");
        assert_eq!(lexer.position, 3);
        let mut lexer = TestLexer::new("/comment without newline");
        assert!(!scan_javascript_line_comment(&mut lexer));
        assert!(lexer.eof());
        let mut lexer = TestLexer::new("* unterminated");
        assert!(!scan_javascript_block_comment(&mut lexer));
        assert!(lexer.eof());
        let mut lexer = TestLexer::new("'\\");
        assert!(!scan_javascript_quoted_string(&mut lexer, i32::from(b'\'')));
        assert_eq!(
            lexer.events,
            [
                Event::Advance(0, false),
                Event::Advance(1, false),
                Event::Advance(2, false),
            ]
        ); // Escapes advance twice even when the first advance reaches EOF.
    }
}
