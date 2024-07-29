use ratatui::{
    layout::{Constraint, Direction, Layout, Margin},
    style::Stylize,
    symbols::border,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

mod calendar;
mod editor;

use crate::editor::{Cursor, Editor};

pub fn ui(f: &mut Frame, editor: &mut Editor) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(1), Constraint::Length(22)])
        .split(f.size());

    editor::draw_editor(f, &chunks[0], editor);

    let side_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(10),
        ])
        .split(chunks[1]);

    calendar::draw_calendar(f, &side_chunks[1]);
}
