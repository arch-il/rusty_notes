use ratatui::{
    layout::{Alignment, Rect},
    style::Stylize,
    text::Line,
    widgets::Paragraph,
    Frame,
};

use crate::{database::Database, title_screen::TitleScreenState};

pub fn draw_title_screen(f: &mut Frame, state: &TitleScreenState, database: &Database) {
    // let block = Block::default()
    //     .borders(Borders::ALL)
    //     .border_set(border::ROUNDED)
    //     .title(Title::from("^_^").alignment(Alignment::Right))
    //     .green();

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

    let some_rect;
    let some_text;

    if state == &TitleScreenState::Stats {
        let notes = database.get_all_notes();
        let num_notes = notes.len();
        let num_words = notes.iter().fold(0, |acc, note| {
            acc + note.text.split("\n").into_iter().fold(0, |acc, line| {
                acc + if line.trim().len() == 0 {
                    0
                } else {
                    line.split(" ").count()
                }
            })
        });
        some_text = vec![
            format!("Days:    {:>4}", num_notes),
            format!("Words:   {:>4}", num_words),
            String::from("Streak:     3"), //? todo
            String::from("Max streak: 3"), //? todo
            String::new(),
            String::from("S - Go back  "),
        ];

        const STATS_SIZE: (u16, u16) = (7, 13);
        some_rect = Rect::new(
            (f.size().width - STATS_SIZE.1) / 2,
            (f.size().height - STATS_SIZE.0) * 3 / 5,
            STATS_SIZE.1,
            STATS_SIZE.0,
        );
    } else if state == &TitleScreenState::Options {
        some_text = vec![
            String::from("T - Open today's entry"),
            String::from("O - Open old entry    "),
            String::from("C - Open calendar     "),
            String::from("S - View stats        "),
            String::from("Q or Esc - Exit       "),
        ];

        const OPTIONS_SIZE: (u16, u16) = (7, 22);
        some_rect = Rect::new(
            (f.size().width - OPTIONS_SIZE.1) / 2,
            (f.size().height - OPTIONS_SIZE.0) * 3 / 5,
            OPTIONS_SIZE.1,
            OPTIONS_SIZE.0,
        );
    } else {
        some_text = vec![String::new()];
        some_rect = Rect::new(0, 0, 0, 0);
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

    let some_paragraph = Paragraph::new(
        some_text
            .iter()
            .map(|x| Line::raw(String::from(x)))
            .collect::<Vec<_>>(),
    )
    .blue()
    .alignment(Alignment::Center);

    f.render_widget(title, title_rect);
    f.render_widget(some_paragraph, some_rect);
}
