mod audio;
mod key_mapper;
mod utils;

use crate::audio::Audio;
use crate::key_mapper::KeyMapper;
use core::emulator;
use core::emulator::cpu::Cpu;
use core::emulator::display::Display;
use core::emulator::keyboard::KeyboardState;
use core::emulator::memory::Memory;
use gloo_events::{EventListener, EventListenerOptions, EventListenerPhase};
use gloo_timers::callback::Interval;
use std::sync::Arc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;
use wasm_bindgen::__rt::std::sync::RwLock;
use web_sys::{CanvasRenderingContext2d, Document, ImageData, KeyboardEvent, Window};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn run_emu(rom_bytes: &[u8]) {
    utils::set_panic_hook();

    let window: Window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let canvas = document
        .get_element_by_id("display")
        .expect("document should have a body");

    let canvas: web_sys::HtmlCanvasElement =
        canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let keys: Arc<RwLock<Vec<String>>> = Arc::new(RwLock::new(vec![]));

    setup_key_down_listener(&document, keys.clone());
    setup_key_up_listener(&document, keys.clone());

    run(context, rom_bytes, keys.clone());
}

fn setup_key_down_listener(document: &Document, keys: Arc<RwLock<Vec<String>>>) {
    let on_key_down = EventListener::new_with_options(
        &document,
        "keydown",
        EventListenerOptions {
            phase: EventListenerPhase::Capture,
            passive: true,
        },
        move |event| {
            let mut keys = keys.write().unwrap();
            let keyboard_event = event.clone().dyn_into::<KeyboardEvent>().unwrap();
            let mut event_string = String::from("");
            event_string.push_str(&keyboard_event.key());
            keys.push(event_string);
        },
    );

    on_key_down.forget();
}

fn setup_key_up_listener(document: &Document, keys: Arc<RwLock<Vec<String>>>) {
    let on_key_up = EventListener::new_with_options(
        &document,
        "keyup",
        EventListenerOptions {
            phase: EventListenerPhase::Capture,
            passive: true,
        },
        move |_event| {
            let mut keys = keys.write().unwrap();
            keys.clear()
        },
    );

    on_key_up.forget();
}

fn run(context: CanvasRenderingContext2d, rom_bytes: &[u8], keys: Arc<RwLock<Vec<String>>>) {
    let mut memory = Memory::default();
    memory.load_rom(rom_bytes);
    let mut cpu = Cpu::new(memory);
    let mut display = Display::default();

    let mut audio = Audio {};

    let i = Interval::new(1 / 100, move || {
        let state = KeyboardState::new(keys.read().unwrap().clone(), &KeyMapper {});

        cpu.cycle(&mut display, &mut audio, state).unwrap();

        let data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(
                display
                    .buffer()
                    .iter()
                    .map(|b| {
                        if *b {
                            vec![0xFF, 0xFF, 0xFF, 0xFF]
                        } else {
                            vec![0x00, 0x00, 0x00, 0xFF]
                        }
                    })
                    .flatten()
                    .collect::<Vec<u8>>()
                    .as_mut_slice(),
            ),
            (emulator::display::Display::width()) as u32,
            (emulator::display::Display::height()) as u32,
        )
        .expect("Error converting display into js byte data");

        context
            .put_image_data(&data, 0.0, 0.0)
            .expect("Error updating canvas");
    });

    i.forget();
}
