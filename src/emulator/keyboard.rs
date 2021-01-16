pub trait KeyMapper<K>
where
    K: PartialEq + Eq,
{
    fn map_key(&self, key: K) -> Result<u8, Box<dyn std::error::Error>>;
}

#[derive(Debug)]
pub struct KeyboardState {
    keys_pressed: Vec<u8>,
}

impl KeyboardState {
    pub fn new<K>(keys: Vec<K>, key_mapper: &dyn KeyMapper<K>) -> Self
    where
        K: PartialEq + Eq,
    {
        Self {
            keys_pressed: keys
                .into_iter()
                .map(|key| key_mapper.map_key(key))
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
