mod ops;
mod print;
mod regs;
mod sym;
mod utils;

use ops::RspOpcode;
use std::{
    collections::HashMap,
    fmt::{self, Write},
};

use print::Print;
pub use print::PrintOpts;

#[derive(Debug, Clone)]
pub enum RspDisasmError {
    UnalignedInput(usize),
}

impl fmt::Display for RspDisasmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnalignedInput(size) => write!(
                f,
                "data was not aligned to four-byte size (was {} byte{} sized)",
                size,
                if *size > 1 { "s" } else { "" }
            ),
        }
    }
}

impl std::error::Error for RspDisasmError {}

pub fn disassemble_bytes(
    data: &[u8],
    vaddr: u32,
    opts: PrintOpts,
) -> Result<String, RspDisasmError> {
    if data.len() % 4 != 0 {
        return Err(RspDisasmError::UnalignedInput(data.len()));
    }

    let n_instr = data.len() / 4;
    let (syms, ops) = data
        .chunks_exact(4)
        .enumerate()
        .map(|(i, bytes)| (vaddr + i as u32 * 4, bytes))
        .map(parse_op)
        .fold(
            (HashMap::new(), Vec::with_capacity(n_instr)),
            |(mut syms, mut arr), (pc, word, op)| {
                let sym = op.get_symbol().map(|s| (s.value(), s));
                // todo: combine syms to preserve global
                syms.extend(sym);
                arr.push((pc, word, op));
                (syms, arr)
            },
        );

    let mut s = String::with_capacity(n_instr * 32);
    for (pc, word, op) in ops {
        if let Some(sym) = syms.get(&pc) {
            if sym.is_global() {
                writeln!(&mut s).unwrap();
            }
            writeln!(&mut s, "{}:", sym).unwrap();
        }
        write!(&mut s, "/* {:08X} {:08X} */\t", pc, word).unwrap();
        op.print(opts, &mut s).unwrap();
        writeln!(&mut s).unwrap();
    }

    Ok(s)
}

fn parse_op((pc, bytes): (u32, &[u8])) -> (u32, u32, RspOpcode) {
    let word = u32::from_be_bytes(bytes.try_into().unwrap());
    let op = RspOpcode::decode(word, pc);
    (pc, word, op)
}
