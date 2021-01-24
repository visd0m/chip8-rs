const WIDTH: usize = 64;
const HEIGHT: usize = 32;

#[derive(Debug)]
pub struct Display {
    buffer: Vec<bool>,
}

impl Default for Display {
    fn default() -> Self {
        Self {
            buffer: vec![false; WIDTH * HEIGHT],
        }
    }
}

impl Display {
    pub fn buffer(&mut self) -> &mut Vec<bool> {
        &mut self.buffer
    }

    pub fn pixel(&mut self, x: usize, y: usize) -> bool {
        let index = x + (y * WIDTH);
        self.buffer[index]
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, value: bool) {
        let index = x + (y * WIDTH);
        self.buffer[index] ^= value;
    }

    pub fn clear(&mut self) {
        self.buffer = vec![false; WIDTH * HEIGHT]
    }

    pub fn width() -> usize {
        WIDTH
    }

    pub fn height() -> usize {
        HEIGHT
    }
}
