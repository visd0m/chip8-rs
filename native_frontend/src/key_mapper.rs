use crate::emulator;
use minifb::Key;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KeyboardError {
    #[error("Unhandled key")]
    UnhandledKey,
}

pub struct KeyMapper;

impl emulator::keyboard::KeyMapper<Key> for KeyMapper {
    fn map_key(&self, key: Key) -> Result<u8, Box<dyn std::error::Error>> {
        match key {
            Key::Key0 => Ok(0x0),
            Key::Key1 => Ok(0x1),
            Key::Key2 => Ok(0x2),
            Key::Key3 => Ok(0x3),
            Key::Key4 => Ok(0x4),
            Key::Key5 => Ok(0x5),
            Key::Key6 => Ok(0x6),
            Key::Key7 => Ok(0x7),
            Key::Key8 => Ok(0x8),
            Key::Key9 => Ok(0x9),
            Key::A => Ok(0xA),
            Key::B => Ok(0xB),
            Key::C => Ok(0xC),
            Key::D => Ok(0xD),
            Key::E => Ok(0xE),
            Key::F => Ok(0xF),
            _ => Err(Box::new(KeyboardError::UnhandledKey)),
        }
    }
}
