use crate::regs::{cop0::Cop0Reg, su::GpReg};
use crate::utils;
use std::fmt::{self, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Cop0Op {
    MFC0(GpReg, Cop0Reg),
    MTC0(GpReg, Cop0Reg),
}

impl Cop0Op {
    pub(crate) fn decode(op: u32) -> Option<Self> {
        let direction = utils::u8_at(21, 5, op);
        let rt = GpReg::at_bit(16, op).ok()?;
        let rd = Cop0Reg::at_bit(11, op).ok()?;
        match direction {
            0x00 => Some(Self::MFC0(rt, rd)),
            0x04 => Some(Self::MTC0(rt, rd)),
            _ => None,
        }
    }

    pub(crate) fn print(&self, w: &mut impl Write) -> fmt::Result {
        let (rt, rd) = match self {
            Self::MFC0(rt,rd) => {
                write!(w, "mfc0 ")?;
                (rt, rd)
            }
            Self::MTC0(rt, rd) => {
                write!(w, "mtc0 ")?;
                (rt, rd)
            }
        };

        rt.print(w)?;
        write!(w, ", ")?;
        rd.print(w)
    }
}
