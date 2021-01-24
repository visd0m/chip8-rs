use thiserror::Error;

use crate::emulator::audio::Audio;
use crate::emulator::cpu::{Cpu, CpuError};
use crate::emulator::display::Display;
use crate::emulator::frontend::Frontend;
use crate::emulator::keyboard::{KeyMapper, KeyboardState};
use crate::emulator::memory::Memory;

pub mod audio;
pub mod cpu;
pub mod display;
pub mod frontend;
pub mod keyboard;
pub mod memory;

#[derive(Debug, Error)]
pub enum EmulatorError {
    #[error(transparent)]
    EmulatorError(#[from] Box<dyn std::error::Error>),
    #[error(transparent)]
    CpuError(#[from] CpuError),
}

pub fn run(rom: &[u8], frontend: &mut dyn Frontend) -> Result<(), EmulatorError> {
    let mut memory = Memory::default();
    memory.load_rom(rom);

    frontend.run(&mut Cpu::new(memory), &mut Display::default())?;

    Ok(())
}

pub fn tick<K>(
    cpu: &mut Cpu,
    display: &mut Display,
    audio: &mut dyn Audio,
    keys: Vec<K>,
    key_mapper: &dyn KeyMapper<K>,
) -> Result<(), EmulatorError>
where
    K: PartialEq + Eq,
{
    Ok(cpu.cycle(display, audio, KeyboardState::new(keys, key_mapper))?)
}
