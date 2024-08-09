use ratatui::{
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::title_screen::EntryPicker;

pub fn draw_entry_picker(f: &mut Frame, entry_picker: &EntryPicker) {
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

    let date = format!(
        "{}.{}.{}",
        &entry_picker.input[0..2],
        &entry_picker.input[2..4],
        &entry_picker.input[4..]
    );
    let split = entry_picker.cursor + entry_picker.cursor / 2;
    let spans = vec![
        Span::from("Enter date: "),
        Span::from(date.split_at(split).0),
        Span::from(date.chars().nth(split).unwrap().to_string())
            .black()
            .on_white(),
        Span::from(date.split_at(split + 1).1),
    ];
    let lines = vec![
        Line::from(spans),
        Line::from(""),
        Line::from("Y - Confirm  Esc - Exit"),
    ];
    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(Clear, rect);
    f.render_widget(paragraph, rect);
}
