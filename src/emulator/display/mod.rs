use minifb::Window;

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

#[derive(Debug)]
pub struct Display {
    window: Window,
    buffer: Vec<u32>,
}

impl Display {
    pub fn new(window: Window) -> Self {
        Self {
            window,
            buffer: vec![0x00000000; WIDTH * HEIGHT],
        }
    }
}

impl Display {
    pub fn pixel(&mut self, x: usize, y: usize) -> bool {
        let index = x + (y * WIDTH);
        self.buffer[index] > 0
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, value: u32) {
        let index = x + (y * WIDTH);
        self.buffer[index] ^= value;
    }

    pub fn update(&mut self) {
        self.window
            .update_with_buffer(&self.buffer, WIDTH, HEIGHT)
            .unwrap();
    }

    pub fn clear(&mut self) {
        self.buffer = vec![0x00000000; WIDTH * HEIGHT]
    }

    pub fn window(&self) -> &Window {
        &self.window
    }
}
