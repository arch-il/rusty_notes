use ratatui::{layout::Rect, text::Line, widgets::Paragraph, Frame};

use crate::editor::{Editor, Search};

pub fn draw_search(f: &mut Frame, rect: &Rect, editor: &mut Editor, search: &Search) {
	let line = Line::from(format!(" search: {}", search.text));
	let paragraph = Paragraph::new(line);
	f.render_widget(paragraph, *rect);
}