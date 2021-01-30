use core::emulator;

pub struct Audio {}

impl emulator::audio::Audio for Audio {
    fn beep(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn stop_beep(&mut self) {}
}
