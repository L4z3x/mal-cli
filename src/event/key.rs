use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use serde::{Deserialize, Serialize};

/// Represends a Key Press
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Key {
    Enter,
    Tab,
    BackTab,
    Backspace,
    Esc,
    Left,
    Right,
    Up,
    Down,
    Ins,
    Delete,
    Home,
    PageUp,
    PageDown,
    Char(char),
    Ctrl(char),
    Alt(char),
    Unknown,
    F0,
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
}

impl Key {
    /// Returns the function key corresponding to the given number
    ///
    /// 1 -> F1, etc...
    ///
    /// # Panics
    ///
    /// If `n == 0 || n > 12`
    pub fn from_f(n: u8) -> Key {
        match n {
            0 => Key::F0,
            1 => Key::F1,
            2 => Key::F2,
            3 => Key::F3,
            4 => Key::F4,
            5 => Key::F5,
            6 => Key::F6,
            7 => Key::F7,
            8 => Key::F8,
            9 => Key::F9,
            10 => Key::F10,
            11 => Key::F11,
            12 => Key::F12,
            _ => panic!("unknown function key: F{}", n),
        }
    }
}

impl From<KeyEvent> for Key {
    fn from(key_event: KeyEvent) -> Self {
        match key_event {
            KeyEvent {
                code: KeyCode::Enter,
                ..
            } => Key::Enter,
            KeyEvent {
                code: KeyCode::Tab, ..
            } => Key::Tab,
            KeyEvent {
                code: KeyCode::BackTab,
                ..
            } => Key::BackTab,
            KeyEvent {
                code: KeyCode::Backspace,
                ..
            } => Key::Backspace,
            KeyEvent {
                code: KeyCode::Esc, ..
            } => Key::Esc,
            KeyEvent {
                code: KeyCode::Left,
                ..
            } => Key::Left,
            KeyEvent {
                code: KeyCode::Right,
                ..
            } => Key::Right,
            KeyEvent {
                code: KeyCode::Up, ..
            } => Key::Up,
            KeyEvent {
                code: KeyCode::Down,
                ..
            } => Key::Down,
            KeyEvent {
                code: KeyCode::Insert,
                ..
            } => Key::Ins,
            KeyEvent {
                code: KeyCode::Delete,
                ..
            } => Key::Delete,
            KeyEvent {
                code: KeyCode::Home,
                ..
            } => Key::Home,
            KeyEvent {
                code: KeyCode::PageUp,
                ..
            } => Key::PageUp,
            KeyEvent {
                code: KeyCode::PageDown,
                ..
            } => Key::PageDown,
            KeyEvent {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::ALT,
                ..
            } => Key::Alt(c),
            KeyEvent {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => Key::Ctrl(c),
            KeyEvent {
                code: KeyCode::Char(c),
                ..
            } => Key::Char(c),
            _ => Key::Unknown,
        }
    }
}
