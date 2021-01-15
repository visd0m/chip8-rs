use crate::emulator::Emulator;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use structopt::StructOpt;

mod emulator;

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

    let mut emu = Emulator::new(&rom)?;

    emu.run()?;

    Ok(())
}
