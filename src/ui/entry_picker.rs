use ratatui::{
    layout::Rect,
    symbols::border,
    text::Line,
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

pub fn draw_entry_picker(f: &mut Frame) {
    const SIZE: (u16, u16) = (5, 25);
    let rect = Rect::new(
        (f.size().width - SIZE.1) / 2,
        (f.size().height - SIZE.0) / 2,
        SIZE.1,
        SIZE.0,
    );

    let block = Block::default()
        .borders(Borders::ALL)
        .border_set(border::ROUNDED)
        .title("Entry picker");

    let lines = vec![
        Line::from("Enter date: 00.00.00"),
        Line::from(""),
        Line::from("Y - Confirm   N - Exit"),
    ];
    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(Clear, rect);
    f.render_widget(paragraph, rect);
}
