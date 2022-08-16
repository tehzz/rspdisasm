use num_enum::{TryFromPrimitive, TryFromPrimitiveError};

pub(crate) mod special;
pub(crate) mod cop0;
pub(crate) mod vu;

use std::fmt::{self, Write};

use crate::regs::vu::VUReg;
use crate::{sym::Sym, regs::su::GpReg};
use crate::utils::*;
use self::{special::Special, cop0::Cop0Op, vu::VUOp};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum RspOpcode {
    Nop,
    Special(Special), // 0x00
    RegImm(RegImmOp), // 0x01
    J(Sym), // 0x02
    JAL(Sym), // 0x03
    BEQ(BrTwoReg), // 0x04
    BNE(BrTwoReg), // 0x05
    BLEZ(BrOneReg), // 0x06
    BGTZ(BrOneReg), // 0x07
    ADDI(TwoRegImm), // 0x08
    ADDIU(TwoRegImm), // 0x09
    SLTI(TwoRegImm), // 0x0A
    SLTIU(TwoRegImm), // 0x0B
    ANDI(TwoRegImm), // 0x0C
    ORI(TwoRegImm), // 0x0D
    XORI(TwoRegImm), // 0x0E
    LUI(OneRegImm), // 0x0F
    COP0(Cop0Op), // 0x10
    COP2(VUOp), // 0x12
    LB(MipsLoadStore), // 0x20
    LH(MipsLoadStore), // 0x21
    LW(MipsLoadStore), // 0x23
    LBU(MipsLoadStore), // 0x24
    LHU(MipsLoadStore), // 0x25
    LWU(MipsLoadStore), // 0x27
    SB(MipsLoadStore), // 0x28
    SH(MipsLoadStore), // 0x29
    SW(MipsLoadStore), // 0x2B
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
            0x00 => Special::from_op(op).map_or(
                Self::Unsupported(op),
                Self::Special
            ),
            0x01 => RegImmOp::decode(op, vaddr).map_or(
                Self::Unsupported(op),
                Self::RegImm
            ),
            0x02 => Self::J(Sym::from_jmp(op, vaddr)),
            0x03 => Self::JAL(Sym::from_jmp(op, vaddr)),
            0x04 => BrTwoReg::from_op(op, vaddr).map_or(
                Self::Unsupported(op), 
                Self::BEQ
            ),
            0x05 => BrTwoReg::from_op(op, vaddr).map_or(
                Self::Unsupported(op), 
                Self::BNE
            ),
            0x06 => BrOneReg::from_op(op, vaddr).map_or(
                Self::Unsupported(op),
                Self::BLEZ
            ),
            0x07 => BrOneReg::from_op(op, vaddr).map_or(
                Self::Unsupported(op),
                Self::BGTZ
            ),
            0x08 => TwoRegImm::from_op(op).map_or(
                Self::Unsupported(op),
                Self::ADDI
            ),
            0x09 => TwoRegImm::from_op(op).map_or(
                Self::Unsupported(op),
                Self::ADDIU
            ),
            0x0A => TwoRegImm::from_op(op).map_or(
                Self::Unsupported(op),
                Self::SLTI
            ),
            0x0B => TwoRegImm::from_op(op).map_or(
                Self::Unsupported(op),
                Self::SLTIU
            ),
            0x0C => TwoRegImm::from_op(op).map_or(
                Self::Unsupported(op),
                Self::ANDI
            ),
            0x0D => TwoRegImm::from_op(op).map_or(
                Self::Unsupported(op),
                Self::ORI
            ),
            0x0E => TwoRegImm::from_op(op).map_or(
                Self::Unsupported(op),
                Self::XORI
            ),
            0x0F => OneRegImm::from_op(op).map_or(
                Self::Unsupported(op),
                Self::LUI
            ),
            0x10 => Cop0Op::decode(op).map_or(
                Self::Unsupported(op),
                Self::COP0
            ),
            // vec cop2 operations
            0x12 => VUOp::from_op(op).map_or(
                Self::Unsupported(op),
                Self::COP2
            ),
            0x20 => MipsLoadStore::decode(op).map_or(
                Self::Unsupported(op),
                Self::LB
            ),
            0x21 => MipsLoadStore::decode(op).map_or(
                Self::Unsupported(op),
                Self::LH
            ),
            0x23 => MipsLoadStore::decode(op).map_or(
                Self::Unsupported(op),
                Self::LW
            ),
            0x24 => MipsLoadStore::decode(op).map_or(
                Self::Unsupported(op),
                Self::LBU
            ),
            0x25 => MipsLoadStore::decode(op).map_or(
                Self::Unsupported(op),
                Self::LHU
            ),
            0x27 => MipsLoadStore::decode(op).map_or(
                Self::Unsupported(op),
                Self::LWU
            ),
            0x28 => MipsLoadStore::decode(op).map_or(
                Self::Unsupported(op),
                Self::SB
            ),
            0x29 => MipsLoadStore::decode(op).map_or(
                Self::Unsupported(op),
                Self::SH
            ),
            0x2B => MipsLoadStore::decode(op).map_or(
                Self::Unsupported(op),
                Self::SW
            ),
            0x32 => Cop2LoadStore::decode(op).map_or(
                Self::Unsupported(op),
                Self::LWC2
            ),
            0x3A => Cop2LoadStore::decode(op).map_or(
                Self::Unsupported(op),
                Self::SWC2
            ),
            _ => Self::Unsupported(op)
        };

        return decoded
    }

    pub(crate) fn print(&self, w: &mut impl fmt::Write) -> fmt::Result {
        match self {
            Self::Nop => write!(w, "nop"),
            Self::Special(sub) => sub.print(w),
            Self::RegImm(sub) => write!(w, "regimm todo {:?}", sub),
            Self::J(s) => {
                // `j {symbol}`
                write!(w, "j ")?;
                s.print_glabel(w)
            }
            Self::JAL(s) => {
                // `jal {symbol}`
                write!(w, "jal ")?;
                s.print_glabel(w)
            }
            Self::BEQ(regs) => {
                // `beq {rs, rt, local}
                write!(w, "beq ")?;
                regs.print(w)
            }
            Self::BNE(regs) => {
                // `bne {rs, rt, local}`
                write!(w, "bne ")?;
                regs.print(w)
            },
            Self::BLEZ(d) => {
                // 'blez {rs, offset}
                write!(w, "blez ")?;
                d.print(w)
            },
            Self::BGTZ(d) => {
                // 'bgtz {rs, offset}
                write!(w, "bgtz ")?;
                d.print(w)
            },
            Self::ADDI(d) => {
                // addi {rt, rs, imm}
                write!(w, "addi ")?;
                d.print(w, false)
            },
            Self::ADDIU(d) => {
                // addiu {rt, rs, imm}
                write!(w, "addiu ")?;
                d.print(w, false)
            },
            Self::SLTI(d) => {
                // slti {rt, rs, imm}
                write!(w, "slti ")?;
                d.print(w, false)
            },
            Self::SLTIU(d) => {
                // sltiu {rt, rs, imm}
                write!(w, "sltiu ")?;
                d.print(w, false)
            },
            Self::ANDI(d) => {
                // andi {rt, rs, imm}
                write!(w, "andi ")?;
                d.print(w, true)
            },
            Self::ORI(d) => {
                // ori {rt, rs, imm}
                write!(w, "ori ")?;
                d.print(w, true)
            },
            Self::XORI(d) => {
                // xori {rt, rs, imm}
                write!(w, "xori ")?;
                d.print(w, true)
            },
            Self::LUI(d) => {
                // lui {rt, imm}
                write!(w, "xori ")?;
                d.print(w)
            },
            Self::COP0(sub) => sub.print(w),
            Self::COP2(sub) => write!(w, "cop2 todo {:?}", sub),
            Self::LB(d) => {
                // lb {rt, offset(base)
                write!(w, "lb ")?;
                d.print(w)
            },
            Self::LH(d) =>{
                // lh {rt, offset(base)
                write!(w, "ld ")?;
                d.print(w)
            },
            Self::LW(d) =>{
                // lw {rt, offset(base)
                write!(w, "lw ")?;
                d.print(w)
            },
            Self::LBU(d) =>{
                // lbu {rt, offset(base)
                write!(w, "lbu ")?;
                d.print(w)
            },
            Self::LHU(d) =>{
                // lhu {rt, offset(base)
                write!(w, "lhu ")?;
                d.print(w)
            },
            Self::LWU(d) =>{
                // lwu {rt, offset(base)
                write!(w, "lwu ")?;
                d.print(w)
            },
            Self::SB(d) =>{
                // sb {rt, offset(base)
                write!(w, "sb ")?;
                d.print(w)
            },
            Self::SH(d) =>{
                // sh {rt, offset(base)
                write!(w, "sh ")?;
                d.print(w)
            },
            Self::SW(d) =>{
                // sw {rt, offset(base)
                write!(w, "sw ")?;
                d.print(w)
            },
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
    target: Sym
}

impl BrTwoReg {
    fn from_op(op: u32, vaddr: u32) -> Option<Self> {
        let rs = GpReg::at_bit(21, op).ok()?;
        let rt = GpReg::at_bit(16, op).ok()?;
        let target = Sym::from_branch(op, vaddr);

        Some(Self{rs, rt, target})
    }

    fn print(&self, w: &mut impl Write) -> fmt::Result {
        self.rs.print(w)?;
        write!(w, ", ")?;
        self.rt.print(w)?;
        write!(w, ", ")?;
        self.target.print_local(w)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct BrOneReg {
    rs: GpReg,
    target: Sym
}

impl BrOneReg {
    fn from_op(op: u32, vaddr: u32) -> Option<Self> {
        let rs = GpReg::at_bit(21, op).ok()?;
        let target = Sym::from_branch(op, vaddr);

        Some(Self{rs, target})
    }

    fn print(&self, w: &mut impl Write) -> fmt::Result {
        self.rs.print(w)?;
        write!(w, ", ")?;
        self.target.print_local(w)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct TwoRegImm {
    rs: GpReg,
    rt: GpReg,
    imm: i16,
}

impl TwoRegImm {
    fn from_op(op: u32) -> Option<Self> {
        let rs = GpReg::at_bit(21, op).ok()?;
        let rt = GpReg::at_bit(16, op).ok()?;
        let imm = op as i16;

        Some(Self{rs, rt, imm})
    }

    fn print(&self, w: &mut impl Write, hex: bool) -> fmt::Result {
        self.rt.print(w)?;
        write!(w, ", ")?;
        self.rs.print(w)?;
        if hex {
            write!(w, ", {:#x}", self.imm)
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

        Some(Self{rt, imm})
    }

    fn print(&self, w: &mut impl Write) -> fmt::Result {
        self.rt.print(w)?;
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

        Some(Self {dst, base, offset})
    }

    fn print(&self, w: &mut impl Write) -> fmt::Result {
        self.dst.print(w)?;
        write!(w, ", {:#x}(", self.offset)?;
        self.base.print(w)?;
        write!(w, ")")
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Cop2LoadStore {
    opcode: RspLsOpcode,
    vt: VUReg,
    element: u8,
    base: GpReg,
    offset: i16,
}

impl Cop2LoadStore {
    fn decode(op: u32) -> Option<Self> {
        let opcode = RspLsOpcode::at_bit(11, op).ok()?;
        let vt = VUReg::at_bit(16, op);
        let element = u8_at(7, 4, op);
        let base = GpReg::at_bit(21, op).ok()?;
        // offset is shifted by the size of the item to load when encoded
        let offset = (i8_at(0, 7, op) as i16) * opcode.item_size() as i16;

        Some(Self { opcode, vt, element, base, offset })
    }
}



#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct RsSym {
    rs: GpReg,
    sym: Sym,
}

impl RsSym {
    fn decode(op: u32, vaddr: u32) -> Option<Self> {
        let rs = GpReg::at_bit(21, op).ok()?;
        let sym = Sym::from_branch(op, vaddr);

        Some(Self{rs, sym})
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum RegImmOp {
    BLTZ(RsSym),
    BGEZ(RsSym),
    BLTZAL(RsSym),
    BGEZAL(RsSym),
}

impl RegImmOp {
    fn decode(op: u32, vaddr: u32) -> Option<Self> {
        let subop = u8_at(16, 5, op);
        
        RsSym::decode(op, vaddr)
            .and_then(|info| {
                match subop {
                    0x00 => Some(Self::BLTZ(info)),
                    0x01 => Some(Self::BGEZ(info)),
                    0x10 => Some(Self::BLTZAL(info)),
                    0x11 => Some(Self::BGEZAL(info)),
                    _ => None,
                }
            })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
enum RspLsOpcode {
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

impl RspLsOpcode {
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
