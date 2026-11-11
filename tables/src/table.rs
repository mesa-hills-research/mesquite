//! [`Table`]: an array of a grammar's table blob, read where it lies.

use std::fmt;
use std::ops::RangeBounds;

use crate::{LexMode, ParseAction, ParseActionEntry};

/// A value stored in a table blob as a fixed number of little-endian bytes.
pub trait Record: Copy + 'static {
    /// The value's bytes in the blob (`[u8; N]`).
    type Bytes: Copy + Eq + 'static;

    /// Reads a value from its bytes.
    fn read(bytes: Self::Bytes) -> Self;

    /// The bytes [`Record::read`] turns back into `self`.
    fn write(self) -> Self::Bytes;

    /// Splits `bytes` into records, leaving out a partial one at the end.
    fn records(bytes: &[u8]) -> &[Self::Bytes];
}

impl Record for u16 {
    type Bytes = [u8; 2];

    #[inline(always)]
    fn read(bytes: [u8; 2]) -> Self {
        u16::from_le_bytes(bytes)
    }

    fn write(self) -> [u8; 2] {
        self.to_le_bytes()
    }

    fn records(bytes: &[u8]) -> &[[u8; 2]] {
        bytes.as_chunks().0
    }
}

impl Record for u32 {
    type Bytes = [u8; 4];

    #[inline(always)]
    fn read(bytes: [u8; 4]) -> Self {
        u32::from_le_bytes(bytes)
    }

    fn write(self) -> [u8; 4] {
        self.to_le_bytes()
    }

    fn records(bytes: &[u8]) -> &[[u8; 4]] {
        bytes.as_chunks().0
    }
}

/// Three `u16`s: `lex_state`, `external_lex_state`, `reserved_word_set_id`.
impl Record for LexMode {
    type Bytes = [u8; 6];

    #[inline(always)]
    fn read(b: [u8; 6]) -> Self {
        LexMode {
            lex_state: u16::from_le_bytes([b[0], b[1]]),
            external_lex_state: u16::from_le_bytes([b[2], b[3]]),
            reserved_word_set_id: u16::from_le_bytes([b[4], b[5]]),
        }
    }

    fn write(self) -> [u8; 6] {
        let [a, b] = self.lex_state.to_le_bytes();
        let [c, d] = self.external_lex_state.to_le_bytes();
        let [e, f] = self.reserved_word_set_id.to_le_bytes();
        [a, b, c, d, e, f]
    }

    fn records(bytes: &[u8]) -> &[[u8; 6]] {
        bytes.as_chunks().0
    }
}

/// Eight bytes: the kind (0 header, 1 shift, 2 reduce, 3 accept, 4 recover), a byte
/// `a` and three `u16`s `b`, `c`, `d`. A header is `{count: a, reusable: b}`, a shift
/// `{state: b, extra: c, repetition: d}` and a reduce `{child_count: a, symbol: b,
/// dynamic_precedence: c, production_id: d}`.
impl Record for ParseActionEntry {
    type Bytes = [u8; 8];

    #[inline(always)]
    fn read(b: [u8; 8]) -> Self {
        if b[0] == 0 {
            ParseActionEntry::Header {
                count: b[1],
                reusable: u16::from_le_bytes([b[2], b[3]]) != 0,
            }
        } else {
            ParseActionEntry::Action(ParseAction::read(b))
        }
    }

    fn write(self) -> [u8; 8] {
        match self {
            ParseActionEntry::Header { count, reusable } => {
                [0, count, u8::from(reusable), 0, 0, 0, 0, 0]
            }
            ParseActionEntry::Action(action) => action.write(),
        }
    }

    fn records(bytes: &[u8]) -> &[[u8; 8]] {
        bytes.as_chunks().0
    }
}

/// The records of [`ParseActionEntry`] other than headers.
impl Record for ParseAction {
    type Bytes = [u8; 8];

    #[inline(always)]
    fn read(b: [u8; 8]) -> Self {
        let field = |i: usize| u16::from_le_bytes([b[i], b[i + 1]]);
        match b[0] {
            1 => ParseAction::Shift {
                state: field(2),
                extra: field(4) != 0,
                repetition: field(6) != 0,
            },
            2 => ParseAction::Reduce {
                symbol: field(2),
                child_count: b[1],
                dynamic_precedence: field(4) as i16,
                production_id: field(6),
            },
            3 => ParseAction::Accept,
            4 => ParseAction::Recover,
            kind => invalid_action_kind(kind),
        }
    }

    fn write(self) -> [u8; 8] {
        let record = |kind: u8, a: u8, b: u16, c: u16, d: u16| {
            let [b0, b1] = b.to_le_bytes();
            let [c0, c1] = c.to_le_bytes();
            let [d0, d1] = d.to_le_bytes();
            [kind, a, b0, b1, c0, c1, d0, d1]
        };
        match self {
            ParseAction::Shift {
                state,
                extra,
                repetition,
            } => record(1, 0, state, u16::from(extra), u16::from(repetition)),
            ParseAction::Reduce {
                symbol,
                child_count,
                dynamic_precedence,
                production_id,
            } => record(
                2,
                child_count,
                symbol,
                dynamic_precedence as u16,
                production_id,
            ),
            ParseAction::Accept => record(3, 0, 0, 0, 0),
            ParseAction::Recover => record(4, 0, 0, 0, 0),
        }
    }

    fn records(bytes: &[u8]) -> &[[u8; 8]] {
        bytes.as_chunks().0
    }
}

#[cold]
#[inline(never)]
fn invalid_action_kind(kind: u8) -> ! {
    panic!("parse action of kind {kind} (a header, or not an action)")
}

/// An array of a grammar's tables, read in place from the blob the grammar crate
/// embeds, as C reads its static arrays. Out-of-range reads panic.
pub struct Table<T: Record> {
    records: &'static [T::Bytes],
}

impl<T: Record> Clone for Table<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Record> Copy for Table<T> {}

impl<T: Record> Default for Table<T> {
    fn default() -> Self {
        Self { records: &[] }
    }
}

impl<T: Record> PartialEq for Table<T> {
    fn eq(&self, other: &Self) -> bool {
        self.records == other.records
    }
}

impl<T: Record> Eq for Table<T> {}

impl<T: Record + fmt::Debug> fmt::Debug for Table<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T: Record> Table<T> {
    /// The table over `records`.
    pub const fn from_records(records: &'static [T::Bytes]) -> Self {
        Self { records }
    }

    /// The table over `bytes`, a whole number of records.
    pub fn from_bytes(bytes: &'static [u8]) -> Self {
        Self {
            records: T::records(bytes),
        }
    }

    /// A table of `values`, which stay allocated for the rest of the process, as tables
    /// do. For building tables in memory rather than from a blob.
    pub fn leak(values: Vec<T>) -> Self {
        Self {
            records: Box::leak(values.into_iter().map(T::write).collect()),
        }
    }

    /// The records, as stored.
    pub const fn records(self) -> &'static [T::Bytes] {
        self.records
    }

    /// The number of values.
    #[inline(always)]
    pub const fn len(self) -> usize {
        self.records.len()
    }

    /// Whether the table has no values.
    #[inline(always)]
    pub const fn is_empty(self) -> bool {
        self.records.is_empty()
    }

    /// The value at `index`; panics if it is out of range.
    #[inline(always)]
    pub fn get(self, index: usize) -> T {
        T::read(self.records[index])
    }

    /// The first value.
    #[inline]
    pub fn first(self) -> Option<T> {
        self.records.first().map(|&bytes| T::read(bytes))
    }

    /// The last value.
    #[inline]
    pub fn last(self) -> Option<T> {
        self.records.last().map(|&bytes| T::read(bytes))
    }

    /// The values in `range`; panics if it is out of range.
    #[inline]
    pub fn slice(self, range: impl RangeBounds<usize>) -> Self {
        let range = (range.start_bound().cloned(), range.end_bound().cloned());
        Self {
            records: &self.records[range],
        }
    }

    /// The values in order.
    #[inline]
    pub fn iter(self) -> impl DoubleEndedIterator<Item = T> + ExactSizeIterator + Clone + 'static {
        self.records.iter().map(|&bytes| T::read(bytes))
    }

    /// The values, copied into a `Vec`.
    pub fn to_vec(self) -> Vec<T> {
        self.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn records_read_back_what_they_write() {
        let actions = vec![
            ParseActionEntry::Header {
                count: 3,
                reusable: true,
            },
            ParseActionEntry::Action(ParseAction::Shift {
                state: 513,
                extra: false,
                repetition: true,
            }),
            ParseActionEntry::Action(ParseAction::Reduce {
                symbol: 700,
                child_count: 4,
                dynamic_precedence: -3,
                production_id: 65535,
            }),
            ParseActionEntry::Action(ParseAction::Accept),
            ParseActionEntry::Action(ParseAction::Recover),
        ];
        assert_eq!(Table::leak(actions.clone()).to_vec(), actions);
        let modes = vec![LexMode {
            lex_state: 1,
            external_lex_state: 258,
            reserved_word_set_id: 65535,
        }];
        assert_eq!(Table::leak(modes.clone()).to_vec(), modes);
        let table = Table::<u16>::leak(vec![1, 2, 0x1234]);
        assert_eq!(table.records(), &[[1, 0], [2, 0], [0x34, 0x12]]);
        assert_eq!(table.slice(1..).to_vec(), [2, 0x1234]);
        assert_eq!((table.first(), table.last()), (Some(1), Some(0x1234)));
        assert_eq!(Table::<u32>::from_bytes(&[1, 0, 0, 0, 9]).to_vec(), [1]);
    }

    #[test]
    #[should_panic(expected = "parse action of kind 0")]
    fn a_header_is_not_an_action() {
        ParseAction::read([0; 8]);
    }
}
