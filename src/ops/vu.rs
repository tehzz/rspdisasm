use std::fmt;

use crate::{
    print::Print,
    regs::{
        su::GpReg,
        vu::{Element, VUCtrlReg, VUReg},
    },
    utils, PrintOpts,
};
use num_enum::TryFromPrimitive;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VUOp {
    MFC2(MoveVU),
    CFC2(CtrlVU),
    MTC2(MoveVU),
    CTC2(CtrlVU),
    Nop,
    Compute(VUCompute),
}

impl VUOp {
    pub(crate) fn from_op(op: u32) -> Option<Self> {
        if op & 0x7FF != 0 {
            return Self::decode_vector_op(op);
        }
        let subop = utils::u8_at(21, 5, op);
        match subop {
            0x00 => MoveVU::from_op(op).map(Self::MFC2),
            0x02 => CtrlVU::from_op(op).map(Self::CFC2),
            0x04 => MoveVU::from_op(op).map(Self::MTC2),
            0x06 => CtrlVU::from_op(op).map(Self::CTC2),
            _ => None,
        }
    }

    fn decode_vector_op(op: u32) -> Option<Self> {
        let opcode = VUOpcode::try_from(utils::u8_at(0, 5, op)).ok()?;
        if opcode == VUOpcode::VNOP {
            return Some(Self::Nop);
        }

        let element = Element::at_bit(21, 4, op)?;
        let vt = VUReg::at_bit(16, op);
        let vd = VUReg::at_bit(6, op);
        let vs = match opcode {
            VUOpcode::VRCP
            | VUOpcode::VRCPL
            | VUOpcode::VMOV
            | VUOpcode::VRSQ
            | VUOpcode::VRSQL
            | VUOpcode::VRSQH => Element::at_bit(11, 5, op)?.into(),
            _ => VUReg::at_bit(11, op).into(),
        };

        let info = VUCompute {
            op: opcode,
            vt,
            vd,
            vs,
            element,
        };

        Some(Self::Compute(info))
    }
}

impl Print for VUOp {
    fn print(&self, opts: PrintOpts, w: &mut impl fmt::Write) -> fmt::Result {
        match self {
            VUOp::MFC2(sub) => {
                write!(w, "mfc2 ")?;
                sub.print(opts, w)
            }
            VUOp::MTC2(sub) => {
                write!(w, "mtc2 ")?;
                sub.print(opts, w)
            }
            VUOp::CFC2(sub) => {
                write!(w, "cfc2 ")?;
                sub.print(opts, w)
            }
            VUOp::CTC2(sub) => {
                write!(w, "ctc2 ")?;
                sub.print(opts, w)
            }
            VUOp::Nop => write!(w, "vnop"),
            VUOp::Compute(com) => com.print(opts, w),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveVU {
    rt: GpReg,
    vd: VUReg,
    // not the same as a compute Element
    element: u8,
}

impl MoveVU {
    fn from_op(op: u32) -> Option<Self> {
        let rt = GpReg::at_bit(16, 5).ok()?;
        let vd = VUReg::at_bit(11, op);
        let element = utils::u8_at(7, 4, op);

        Some(Self { rt, vd, element })
    }
}

impl Print for MoveVU {
    fn print(&self, opts: PrintOpts, w: &mut impl fmt::Write) -> fmt::Result {
        self.rt.print(opts, w)?;
        write!(w, ", {}[{}]", self.vd, self.element)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CtrlVU {
    rt: GpReg,
    vs: VUCtrlReg,
}

impl CtrlVU {
    fn from_op(op: u32) -> Option<Self> {
        let rt = GpReg::at_bit(16, 5).ok()?;
        let vs = VUCtrlReg::at_bit(11, op).ok()?;

        Some(Self { rt, vs })
    }
}

impl Print for CtrlVU {
    fn print(&self, opts: PrintOpts, w: &mut impl fmt::Write) -> fmt::Result {
        self.rt.print(opts, w)?;
        write!(w, ", {}", self.vs)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VUCompute {
    op: VUOpcode,
    vt: VUReg,
    vs: RegEl, // element idx in scalar ops
    vd: VUReg,
    element: Element,
}

impl Print for VUCompute {
    fn print(&self, opts: PrintOpts, w: &mut impl fmt::Write) -> fmt::Result {
        // print op code and vd
        write!(w, "{} {}", self.op.as_mnemonic(), self.vd)?;
        // then write the rest of the op depending on if there is one or two scalar regs
        match self.vs {
            RegEl::Reg(vs) => {
                // op vd, vs, vt[e]
                write!(w, ", {}, {}", vs, self.vt)?;
                self.element.print(opts, w)?;
            }
            RegEl::Element(de) => {
                // op vd[de], vt[e]
                de.print(opts, w)?;
                write!(w, ", {}", self.vt)?;
                self.element.print(opts, w)?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RegEl {
    Reg(VUReg),
    Element(Element),
}

impl From<VUReg> for RegEl {
    fn from(v: VUReg) -> Self {
        Self::Reg(v)
    }
}

impl From<Element> for RegEl {
    fn from(e: Element) -> Self {
        Self::Element(e)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum VUOpcode {
    VMULF = 0x00, // Vector (Frac) Multiply
    VMULU = 0x01, // Vector (Unsigned Frac) Multiply
    VRNDP = 0x02, // Vector DCT Round (+)
    VMULQ = 0x03, // Vector (Integer) Multiply
    VMUDL = 0x04, // Vector low multiply
    VMUDM = 0x05, // Vector mid-m multiply
    VMUDN = 0x06, // Vector mid-n multiply
    VMUDH = 0x07, // Vector high multiply
    VMACF = 0x08, // Vector (Frac) Multiply Accumulate
    VMACU = 0x09, // Vector (Unsigned Frac) Multiply Accumulate
    VRNDN = 0x0A, // Vector DCT Round (-)
    VMACQ = 0x0B, // Vector (Integer) Multiply Accumulate
    VMADL = 0x0C, // Vector low multiply accumulate
    VMADM = 0x0D, // Vector mid-m multiply accumulate
    VMADN = 0x0E, // Vector mid-n multiply accumulate
    VMADH = 0x0F, // Vector high multiply accumulate
    VADD = 0x10,  // Vector add
    VSUB = 0x11,  // Vector subtract
    VABS = 0x13,  // Vector absolute value
    VADDC = 0x14, // Vector add with carry
    VSUBC = 0x15, // Vector subtract with carry
    VSAR = 0x1D,  // Vector accumulator read (and write)
    VLT = 0x20,   // Vector select (Less than)
    VEQ = 0x21,   // Vector select (Equal)
    VNE = 0x22,   // Vector select (Not equal)
    VGE = 0x23,   // Vector select (Greater than or equal)
    VCL = 0x24,   // Vector select clip (Test low)
    VCH = 0x25,   // Vector select clip (Test high)
    VCR = 0x26,   // Vector select crimp (Test low)
    VMRG = 0x27,  // Vector select merge
    VAND = 0x28,  // Vector AND
    VNAND = 0x29, // Vector NAND
    VOR = 0x2A,   // Vector OR
    VNOR = 0x2B,  // Vector NOR
    VXOR = 0x2C,  // Vector XOR
    VNXOR = 0x2D, // Vector NXOR
    VRCP = 0x30,  // Vector element scalar reciprocal (Single precision)
    VRCPL = 0x31, // Vector element scalar reciprocal (Double precision, Low)
    VRCPH = 0x32, // Vector element scalar reciprocal (Double precision, High)
    VMOV = 0x33,  // Vector element scalar move
    VRSQ = 0x34,  // Vector element scalar SQRT reciprocal
    VRSQL = 0x35, // Vector element scalar SQRT reciprocal (Double precision, Low)
    VRSQH = 0x36, // Vector element scalar SQRT reciprocal (Double precision, High)
    VNOP = 0x37,  // Vector null instruction
}

impl VUOpcode {
    const fn as_mnemonic(&self) -> &'static str {
        match self {
            Self::VMULF => "vmulf",
            Self::VMULU => "vmulu",
            Self::VRNDP => "vrndp",
            Self::VMULQ => "vmulq",
            Self::VMUDL => "vmudl",
            Self::VMUDM => "vmudm",
            Self::VMUDN => "vmudn",
            Self::VMUDH => "vmudh",
            Self::VMACF => "vmacf",
            Self::VMACU => "vmacu",
            Self::VRNDN => "vrndn",
            Self::VMACQ => "vmacq",
            Self::VMADL => "vmadl",
            Self::VMADM => "vmadm",
            Self::VMADN => "vmadn",
            Self::VMADH => "vmadh",
            Self::VADD => "vadd",
            Self::VSUB => "vsub",
            Self::VABS => "vabs",
            Self::VADDC => "vaddc",
            Self::VSUBC => "vsubc",
            Self::VSAR => "vsar",
            Self::VLT => "vlt",
            Self::VEQ => "veq",
            Self::VNE => "vne",
            Self::VGE => "vge",
            Self::VCL => "vcl",
            Self::VCH => "vch",
            Self::VCR => "vcr",
            Self::VMRG => "vmrg",
            Self::VAND => "vand",
            Self::VNAND => "vnand",
            Self::VOR => "vor",
            Self::VNOR => "vnor",
            Self::VXOR => "vxor",
            Self::VNXOR => "vnxor",
            Self::VRCP => "vrcp",
            Self::VRCPL => "vrcpl",
            Self::VRCPH => "vrcph",
            Self::VMOV => "vmov",
            Self::VRSQ => "vrsq",
            Self::VRSQL => "vrsql",
            Self::VRSQH => "vrsqh",
            Self::VNOP => "vnop",
        }
    }
}
