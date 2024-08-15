use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

pub mod calendar;
mod entry_picker;
mod search;
mod side_details;
mod text_editor;
mod title_screen;

use crate::editor::Editor;
pub use entry_picker::draw_entry_picker;
pub use title_screen::draw_title_screen;

pub fn draw_editor(f: &mut Frame, editor: &mut Editor) {
    if editor.side_panel {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(1), Constraint::Length(22)])
            .split(f.size());

        match editor.state.clone() {
            crate::editor::EditorState::Edit => {
                text_editor::draw_text_editor(f, &chunks[0], editor)
            }
            crate::editor::EditorState::Search(search) => {
                let search_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Min(1), Constraint::Length(1)])
                    .split(chunks[0]);
                text_editor::draw_text_editor(f, &search_chunks[0], editor);
                search::draw_search(f, &search_chunks[1], &search);
            }
            _ => (),
        }

        let side_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(10)])
            .split(chunks[1]);

        side_details::draw_side_details(
            f,
            &side_chunks[0],
            editor.creation_date,
            editor.last_edited,
            editor
                .text
                .lines
                .iter()
                .map(|line| {
                    if line.trim() == "" {
                        0
                    } else {
                        line.trim().split(" ").count()
                    }
                })
                .sum(),
        );
        calendar::draw_calendar_month(f, &side_chunks[1]);
    } else {
        match editor.state.clone() {
            crate::editor::EditorState::Edit => text_editor::draw_text_editor(f, &f.size(), editor),
            crate::editor::EditorState::Search(search) => {
                let search_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Min(1), Constraint::Length(1)])
                    .split(f.size());
                text_editor::draw_text_editor(f, &search_chunks[0], editor);
                search::draw_search(f, &search_chunks[1], &search);
            }
            _ => (),
        }
    }
}
