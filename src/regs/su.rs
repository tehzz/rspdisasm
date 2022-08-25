use crate::{print::Print, utils};
use num_enum::{TryFromPrimitive, TryFromPrimitiveError};
use std::fmt::{self, Write};

#[derive(Debug, Copy, Clone, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub(crate) enum GpReg {
    R0 = 0,
    AT,
    V0,
    V1,
    A0,
    A1,
    A2,
    A3,
    T0,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
    S0,
    S1,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    T8,
    T9,
    K0,
    K1,
    GP,
    SP,
    S8,
    RA,
}

impl GpReg {
    // read five bits start at `b`
    pub(crate) fn at_bit(b: u8, op: u32) -> Result<Self, TryFromPrimitiveError<Self>> {
        Self::try_from(utils::u8_at(b, 5, op))
    }

    pub(crate) fn as_armips_id(&self) -> &'static str {
        match self {
            GpReg::R0 => "$0",
            GpReg::AT => "$1",
            GpReg::V0 => "$2",
            GpReg::V1 => "$3",
            GpReg::A0 => "$4",
            GpReg::A1 => "$5",
            GpReg::A2 => "$6",
            GpReg::A3 => "$7",
            GpReg::T0 => "$8",
            GpReg::T1 => "$9",
            GpReg::T2 => "$10",
            GpReg::T3 => "$11",
            GpReg::T4 => "$12",
            GpReg::T5 => "$13",
            GpReg::T6 => "$14",
            GpReg::T7 => "$15",
            GpReg::S0 => "$16",
            GpReg::S1 => "$17",
            GpReg::S2 => "$18",
            GpReg::S3 => "$19",
            GpReg::S4 => "$20",
            GpReg::S5 => "$21",
            GpReg::S6 => "$22",
            GpReg::S7 => "$23",
            GpReg::T8 => "$24",
            GpReg::T9 => "$25",
            GpReg::K0 => "$26",
            GpReg::K1 => "$27",
            GpReg::GP => "$28",
            GpReg::SP => "$29",
            GpReg::S8 => "$30",
            GpReg::RA => "$31",
        }
    }

    pub(crate) fn as_mnemonic(&self) -> &'static str {
        match self {
            GpReg::R0 => "r0",
            GpReg::AT => "at",
            GpReg::V0 => "v0",
            GpReg::V1 => "v1",
            GpReg::A0 => "a0",
            GpReg::A1 => "a1",
            GpReg::A2 => "a2",
            GpReg::A3 => "a3",
            GpReg::T0 => "t0",
            GpReg::T1 => "t1",
            GpReg::T2 => "t2",
            GpReg::T3 => "t3",
            GpReg::T4 => "t4",
            GpReg::T5 => "t5",
            GpReg::T6 => "t6",
            GpReg::T7 => "t7",
            GpReg::S0 => "s0",
            GpReg::S1 => "s1",
            GpReg::S2 => "s2",
            GpReg::S3 => "s3",
            GpReg::S4 => "s4",
            GpReg::S5 => "s5",
            GpReg::S6 => "s6",
            GpReg::S7 => "s7",
            GpReg::T8 => "t8",
            GpReg::T9 => "t9",
            GpReg::K0 => "k0",
            GpReg::K1 => "k1",
            GpReg::GP => "gp",
            GpReg::SP => "sp",
            GpReg::S8 => "s8",
            GpReg::RA => "ra",
        }
    }
}

impl Print for GpReg {
    fn print(&self, opts: crate::PrintOpts, w: &mut impl Write) -> fmt::Result {
        let r = if opts.reg_names {
            self.as_mnemonic()
        } else {
            self.as_armips_id()
        };

        write!(w, "{}", r)
    }
}
