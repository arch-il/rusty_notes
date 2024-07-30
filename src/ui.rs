use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

mod calendar;
mod editor;
mod search;

use crate::editor::Editor;

pub fn ui(f: &mut Frame, editor: &mut Editor) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(1), Constraint::Length(22)])
        .split(f.size());

    match editor.state.clone() {
        crate::editor::State::Edit 
            => editor::draw_editor(f, &chunks[0], editor),
        crate::editor::State::Search(search) => {
            let search_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(1), Constraint::Length(1)])
                .split(chunks[0]);
            editor::draw_editor(f, &search_chunks[0], editor);
            search::draw_search(f, &search_chunks[1], editor, &search);
        },
        _ => (),
    }
    

    let side_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(10),
        ])
        .split(chunks[1]);

    calendar::draw_calendar(f, &side_chunks[1]);
}
