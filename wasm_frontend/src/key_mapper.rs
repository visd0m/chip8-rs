use core::emulator;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KeyboardError {
    #[error("Unhandled key")]
    UnhandledKey,
}

pub struct KeyMapper;

impl emulator::keyboard::KeyMapper<String> for KeyMapper {
    fn map_key(&self, key: String) -> Result<u8, Box<dyn std::error::Error>> {
        match key.as_str() {
            "0" => Ok(0x0),
            "1" => Ok(0x1),
            "2" => Ok(0x2),
            "3" => Ok(0x3),
            "4" => Ok(0x4),
            "5" => Ok(0x5),
            "6" => Ok(0x6),
            "7" => Ok(0x7),
            "8" => Ok(0x8),
            "9" => Ok(0x9),
            "a" => Ok(0xa),
            "b" => Ok(0xb),
            "c" => Ok(0xc),
            "d" => Ok(0xd),
            "e" => Ok(0xe),
            "f" => Ok(0xf),
            _ => Err(Box::new(KeyboardError::UnhandledKey)),
        }
    }
}
