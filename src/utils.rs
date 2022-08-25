/// extract a u8 of `size` bits from bit `b` from `src`
pub(crate) fn u8_at(b: u8, size: u8, src: u32) -> u8 {
    ((src >> b) & ((1 << size) - 1)) as u8
}

/// sign extend bits to an i8
pub(crate) fn i8_at(b: u8, size: u8, src: u32) -> i8 {
    let base = u8_at(b, size, src);
    ((base << (8 - size)) as i8) >> (8 - size)
}
