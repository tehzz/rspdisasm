use std::fmt;

/// extract a u8 of `size` bits from bit `b` from `src`
pub(crate) fn u8_at(b: u8, size: u8, src: u32) -> u8 {
    ((src >> b) & ((1 << size) - 1)) as u8
}

/// sign extend bits to an i8
pub(crate) fn i8_at(b: u8, size: u8, src: u32) -> i8 {
    let base = u8_at(b, size, src);
    ((base << (8 - size)) as i8) >> (8 - size)
}

/// convience wrapper for pretty printing load/store offsets
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Offset(pub(crate) i16);

impl fmt::Display for Offset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let v = self.0;
        if v < 0x10 && v > -0x10 {
            write!(f, "{}", v)
        } else if v.is_negative() {
            write!(f, "-{:#x}", -v)
        } else {
            write!(f, "{:#x}", v)
        }
    }
}
