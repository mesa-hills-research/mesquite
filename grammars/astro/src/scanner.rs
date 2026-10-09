//! The Astro external scanner, translated from `src/scanner.c` and `src/tag.h`.
//!
//! The scanner extends tree-sitter-html's: it also scans the frontmatter's script,
//! attribute and template expressions, backtick strings, fragments and text, keeping
//! `{` interpolations and fragments on the tag stack beside the elements.

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
const HTML_INTERPOLATION_START: usize = 9;
const HTML_INTERPOLATION_END: usize = 10;
const FRONTMATTER_JS_BLOCK: usize = 11;
const ATTRIBUTE_JS_EXPR: usize = 12;
const ATTRIBUTE_BACKTICK_STRING: usize = 13;
const PERMISSIBLE_TEXT: usize = 14;
const FRAGMENT_TAG_DELIM: usize = 15;

// The tag types in the order of tag.h's enum, which gives the serialized
// discriminants, and the names of those with an entry in its name map.
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
                match name {
                    $($($name => Self::$variant,)?)*
                    // tag.h's map has room for 126 entries but lists 124, and the two
                    // zeroed entries left over name AREA with an empty string.
                    b"" => Self::Area,
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
    Area: b"area",
    Base: b"base",
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
    Interpolation,
    Fragment,
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
        Self::of_kind(TagType::End)
    }
}

impl Tag {
    fn of_kind(kind: TagType) -> Self {
        Self {
            kind,
            custom_tag_name: Vec::new(),
        }
    }

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
        if child == Interpolation {
            return true;
        }
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

// wctype.h in C's default locale, not Unicode character classes. C whitespace
// includes vertical tab, unlike Rust's u8::is_ascii_whitespace.
fn is_space(c: i32) -> bool {
    matches!(c, 0x09..=0x0d | 0x20)
}

fn is_alnum(c: i32) -> bool {
    matches!(c, 0x30..=0x39 | 0x41..=0x5a | 0x61..=0x7a)
}

fn is_ascii_alpha(c: i32) -> bool {
    matches!(c, 0x41..=0x5a | 0x61..=0x7a)
}

fn scan_tag_name(lexer: &mut dyn Lexer) -> Vec<u8> {
    let mut tag_name = Vec::new();
    loop {
        let c = lexer.lookahead();
        if !(is_alnum(c) || c == 0x2d || c == 0x3a || c == 0x2e) {
            return tag_name;
        }
        tag_name.push(c as u8);
        lexer.advance(false);
    }
}

fn scan_comment(lexer: &mut dyn Lexer) -> bool {
    if lexer.lookahead() != 0x2d {
        return false;
    }
    lexer.advance(false);
    if lexer.lookahead() != 0x2d {
        return false;
    }
    lexer.advance(false);

    let mut dashes = 0u32;
    while lexer.lookahead() != 0 {
        match lexer.lookahead() {
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
    false
}

/// Where a JavaScript expression ends (C's `RawTextEndType`).
#[derive(Clone, Copy, PartialEq, Eq)]
enum EndType {
    /// At `"\n---"`, the frontmatter's closing fence.
    Frontmatter,
    /// At the `}` that balances the braces.
    Curly,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum CommentState {
    NotInComment,
    SingleLine,
    MultiLine,
}

/// One activation of C's mutually recursive `scan_js_expr_with_delimiter` and
/// `scan_js_backtick_string`. Template strings and the expressions in them nest
/// without bound, so the scan keeps its activations in a vector rather than on the
/// call stack.
enum Frame {
    Expr {
        end_type: EndType,
        delimiter_index: usize,
        curly_count: u32,
        in_comment: CommentState,
    },
    Backtick {
        /// An expression in `${...}` returned: the loop's closing `advance` is due.
        resume: bool,
    },
}

const FRONTMATTER_END: &[u8; 4] = b"\n---";

/// C's `scan_js_expr_with_delimiter`.
fn scan_js_expr_with_delimiter(lexer: &mut dyn Lexer, end_type: EndType) {
    let mut frames = vec![expr_frame(lexer, end_type)];
    run_js(lexer, &mut frames);
}

/// C's `scan_js_backtick_string`, with the lexer on the opening backtick.
fn scan_js_backtick_string(lexer: &mut dyn Lexer) {
    lexer.advance(false);
    let mut frames = vec![Frame::Backtick { resume: false }];
    run_js(lexer, &mut frames);
}

/// C's `scan_js_string`: the lexer is on a quote, a double quote or a backtick.
fn scan_js_string(lexer: &mut dyn Lexer) {
    if lexer.lookahead() == 0x60 {
        scan_js_backtick_string(lexer);
    } else {
        scan_quoted_string(lexer);
    }
}

/// The quoted-string half of C's `scan_js_string`.
fn scan_quoted_string(lexer: &mut dyn Lexer) {
    let str_end_char = lexer.lookahead();
    lexer.advance(false);
    loop {
        let c = lexer.lookahead();
        if c == 0 {
            return;
        }
        if c == 0x5c {
            // Accept any next character.
            lexer.advance(false);
        } else if c == str_end_char {
            lexer.advance(false);
            return;
        }
        lexer.advance(false);
    }
}

/// Enters `scan_js_expr_with_delimiter`.
fn expr_frame(lexer: &mut dyn Lexer, end_type: EndType) -> Frame {
    lexer.mark_end();
    // 1: the newline that ends the opening fence (which the parser consumed) also
    // starts the closing one, so an empty frontmatter ends at once.
    Frame::Expr {
        end_type,
        delimiter_index: 1,
        curly_count: 0,
        in_comment: CommentState::NotInComment,
    }
}

/// Runs the loops of the activations in `frames` until the outermost returns. Each
/// step is one round of the innermost activation's loop.
fn run_js(lexer: &mut dyn Lexer, frames: &mut Vec<Frame>) {
    while let Some(frame) = frames.last_mut() {
        match frame {
            Frame::Expr {
                end_type,
                delimiter_index,
                curly_count,
                in_comment,
            } => {
                let c = lexer.lookahead();
                if c == 0 {
                    frames.pop();
                    continue;
                }
                match *in_comment {
                    CommentState::NotInComment => {
                        if *end_type == EndType::Frontmatter {
                            // mark_end at index 0, always.
                            if *delimiter_index == 0 {
                                lexer.mark_end();
                            }
                            if c == i32::from(FRONTMATTER_END[*delimiter_index]) {
                                *delimiter_index += 1;
                                if *delimiter_index == FRONTMATTER_END.len() {
                                    frames.pop();
                                    continue;
                                }
                            } else {
                                lexer.mark_end();
                                *delimiter_index = usize::from(c == 0x0a);
                            }
                        } else {
                            lexer.mark_end();
                            // Balance braces.
                            if c == 0x7b {
                                *curly_count = curly_count.wrapping_add(1);
                            } else if c == 0x7d {
                                if *curly_count == 0 {
                                    lexer.mark_end();
                                    frames.pop();
                                    continue;
                                }
                                *curly_count -= 1;
                            }
                        }
                        if c == 0x60 {
                            lexer.advance(false);
                            frames.push(Frame::Backtick { resume: false });
                            continue;
                        }
                        if c == 0x22 || c == 0x27 {
                            scan_quoted_string(lexer);
                            continue;
                        }
                        if c == 0x2f {
                            // A comment?
                            lexer.advance(false);
                            match lexer.lookahead() {
                                0x2f => *in_comment = CommentState::SingleLine,
                                0x2a => *in_comment = CommentState::MultiLine,
                                _ => {}
                            }
                            continue;
                        }
                    }
                    CommentState::SingleLine => {
                        if c == 0x0a {
                            *in_comment = CommentState::NotInComment;
                            // Frontmatter fences start with a newline.
                            *delimiter_index = usize::from(*end_type == EndType::Frontmatter);
                            // So the token ends before a fence that follows the comment.
                            lexer.mark_end();
                        }
                    }
                    CommentState::MultiLine => {
                        if c == 0x2a {
                            lexer.advance(false);
                            if lexer.lookahead() == 0x2f {
                                *in_comment = CommentState::NotInComment;
                                *delimiter_index = 0;
                            } else {
                                continue;
                            }
                        }
                    }
                }
                lexer.advance(false);
            }
            Frame::Backtick { resume } => {
                if *resume {
                    // Past the `}` that ended the interpolation.
                    *resume = false;
                    lexer.advance(false);
                }
                let c = lexer.lookahead();
                if c == 0 {
                    frames.pop();
                    continue;
                }
                if c == 0x24 {
                    lexer.advance(false);
                    if lexer.lookahead() == 0x7b {
                        // String interpolation.
                        lexer.advance(false);
                        *resume = true;
                        let expr = expr_frame(lexer, EndType::Curly);
                        frames.push(expr);
                    }
                    // Otherwise this character is looked at again.
                    continue;
                }
                if c == 0x60 {
                    // The end of the string.
                    lexer.advance(false);
                    frames.pop();
                    continue;
                }
                lexer.advance(false);
            }
        }
    }
}

fn scan_permissible_text(lexer: &mut dyn Lexer) -> bool {
    let mut there_is_text = false;

    'text: while lexer.lookahead() != 0 {
        // C's `goto text_found` is a `break 'found`.
        'found: {
            let c = lexer.lookahead();
            if c == 0x7b || c == 0x7d {
                // The start or end of an interpolation.
                break 'text;
            }
            if c == 0x27 || c == 0x22 || c == 0x60 {
                scan_js_string(lexer);
                break 'found;
            }
            if c == 0x2f {
                lexer.advance(false);
                if lexer.lookahead() == 0x2f {
                    // A single-line comment.
                    while !matches!(lexer.lookahead(), 0x0d | 0x0a | 0) {
                        lexer.advance(false);
                    }
                }
                if lexer.lookahead() == 0x2a {
                    // A multi-line comment.
                    while lexer.lookahead() != 0 {
                        lexer.advance(false);
                        if lexer.lookahead() == 0x2a {
                            lexer.advance(false);
                            if lexer.lookahead() == 0x2f {
                                lexer.advance(false);
                                break;
                            }
                        }
                    }
                }
                break 'found;
            }
            if c == 0x3c {
                lexer.advance(false);
                // How the Astro compiler ends a text node: at what may be a start
                // tag, an end tag, `<?` or a fragment. C tests '/' twice where the
                // second test means `<!`, so `<!` stays text.
                let next = lexer.lookahead();
                if is_ascii_alpha(next) || next == 0x2f || next == 0x3f || next == 0x3e {
                    break 'text;
                }
                break 'found;
            }
            lexer.advance(false);
        }
        there_is_text = true;
        lexer.mark_end();
    }

    if there_is_text {
        lexer.set_result_symbol(PERMISSIBLE_TEXT as u16);
        true
    } else {
        false
    }
}

/// The scanner's tag stack: elements, interpolations and fragments.
#[derive(Default)]
pub(crate) struct Scanner {
    tags: Vec<Tag>,
}

impl Scanner {
    fn scan_raw_text(&self, lexer: &mut dyn Lexer) -> bool {
        lexer.mark_end();

        // C reads the top of the stack without checking that there is one: the script
        // or style element whose start tag came before the raw text.
        let end_delimiter: &[u8] = if self
            .tags
            .last()
            .is_some_and(|tag| tag.kind == TagType::Script)
        {
            b"</script"
        } else {
            b"</style"
        };

        let mut delimiter_index = 0;
        while lexer.lookahead() != 0 {
            if lexer.lookahead() == i32::from(end_delimiter[delimiter_index]) {
                delimiter_index += 1;
                if delimiter_index == end_delimiter.len() {
                    break;
                }
                lexer.advance(false);
            } else {
                delimiter_index = 0;
                lexer.advance(false);
                lexer.mark_end();
            }
        }

        lexer.set_result_symbol(RAW_TEXT as u16);
        true
    }

    fn scan_implicit_end_tag(&mut self, lexer: &mut dyn Lexer) -> bool {
        let is_closing_tag = lexer.lookahead() == 0x2f;
        if is_closing_tag {
            lexer.advance(false);
        } else if self.tags.last().is_some_and(Tag::is_void) {
            self.tags.pop();
            lexer.set_result_symbol(IMPLICIT_END_TAG as u16);
            return true;
        }

        let tag_name = scan_tag_name(lexer);
        if tag_name.is_empty() && !lexer.eof() {
            return false;
        }
        let next_tag = Tag::for_name(tag_name);

        if is_closing_tag {
            // A matching topmost tag is left to the explicit end tag.
            if self.tags.last() == Some(&next_tag) {
                return false;
            }

            // Otherwise close one tag if any open tag has the same type (custom
            // names aside), to be lenient with malformed markup.
            if self.tags.iter().any(|tag| tag.kind == next_tag.kind) {
                self.tags.pop();
                lexer.set_result_symbol(IMPLICIT_END_TAG as u16);
                return true;
            }
        } else if self.tags.last().is_some_and(|parent| {
            !parent.can_contain(&next_tag)
                || (matches!(parent.kind, TagType::Html | TagType::Head | TagType::Body)
                    && lexer.eof())
        }) {
            self.tags.pop();
            lexer.set_result_symbol(IMPLICIT_END_TAG as u16);
            return true;
        }
        false
    }

    fn scan_start_tag_name(&mut self, lexer: &mut dyn Lexer) -> bool {
        let tag_name = scan_tag_name(lexer);
        if tag_name.is_empty() {
            // Fragment tags have no spaces.
            if lexer.lookahead() == 0x3e {
                lexer.advance(false);
                self.tags.push(Tag::of_kind(TagType::Fragment));
                lexer.set_result_symbol(FRAGMENT_TAG_DELIM as u16);
                return true;
            }
            return false;
        }

        let tag = Tag::for_name(tag_name);
        let kind = tag.kind;
        self.tags.push(tag);
        lexer.set_result_symbol(match kind {
            TagType::Script => SCRIPT_START_TAG_NAME as u16,
            TagType::Style => STYLE_START_TAG_NAME as u16,
            _ => START_TAG_NAME as u16,
        });
        true
    }

    fn scan_end_tag_name(&mut self, lexer: &mut dyn Lexer) -> bool {
        let tag_name = scan_tag_name(lexer);
        if tag_name.is_empty() {
            // Closing fragment tags have no spaces.
            if lexer.lookahead() == 0x3e {
                lexer.advance(false);
                if self
                    .tags
                    .last()
                    .is_some_and(|tag| tag.kind == TagType::Fragment)
                {
                    self.tags.pop();
                    lexer.set_result_symbol(FRAGMENT_TAG_DELIM as u16);
                } else {
                    lexer.set_result_symbol(ERRONEOUS_END_TAG_NAME as u16);
                }
                return true;
            }
            return false;
        }

        let tag = Tag::for_name(tag_name);
        if self.tags.last() == Some(&tag) {
            self.tags.pop();
            lexer.set_result_symbol(END_TAG_NAME as u16);
        } else {
            lexer.set_result_symbol(ERRONEOUS_END_TAG_NAME as u16);
        }
        true
    }

    fn scan_self_closing_tag_delimiter(&mut self, lexer: &mut dyn Lexer) -> bool {
        lexer.advance(false);
        if lexer.lookahead() == 0x3e {
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
        if valid_symbols[FRONTMATTER_JS_BLOCK] && self.tags.is_empty() {
            scan_js_expr_with_delimiter(lexer, EndType::Frontmatter);
            lexer.set_result_symbol(FRONTMATTER_JS_BLOCK as u16);
            return true;
        }

        if valid_symbols[RAW_TEXT] && !valid_symbols[START_TAG_NAME] && !valid_symbols[END_TAG_NAME]
        {
            return self.scan_raw_text(lexer);
        }

        if valid_symbols[ATTRIBUTE_JS_EXPR] {
            scan_js_expr_with_delimiter(lexer, EndType::Curly);
            lexer.set_result_symbol(ATTRIBUTE_JS_EXPR as u16);
            return true;
        }

        if valid_symbols[PERMISSIBLE_TEXT] {
            if is_space(lexer.lookahead()) {
                // Can't be anything else.
                return scan_permissible_text(lexer);
            }
        } else {
            while is_space(lexer.lookahead()) {
                lexer.advance(true);
            }
        }

        let mut definitely_not_permissible_text = false;

        match lexer.lookahead() {
            0x3c => {
                lexer.mark_end();
                lexer.advance(false);

                if lexer.lookahead() == 0x21 {
                    lexer.advance(false);
                    return scan_comment(lexer);
                }

                if valid_symbols[IMPLICIT_END_TAG] {
                    return self.scan_implicit_end_tag(lexer);
                }

                if valid_symbols[PERMISSIBLE_TEXT] {
                    let c = lexer.lookahead();
                    if is_ascii_alpha(c) || c == 0x2f || c == 0x3f || c == 0x3e {
                        // This looks like an element, so it can't be text.
                        definitely_not_permissible_text = true;
                    }
                }
            }
            0 => {
                definitely_not_permissible_text = true;
                if valid_symbols[IMPLICIT_END_TAG] {
                    return self.scan_implicit_end_tag(lexer);
                }
            }
            0x2f => {
                if valid_symbols[SELF_CLOSING_TAG_DELIMITER] {
                    return self.scan_self_closing_tag_delimiter(lexer);
                }
            }
            0x7b => {
                if valid_symbols[HTML_INTERPOLATION_START] {
                    lexer.advance(false);
                    self.tags.push(Tag::of_kind(TagType::Interpolation));
                    lexer.set_result_symbol(HTML_INTERPOLATION_START as u16);
                    return true;
                }
            }
            0x7d => {
                // Close any void tags before leaving the interpolation.
                if valid_symbols[IMPLICIT_END_TAG] {
                    return self.scan_implicit_end_tag(lexer);
                }

                if valid_symbols[HTML_INTERPOLATION_END]
                    && self
                        .tags
                        .last()
                        .is_some_and(|tag| tag.kind == TagType::Interpolation)
                {
                    lexer.advance(false);
                    self.tags.pop();
                    lexer.set_result_symbol(HTML_INTERPOLATION_END as u16);
                    return true;
                }
            }
            0x60 => {
                if valid_symbols[ATTRIBUTE_BACKTICK_STRING] {
                    scan_js_backtick_string(lexer);
                    lexer.mark_end();
                    lexer.set_result_symbol(ATTRIBUTE_BACKTICK_STRING as u16);
                    return true;
                }
            }
            _ => {
                if (valid_symbols[START_TAG_NAME] || valid_symbols[END_TAG_NAME])
                    && !valid_symbols[RAW_TEXT]
                {
                    return if valid_symbols[START_TAG_NAME] {
                        self.scan_start_tag_name(lexer)
                    } else {
                        self.scan_end_tag_name(lexer)
                    };
                }
            }
        }

        if !definitely_not_permissible_text && valid_symbols[PERMISSIBLE_TEXT] {
            // There are no other choices.
            return scan_permissible_text(lexer);
        }

        false
    }

    fn serialize(&mut self, buffer: &mut [u8]) -> usize {
        let tag_count = self.tags.len().min(usize::from(u16::MAX)) as u16;
        let mut serialized_tag_count = 0u16;
        // C copies both u16 counts with memcpy, so the format is native-endian.
        buffer[2..4].copy_from_slice(&tag_count.to_ne_bytes());
        let mut size = 4;

        for tag in self.tags.iter().take(usize::from(tag_count)) {
            if tag.kind == TagType::Custom {
                let name_length = tag.custom_tag_name.len().min(usize::from(u8::MAX));
                if size + 2 + name_length >= SERIALIZATION_BUFFER_SIZE {
                    break;
                }
                buffer[size] = tag.kind as u8;
                buffer[size + 1] = name_length as u8;
                size += 2;

                // strncpy: stop at a NUL and pad with zeros.
                let name = &tag.custom_tag_name[..name_length];
                let copy_length = name.iter().position(|&c| c == 0).unwrap_or(name_length);
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
        size
    }

    fn deserialize(&mut self, buffer: &[u8]) {
        self.tags.clear();
        if buffer.is_empty() {
            return;
        }

        let serialized_tag_count = u16::from_ne_bytes([buffer[0], buffer[1]]);
        let tag_count = u16::from_ne_bytes([buffer[2], buffer[3]]);
        let mut size = 4;
        self.tags.reserve(usize::from(tag_count));
        if tag_count > 0 {
            for _ in 0..serialized_tag_count {
                let kind = TagType::from_byte(buffer[size]);
                size += 1;
                let custom_tag_name = if kind == TagType::Custom {
                    let name_length = usize::from(buffer[size]);
                    size += 1;
                    let name = buffer[size..size + name_length].to_vec();
                    size += name_length;
                    name
                } else {
                    Vec::new()
                };
                self.tags.push(Tag {
                    kind,
                    custom_tag_name,
                });
            }
            // Tags that didn't fit in the buffer come back as END_ tags, which
            // keeps the stack's depth.
            for _ in serialized_tag_count..tag_count {
                self.tags.push(Tag::default());
            }
        }
    }
}

/// Creates a scanner (C's `tree_sitter_astro_external_scanner_create`).
pub(crate) fn create() -> Box<dyn ExternalScanner> {
    Box::<Scanner>::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A lexer over a string, recording where `mark_end` was called.
    struct TestLexer {
        input: Vec<i32>,
        position: usize,
        end: Option<usize>,
        symbol: u16,
    }

    impl TestLexer {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().map(|c| c as i32).collect(),
                position: 0,
                end: None,
                symbol: u16::MAX,
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
        }

        fn advance(&mut self, _skip: bool) {
            if self.position < self.input.len() {
                self.position += 1;
            }
        }

        fn mark_end(&mut self) {
            self.end = Some(self.position);
        }

        fn eof(&self) -> bool {
            self.position == self.input.len()
        }

        fn get_column(&mut self) -> u32 {
            panic!("the Astro scanner does not call get_column")
        }

        fn is_at_included_range_start(&self) -> bool {
            panic!("the Astro scanner does not check included ranges")
        }
    }

    fn scan(scanner: &mut Scanner, input: &str, symbols: &[usize]) -> (bool, TestLexer) {
        let mut lexer = TestLexer::new(input);
        let mut valid = [false; 16];
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
                .map(|name| Tag::for_name(name.as_bytes().to_vec()))
                .collect(),
        }
    }

    #[test]
    fn tag_map_and_serialized_discriminants() {
        assert_eq!(TAG_TYPES_BY_TAG_NAME.len(), 124);
        for (i, &(name, kind)) in TAG_TYPES_BY_TAG_NAME.iter().enumerate() {
            let id = if i < 21 { i } else { i + 1 } as u8;
            let id = if kind == TagType::Custom { 126 } else { id };
            assert_eq!(kind as u8, id);
            assert_eq!(TagType::from_byte(id), kind);
            assert_eq!(Tag::for_name(name.to_vec()).kind, kind);
            assert_eq!(Tag::for_name(name.to_vec()).is_void(), i < 21);
        }
        assert_eq!(TagType::Interpolation as u8, 124);
        assert_eq!(TagType::Fragment as u8, 125);
        assert_eq!(TagType::End as u8, 127);
        // Names are case-sensitive, and the map's two empty entries name AREA.
        assert_eq!(TagType::for_name(b"DIV"), TagType::Custom);
        assert_eq!(TagType::for_name(b""), TagType::Area);
    }

    #[test]
    fn serialization_round_trip() {
        let mut scanner = with_tags(&["html", "my-element", "p"]);
        scanner.tags.push(Tag::of_kind(TagType::Interpolation));
        scanner.tags.push(Tag::of_kind(TagType::Fragment));
        let mut buffer = [0u8; SERIALIZATION_BUFFER_SIZE];
        let size = scanner.serialize(&mut buffer);
        assert_eq!(size, 4 + 1 + 2 + 10 + 1 + 1 + 1);
        let mut restored = Scanner::default();
        restored.deserialize(&buffer[..size]);
        assert_eq!(restored.tags, scanner.tags);
        restored.deserialize(&[]);
        assert!(restored.tags.is_empty());
    }

    #[test]
    fn frontmatter_ends_before_the_fence() {
        let mut scanner = Scanner::default();
        let input = "const a = `x${'---'}`; // ---\n/* \n--- */\n---\n<div/>";
        let (accepted, lexer) = scan(&mut scanner, input, &[FRONTMATTER_JS_BLOCK]);
        assert!(accepted);
        assert_eq!(lexer.symbol, FRONTMATTER_JS_BLOCK as u16);
        assert_eq!(lexer.end, Some(input.find("\n---\n<div").unwrap()));
    }

    #[test]
    fn attribute_expression_ends_at_the_balancing_brace() {
        let mut scanner = Scanner::default();
        let input = "{a: `${b}}`, c: '}'}} rest";
        let (accepted, lexer) = scan(&mut scanner, input, &[ATTRIBUTE_JS_EXPR]);
        assert!(accepted);
        assert_eq!(lexer.end, Some(input.find("} rest").unwrap()));
    }

    #[test]
    fn nested_template_strings_of_any_depth() {
        let depth = 200_000;
        let input = format!("`{}", "${`".repeat(depth));
        let mut scanner = Scanner::default();
        let (accepted, lexer) = scan(&mut scanner, &input, &[ATTRIBUTE_BACKTICK_STRING]);
        assert!(accepted);
        assert_eq!(lexer.end, Some(input.chars().count()));
    }

    #[test]
    fn text_stops_at_tags_and_interpolations() {
        let mut scanner = Scanner::default();
        let (accepted, lexer) = scan(&mut scanner, "a < b <!x> c<span>", &[PERMISSIBLE_TEXT]);
        assert!(accepted);
        assert_eq!(lexer.symbol, PERMISSIBLE_TEXT as u16);
        assert_eq!(lexer.end, Some(12));
        let (_, lexer) = scan(&mut scanner, "price {x}", &[PERMISSIBLE_TEXT]);
        assert_eq!(lexer.end, Some(6));
        let (accepted, _) = scan(&mut scanner, "<div>", &[PERMISSIBLE_TEXT]);
        assert!(!accepted);
    }

    #[test]
    fn fragments_and_interpolations_use_the_tag_stack() {
        let mut scanner = Scanner::default();
        let (accepted, lexer) = scan(&mut scanner, ">", &[START_TAG_NAME]);
        assert!(accepted);
        assert_eq!(lexer.symbol, FRAGMENT_TAG_DELIM as u16);
        let (_, lexer) = scan(&mut scanner, "{", &[HTML_INTERPOLATION_START]);
        assert_eq!(lexer.symbol, HTML_INTERPOLATION_START as u16);
        assert_eq!(scanner.tags.len(), 2);
        let (_, lexer) = scan(&mut scanner, "}", &[HTML_INTERPOLATION_END]);
        assert_eq!(lexer.symbol, HTML_INTERPOLATION_END as u16);
        let (_, lexer) = scan(&mut scanner, ">", &[END_TAG_NAME]);
        assert_eq!(lexer.symbol, FRAGMENT_TAG_DELIM as u16);
        assert!(scanner.tags.is_empty());
        let (_, lexer) = scan(&mut scanner, ">", &[END_TAG_NAME]);
        assert_eq!(lexer.symbol, ERRONEOUS_END_TAG_NAME as u16);
    }

    #[test]
    fn raw_text_ends_before_the_lowercase_end_tag() {
        let mut scanner = with_tags(&["script"]);
        let (accepted, lexer) = scan(&mut scanner, "a</SCRIPT></script>", &[RAW_TEXT]);
        assert!(accepted);
        assert_eq!(lexer.end, Some(10));
        let mut scanner = with_tags(&["style"]);
        let (_, lexer) = scan(&mut scanner, "a{}</style>", &[RAW_TEXT]);
        assert_eq!(lexer.end, Some(3));
    }

    #[test]
    fn implicit_end_tags() {
        let mut scanner = with_tags(&["p"]);
        let (accepted, lexer) = scan(&mut scanner, "<div>", &[IMPLICIT_END_TAG]);
        assert!(accepted);
        assert_eq!(lexer.symbol, IMPLICIT_END_TAG as u16);
        assert!(scanner.tags.is_empty());
        let mut scanner = with_tags(&["br"]);
        let (accepted, _) = scan(&mut scanner, "}", &[IMPLICIT_END_TAG]);
        assert!(accepted);
        assert!(scanner.tags.is_empty());
        let mut scanner = with_tags(&["ul", "li"]);
        let (accepted, _) = scan(&mut scanner, "</ul>", &[IMPLICIT_END_TAG]);
        assert!(accepted);
        assert_eq!(scanner.tags.len(), 1);
    }
}
