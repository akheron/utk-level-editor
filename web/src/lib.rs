use log::info;
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(target_arch = "wasm32")]
use log::Level;

#[wasm_bindgen]
pub struct State {
    width: u32,
    height: u32,
    pixels: Vec<u8>,

    frame: u64,
}

#[wasm_bindgen]
impl State {
    pub fn new() -> Self {
        #[cfg(target_arch = "wasm32")]
        console_log::init_with_level(Level::Debug).unwrap();

        let width = 320u32;
        let height = 200u32;
        let mut pixels = vec![0; (width * height * 4) as usize];
        (0..width * height).for_each(|i| {
            pixels[(i * 4 + 3) as usize] = 255;
        });
        Self {
            width,
            height,
            pixels,
            frame: 0,
        }
    }

    pub fn screen(&self) -> *const u8 {
        self.pixels.as_ptr()
    }
    pub fn screen_width(&self) -> u32 {
        self.width
    }
    pub fn screen_height(&self) -> u32 {
        self.height
    }

    pub fn mouse_move(&mut self, x: u32, y: u32) {
        info!("mouse move: {x} {y}");
        if x < self.width && y < self.height {
            let index = (y * self.width + x) as usize * 4;
            self.pixels[index] = 255;
        }
    }
    pub fn mouse_down(&mut self, button: MouseButton) {
        info!("mouse down: {:?}", button);
    }
    pub fn mouse_up(&mut self, button: MouseButton) {
        info!("mouse up: {:?}", button);
    }
    pub fn key_down(&mut self, key: Keycode) {
        info!("key down: {:?}", key);
    }
    pub fn key_up(&mut self, key: Keycode) {
        info!("key up: {:?}", key);
    }

    pub fn frame(&mut self) {
        self.frame += 1;
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub enum MouseButton {
    Left,
    Right,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum Keycode {
    Escape,
    Backspace,
    Return,
    Space,
    PageDown,
    PageUp,
    Up,
    Down,
    Left,
    Right,
    KpEnter,
    KpMinus,
    KpPlus,
    Minus,
    Plus,
    A,
    C,
    Q,
    S,
    W,
    X,
    Y,
    Z,
    Num1,
    Num2,
    F1,
    F2,
    F3,
    F4,
    F6,
    F7,
    F8,
    F9,
}
