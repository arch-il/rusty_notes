use ratatui::{
    layout::Rect,
    style::Stylize,
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::editor::Search;

pub fn draw_search(f: &mut Frame, rect: &Rect, search: &Search) {
    let line = Line::from(vec![
        Span::raw(" search: "),
        Span::raw(&search.text.lines[0]).black().on_white(),
    ]);
    let paragraph = Paragraph::new(line);
    f.render_widget(paragraph, *rect);
}
