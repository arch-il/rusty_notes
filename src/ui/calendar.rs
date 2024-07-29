use ratatui::{
    layout::Rect,
    symbols::border,
    widgets::{Block, Borders},
    Frame,
};

pub fn draw_calendar(f: &mut Frame, rect: &Rect) {
    let side_block = Block::default()
        .borders(Borders::ALL)
        .border_set(border::ROUNDED);

    f.render_widget(side_block, *rect);
}
