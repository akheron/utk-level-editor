pub enum Event {
    Quit,
    Window { win_event: WindowEvent },
    KeyDown { keycode: Keycode },
    MouseButtonDown { button: MouseButton },
    MouseButtonUp { button: MouseButton },
    MouseMotion { x: u32, y: u32 },
    TextInput { text: String },
}

pub enum WindowEvent {
    Resized { width: u32, height: u32 },
    Maximized,
}

pub enum MouseButton {
    Left,
    Right,
}

#[derive(PartialEq)]
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
