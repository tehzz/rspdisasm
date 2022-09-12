use std::fmt;

use crate::{print::Print, utils, PrintOpts};
use num_enum::{TryFromPrimitive, TryFromPrimitiveError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct VUReg(u8);

impl VUReg {
    pub(crate) fn at_bit(b: u8, op: u32) -> Self {
        Self(utils::u8_at(b, 5, op))
    }
}

impl fmt::Display for VUReg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "$v{}", self.0)
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

    const fn as_mnemonic(&self) -> &'static str {
        match self {
            Self::Vco => "vco",
            Self::Vcc => "vcc",
            Self::Vce => "vce",
        }
    }
}

impl fmt::Display for VUCtrlReg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "${}", self.as_mnemonic())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Element {
    Vector,
    Quarter(u8),
    Half(u8),
    Whole(u8),
}

impl Element {
    pub(crate) fn at_bit(b: u8, size: u8, op: u32) -> Option<Self> {
        Self::from_u8(utils::u8_at(b, size, op))
    }

    fn from_u8(val: u8) -> Option<Self> {
        if val & 0b11110000 != 0 {
            None
        } else if val == 0b0000 {
            Some(Self::Vector)
        } else if val & 0b1110 == 0b0010 {
            Some(Self::Quarter(val & 1))
        } else if val & 0b1100 == 0b0100 {
            Some(Self::Half(val & 0b0011))
        } else if val & 0b1000 == 0b1000 {
            Some(Self::Whole(val & 0b0111))
        } else {
            None
        }
    }
}

impl Print for Element {
    fn print(&self, _opts: PrintOpts, w: &mut impl fmt::Write) -> fmt::Result {
        match self {
            Self::Vector => write!(w, ""),
            Self::Quarter(x) => write!(w, "[{}q]", x),
            Self::Half(x) => write!(w, "[{}h]", x),
            Self::Whole(x) => write!(w, "[{}]", x),
        }
    }
}
