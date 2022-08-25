use std::{
    io::{Read, Seek, SeekFrom},
    path::PathBuf,
};

use clap::Parser;

/// Disassemble N64 RSP microcode
#[derive(Debug, Parser)]
struct Args {
    /// input ROM or binary
    #[clap(short, long, value_parser)]
    input: PathBuf,
    /// output for disassembled text, or stdout if not present
    #[clap(short, long, value_parser)]
    output: Option<PathBuf>,
    /// offset in `input` to begin disassembly
    #[clap(short = 'p', long, value_parser, default_value_t = 0)]
    offset: u64,
    /// number of bytes to disassemble
    #[clap(short = 'n', long, value_parser)]
    size: usize,
    /// vram of first instruction (not really important)
    #[clap(short, long, value_parser, default_value_t = 0x84000000)]
    vram: u32,
}
fn main() {
    let args = Args::parse();
    let opts = rspdisasm::PrintOpts::default();
    let mut f = std::fs::File::open(args.input).unwrap();
    f.seek(SeekFrom::Start(args.offset)).unwrap();
    let mut data = vec![0u8; args.size];
    f.read_exact(&mut data).unwrap();

    let result = rspdisasm::disassemble_bytes(&data, args.vram, opts).unwrap();
    println!("{result}");
}
