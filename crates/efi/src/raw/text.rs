use crate::{debugln, print};
use crate::{println, status::Status, Event, Int32, UInt16, UIntN, WString};
#[repr(C)]
#[derive(Default)]
pub struct TextKey {
    pub code: UInt16,
    pub unicode_char: UInt16,
}

#[repr(C)]
pub struct TextInput {
    pub reset: extern "win64" fn(&TextInput, bool) -> Status,
    pub read_key_stroke: extern "win64" fn(&TextInput, &mut TextKey) -> Status,
    pub wait_for_key: Event,
}

#[repr(C)]
pub struct TextOutputMode {
    pub max: Int32,
    pub mode: Int32,
    pub attr: Int32,
    pub column: Int32,
    pub row: Int32,
    pub cursor_visible: bool,
}

#[repr(C)]
pub struct TextOutput {
    pub reset: extern "win64" fn(&TextOutput, bool) -> Status,
    pub output_string: extern "win64" fn(&TextOutput, WString) -> Status,
    pub test_string: extern "win64" fn(&TextOutput, WString) -> Status,
    pub query_mode: extern "win64" fn(&TextOutput, UIntN, &mut UIntN, &mut UIntN) -> Status,
    pub set_mode: extern "win64" fn(&TextOutput, UIntN) -> Status,
    pub set_attr: extern "win64" fn(&TextOutput, usize) -> Status,
    pub clear_screen: extern "win64" fn(&TextOutput) -> Status,
    pub set_cursor_position: extern "win64" fn(&TextOutput, usize, usize) -> Status,
    pub enable_cursor: extern "win64" fn(&TextOutput, bool) -> Status,
    pub mode: &'static TextOutputMode,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(u64)]
pub enum KeyCode {
    Backspace = 0,
    Tab = 1,
    Enter = 2,
    Character(char) = 3,
    Up = 4,
    Down = 5,
    Right = 6,
    Left = 7,
    Home = 8,
    End = 9,
    Insert = 10,
    Delete = 11,
    PageUp = 12,
    PageDown = 13,
    Fn1 = 14,
    Fn2 = 15,
    Fn3 = 16,
    Fn4 = 17,
    Fn5 = 18,
    Fn6 = 19,
    Fn7 = 20,
    Fn8 = 21,
    Fn9 = 22,
    Fn10 = 23,
    Fn11 = 24,
    Fn12 = 25,
    Escape = 26,
    Scancode(u16) = 27,
}

impl From<TextKey> for KeyCode {
    fn from(k: TextKey) -> Self {
        if k.code > 23 {
            KeyCode::Scancode(k.code)
        } else if k.code == 0 {
            unsafe {
                match char::from_u32_unchecked(k.unicode_char as u32) {
                    '\u{8}' => KeyCode::Backspace,
                    '\t' => KeyCode::Tab,
                    '\r' => KeyCode::Enter,
                    c => KeyCode::Character(c),
                }
            }
        } else {
            match k.code {
                1 => KeyCode::Up,
                2 => KeyCode::Down,
                3 => KeyCode::Right,
                4 => KeyCode::Left,
                5 => KeyCode::Home,
                6 => KeyCode::End,
                7 => KeyCode::Insert,
                8 => KeyCode::Delete,
                9 => KeyCode::PageUp,
                10 => KeyCode::PageDown,
                11 => KeyCode::Fn1,
                12 => KeyCode::Fn2,
                13 => KeyCode::Fn3,
                14 => KeyCode::Fn4,
                15 => KeyCode::Fn5,
                16 => KeyCode::Fn6,
                17 => KeyCode::Fn7,
                18 => KeyCode::Fn8,
                19 => KeyCode::Fn9,
                20 => KeyCode::Fn10,
                21 => KeyCode::Fn11,
                22 => KeyCode::Fn12,
                23 => KeyCode::Escape,
                _ => unreachable!(),
            }
        }
    }
}
