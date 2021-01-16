const WIDTH: usize = 64;
const HEIGHT: usize = 32;

#[derive(Debug)]
pub struct Display {
    buffer: Vec<u32>,
}

impl Default for Display {
    fn default() -> Self {
        Self {
            buffer: vec![0x00000000; WIDTH * HEIGHT],
        }
    }
}

impl Display {
    pub fn buffer(&mut self) -> &mut Vec<u32> {
        &mut self.buffer
    }

    pub fn pixel(&mut self, x: usize, y: usize) -> bool {
        let index = x + (y * WIDTH);
        self.buffer[index] > 0
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, value: u32) {
        let index = x + (y * WIDTH);
        self.buffer[index] ^= value;
    }

    pub fn clear(&mut self) {
        self.buffer = vec![0x00000000; WIDTH * HEIGHT]
    }

    pub fn width() -> usize {
        WIDTH
    }

    pub fn height() -> usize {
        HEIGHT
    }
}
