//! The bounded, ASCII-only regex/subscript and glob/readline heuristics.
//! These retain the C scanner's deliberate approximations to Perl's lexer.

use super::keywords::is_perl_keyword;

fn word(c: u8) -> bool {
    c.is_ascii_alphanumeric() || c == b'_'
}
fn blank(c: u8) -> bool {
    matches!(c, b' ' | b'\t')
}
fn space(c: u8) -> bool {
    matches!(c, b' ' | b'\t' | b'\n' | b'\r' | 12 | 11)
}
fn in_set(set: &[u8], c: u8) -> bool {
    c != 0 && set.contains(&c)
}

fn regcurly(s: &[u8]) -> bool {
    if s.first() != Some(&b'{') {
        return false;
    }
    let mut i = 1;
    while i < s.len() && blank(s[i]) {
        i += 1;
    }
    let min_present = i < s.len() && s[i].is_ascii_digit();
    while i < s.len() && s[i].is_ascii_digit() {
        i += 1;
    }
    while i < s.len() && blank(s[i]) {
        i += 1;
    }
    let mut max_present = false;
    if s.get(i) == Some(&b',') {
        i += 1;
        while i < s.len() && blank(s[i]) {
            i += 1;
        }
        max_present = i < s.len() && s[i].is_ascii_digit();
        while i < s.len() && s[i].is_ascii_digit() {
            i += 1;
        }
        while i < s.len() && blank(s[i]) {
            i += 1;
        }
    }
    s.get(i) == Some(&b'}') && (min_present || max_present)
}

/// True means subscript; false means a character class or quantifier.
pub(super) fn intuit_more(s: &[u8]) -> bool {
    if s.is_empty() {
        return true;
    }
    if s[0] == b'{' {
        return !regcurly(s);
    }
    if s[0] != b'[' || s.len() == 1 {
        return true;
    }
    if matches!(s[1], b']' | b'^') {
        return false;
    }
    let Some(send) = s[1..].iter().position(|&c| c == b']').map(|i| i + 1) else {
        return true;
    };
    if s[1].is_ascii_digit() && send - 1 <= 2 && (send == 2 || s[2].is_ascii_digit()) {
        return true;
    }
    let mut weight: i32 = if s[1] == b'$' { -1 } else { 2 };
    let mut seen = [0u8; 256];
    let mut un_char = 0u8;
    let mut first_time = true;
    let mut i = 1;
    while i < send {
        let prev = un_char;
        un_char = s[i];
        let next = s.get(i + 1).copied().unwrap_or(0);
        match s[i] {
            b'@' | b'&' | b'$' => {
                weight -= i32::from(seen[un_char as usize]) * 10;
                if word(next) {
                    weight -= 10;
                } else if s[i] == b'$' && in_set(b"[#!%*<>()-=", next) {
                    let s2 = s.get(i + 2).copied().unwrap_or(0);
                    weight -= if in_set(b"])} =", s2) { 10 } else { 1 };
                }
            }
            b'\\' => {
                if next != 0 {
                    if in_set(b"wds]", next) {
                        weight += 100;
                    } else if seen[b'\'' as usize] != 0 || seen[b'"' as usize] != 0 {
                        weight += 1;
                    } else if in_set(b"abcfnrtvx", next) {
                        weight += 40;
                    } else if next.is_ascii_digit() {
                        weight += 40;
                        while i + 1 < send && s[i + 1].is_ascii_digit() {
                            i += 1;
                        }
                    }
                } else {
                    weight += 100;
                }
            }
            b'-' => {
                if next == b'\\' {
                    weight += 50;
                }
                if !first_time && in_set(b"aA01! ", prev) {
                    weight += 30;
                }
                if in_set(b"zZ79~", next) {
                    weight += 30;
                }
                if first_time && (next.is_ascii_digit() || next == b'$') {
                    weight -= 5;
                }
            }
            _ => {
                if (first_time || (!word(prev) && !matches!(prev, b'$' | b'@' | b'&')))
                    && s[i].is_ascii_alphabetic()
                    && i + 1 < send
                    && s[i + 1].is_ascii_alphabetic()
                {
                    let d = i;
                    while i < send && s[i].is_ascii_alphabetic() {
                        i += 1;
                    }
                    if is_perl_keyword(&s[d..i]) {
                        weight -= 150;
                    }
                    // Intentionally leave i at the first non-alpha; the loop
                    // increment skips it, exactly as Perl's pointer walk does.
                }
                if !first_time && un_char == prev.wrapping_add(1) {
                    weight += 5;
                }
                weight -= i32::from(seen[un_char as usize]);
            }
        }
        seen[un_char as usize] = seen[un_char as usize].wrapping_add(1);
        i += 1;
        first_time = false;
    }
    weight < 0
}

fn infix_word(s: &[u8]) -> bool {
    matches!(
        s,
        b"and"
            | b"or"
            | b"xor"
            | b"not"
            | b"cmp"
            | b"eq"
            | b"ne"
            | b"lt"
            | b"gt"
            | b"le"
            | b"ge"
            | b"x"
    )
}

fn content_has_infix_op(s: &[u8]) -> bool {
    let mut i = 0;
    while i < s.len() {
        while i < s.len() && space(s[i]) {
            i += 1;
        }
        let start = i;
        let mut clean = true;
        while i < s.len() && !space(s[i]) {
            if !s[i].is_ascii_alphabetic() {
                clean = false;
            }
            i += 1;
        }
        if clean && i > start && infix_word(&s[start..i]) {
            return true;
        }
    }
    false
}

fn content_is_globshaped(s: &[u8]) -> bool {
    for (i, &c) in s.iter().enumerate() {
        if matches!(c, b'*' | b'?' | b'[' | b']' | b'{' | b'}' | b'~' | b'/') {
            return true;
        }
        if c == b'.' && i > 0 && i + 1 < s.len() && word(s[i - 1]) && word(s[i + 1]) {
            return true;
        }
    }
    false
}

fn is_filehandle(s: &[u8]) -> bool {
    if s.is_empty() || space(s[0]) {
        return false;
    }
    let mut i = usize::from(s[0] == b'$');
    if i == s.len() {
        return false;
    }
    let mut last_was_word = false;
    while i < s.len() {
        if word(s[i]) {
            last_was_word = true;
            i += 1;
        } else if s[i] == b':' {
            if !last_was_word || s.get(i + 1) != Some(&b':') {
                return false;
            }
            last_was_word = false;
            i += 2;
        } else if s[i] == b'\'' {
            if !last_was_word {
                return false;
            }
            last_was_word = false;
            i += 1;
        } else if space(s[i]) {
            return last_was_word && s[i + 1..].iter().all(|&c| space(c));
        } else {
            return false;
        }
    }
    last_was_word
}

pub(super) fn is_fileglob(content: &[u8], after: &[u8]) -> bool {
    if content.is_empty() {
        return true;
    }
    if content_has_infix_op(content) {
        return false;
    }
    if !content_is_globshaped(content) && !is_filehandle(content) {
        return false;
    }
    let Some(&nc) = after.iter().find(|&&c| !space(c)) else {
        return true;
    };
    // C explicitly accepts both trailing keywords and unknown alpha words.
    if nc.is_ascii_alphabetic() {
        return true;
    }
    !(matches!(nc, b'$' | b'@' | b'%' | b'"' | b'\'' | b'(' | b'[' | b'{') || nc.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quantifiers_and_subscripts() {
        for text in [
            b"{3}".as_slice(),
            b"{3,}",
            b"{3,5}",
            b"{,5}",
            b"{ \t3 , 5\t}",
        ] {
            assert!(!intuit_more(text), "{text:?}");
        }
        for text in [
            b"{}".as_slice(),
            b"{,}",
            b"{foo}",
            b"{3",
            b"{3,5x}",
            b"{\n3}",
        ] {
            assert!(intuit_more(text), "{text:?}");
        }
        for text in [
            b"[0]".as_slice(),
            b"[12]",
            b"[$x]",
            b"[length]",
            b"[-1]",
            b"[",
        ] {
            assert!(intuit_more(text), "{text:?}");
        }
        for text in [b"[]".as_slice(), b"[^x]", b"[a-z]", b"[\\d]", b"[123]"] {
            assert!(!intuit_more(text), "{text:?}");
        }
    }

    #[test]
    fn globs_filehandles_and_relational_operators() {
        for (content, after) in [
            (b"*.c".as_slice(), b" + 1".as_slice()),
            (b"$sner ", b""),
            (b"Foo::BAR", b" if $x"),
            (b"foo.c", b" bareword"),
        ] {
            assert!(is_fileglob(content, after));
        }
        for (content, after) in [
            (b" 7 ".as_slice(), b" $x".as_slice()),
            (b" $a ", b" foo"),
            (b"$a/$b", b" $c"),
            (b" *.c and x ", b""),
            (b"Foo::", b""),
            (b"$", b""),
        ] {
            assert!(!is_fileglob(content, after));
        }
    }

    #[test]
    fn keyword_trie_acceptance() {
        for word in [b"q".as_slice(), b"END", b"length", b"getprotobynumber"] {
            assert!(is_perl_keyword(word));
        }
        for word in [
            b"".as_slice(),
            b"leng",
            b"lengthx",
            b"Length",
            b"getprotobynumber\0",
        ] {
            assert!(!is_perl_keyword(word));
        }
    }
}
