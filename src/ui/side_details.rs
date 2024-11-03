use chrono::{DateTime, Local, NaiveDate};
use ratatui::{
    layout::{Alignment, Rect},
    symbols::border,
    text::Line,
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub fn draw_side_details(
    f: &mut Frame,
    rect: &Rect,
    creation_date: NaiveDate,
    last_edited: DateTime<Local>,
    word_count: usize,
) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_set(border::ROUNDED)
        .title("Details")
        .title_alignment(Alignment::Left);

    let lines = vec![
        Line::from("Date:"),
        Line::from(creation_date.format("%d/%m/%Y").to_string()),
        Line::from(""),
        Line::from("Edited:"),
        Line::from(last_edited.format("%d/%m/%Y %H:%M").to_string()),
        Line::from(""),
        Line::from(format!("Words: {}", word_count)),
    ];

    let paragraph = Paragraph::new(lines)
        .wrap(Wrap { trim: false })
        .block(block);
    f.render_widget(paragraph, *rect);
}
