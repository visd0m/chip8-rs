use crate::audio::Audio;
use crate::key_mapper::KeyMapper;
use core::emulator;
use gloo_timers::callback::Interval;
use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, ImageData, Window};

pub struct WasmFrontend {
    key_mapper: KeyMapper,
    window: Window,
    canvas_context: CanvasRenderingContext2d,
    audio: Audio,
}

impl WasmFrontend {
    pub fn new() -> Self {
        let window: Window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let canvas = document
            .get_element_by_id("display")
            .expect("document should have a body");

        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        Self {
            window,
            key_mapper: KeyMapper,
            canvas_context: context,
            audio: Audio {},
        }
    }
}

impl emulator::frontend::Frontend for WasmFrontend {
    fn run(
        &mut self,
        cpu: &mut emulator::cpu::Cpu,
        display: &mut emulator::display::Display,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // let i = Interval::new(16, move || {
        //     let keys = vec![];
        //     emulator::tick(cpu, display, &mut self.audio, keys, &self.key_mapper)
        //         .expect_err("Error ticking");
        //
        //     let data = ImageData::new_with_u8_clamped_array_and_sh(
        //         Clamped(
        //             display
        //                 .buffer()
        //                 .iter()
        //                 .map(|b| b.to_be_bytes().to_vec())
        //                 .flatten()
        //                 .collect::<Vec<u8>>()
        //                 .as_mut_slice(),
        //         ),
        //         (emulator::display::Display::width()) as u32,
        //         (emulator::display::Display::height()) as u32,
        //     )
        //     .expect("Error converting display into js byte data");
        //
        //     self.canvas_context
        //         .put_image_data(&data, 0.0, 0.0)
        //         .expect("Error updating canvas");
        // });
        //
        // i.forget();

        Ok(())
    }
}
