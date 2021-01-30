use thiserror::Error;

use crate::emulator::audio::Audio;
use crate::emulator::cpu::{Cpu, CpuError};
use crate::emulator::display::Display;
use crate::emulator::keyboard::{KeyMapper, KeyboardState};

pub mod audio;
pub mod cpu;
pub mod display;
pub mod keyboard;
pub mod memory;

#[derive(Debug, Error)]
pub enum EmulatorError {
    #[error(transparent)]
    EmulatorError(#[from] Box<dyn std::error::Error>),
    #[error(transparent)]
    CpuError(#[from] CpuError),
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
