use std::{fs, io};

use editor::EditorState;
use input::take_input;
use state::State;

mod editor;
mod input;
mod state;
mod terminal;
mod ui;

use crate::editor::Editor;

fn main() -> io::Result<()> {
    let mut terminal = terminal::init()?;
    
    let file_name = "notes/note.md";
    let text = fs::read_to_string(file_name)?;

    // let temp = Editor::from_string(text);
    // let mut state = State::Editor(temp);
    let mut state = State::TitleScreen;
    
    while state != State::Exit {
        match state {
            State::TitleScreen => {
                terminal.draw(|f| ui::draw_title_screen(f))?;
            },
            State::Editor(ref mut editor) => {
                terminal.draw(|f| ui::draw_editor(f, editor))?;
                take_input(editor);
                if editor.state == EditorState::Exit {
                    state = State::Exit;
                }
            },
            State::Exit => todo!(),
        }
    }

    terminal::restore()?;

    Ok(())
}
