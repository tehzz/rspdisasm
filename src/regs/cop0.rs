use std::fmt::{Write, self};

use num_enum::{TryFromPrimitive, TryFromPrimitiveError};
use crate::utils;

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub(crate) enum Cop0Reg {
    DmaCache = 0,
    DmaRead,
    DmaReadLength,
    DmaWriteLength,
    SpStatus,
    DmaFull,
    DmaBusy,
    SpReserved,
    CmdStart,
    CmdEnd,
    CmdCurrent,
    CmdStatus,
    CmdClock,
    CmdBusy,
    CmdPipeBusy,
    CmdTmemBusy,
}

impl Cop0Reg {
    pub(crate) fn at_bit(b: u8, src: u32) -> Result<Self,TryFromPrimitiveError<Self>> {
        Self::try_from(utils::u8_at(b, 5, src))
    }

    fn nintendo_name(&self) -> &'static str {
        match self {
            Cop0Reg::DmaCache => "DMA_CACHE",
            Cop0Reg::DmaRead => "DMA_READ",
            Cop0Reg::DmaReadLength => "DMA_READ_LENGTH",
            Cop0Reg::DmaWriteLength => "DMA_WRITE_LENGTH",
            Cop0Reg::SpStatus => "SP_STATUS",
            Cop0Reg::DmaFull => "DMA_FULL",
            Cop0Reg::DmaBusy => "DMA_BUSY",
            Cop0Reg::SpReserved => "SP_RESERVED",
            Cop0Reg::CmdStart => "CMD_START",
            Cop0Reg::CmdEnd => "CMD_END",
            Cop0Reg::CmdCurrent => "CMD_CURRENT",
            Cop0Reg::CmdStatus => "CMD_STATUS",
            Cop0Reg::CmdClock => "CMD_CLOCK",
            Cop0Reg::CmdBusy => "CMD_BUSY",
            Cop0Reg::CmdPipeBusy => "CMD_PIPE_BUSY",
            Cop0Reg::CmdTmemBusy => "CMD_TMEM_BUSY",
        }
    }

    pub(crate) fn print(&self, w: &mut impl Write) -> fmt::Result {
        write!(w, "{}", self.nintendo_name())
    }
}
