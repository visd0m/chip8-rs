use crate::emulator;
use crate::emulator::cpu::Cpu;
use crate::emulator::display::Display;
use crate::emulator::frontend;
use crate::emulator::frontend::Frontend;
use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};

pub mod audio;
pub mod key_mapper;

pub struct NativeWindowFrontend {
    audio: frontend::native_window::audio::Audio,
    window: Window,
    key_mapper: frontend::native_window::key_mapper::KeyMapper,
}

impl NativeWindowFrontend {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut window = Window::new(
            "Chip-8 - ESC to exit",
            Display::width(),
            Display::height(),
            WindowOptions {
                scale: Scale::X16,
                scale_mode: ScaleMode::Stretch,
                ..WindowOptions::default()
            },
        )?;

        window.limit_update_rate(Some(std::time::Duration::from_millis(2)));

        Ok(Self {
            audio: frontend::native_window::audio::Audio::new()?,
            window,
            key_mapper: frontend::native_window::key_mapper::KeyMapper,
        })
    }
}

impl Frontend for NativeWindowFrontend {
    fn run(
        &mut self,
        cpu: &mut Cpu,
        display: &mut Display,
    ) -> Result<(), Box<dyn std::error::Error>> {
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            let keys = self.window.get_keys().unwrap_or_default();

            emulator::tick(cpu, display, &mut self.audio, keys, &self.key_mapper)?;

            self.window
                .update_with_buffer(display.buffer(), Display::width(), Display::height())?
        }

        Ok(())
    }
}
