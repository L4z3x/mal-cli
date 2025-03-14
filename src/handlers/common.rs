// use crate::app::{ActiveBlock, App};
use crate::event::Key;

pub fn down_event(key: Key) -> bool {
    match key {
        Key::Down | Key::Char('j') | Key::Ctrl('n') => true,
        _ => false,
    }
}

pub fn up_event(key: Key) -> bool {
    match key {
        Key::Up | Key::Char('k') | Key::Ctrl('p') => true,
        _ => false,
    }
}

pub fn left_event(key: Key) -> bool {
    match key {
        Key::Left | Key::Char('h') | Key::Ctrl('b') => true,
        _ => false,
    }
}

pub fn right_event(key: Key) -> bool {
    match key {
        Key::Right | Key::Char('l') | Key::Ctrl('f') => true,
        _ => false,
    }
}

// pub fn high_event(key: Key) -> bool {
//     match key {
//         Key::Char('H') => true,
//         _ => false,
//     }
// }

// pub fn middle_event(key: Key) -> bool {
//     match key {
//         Key::Char('M') => true,
//         _ => false,
//     }
// }

// pub fn low_event(key: Key) -> bool {
//     match key {
//         Key::Char('L') => true,
//         _ => false,
//     }
// }

pub fn on_down_press<T>(selection_data: &[T], selection_index: Option<usize>) -> usize {
    match selection_index {
        Some(selection_index) => {
            if !selection_data.is_empty() {
                let next_index = selection_index + 1;
                if next_index > selection_data.len() - 1 {
                    return 0;
                } else {
                    return next_index;
                }
            }
            0
        }
        None => 0,
    }
}

pub fn on_up_press<T>(selection_data: &[T], selection_index: Option<usize>) -> usize {
    match selection_index {
        Some(selection_index) => {
            if !selection_data.is_empty() {
                if selection_index > 0 {
                    return selection_index - 1;
                } else {
                    return selection_data.len() - 1;
                }
            }
            0
        }
        None => 0,
    }
}

// pub fn on_high_press() -> usize {
//     0
// }

// pub fn on_middle_press<T>(selection_data: &[T]) -> usize {
//     let mut index = selection_data.len() / 2;
//     if selection_data.len() % 2 == 0 {
//         index -= 1
//     }
//     index
// }

// pub fn on_low_press<T>(selection_data: &[T]) -> usize {
//     selection_data.len() - 1
// }
