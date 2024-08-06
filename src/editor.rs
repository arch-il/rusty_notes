use chrono::{DateTime, Local};
pub use cursor::Cursor;

mod copy_paste;
mod cursor;
mod editor_state;
mod scroll;
mod search;
mod text;

pub use editor_state::EditorState;
pub use search::Search;
pub use text::Text;

#[derive(Debug)]
pub struct Editor {
    pub state: EditorState,
    pub text: Text,
    pub date: DateTime<Local>,
    pub current_file: Option<String>,
    pub scroll_offset: (u16, u16),
    pub screen_size: (u16, u16),
}

#[allow(dead_code)] //?
impl Editor {
    pub fn new() -> Editor {
        Editor {
            state: EditorState::Edit,
            text: Text::new(),
            date: Local::now(),
            current_file: None,
            scroll_offset: (0, 0),
            screen_size: (0, 0),
        }
    }

    pub fn from_string(text: String) -> Editor {
        Editor {
            state: EditorState::Edit,
            text: Text::from_string(text),
            date: Local::now(),
            current_file: None,
            scroll_offset: (0, 0),
            screen_size: (0, 0),
        }
    }

    pub fn default() -> Editor {
        Editor {
            state: EditorState::Edit,
            text: Text::new(),
            date: Local::now(),
            current_file: None,
            scroll_offset: (0, 0),
            screen_size: (0, 0),
        }
    }
}
