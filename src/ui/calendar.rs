use chrono::{Datelike, Days, Local, Months, TimeZone, Utc};
use ratatui::{
    layout::{Alignment, Rect},
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub fn draw_calendar(f: &mut Frame, rect: &Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_set(border::ROUNDED)
        .title("Month")
        .title_alignment(Alignment::Left);

    let now = Local::now();

    let month = now.format("%B").to_string();
    let year = now.year().to_string();
    let spaces = rect.width as usize - 2 - month.len() - year.len();
    let mut lines = vec![
        Line::from(format!("{}{:>spaces$}{}", month, "", year)),
        Line::from(" M  T  W  T  F  S  S").bold().green(),
    ];

    let start_offset = (now.weekday() as i16 - now.day0() as i16 % 7) as usize;
    let num_of_days = Utc
        .with_ymd_and_hms(now.year(), now.month(), 1, 0, 0, 0)
        .unwrap()
        .checked_add_months(Months::new(1))
        .unwrap()
        .checked_sub_days(Days::new(1))
        .unwrap()
        .day();

    let mut line = format!("{:<start_offset$}", "");
    for day in 0..num_of_days {
        line += &format!("{:>2} ", day + 1);
    }
    lines.push(Line::from(line));

    let paragraph = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false });
    f.render_widget(paragraph, *rect);
}
