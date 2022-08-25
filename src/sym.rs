use crate::print::{Print, PrintOpts};
use std::fmt::{self, Write};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Sym {
    Global(u32),
    Static(u32),
}

impl Sym {
    pub(crate) fn from_jmp(op: u32, vaddr: u32) -> Self {
        Self::Global(((op & 0x03FFFFFF) << 2) | (vaddr & 0xF0000000))
    }
    pub(crate) fn from_branch(op: u32, vaddr: u32) -> Self {
        let imm = (op & 0xFFFF) as i16;
        let target = (vaddr + 4) as i32 + ((imm as i32) * 4);
        Self::Static(target as u32)
    }
}

impl Print for Sym {
    fn print(&self, _opts: PrintOpts, w: &mut impl Write) -> fmt::Result {
        match self {
            Self::Global(addr) => write!(w, "subr_{:08X}", addr),
            Self::Static(addr) => write!(w, "@L{:08X}", addr),
        }
    }
}
