use crate::regs::su::GpReg;
use crate::utils;
use std::fmt::{self, Write};
use num_enum::{TryFromPrimitive};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Special {
    opcode: SpecialOpCode,
    data: SpecialData,
}

impl Special {
    pub(crate) fn from_op(op: u32) -> Option<Self> {
        use SpecialOpCode::*;

        let opcode = SpecialOpCode::try_from((op & 0x3F) as u8).ok()?;
        let data = match opcode {
            SLL | SRL | SRA => ShiftImm::from_op(op).map(SpecialData::ShiftImm),

            JR => GpReg::at_bit(11, op).ok().map(SpecialData::Jr),

            JALR => JalrReg::from_op(op).map(SpecialData::JalrReg),

            BREAK => Some(SpecialData::Break((op >> 6) & 0xFFFFF)),

            SLLV |
            SRLV |
            SRAV |
            ADD |
            ADDU |
            SUB |
            SUBU |
            AND |
            OR |
            XOR |
            NOR |
            SLT |
            SLTU => ThreeReg::from_op(op).map(SpecialData::ThreeReg),
        }?;

        Some(Self { opcode, data })
    }
    
    pub(crate) fn print(&self, w: &mut impl Write) -> fmt::Result {
        write!(w, "{} ", self.opcode)?;
        self.data.print(w)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
enum SpecialOpCode {
    SLL   = 0x00,
    SRL   = 0x02,
    SRA   = 0x03,
    SLLV  = 0x04,
    SRLV  = 0x06,
    SRAV  = 0x07,
    JR    = 0x08,
    JALR  = 0x09,
    BREAK = 0x0D,
    ADD   = 0x20,
    ADDU  = 0x21,
    SUB   = 0x22,
    SUBU  = 0x23,
    AND   = 0x24,
    OR    = 0x25,
    XOR   = 0x26,
    NOR   = 0x27,
    SLT   = 0x2A,
    SLTU  = 0x2B,
}

impl fmt::Display for SpecialOpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", s.to_ascii_lowercase())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SpecialData {
    ShiftImm(ShiftImm),
    ThreeReg(ThreeReg),
    JalrReg(JalrReg),
    Jr(GpReg),
    Break(u32),
}

impl SpecialData {
    fn print(&self, w: &mut impl Write) -> fmt::Result {
        match self {
            SpecialData::ShiftImm(d) => d.print(w),
            SpecialData::ThreeReg(d) => d.print(w),
            SpecialData::JalrReg(d) => d.print(w),
            SpecialData::Jr(reg) => reg.print(w),
            SpecialData::Break(code) => write!(w, "{code}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ShiftImm {
    dst: GpReg,
    src: GpReg,
    by: u8,
}

impl ShiftImm {
    fn from_op(op: u32) -> Option<Self> {
        let dst = GpReg::at_bit(11, op).ok()?;
        let src = GpReg::at_bit(16, op).ok()?;
        let by = utils::u8_at(6, 5, op);

        Some(Self{dst, src, by})
    }

    fn print(&self, w: &mut impl Write) -> fmt::Result {
        self.dst.print(w)?;
        write!(w, ", ")?;
        self.src.print(w)?;
        write!(w, ", {}", self.by)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ThreeReg {
    rd: GpReg,
    rs: GpReg,
    rt: GpReg,
}

impl ThreeReg {
    fn from_op(op: u32) -> Option<Self> {
        let rd = GpReg::at_bit(11, op).ok()?;
        let rt = GpReg::at_bit(16, op).ok()?;
        let rs = GpReg::at_bit(21, op).ok()?;
        Some(Self{rd, rs, rt})
    }

    fn print(&self, w: &mut impl Write) -> fmt::Result {
        self.rd.print(w)?;
        write!(w, ", ")?;
        self.rs.print(w)?;
        write!(w, ", ")?;
        self.rt.print(w)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct JalrReg {
    rd: GpReg,
    rs: GpReg
}

impl JalrReg {
    fn from_op(op: u32) -> Option<Self> {
        let rd = GpReg::at_bit(11, op).ok()?;
        let rs = GpReg::at_bit(21, op).ok()?;

        Some(Self{rd, rs})
    }

    fn print(&self, w: &mut impl Write) -> fmt::Result {
        if self.rd == GpReg::RA {
            self.rs.print(w)
        } else {
            self.rd.print(w)?;
            write!(w, ", ")?;
            self.rs.print(w)
        }
    }
}
