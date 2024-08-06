use std::io;

use calendar::{CalendarPosition, CalendarState};
use database::Database;
use editor::EditorState;
use state::State;
use title_screen::TitleScreenState;

pub mod calendar;
pub mod database;
mod editor;
mod input;
pub mod note;
mod state;
mod terminal;
mod title_screen;
mod ui;

use crate::editor::Editor;

fn main() -> io::Result<()> {
    let mut terminal = terminal::init()?;

    let database = Database::new();
    // let file_name = "notes/note.md";
    // let text = fs::read_to_string(file_name)?;
    // let temp = Editor::from_string(text);
    // let mut state = State::Editor(temp);
    let mut state = State::TitleScreen(TitleScreenState::None);

    while state != State::Exit {
        match state {
            State::TitleScreen(ref mut title_screen_state) => {
                terminal.draw(|f| ui::draw_title_screen(f))?;

                input::take_title_screen_input(title_screen_state);

                match title_screen_state {
                    TitleScreenState::None => (),
                    TitleScreenState::OpenNew => state = State::Editor(Editor::new()),
                    TitleScreenState::OpenExisting => (), //? todo
                    TitleScreenState::Calendar => {
                        state = State::Calendar(CalendarState::Browse(CalendarPosition::new()))
                    }
                    TitleScreenState::Exit => state = State::Exit,
                }
            }
            State::Editor(ref mut editor) => {
                terminal.draw(|f| ui::draw_editor(f, editor))?;
                input::take_editor_input(editor);
                if editor.state == EditorState::Exit {
                    state = State::Exit;
                }
            }
            State::Calendar(ref mut cal_state) => match cal_state {
                CalendarState::Browse(ref mut cal_position) => {
                    terminal.draw(|f: &mut ratatui::Frame| {
                        ui::calendar::draw_calendar_year(f, &f.size(), cal_position)
                    })?;
                    input::take_calendar_input(cal_state);
                }
                CalendarState::Open(date) => {
                    let note = database.get_or_create_note(&date);
                    let mut editor = Editor::from_string(note.text);
                    editor.date = *date;
                    state = State::Editor(editor);
                }
                CalendarState::Exit => state = State::Exit,
            },
            State::Exit => todo!(),
        }
    }

    terminal::restore()?;

    Ok(())
}
