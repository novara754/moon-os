use circbuf::CircBuf;

pub static mut KEYBOARD: Keyboard = Keyboard::new();

const KEYMAP: [Key; 86] = [
    Key::Character(0),
    Key::Esc,
    Key::Character(b'1'),
    Key::Character(b'2'),
    Key::Character(b'3'),
    Key::Character(b'4'),
    Key::Character(b'5'),
    Key::Character(b'6'),
    Key::Character(b'7'),
    Key::Character(b'8'),
    Key::Character(b'9'),
    Key::Character(b'0'),
    Key::Character(b'-'),
    Key::Character(b'='),
    Key::Backspace,
    Key::Tab,
    Key::Character(b'q'),
    Key::Character(b'w'),
    Key::Character(b'e'),
    Key::Character(b'r'),
    Key::Character(b't'),
    Key::Character(b'y'),
    Key::Character(b'u'),
    Key::Character(b'i'),
    Key::Character(b'o'),
    Key::Character(b'p'),
    Key::Character(b'['),
    Key::Character(b']'),
    Key::Enter,
    Key::LCtrl,
    Key::Character(b'a'),
    Key::Character(b's'),
    Key::Character(b'd'),
    Key::Character(b'f'),
    Key::Character(b'g'),
    Key::Character(b'h'),
    Key::Character(b'j'),
    Key::Character(b'k'),
    Key::Character(b'l'),
    Key::Character(b';'),
    Key::Character(b'\\'),
    Key::Character(b'`'),
    Key::LShift,
    Key::Character(b'\\'),
    Key::Character(b'z'),
    Key::Character(b'x'),
    Key::Character(b'c'),
    Key::Character(b'v'),
    Key::Character(b'b'),
    Key::Character(b'n'),
    Key::Character(b'm'),
    Key::Character(b','),
    Key::Character(b'.'),
    Key::Character(b'/'),
    Key::RShift,
    Key::Character(b'*'),
    Key::LAlt,
    Key::Character(b' '),
    Key::Caps,
    Key::F1,
    Key::F2,
    Key::F3,
    Key::F4,
    Key::F5,
    Key::F6,
    Key::F7,
    Key::F8,
    Key::F9,
    Key::F10,
    Key::NumLock,
    Key::ScrollLock,
    Key::Character(b'7'),
    Key::Character(b'8'),
    Key::Character(b'9'),
    Key::Character(b'-'),
    Key::Character(b'4'),
    Key::Character(b'5'),
    Key::Character(b'6'),
    Key::Character(b'+'),
    Key::Character(b'1'),
    Key::Character(b'2'),
    Key::Character(b'3'),
    Key::Character(b'0'),
    Key::Character(b'.'),
    Key::F11,
    Key::F12,
];

#[derive(Debug, Clone, Copy)]
pub enum Key {
    Character(u8),
    Esc,
    Backspace,
    Tab,
    Enter,
    LCtrl,
    // RCtrl,
    LShift,
    RShift,
    LAlt,
    // RAlt,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    NumLock,
    ScrollLock,
    Caps,
}

impl Key {
    pub fn as_ascii(self) -> u8 {
        use Key::*;
        match self {
            Character(c) => c,
            Enter => b'\n',
            _ => panic!("tried to convert Key::{:?} to ascii", self),
        }
    }
}

pub struct Keyboard {
    key_buffer: CircBuf<Key, 64>,
    shift_held: bool,
}

impl Keyboard {
    const fn new() -> Self {
        Self {
            key_buffer: CircBuf::new(),
            shift_held: false,
        }
    }

    pub fn push_keycode(&mut self, code: u8) {
        match code {
            0x2A | 0x36 => self.shift_held = true,
            0xAA | 0xB6 => self.shift_held = false,
            _ if (code as usize) < KEYMAP.len() => {
                self.key_buffer.push(self.translate(KEYMAP[code as usize]))
            }
            _ => {}
        }
    }

    fn translate(&self, key: Key) -> Key {
        if !self.shift_held {
            return key;
        }

        if let Key::Character(c) = key {
            if c.is_ascii_digit() {
                return Key::Character(b")!@#$%^*("[(c - b'0') as usize]);
            }

            todo!()
        } else {
            key
        }
    }

    pub fn get_key(&mut self) -> Key {
        while self.key_buffer.is_empty() {}
        unsafe { self.key_buffer.pop().unwrap_unchecked() }
    }
}
