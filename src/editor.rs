use chrono::{DateTime, Local, NaiveDate};
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
    pub creation_date: NaiveDate,
    pub last_edited: DateTime<Local>,
    pub scroll_offset: (u16, u16),
    pub screen_size: (u16, u16),
    pub write: bool,
    pub side_panel: bool,
}

#[allow(dead_code)]
impl Editor {
    pub fn new() -> Editor {
        let now = Local::now();
        Editor {
            state: EditorState::Edit,
            text: Text::new(),
            creation_date: now.date_naive(),
            last_edited: now,
            scroll_offset: (0, 0),
            screen_size: (0, 0),
            write: false,
            side_panel: true,
        }
    }

    pub fn from_string(text: String) -> Editor {
        let now = Local::now();
        Editor {
            state: EditorState::Edit,
            text: Text::from_string(text),
            creation_date: now.date_naive(),
            last_edited: now,
            scroll_offset: (0, 0),
            screen_size: (0, 0),
            write: false,
            side_panel: true,
        }
    }

    pub fn default() -> Editor {
        let now = Local::now();
        Editor {
            state: EditorState::Edit,
            text: Text::new(),
            creation_date: now.date_naive(),
            last_edited: now,
            scroll_offset: (0, 0),
            screen_size: (0, 0),
            write: false,
            side_panel: true,
        }
    }
}
