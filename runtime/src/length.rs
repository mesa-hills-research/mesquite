use crate::point::*;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct Length {
    pub bytes: u32,
    pub extent: Point,
}
pub(crate) const LENGTH_UNDEFINED: Length = Length {
    bytes: 0,
    extent: Point { row: 0, column: 1 },
};
pub(crate) const LENGTH_MAX: Length = Length {
    bytes: u32::MAX,
    extent: POINT_MAX,
};
pub(crate) const fn length_zero() -> Length {
    Length {
        bytes: 0,
        extent: POINT_ZERO,
    }
}
pub(crate) fn length_is_undefined(length: Length) -> bool {
    length.bytes == 0 && length.extent.column != 0
}
pub(crate) fn length_min(a: Length, b: Length) -> Length {
    if a.bytes < b.bytes { a } else { b }
}
pub(crate) fn length_add(a: Length, b: Length) -> Length {
    Length {
        bytes: a.bytes.wrapping_add(b.bytes),
        extent: point_add(a.extent, b.extent),
    }
}
pub(crate) fn length_sub(a: Length, b: Length) -> Length {
    Length {
        bytes: a.bytes.saturating_sub(b.bytes),
        extent: point_sub(a.extent, b.extent),
    }
}
pub(crate) fn length_saturating_sub(a: Length, b: Length) -> Length {
    if a.bytes > b.bytes {
        length_sub(a, b)
    } else {
        length_zero()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn extents_are_not_vectors() {
        let a = Length {
            bytes: 8,
            extent: point_new(2, 3),
        };
        let b = Length {
            bytes: 4,
            extent: point_new(1, 1),
        };
        assert_eq!(length_add(a, b).extent, point_new(3, 1));
        assert_eq!(length_sub(a, b).extent, point_new(1, 3));
        assert_eq!(length_sub(b, a), length_zero());
        assert!(length_is_undefined(LENGTH_UNDEFINED));
        assert!(!length_is_undefined(length_zero()));
    }
}
