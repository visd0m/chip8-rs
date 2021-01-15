use crate::emulator::audio::{Audio, AudioError};
use crate::emulator::cpu::{Cpu, CpuError};
use crate::emulator::display::{Display, DisplayError, HEIGHT, WIDTH};
use crate::emulator::keyboard::KeyboardState;
use crate::emulator::memory::Memory;
use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};
use thiserror::Error;

mod audio;
mod cpu;
mod display;
mod keyboard;
mod memory;
mod registers;

#[derive(Debug, Error)]
pub enum EmulatorError {
    #[error(transparent)]
    CpuError(#[from] CpuError),
    #[error(transparent)]
    SoundError(#[from] AudioError),
    #[error(transparent)]
    WindowError(#[from] minifb::Error),
    #[error(transparent)]
    DisplayError(#[from] DisplayError),
}

pub struct Emulator {
    display: Display,
    sound: Audio,
    cpu: Cpu,
}

impl Emulator {
    pub fn new(rom: &[u8]) -> Result<Self, EmulatorError> {
        let mut memory: Memory = Default::default();
        memory.load_rom(rom);

        let mut window = Window::new(
            "Chip-8 - ESC to exit",
            WIDTH,
            HEIGHT,
            WindowOptions {
                scale: Scale::X16,
                scale_mode: ScaleMode::Stretch,
                ..WindowOptions::default()
            },
        )?;

        window.limit_update_rate(Some(std::time::Duration::from_millis(2)));

        Ok(Self {
            display: Display::new(window),
            cpu: Cpu::new(memory),
            sound: Audio::new()?,
        })
    }

    pub fn run(&mut self) -> Result<(), EmulatorError> {
        while self.display.window_mut().is_open()
            && !self.display.window_mut().is_key_down(Key::Escape)
        {
            let keys = self.display.window_mut().get_keys().unwrap_or_default();

            self.cpu
                .cycle(&mut self.display, &mut self.sound, KeyboardState::new(keys))?;

            self.display.draw()?;
        }

        Ok(())
    }
}
