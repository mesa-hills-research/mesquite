//! Unicode decoding helpers for the lexer. Malformed UTF-8 preserves ICU's
//! consumed-prefix length and -1 lookahead, rather than replacement characters.
pub(crate) const DECODE_ERROR: i32 = -1;

#[inline]
pub(crate) fn ts_decode_utf8(input: &[u8]) -> (u32, i32) {
    let Some(&lead) = input.first() else {
        // C's U8_NEXT requires nonempty input. Avoid an out-of-bounds read if a
        // callback returns an empty chunk when the lexer retries decoding.
        return (0, DECODE_ERROR);
    };
    if lead < 0x80 {
        return (1, i32::from(lead));
    }

    decode_utf8_non_ascii(input, lead)
}

// Separate the multi-byte decoder so callers can inline ASCII decoding without
// duplicating Unicode validation at each call site.
#[inline(never)]
fn decode_utf8_non_ascii(input: &[u8], lead: u8) -> (u32, i32) {
    let (width, mut code_point, second_min, second_max) = match lead {
        0xc2..=0xdf => (2, i32::from(lead & 0x1f), 0x80, 0xbf),
        0xe0 => (3, 0, 0xa0, 0xbf),
        0xe1..=0xec | 0xee..=0xef => (3, i32::from(lead & 0x0f), 0x80, 0xbf),
        0xed => (3, 0x0d, 0x80, 0x9f),
        0xf0 => (4, 0, 0x90, 0xbf),
        0xf1..=0xf3 => (4, i32::from(lead & 0x07), 0x80, 0xbf),
        0xf4 => (4, 4, 0x80, 0x8f),
        _ => return (1, DECODE_ERROR),
    };

    for i in 1..width {
        let Some(&byte) = input.get(i) else {
            return (i as u32, DECODE_ERROR);
        };
        let valid = if i == 1 {
            (second_min..=second_max).contains(&byte)
        } else {
            (0x80..=0xbf).contains(&byte)
        };
        if !valid {
            return (i as u32, DECODE_ERROR);
        }
        code_point = (code_point << 6) | i32::from(byte & 0x3f);
    }
    (width as u32, code_point)
}

pub(crate) fn ts_decode_utf16_le(input: &[u8]) -> (u32, i32) {
    decode_utf16(input, u16::from_le_bytes)
}

pub(crate) fn ts_decode_utf16_be(input: &[u8]) -> (u32, i32) {
    decode_utf16(input, u16::from_be_bytes)
}

fn decode_utf16(input: &[u8], decode_first: fn([u8; 2]) -> u16) -> (u32, i32) {
    let Some(first) = input.get(..2) else {
        // The C macros read a u16 without checking the byte length. Truncated
        // code units have no defined C result; report an error safely instead.
        return (input.len() as u32, DECODE_ERROR);
    };
    let lead = decode_first([first[0], first[1]]);
    if (0xd800..=0xdbff).contains(&lead)
        && let Some(second) = input.get(2..4)
    {
        // Preserve unicode.h's U16_NEXT_LE/BE quirk: only the first code
        // unit is endian-converted; the second is read in native order.
        let trail = u16::from_ne_bytes([second[0], second[1]]);
        if (0xdc00..=0xdfff).contains(&trail) {
            let code_point =
                ((i32::from(lead) - 0xd800) << 10) + (i32::from(trail) - 0xdc00) + 0x10000;
            return (4, code_point);
        }
    }
    // Unlike UTF-8 decoding, C's U16_NEXT macros return unpaired surrogates
    // unchanged, not TS_DECODE_ERROR.
    (2, i32::from(lead))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utf8_decodes_every_scalar() {
        let mut buffer = [0; 4];
        for value in 0..=0x10ffff {
            if let Some(character) = char::from_u32(value) {
                let encoded = character.encode_utf8(&mut buffer);
                assert_eq!(
                    ts_decode_utf8(encoded.as_bytes()),
                    (encoded.len() as u32, value as i32)
                );
            }
        }
    }

    #[test]
    fn utf8_invalid_sequences_preserve_consumed_prefix() {
        let cases: &[(&[u8], u32)] = &[
            (&[], 0),
            (&[0x80], 1),
            (&[0xc0, 0x80], 1),
            (&[0xc1, 0xbf], 1),
            (&[0xc2], 1),
            (&[0xc2, b'a'], 1),
            (&[0xe0, 0x9f, 0x80], 1),
            (&[0xe0, 0xa0], 2),
            (&[0xe0, 0xa0, b'a'], 2),
            (&[0xed, 0xa0, 0x80], 1),
            (&[0xf0, 0x8f, 0xbf, 0xbf], 1),
            (&[0xf0, 0x90], 2),
            (&[0xf0, 0x90, b'a'], 2),
            (&[0xf0, 0x90, 0x80], 3),
            (&[0xf0, 0x90, 0x80, b'a'], 3),
            (&[0xf4, 0x90, 0x80, 0x80], 1),
            (&[0xf5, 0x80, 0x80, 0x80], 1),
            (&[0xff, b'a'], 1),
        ];
        for &(input, consumed) in cases {
            assert_eq!(
                ts_decode_utf8(input),
                (consumed, DECODE_ERROR),
                "{input:x?}"
            );
        }
    }

    #[test]
    fn utf8_two_byte_inputs_match_standard_validation() {
        for first in 0..=u8::MAX {
            for second in 0..=u8::MAX {
                let bytes = [first, second];
                let expected = match std::str::from_utf8(&bytes) {
                    Ok(text) => {
                        let character = text.chars().next().unwrap();
                        (character.len_utf8() as u32, character as i32)
                    }
                    Err(error) if error.valid_up_to() > 0 => (1, i32::from(first)),
                    Err(error) => (
                        error.error_len().unwrap_or(bytes.len()) as u32,
                        DECODE_ERROR,
                    ),
                };
                assert_eq!(ts_decode_utf8(&bytes), expected, "{bytes:x?}");
            }
        }
    }

    #[test]
    fn utf16_preserves_native_order_trail_and_unpaired_surrogates() {
        assert_eq!(ts_decode_utf16_le(&[0xac, 0x20]), (2, 0x20ac));
        assert_eq!(ts_decode_utf16_be(&[0x20, 0xac]), (2, 0x20ac));
        assert_eq!(ts_decode_utf16_le(&[0x00, 0xdc]), (2, 0xdc00));
        assert_eq!(ts_decode_utf16_be(&[0xdc, 0x00]), (2, 0xdc00));
        assert_eq!(ts_decode_utf16_le(&[0x00, 0xd8, 0, 0]), (2, 0xd800));
        assert_eq!(ts_decode_utf16_be(&[0xd8, 0x00, 0, 0]), (2, 0xd800));

        let [a, b] = 0xde00_u16.to_ne_bytes();
        assert_eq!(ts_decode_utf16_le(&[0x3d, 0xd8, a, b]), (4, 0x1f600));
        assert_eq!(ts_decode_utf16_be(&[0xd8, 0x3d, a, b]), (4, 0x1f600));
        let [a, b] = 0xde00_u16.swap_bytes().to_ne_bytes();
        assert_eq!(ts_decode_utf16_le(&[0x3d, 0xd8, a, b]), (2, 0xd83d));
        assert_eq!(ts_decode_utf16_be(&[0xd8, 0x3d, a, b]), (2, 0xd83d));
    }
}
