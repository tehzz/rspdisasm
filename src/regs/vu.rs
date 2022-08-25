use crate::utils;
use num_enum::{TryFromPrimitive, TryFromPrimitiveError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct VUReg(u8);

impl VUReg {
    pub(crate) fn at_bit(b: u8, op: u32) -> Self {
        Self(utils::u8_at(b, 5, op))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub(crate) enum VUCtrlReg {
    Vco = 0,
    Vcc = 1,
    Vce = 2,
}

impl VUCtrlReg {
    pub(crate) fn at_bit(b: u8, op: u32) -> Result<Self, TryFromPrimitiveError<Self>> {
        Self::try_from(utils::u8_at(b, 5, op))
    }
}
