use crossterm::event::{read, Event, KeyCode, KeyEventKind};

pub enum KeyPress {
    Left,
    Right,
    Up,
    Down,
    Enter,
    Esc,
    BackSpace,
}

/// function to get user input
/// blocks the application until user action
pub(crate) fn await_user_input() -> Option<KeyPress> {
    match read().unwrap() {
        Event::Key(event) => match event.kind {
            KeyEventKind::Press => match event.code {
                KeyCode::Up => Some(KeyPress::Up),
                KeyCode::Down => Some(KeyPress::Down),
                KeyCode::Left => Some(KeyPress::Left),
                KeyCode::Right => Some(KeyPress::Right),
                KeyCode::Enter => Some(KeyPress::Enter),
                KeyCode::Esc => Some(KeyPress::Esc),
                KeyCode::Backspace => Some(KeyPress::BackSpace),
                _ => None,
            },
            _ => None,
        },
        _ => None,
    }
}
