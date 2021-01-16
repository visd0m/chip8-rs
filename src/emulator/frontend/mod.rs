use crate::emulator::cpu::Cpu;
use crate::emulator::display::Display;

pub mod native_window;

pub trait Frontend {
    fn run(
        &mut self,
        cpu: &mut Cpu,
        display: &mut Display,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
