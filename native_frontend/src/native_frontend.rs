use crate::audio::Audio;
use crate::key_mapper::KeyMapper;
use core::emulator;
use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};

pub struct NativeWindowFrontend {
    audio: Audio,
    window: Window,
    key_mapper: KeyMapper,
}

impl NativeWindowFrontend {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut window = Window::new(
            "Chip-8 - ESC to exit",
            emulator::display::Display::width(),
            emulator::display::Display::height(),
            WindowOptions {
                scale: Scale::X16,
                scale_mode: ScaleMode::Stretch,
                ..WindowOptions::default()
            },
        )?;

        window.limit_update_rate(Some(std::time::Duration::from_millis(2)));

        Ok(Self {
            audio: Audio::new()?,
            window,
            key_mapper: KeyMapper,
        })
    }
}

impl NativeWindowFrontend {
    pub fn run(
        &mut self,
        cpu: &mut emulator::cpu::Cpu,
        display: &mut emulator::display::Display,
    ) -> Result<(), Box<dyn std::error::Error>> {
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            let keys = self.window.get_keys().unwrap_or_default();

            emulator::tick(cpu, display, &mut self.audio, keys, &self.key_mapper)?;

            self.window.update_with_buffer(
                display
                    .buffer()
                    .iter()
                    .map(|bit| {
                        if *bit {
                            0xFFFFFFFF as u32
                        } else {
                            0x00000000 as u32
                        }
                    })
                    .collect::<Vec<u32>>()
                    .as_slice(),
                emulator::display::Display::width(),
                emulator::display::Display::height(),
            )?
        }

        Ok(())
    }
}
