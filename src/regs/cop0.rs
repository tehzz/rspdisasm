use std::fmt::{self, Write};

use crate::{print::Print, utils};
use num_enum::{TryFromPrimitive, TryFromPrimitiveError};

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
    pub(crate) fn at_bit(b: u8, src: u32) -> Result<Self, TryFromPrimitiveError<Self>> {
        Self::try_from(utils::u8_at(b, 5, src))
    }

    const fn nintendo_name(&self) -> &'static str {
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

    const fn armips_name(&self) -> &'static str {
        match self {
            Cop0Reg::DmaCache => "sp_mem_addr",
            Cop0Reg::DmaRead => "sp_dram_addr",
            Cop0Reg::DmaReadLength => "sp_rd_len",
            Cop0Reg::DmaWriteLength => "sp_wr_len",
            Cop0Reg::SpStatus => "sp_status",
            Cop0Reg::DmaFull => "sp_dma_full",
            Cop0Reg::DmaBusy => "sp_dma_busy",
            Cop0Reg::SpReserved => "sp_semaphore",
            Cop0Reg::CmdStart => "dpc_start",
            Cop0Reg::CmdEnd => "dpc_end",
            Cop0Reg::CmdCurrent => "dpc_current",
            Cop0Reg::CmdStatus => "dpc_status",
            Cop0Reg::CmdClock => "dpc_clock",
            Cop0Reg::CmdBusy => "dpc_bufbusy",
            Cop0Reg::CmdPipeBusy => "dpc_pipebusy",
            Cop0Reg::CmdTmemBusy => "dpc_tmem",
        }
    }
}

impl Print for Cop0Reg {
    fn print(&self, opts: crate::PrintOpts, w: &mut impl Write) -> fmt::Result {
        let name = if opts.armips_cop0_names {
            self.armips_name()
        } else {
            self.nintendo_name()
        };
        write!(w, "{}", name)
    }
}
