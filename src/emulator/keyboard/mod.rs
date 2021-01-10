use minifb::Key;
use std::convert::TryFrom;
use std::convert::TryInto;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KeyboardError {
    #[error("Invalid key `{0}`")]
    InvalidKey(u8),
    #[error("Unhandled key")]
    UnhandledKey,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum KeypadKey {
    K1,
    K2,
    K3,
    KC,
    K4,
    K5,
    K6,
    KD,
    K7,
    K8,
    K9,
    KE,
    KA,
    K0,
    KB,
    KF,
}

impl From<KeypadKey> for u8 {
    fn from(key: KeypadKey) -> Self {
        match key {
            KeypadKey::K1 => 0x1,
            KeypadKey::K2 => 0x2,
            KeypadKey::K3 => 0x3,
            KeypadKey::KC => 0xC,
            KeypadKey::K4 => 0x4,
            KeypadKey::K5 => 0x5,
            KeypadKey::K6 => 0x6,
            KeypadKey::KD => 0xD,
            KeypadKey::K7 => 0x7,
            KeypadKey::K8 => 0x8,
            KeypadKey::K9 => 0x9,
            KeypadKey::KE => 0xE,
            KeypadKey::KA => 0xA,
            KeypadKey::K0 => 0x0,
            KeypadKey::KB => 0xB,
            KeypadKey::KF => 0xF,
        }
    }
}

impl TryFrom<u8> for KeypadKey {
    type Error = KeyboardError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x0 => Ok(Self::K0),
            0x1 => Ok(Self::K1),
            0x2 => Ok(Self::K2),
            0x3 => Ok(Self::K3),
            0x4 => Ok(Self::K4),
            0x5 => Ok(Self::K5),
            0x6 => Ok(Self::K6),
            0x7 => Ok(Self::K7),
            0x8 => Ok(Self::K8),
            0x9 => Ok(Self::K9),
            0xa => Ok(Self::KA),
            0xb => Ok(Self::KB),
            0xc => Ok(Self::KC),
            0xd => Ok(Self::KD),
            0xe => Ok(Self::KE),
            0xf => Ok(Self::KF),
            _ => Err(KeyboardError::InvalidKey(value)),
        }
    }
}

impl TryFrom<Key> for KeypadKey {
    type Error = KeyboardError;

    fn try_from(key: Key) -> Result<Self, Self::Error> {
        match key {
            Key::Key0 => Ok(Self::K0),
            Key::Key1 => Ok(Self::K1),
            Key::Key2 => Ok(Self::K2),
            Key::Key3 => Ok(Self::K3),
            Key::Key4 => Ok(Self::K4),
            Key::Key5 => Ok(Self::K5),
            Key::Key6 => Ok(Self::K6),
            Key::Key7 => Ok(Self::K7),
            Key::Key8 => Ok(Self::K8),
            Key::Key9 => Ok(Self::K9),
            Key::A => Ok(Self::KA),
            Key::B => Ok(Self::KB),
            Key::C => Ok(Self::KC),
            Key::D => Ok(Self::KD),
            Key::E => Ok(Self::KE),
            Key::F => Ok(Self::KF),
            _ => Err(KeyboardError::UnhandledKey),
        }
    }
}

#[derive(Debug)]
pub struct Keyboard {
    keys_pressed: Vec<KeypadKey>,
}

impl Keyboard {
    pub fn new(keys: Vec<Key>) -> Self {
        Self {
            keys_pressed: keys
                .into_iter()
                .map(KeypadKey::try_from)
                .filter_map(|result| result.ok())
                .collect(),
        }
    }

    pub fn is_key_pressed(&self, key: u8) -> Result<bool, KeyboardError> {
        Ok(self.keys_pressed.contains(&(key.try_into()?)))
    }

    pub fn get_key_pressed(&self) -> Option<KeypadKey> {
        self.keys_pressed.first().cloned()
    }
}
