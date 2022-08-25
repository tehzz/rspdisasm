use std::fmt;

use crate::{print::Print, regs::su::GpReg, sym::Sym, utils, PrintOpts};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum RegImm {
    BLTZ(RsSym),
    BGEZ(RsSym),
    BLTZAL(RsSym),
    BGEZAL(RsSym),
}

impl RegImm {
    pub(crate) fn decode(op: u32, vaddr: u32) -> Option<Self> {
        let subop = utils::u8_at(16, 5, op);

        RsSym::decode(op, vaddr).and_then(|info| match subop {
            0x00 => Some(Self::BLTZ(info)),
            0x01 => Some(Self::BGEZ(info)),
            0x10 => Some(Self::BLTZAL(info)),
            0x11 => Some(Self::BGEZAL(info)),
            _ => None,
        })
    }

    fn get_regs(&self) -> RsSym {
        match self {
            Self::BLTZ(r) => *r,
            Self::BGEZ(r) => *r,
            Self::BLTZAL(r) => *r,
            Self::BGEZAL(r) => *r,
        }
    }
}

impl Print for RegImm {
    fn print(&self, opts: PrintOpts, w: &mut impl fmt::Write) -> fmt::Result {
        match self {
            Self::BLTZ(_) => write!(w, "bltz "),
            Self::BGEZ(_) => write!(w, "bgez "),
            Self::BLTZAL(_) => write!(w, "bltzal "),
            Self::BGEZAL(_) => write!(w, "bgezal "),
        }?;

        self.get_regs().print(opts, w)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct RsSym {
    rs: GpReg,
    sym: Sym,
}

impl RsSym {
    fn decode(op: u32, vaddr: u32) -> Option<Self> {
        let rs = GpReg::at_bit(21, op).ok()?;
        let sym = Sym::from_branch(op, vaddr);

        Some(Self { rs, sym })
    }
}

impl Print for RsSym {
    fn print(&self, opts: PrintOpts, w: &mut impl fmt::Write) -> fmt::Result {
        self.rs.print(opts, w)?;
        write!(w, ", ")?;
        self.sym.print(opts, w)
    }
}
