use crate::native_frontend::NativeWindowFrontend;
use core::emulator;
use core::emulator::cpu::Cpu;
use core::emulator::display::Display;
use core::emulator::memory::Memory;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use structopt::StructOpt;

mod audio;
mod key_mapper;
mod native_frontend;

#[derive(Debug, StructOpt)]
pub struct Opt {
    #[structopt(short)]
    file: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt: Opt = Opt::from_args();

    let mut rom = Vec::new();
    let mut file = File::open(Path::new(opt.file.as_str()))?;
    file.read_to_end(&mut rom)?;

    let mut memory = Memory::default();
    memory.load_rom(&rom);

    NativeWindowFrontend::new()?.run(&mut Cpu::new(memory), &mut Display::default())?;

    Ok(())
}
