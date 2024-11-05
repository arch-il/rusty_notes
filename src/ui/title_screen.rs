use chrono::{Days, Local, NaiveDate};
use ratatui::{
    layout::{Alignment, Rect},
    style::Stylize,
    text::Line,
    widgets::Paragraph,
    Frame,
};

use crate::{database::Database, title_screen::TitleScreenState};

pub fn draw_title_screen(f: &mut Frame, state: &TitleScreenState, database: &Database) {
    const TITLE_SIZE: (u16, u16) = (10, 93);
    let title_text = vec![
        String::from(" ██▀███   █    ██   ██████ ▄▄▄█████▓▓██   ██▓    ███▄    █  ▒█████  ▄▄▄█████▓▓█████   ██████ "),
        String::from("▓██ ▒ ██▒ ██  ▓██▒▒██    ▒ ▓  ██▒ ▓▒ ▒██  ██▒    ██ ▀█   █ ▒██▒  ██▒▓  ██▒ ▓▒▓█   ▀ ▒██    ▒ "),
        String::from("▓██ ░▄█ ▒▓██  ▒██░░ ▓██▄   ▒ ▓██░ ▒░  ▒██ ██░   ▓██  ▀█ ██▒▒██░  ██▒▒ ▓██░ ▒░▒███   ░ ▓██▄   "),
        String::from("▒██▀▀█▄  ▓▓█  ░██░  ▒   ██▒░ ▓██▓ ░   ░ ▐██▓░   ▓██▒  ▐▌██▒▒██   ██░░ ▓██▓ ░ ▒▓█  ▄   ▒   ██▒"),
        String::from("░██▓ ▒██▒▒▒█████▓ ▒██████▒▒  ▒██▒ ░   ░ ██▒▓░   ▒██░   ▓██░░ ████▓▒░  ▒██▒ ░ ░▒████▒▒██████▒▒"),
        String::from("░ ▒▓ ░▒▓░░▒▓▒ ▒ ▒ ▒ ▒▓▒ ▒ ░  ▒ ░░      ██▒▒▒    ░ ▒░   ▒ ▒ ░ ▒░▒░▒░   ▒ ░░   ░░ ▒░ ░▒ ▒▓▒ ▒ ░"),
        String::from("  ░▒ ░ ▒░░░▒░ ░ ░ ░ ░▒  ░ ░    ░     ▓██ ░▒░    ░ ░░   ░ ▒░  ░ ▒ ▒░     ░     ░ ░  ░░ ░▒  ░ ░"),
        String::from("  ░░   ░  ░░░ ░ ░ ░  ░  ░    ░       ▒ ▒ ░░        ░   ░ ░ ░ ░ ░ ▒    ░         ░   ░  ░  ░  "),
        String::from("   ░        ░           ░            ░ ░                 ░     ░ ░              ░  ░      ░  "),
        String::from("                                     ░ ░                                                     "),
    ];

    let rect;
    let text;

    match state {
        TitleScreenState::Stats => (text, rect) = get_stats_data(f, database),
        TitleScreenState::Options => (text, rect) = get_options_data(f),
        _ => {
            text = vec![String::new()];
            rect = Rect::new(0, 0, 0, 0);
        }
    }

    let title_rect = Rect::new(
        (f.size().width - TITLE_SIZE.1) / 2,
        (f.size().height - TITLE_SIZE.0) / 5,
        TITLE_SIZE.1,
        TITLE_SIZE.0,
    );

    let title = Paragraph::new(
        title_text
            .iter()
            .map(|x| Line::raw(String::from(x)))
            .collect::<Vec<_>>(),
    )
    .red()
    .alignment(Alignment::Center);

    let paragraph = Paragraph::new(
        text.iter()
            .map(|x| Line::raw(String::from(x)))
            .collect::<Vec<_>>(),
    )
    .blue()
    .alignment(Alignment::Center);

    f.render_widget(title, title_rect);
    f.render_widget(paragraph, rect);
}

fn get_stats_data(f: &mut Frame, database: &Database) -> (Vec<String>, Rect) {
    let mut notes = database.get_all_notes();
    let num_notes = notes.len();
    let num_words = notes.iter().fold(0, |acc, note| {
        acc + note.text.split("\n").fold(0, |acc, line| {
            acc + if line.trim().is_empty() {
                0
            } else {
                line.split(" ").count()
            }
        })
    });

    notes.sort_unstable_by(|left, right| left.creation_date.cmp(&right.creation_date));

    let mut max_streak = 0;
    let mut temp_streak = 0;
    let mut curr_streak = 0;

    let mut temp_date: NaiveDate = match notes.first() {
        Some(note) => note.creation_date,
        None => NaiveDate::default(),
    };

    for date in notes.iter().map(|note| note.creation_date) {
        if temp_date == date {
            temp_streak += 1;
            temp_date = temp_date.checked_add_days(Days::new(1)).unwrap();

            if temp_streak > max_streak {
                max_streak = temp_streak;
            }

            if date == Local::now().date_naive() {
                curr_streak = temp_streak;
            }
        } else {
            temp_date = date.checked_add_days(Days::new(1)).unwrap();
            temp_streak = 1;
        }
    }

    let text = vec![
        format!("Days Entered:   {:>4}", num_notes),
        format!("Total Words:    {:>4}", num_words),
        format!("Current Streak: {:>4}", curr_streak),
        format!("Max Streak:     {:>4}", max_streak),
        String::new(),
        String::from("S - Go back  "),
    ];

    const STATS_SIZE: (u16, u16) = (7, 20);
    let rect = Rect::new(
        (f.size().width - STATS_SIZE.1) / 2,
        (f.size().height - STATS_SIZE.0) * 3 / 5,
        STATS_SIZE.1,
        STATS_SIZE.0,
    );

    (text, rect)
}

fn get_options_data(f: &mut Frame) -> (Vec<String>, Rect) {
    let text = vec![
        String::from("T - Open today's entry"),
        String::from("O - Open old entry    "),
        String::from("C - Open calendar     "),
        String::from("S - View stats        "),
        String::from("Q or Esc - Exit       "),
    ];

    const OPTIONS_SIZE: (u16, u16) = (7, 22);
    let rect = Rect::new(
        (f.size().width - OPTIONS_SIZE.1) / 2,
        (f.size().height - OPTIONS_SIZE.0) * 3 / 5,
        OPTIONS_SIZE.1,
        OPTIONS_SIZE.0,
    );

    (text, rect)
}
