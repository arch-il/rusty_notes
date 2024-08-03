use std::io;

use editor::EditorState;
use input::{take_editor_input, take_title_screen_input};
use state::State;
use title_screen::TitleScreenState;

mod editor;
mod input;
mod state;
mod terminal;
mod title_screen;
mod ui;

use crate::editor::Editor;

fn main() -> io::Result<()> {
    let mut terminal = terminal::init()?;

    // let file_name = "notes/note.md";
    // let text = fs::read_to_string(file_name)?;
    // let temp = Editor::from_string(text);
    // let mut state = State::Editor(temp);
    let mut state = State::TitleScreen(TitleScreenState::None);

    while state != State::Exit {
        match state {
            State::TitleScreen(ref mut title_screen_state) => {
                terminal.draw(|f| ui::draw_title_screen(f))?;

                take_title_screen_input(title_screen_state);

                match title_screen_state {
                    TitleScreenState::None => (),
                    TitleScreenState::OpenNew => state = State::Editor(Editor::new()),
                    TitleScreenState::OpenExisting => (), //? todo
                    TitleScreenState::Calendar => state = State::Calendar,
                    TitleScreenState::Exit => state = State::Exit,
                }
            }
            State::Editor(ref mut editor) => {
                terminal.draw(|f| ui::draw_editor(f, editor))?;
                take_editor_input(editor);
                if editor.state == EditorState::Exit {
                    state = State::Exit;
                }
            }
            State::Calendar => {
                terminal.draw(|f: &mut ratatui::Frame| {
                    ui::calendar::draw_calendar_year(f, &f.size())
                })?;
            }
            State::Exit => todo!(),
        }
    }

    terminal::restore()?;

    Ok(())
}
