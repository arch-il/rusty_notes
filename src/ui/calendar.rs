use chrono::{Datelike, Days, Local, Months, NaiveDate, TimeZone};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Span},
    widgets::{block::Title, Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::{calendar::CalendarPosition, database::Database};

pub fn draw_calendar_year(
    f: &mut Frame,
    rect: &Rect,
    cal_position: &mut CalendarPosition,
    database: &Database,
) {
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

    let starting_point = cal_position.date;
    let mut start = NaiveDate::from_ymd_opt(starting_point.year(), 1, 1).unwrap();

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

            let mut lines;
            match cal_position.editing {
                crate::calendar::CurrentlyEditing::Year => {
                    lines = get_month_in_lines(start); //? todo
                }
                crate::calendar::CurrentlyEditing::Month => {
                    lines = get_month_in_lines(start);
                    if start.month() == cal_position.date.month() {
                        lines = lines.iter().map(|line| line.clone().on_blue()).collect();
                    }
                }
                crate::calendar::CurrentlyEditing::Day => {
                    lines = get_month_in_lines(start);
                }
            }

            let dates: Vec<_> = database
                .get_all_notes()
                .iter()
                .map(|x| x.creation_date)
                .collect();
            highlight_dates(
                &mut lines,
                start.month(),
                start.year(),
                Some(cal_position.date),
                dates,
            );

            let paragraph = Paragraph::new(lines).wrap(Wrap { trim: false });
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

    let paragraph = Paragraph::new(get_month_in_lines(now.date_naive()))
        .wrap(Wrap { trim: false })
        .block(block);
    f.render_widget(paragraph, *rect);
}

fn get_month_in_lines(date: NaiveDate) -> Vec<Line<'static>> {
    let month = date.format("%B").to_string();
    let year = date.year().to_string();
    let spaces = 20 - month.len() - year.len();
    let mut lines = vec![
        Line::from(format!("{}{:>spaces$}{}", month, "", year)).yellow(),
        Line::from(" M  T  W  T  F  S  S").bold().green(),
    ];

    let now = Local::now();

    let start_offset = ((7 + date.weekday() as i16 - date.day0() as i16 % 7) % 7 * 3) as usize;
    let mut start = Local
        .with_ymd_and_hms(date.year(), date.month(), 1, 0, 0, 0)
        .unwrap();

    let mut line = Line::from(format!("{:<start_offset$}", ""));
    while start.month() == date.month() {
        let mut span = Span::from(format!("{:>2} ", start.day()));

        if start.weekday() as u16 >= 5 {
            span = span.red();
        }
        if start.year() == now.year() && start.month() == now.month() && start.day() == now.day() {
            span = span.on_green();
        }
        start = start.checked_add_days(Days::new(1)).unwrap();

        line.spans.push(span);
    }
    lines.push(line);
    lines
}

fn highlight_dates(
    lines: &mut Vec<Line>,
    month: u32,
    year: i32,
    cursor: Option<NaiveDate>,
    dates: Vec<NaiveDate>,
) {
    for span in lines[2].spans.iter_mut() {
        if dates.iter().any(|date| {
            date.year() == year
                && date.month() == month
                && date.day().to_string() == span.to_string().trim()
        }) {
            *span = span.clone().on_yellow();
        }
        if let Some(cursor) = cursor {
            if month == cursor.month() && span.to_string().trim() == cursor.day().to_string() {
                *span = span.clone().on_blue();
            }
        }
    }
}
