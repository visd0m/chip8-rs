use crate::emulator::Emulator;
use std::fs::File;
use std::io::Read;
use std::path::Path;

mod emulator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rom = Vec::new();
    // let mut file = File::open(Path::new("./Brix [Andreas Gustafsson, 1990].ch8"))?;
    // let mut file = File::open(Path::new("./Breakout (Brix hack) [David Winter, 1997].ch8"))?;
    // let mut file = File::open(Path::new("./Pong (alt).ch8"))?;
    // let mut file = File::open(Path::new("./test_opcode.ch8"))?;
    let mut file = File::open(Path::new("./Tetris [Fran Dachille, 1991].ch8"))?;
    file.read_to_end(&mut rom)?;

    let mut emu = Emulator::new(&rom)?;

    emu.run()?;

    Ok(())
}
