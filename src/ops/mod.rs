use num_enum::{TryFromPrimitive, TryFromPrimitiveError};

pub(crate) mod cop0;
pub(crate) mod regimm;
pub(crate) mod special;
pub(crate) mod vu;

use std::fmt::{self, Write};

use self::{cop0::Cop0Op, regimm::RegImm, special::Special, vu::VUOp};
use crate::{
    print::Print,
    regs::{su::GpReg, vu::VUReg},
    sym::Sym,
    utils::*,
    PrintOpts,
};

// todo: refactor into enum struct
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum RspOpcode {
    Nop,
    Special(Special),    // 0x00
    RegImm(RegImm),      // 0x01
    J(Sym),              // 0x02
    JAL(Sym),            // 0x03
    BEQ(BrTwoReg),       // 0x04
    BNE(BrTwoReg),       // 0x05
    BLEZ(BrOneReg),      // 0x06
    BGTZ(BrOneReg),      // 0x07
    ADDI(TwoRegImm),     // 0x08
    ADDIU(TwoRegImm),    // 0x09
    SLTI(TwoRegImm),     // 0x0A
    SLTIU(TwoRegImm),    // 0x0B
    ANDI(TwoRegImm),     // 0x0C
    ORI(TwoRegImm),      // 0x0D
    XORI(TwoRegImm),     // 0x0E
    LUI(OneRegImm),      // 0x0F
    COP0(Cop0Op),        // 0x10
    COP2(VUOp),          // 0x12
    LB(MipsLoadStore),   // 0x20
    LH(MipsLoadStore),   // 0x21
    LW(MipsLoadStore),   // 0x23
    LBU(MipsLoadStore),  // 0x24
    LHU(MipsLoadStore),  // 0x25
    LWU(MipsLoadStore),  // 0x27
    SB(MipsLoadStore),   // 0x28
    SH(MipsLoadStore),   // 0x29
    SW(MipsLoadStore),   // 0x2B
    LWC2(Cop2LoadStore), // 0x32
    SWC2(Cop2LoadStore), // 0x3A
    Unsupported(u32),
}

impl RspOpcode {
    pub(crate) fn decode(op: u32, vaddr: u32) -> Self {
        if op == 0x00000000 {
            return Self::Nop;
        }

        let opcode = ((op >> 26) & 0x3F) as u8;

        let decoded = match opcode {
            0x00 => Special::from_op(op).map(Self::Special),
            0x01 => RegImm::decode(op, vaddr).map(Self::RegImm),
            0x02 => Some(Self::J(Sym::from_jmp(op, vaddr))),
            0x03 => Some(Self::JAL(Sym::from_jmp(op, vaddr))),
            0x04 => BrTwoReg::from_op(op, vaddr).map(Self::BEQ),
            0x05 => BrTwoReg::from_op(op, vaddr).map(Self::BNE),
            0x06 => BrOneReg::from_op(op, vaddr).map(Self::BLEZ),
            0x07 => BrOneReg::from_op(op, vaddr).map(Self::BGTZ),
            0x08 => TwoRegImm::from_op(op).map(Self::ADDI),
            0x09 => TwoRegImm::from_op(op).map(Self::ADDIU),
            0x0A => TwoRegImm::from_op(op).map(Self::SLTI),
            0x0B => TwoRegImm::from_op(op).map(Self::SLTIU),
            0x0C => TwoRegImm::from_op(op).map(|mut r| {
                r.as_hex = true;
                Self::ANDI(r)
            }),
            0x0D => TwoRegImm::from_op(op).map(|mut r| {
                r.as_hex = true;
                Self::ORI(r)
            }),
            0x0E => TwoRegImm::from_op(op).map(|mut r| {
                r.as_hex = true;
                Self::XORI(r)
            }),
            0x0F => OneRegImm::from_op(op).map(Self::LUI),
            0x10 => Cop0Op::decode(op).map(Self::COP0),
            // vec cop2 operations
            0x12 => VUOp::from_op(op).map(Self::COP2),
            0x20 => MipsLoadStore::decode(op).map(Self::LB),
            0x21 => MipsLoadStore::decode(op).map(Self::LH),
            0x23 => MipsLoadStore::decode(op).map(Self::LW),
            0x24 => MipsLoadStore::decode(op).map(Self::LBU),
            0x25 => MipsLoadStore::decode(op).map(Self::LHU),
            0x27 => MipsLoadStore::decode(op).map(Self::LWU),
            0x28 => MipsLoadStore::decode(op).map(Self::SB),
            0x29 => MipsLoadStore::decode(op).map(Self::SH),
            0x2B => MipsLoadStore::decode(op).map(Self::SW),
            0x32 => Cop2LoadStore::decode(op).map(Self::LWC2),
            0x3A => Cop2LoadStore::decode(op).map(Self::SWC2),
            _ => Some(Self::Unsupported(op)),
        };

        return decoded.unwrap_or_else(|| Self::Unsupported(op));
    }

    pub fn get_symbol(&self) -> Option<Sym> {
        match self {
            Self::J(s) | Self::JAL(s) => Some(*s),
            Self::BNE(t) | Self::BEQ(t) => Some(t.target),
            Self::BLEZ(t) | Self::BGTZ(t) => Some(t.target),
            Self::RegImm(imm) => Some(imm.get_regs().sym),
            _ => None,
        }
    }
}

impl Print for RspOpcode {
    fn print(&self, opts: PrintOpts, w: &mut impl fmt::Write) -> fmt::Result {
        match self {
            Self::Nop => write!(w, "nop"),
            Self::Special(sub) => sub.print(opts, w),
            Self::RegImm(sub) => sub.print(opts, w),
            Self::J(s) => {
                // `j {symbol}`
                write!(w, "j ")?;
                s.print(opts, w)
            }
            Self::JAL(s) => {
                // `jal {symbol}`
                write!(w, "jal ")?;
                s.print(opts, w)
            }
            Self::BEQ(regs) => {
                // `beq {rs, rt, local}
                write!(w, "beq ")?;
                regs.print(opts, w)
            }
            Self::BNE(regs) => {
                // `bne {rs, rt, local}`
                write!(w, "bne ")?;
                regs.print(opts, w)
            }
            Self::BLEZ(d) => {
                // 'blez {rs, offset}
                write!(w, "blez ")?;
                d.print(opts, w)
            }
            Self::BGTZ(d) => {
                // 'bgtz {rs, offset}
                write!(w, "bgtz ")?;
                d.print(opts, w)
            }
            Self::ADDI(d) => {
                // addi {rt, rs, imm}
                write!(w, "addi ")?;
                d.print(opts, w)
            }
            Self::ADDIU(d) => {
                // addiu {rt, rs, imm}
                write!(w, "addiu ")?;
                d.print(opts, w)
            }
            Self::SLTI(d) => {
                // slti {rt, rs, imm}
                write!(w, "slti ")?;
                d.print(opts, w)
            }
            Self::SLTIU(d) => {
                // sltiu {rt, rs, imm}
                write!(w, "sltiu ")?;
                d.print(opts, w)
            }
            Self::ANDI(d) => {
                // andi {rt, rs, imm}
                write!(w, "andi ")?;
                d.print(opts, w)
            }
            Self::ORI(d) => {
                // ori {rt, rs, imm}
                write!(w, "ori ")?;
                d.print(opts, w)
            }
            Self::XORI(d) => {
                // xori {rt, rs, imm}
                write!(w, "xori ")?;
                d.print(opts, w)
            }
            Self::LUI(d) => {
                // lui {rt, imm}
                write!(w, "xori ")?;
                d.print(opts, w)
            }
            Self::COP0(sub) => sub.print(opts, w),
            Self::COP2(sub) => write!(w, "cop2 todo {:?}", sub),
            Self::LB(d) => {
                // lb {rt, offset(base)
                write!(w, "lb ")?;
                d.print(opts, w)
            }
            Self::LH(d) => {
                // lh {rt, offset(base)
                write!(w, "ld ")?;
                d.print(opts, w)
            }
            Self::LW(d) => {
                // lw {rt, offset(base)
                write!(w, "lw ")?;
                d.print(opts, w)
            }
            Self::LBU(d) => {
                // lbu {rt, offset(base)
                write!(w, "lbu ")?;
                d.print(opts, w)
            }
            Self::LHU(d) => {
                // lhu {rt, offset(base)
                write!(w, "lhu ")?;
                d.print(opts, w)
            }
            Self::LWU(d) => {
                // lwu {rt, offset(base)
                write!(w, "lwu ")?;
                d.print(opts, w)
            }
            Self::SB(d) => {
                // sb {rt, offset(base)
                write!(w, "sb ")?;
                d.print(opts, w)
            }
            Self::SH(d) => {
                // sh {rt, offset(base)
                write!(w, "sh ")?;
                d.print(opts, w)
            }
            Self::SW(d) => {
                // sw {rt, offset(base)
                write!(w, "sw ")?;
                d.print(opts, w)
            }
            Self::LWC2(_) => write!(w, "todo lwc2"),
            Self::SWC2(_) => write!(w, "todo swc2"),
            Self::Unsupported(b) => write!(w, "; unrecognized op [{:08X}]", b),
        }
    }
}

// todo: error propagation with error sum type
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct BrTwoReg {
    rs: GpReg,
    rt: GpReg,
    target: Sym,
}

impl BrTwoReg {
    fn from_op(op: u32, vaddr: u32) -> Option<Self> {
        let rs = GpReg::at_bit(21, op).ok()?;
        let rt = GpReg::at_bit(16, op).ok()?;
        let target = Sym::from_branch(op, vaddr);

        Some(Self { rs, rt, target })
    }
}

impl Print for BrTwoReg {
    fn print(&self, opts: PrintOpts, w: &mut impl Write) -> fmt::Result {
        self.rs.print(opts, w)?;
        write!(w, ", ")?;
        self.rt.print(opts, w)?;
        write!(w, ", ")?;
        self.target.print(opts, w)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct BrOneReg {
    rs: GpReg,
    target: Sym,
}

impl BrOneReg {
    fn from_op(op: u32, vaddr: u32) -> Option<Self> {
        let rs = GpReg::at_bit(21, op).ok()?;
        let target = Sym::from_branch(op, vaddr);

        Some(Self { rs, target })
    }
}

impl Print for BrOneReg {
    fn print(&self, opts: PrintOpts, w: &mut impl Write) -> fmt::Result {
        self.rs.print(opts, w)?;
        write!(w, ", ")?;
        self.target.print(opts, w)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct TwoRegImm {
    rs: GpReg,
    rt: GpReg,
    imm: i16,
    as_hex: bool,
}

impl TwoRegImm {
    fn from_op(op: u32) -> Option<Self> {
        let rs = GpReg::at_bit(21, op).ok()?;
        let rt = GpReg::at_bit(16, op).ok()?;
        let imm = op as i16;

        Some(Self {
            rs,
            rt,
            imm,
            as_hex: false,
        })
    }
}

impl Print for TwoRegImm {
    fn print(&self, opts: PrintOpts, w: &mut impl Write) -> fmt::Result {
        self.rt.print(opts, w)?;
        write!(w, ", ")?;
        self.rs.print(opts, w)?;
        if self.as_hex {
            write!(w, ", {:#X}", self.imm)
        } else {
            write!(w, ", {}", self.imm)
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct OneRegImm {
    rt: GpReg,
    imm: u16,
}

impl OneRegImm {
    fn from_op(op: u32) -> Option<Self> {
        let rt = GpReg::at_bit(16, op).ok()?;
        let imm = op as u16;

        Some(Self { rt, imm })
    }
}

impl Print for OneRegImm {
    fn print(&self, opts: PrintOpts, w: &mut impl Write) -> fmt::Result {
        self.rt.print(opts, w)?;
        write!(w, ", {:#x}", self.imm)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MipsLoadStore {
    dst: GpReg,
    base: GpReg,
    offset: i16,
}

impl MipsLoadStore {
    fn decode(op: u32) -> Option<Self> {
        let dst = GpReg::at_bit(16, op).ok()?;
        let base = GpReg::at_bit(21, op).ok()?;
        let offset = (op & 0xFFFF) as i16;

        Some(Self { dst, base, offset })
    }
}

impl Print for MipsLoadStore {
    fn print(&self, opts: PrintOpts, w: &mut impl Write) -> fmt::Result {
        self.dst.print(opts, w)?;
        write!(w, ", {:#x}(", self.offset)?;
        self.base.print(opts, w)?;
        write!(w, ")")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Cop2LoadStore {
    opcode: RspAddressMode,
    vt: VUReg,
    element: u8,
    base: GpReg,
    offset: i16,
}

impl Cop2LoadStore {
    fn decode(op: u32) -> Option<Self> {
        let opcode = RspAddressMode::at_bit(11, op).ok()?;
        let vt = VUReg::at_bit(16, op);
        let element = u8_at(7, 4, op);
        let base = GpReg::at_bit(21, op).ok()?;
        // offset is shifted by the size of the item to load when encoded
        let offset = (i8_at(0, 7, op) as i16) * opcode.item_size() as i16;

        Some(Self {
            opcode,
            vt,
            element,
            base,
            offset,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
enum RspAddressMode {
    Byte = 0,
    Short = 1,
    Word = 2,
    Double = 3,
    Quad = 4,
    Rest = 5,
    Pack = 6,
    UPack = 7,
    HalfPack = 8,
    FourthPack = 9,
    Wrap = 10,
    Transpose = 11,
}

impl RspAddressMode {
    fn at_bit(b: u8, op: u32) -> Result<Self, TryFromPrimitiveError<Self>> {
        Self::try_from(((op >> b) & 0x1F) as u8)
    }
    fn item_size(&self) -> u8 {
        match self {
            Self::Byte => 1,
            Self::Short => 2,
            Self::Word => 4,
            Self::Double => 8,
            Self::Quad => 16,
            Self::Rest => 16,
            Self::Pack => 8,
            Self::UPack => 8,
            Self::HalfPack => 16,
            Self::FourthPack => 16,
            Self::Wrap => 16,
            Self::Transpose => 16,
        }
    }
}
