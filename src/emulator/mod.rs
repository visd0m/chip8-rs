use crate::emulator::cpu::{Cpu, CpuError};
use crate::emulator::display::{Display, HEIGHT, WIDTH};
use crate::emulator::keyboard::Keyboard;
use crate::emulator::memory::Memory;
use crate::emulator::sound::{Sound, SoundError};
use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};
use std::thread::sleep;
use std::time::Duration;
use thiserror::Error;

mod cpu;
mod display;
mod keyboard;
mod memory;
mod registers;
mod sound;

#[derive(Debug, Error)]
pub enum EmulatorError {
    #[error(transparent)]
    CpuError(#[from] CpuError),
    #[error(transparent)]
    SoundError(#[from] SoundError),
}

pub struct Emulator {
    display: Display,
    sound: Sound,
    cpu: Cpu,
}

impl Emulator {
    pub fn new(rom: &[u8]) -> Result<Self, EmulatorError> {
        let mut memory: Memory = Default::default();
        memory.load_rom(rom);

        let mut window = Window::new(
            "Test - ESC to exit",
            WIDTH,
            HEIGHT,
            WindowOptions {
                scale: Scale::X16,
                scale_mode: ScaleMode::Stretch,
                ..WindowOptions::default()
            },
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });
        window.limit_update_rate(Some(std::time::Duration::from_micros(16)));

        Ok(Self {
            display: Display::new(window),
            cpu: Cpu::new(memory),
            sound: Sound::new()?,
        })
    }

    pub fn run(&mut self) -> Result<(), EmulatorError> {
        while self.display.window().is_open() && !self.display.window().is_key_down(Key::Escape) {
            let keys = self.display.window().get_keys().unwrap_or_default();

            self.cpu
                .cycle(&mut self.display, &mut self.sound, Keyboard::new(keys))?;

            self.display.update();

            sleep(Duration::from_millis(16));
        }

        Ok(())
    }
}
