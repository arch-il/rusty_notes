pub use cursor::Cursor;

mod copy_paste;
mod cursor;
mod scroll;
mod search;
mod state;
mod text;

pub use state::State;
pub use search::Search;
use text::Text;

#[derive(Debug)]
pub struct Editor {
    pub state: State,
    pub text: Text,
    pub current_file: Option<String>,
    pub scroll_offset: (u16, u16),
    pub screen_size: (u16, u16),
}

#[allow(dead_code)] //?
impl Editor {
    pub fn new() -> Editor {
        Editor {
            state: State::Edit,
            text: Text::new(),
            current_file: None,
            scroll_offset: (0, 0),
            screen_size: (0, 0),
        }
    }

    pub fn from_string(text: String) -> Editor {
        Editor {
            state: State::Edit,
            text: Text::from_string(text),
            current_file: None,
            scroll_offset: (0, 0),
            screen_size: (0, 0),
        }
    }

    pub fn default() -> Editor {
        Editor {
            state: State::Edit,
            text: Text::new(),
            current_file: None,
            scroll_offset: (0, 0),
            screen_size: (0, 0),
        }
    }
}
