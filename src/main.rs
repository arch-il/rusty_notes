use std::io;

use calendar::{CalendarPosition, CalendarState};
use chrono::Local;
use database::Database;
use editor::EditorState;
use note::Note;
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
    let mut state = State::TitleScreen(TitleScreenState::Options);

    while state != State::Exit {
        match state {
            State::TitleScreen(ref mut title_screen_state) => {
                input::take_title_screen_input(title_screen_state);
                let mut entry_picker = None;
                let temp_state = title_screen_state.clone();

                match title_screen_state {
                    TitleScreenState::OpenTodaysEntry => state = State::Editor(Editor::new()),
                    TitleScreenState::EntryPicker(picker) => entry_picker = Some(picker.clone()),
                    TitleScreenState::OpenOldEntry(ref date) => {
                        let note = database.get_or_create_note(date);
                        let mut editor = Editor::from_string(note.text);
                        editor.creation_date = note.creation_date;
                        editor.last_edited = note.last_edited;
                        state = State::Editor(editor);
                    }
                    TitleScreenState::Calendar => {
                        state = State::Calendar(CalendarState::Browse(CalendarPosition::new()))
                    }
                    TitleScreenState::Exit => state = State::Exit,
                    _ => (),
                }

                terminal.draw(|f| {
                    ui::draw_title_screen(f, &temp_state, &database);
                    if let Some(entry_picker) = entry_picker {
                        ui::draw_entry_picker(f, &entry_picker);
                    }
                })?;
            }
            State::Editor(ref mut editor) => {
                terminal.draw(|f| ui::draw_editor(f, editor))?;
                input::take_editor_input(editor);
                if editor.write {
                    database.insert_or_create_note(&Note {
                        id: 0,
                        text: editor.text.lines.join("\n"),
                        creation_date: editor.creation_date,
                        last_edited: Local::now(),
                    });
                }
                if editor.state == EditorState::Exit {
                    state = State::TitleScreen(TitleScreenState::Options);
                }
            }
            State::Calendar(ref mut cal_state) => match cal_state {
                CalendarState::Browse(ref mut cal_position) => {
                    terminal.draw(|f: &mut ratatui::Frame| {
                        ui::calendar::draw_calendar_year(f, &f.size(), cal_position, &database)
                    })?;
                    input::take_calendar_input(cal_state);
                }
                CalendarState::Open(date) => {
                    let note = database.get_or_create_note(&date);
                    let mut editor = Editor::from_string(note.text);
                    editor.creation_date = note.creation_date;
                    editor.last_edited = note.last_edited;
                    state = State::Editor(editor);
                }
                CalendarState::Exit => state = State::TitleScreen(TitleScreenState::Options),
            },
            State::Exit => todo!(),
        }
    }

    terminal::restore()?;

    Ok(())
}
