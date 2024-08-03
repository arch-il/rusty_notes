use chrono::{DateTime, Datelike, Days, Local, Months, TimeZone, Utc};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Span},
    widgets::{block::Title, Block, Borders, Paragraph, Wrap},
    Frame,
};

pub fn draw_calendar_year(f: &mut Frame, rect: &Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_set(border::ROUNDED)
        .title("Calendar")
        .title(Title::from("^_^").alignment(Alignment::Right));
    f.render_widget(block, *rect);

    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(8), Constraint::Min(8), Constraint::Min(8)])
        .split(rect.inner(Margin::new(1, 1)));

    let now = Local::now();
    let mut start = Local.with_ymd_and_hms(now.year(), 1, 1, 0, 0, 0).unwrap();

    for chunk in vertical_chunks.iter() {
        let horizontal_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Min(20),
                Constraint::Min(20),
                Constraint::Min(20),
                Constraint::Min(20),
            ])
            .split(*chunk);

        for chunk in horizontal_chunks.iter() {
            let rect = Rect::new(
                chunk.x + (chunk.width - 20) / 2,
                chunk.y + (chunk.height - 8) / 2,
                20,
                8,
            );
            let paragraph = get_month_paragraph(start);
            start = start.checked_add_months(Months::new(1)).unwrap();
            f.render_widget(paragraph, rect);
        }
    }
}

pub fn draw_calendar_month(f: &mut Frame, rect: &Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_set(border::ROUNDED)
        .title("Month")
        .title_alignment(Alignment::Left);

    let now = Local::now();

    let paragraph = get_month_paragraph(now).block(block);
    f.render_widget(paragraph, *rect);
}

fn get_month_paragraph(date: DateTime<Local>) -> Paragraph<'static> {
    let month = date.format("%B").to_string();
    let year = date.year().to_string();
    let spaces = 20 - month.len() - year.len();
    let mut lines = vec![
        Line::from(format!("{}{:>spaces$}{}", month, "", year)).blue(),
        Line::from(" M  T  W  T  F  S  S").bold().green(),
    ];

    let now = Local::now();

    let start_offset = (date.weekday() as i16 - date.day0() as i16 % 7) as usize * 3;
    let num_of_days = Utc
        .with_ymd_and_hms(date.year(), date.month(), 1, 0, 0, 0)
        .unwrap()
        .checked_add_months(Months::new(1))
        .unwrap()
        .checked_sub_days(Days::new(1))
        .unwrap()
        .day();

    let mut line = Line::from(format!("{:<start_offset$}", ""));
    for day in 0..num_of_days {
        let mut span = Span::from(format!("{:>2} ", day + 1));
        if Local
            .with_ymd_and_hms(date.year(), date.month(), day + 1, 0, 0, 0)
            .unwrap()
            .weekday() as u16
            >= 5
        {
            span = span.red();
        }

        if now.year() == date.year() && now.month0() == date.month0() && day == now.day0() {
            span = span.on_green();
        }

        line.spans.push(span);
    }
    lines.push(line);

    Paragraph::new(lines).wrap(Wrap { trim: false })
}
