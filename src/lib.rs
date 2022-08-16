mod regs;
mod utils;
mod ops;
mod sym;

use std::fmt::{self, Write};
use ops::RspOpcode;

#[derive(Debug, Clone)]
pub enum RspDisasmError {
    UnalignedInput(usize)
}

impl fmt::Display for RspDisasmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnalignedInput(size) => write!(f, "data was not aligned to four-byte size (was {} byte{} sized)", size, if *size > 1 { "s" } else { "" })
        }
    }
}

impl std::error::Error for RspDisasmError {

}

#[derive(Debug, Clone, Copy, Default)]
pub struct FmtOpts {
    use_reg_names: bool
}

pub fn disassemble_bytes(data: &[u8], vaddr: u32, _opts: FmtOpts) -> Result<String, RspDisasmError> {
    if data.len() % 4 != 0 {
        return Err(RspDisasmError::UnalignedInput(data.len()))
    }
    let s = data.chunks_exact(4)
        .enumerate()
        .map(|(i, bytes)| {
            let word = u32::from_be_bytes(bytes.try_into().unwrap());
            let pc = vaddr + i as u32 * 4;
            let op = RspOpcode::decode(word, pc);
            (pc, word, op)
        })
        .fold(
            String::with_capacity(data.len() * 32),
            |mut s, (pc, word, op)| {
                write!(&mut s, "/* {:08X} {:08X} */\t", pc, word).unwrap();
                op.print(&mut s).unwrap();
                writeln!(&mut s).unwrap();
                s
            }
        );

    Ok(s)
}
