use std::fmt::{self, Write};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct Sym(u32);

impl Sym {
    pub(crate) fn from_jmp(op: u32, vaddr: u32) -> Self {
        Self(((op & 0x03FFFFFF) << 2) | (vaddr & 0xF0000000))
    }
    pub(crate) fn from_branch(op: u32, vaddr: u32) -> Self {
        let imm = (op & 0xFFFF) as i16;
        let target = (vaddr + 4) as i32 + ((imm as i32) * 4);
        Self(target as u32)
    }

    pub(crate) fn value(&self) -> u32 {
        self.0
    }

    pub(crate) fn print_glabel(&self, w: &mut impl Write) -> fmt::Result {
        write!(w, "subr_{:08X}", self.0)
    }
    
    pub(crate) fn print_local(&self, w: &mut impl Write) -> fmt::Result {
        write!(w, "L{:08X}", self.0)
    }
}
