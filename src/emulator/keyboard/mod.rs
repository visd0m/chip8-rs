use minifb::Key;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KeyboardError {
    #[error("Unhandled key")]
    UnhandledKey,
}

fn map_key(key: Key) -> Result<u8, KeyboardError> {
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
        _ => Err(KeyboardError::UnhandledKey),
    }
}

#[derive(Debug)]
pub struct KeyboardState {
    keys_pressed: Vec<u8>,
}

impl KeyboardState {
    pub fn new(keys: Vec<Key>) -> Self {
        Self {
            keys_pressed: keys
                .into_iter()
                .map(map_key)
                .filter_map(|result| result.ok())
                .collect(),
        }
    }

    pub fn is_key_pressed(&self, key: u8) -> bool {
        self.keys_pressed.contains(&key)
    }

    pub fn get_key_pressed(&self) -> Option<u8> {
        self.keys_pressed.first().cloned()
    }
}
