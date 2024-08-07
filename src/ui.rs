use std::iter;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::Stylize,
    text::Line,
    widgets::Paragraph,
    Frame,
};

pub mod calendar;
mod search;
mod side_details;
mod text_editor;

use crate::editor::Editor;

pub fn draw_title_screen(f: &mut Frame) {
    // let block = Block::default()
    //     .borders(Borders::ALL)
    //     .border_set(border::ROUNDED)
    //     .title(Title::from("^_^").alignment(Alignment::Right))
    //     .green();

    let title = vec![
        "",
        "",
        "",
        " ██▀███   █    ██   ██████ ▄▄▄█████▓▓██   ██▓    ███▄    █  ▒█████  ▄▄▄█████▓▓█████   ██████ ",
        "▓██ ▒ ██▒ ██  ▓██▒▒██    ▒ ▓  ██▒ ▓▒ ▒██  ██▒    ██ ▀█   █ ▒██▒  ██▒▓  ██▒ ▓▒▓█   ▀ ▒██    ▒ ",
        "▓██ ░▄█ ▒▓██  ▒██░░ ▓██▄   ▒ ▓██░ ▒░  ▒██ ██░   ▓██  ▀█ ██▒▒██░  ██▒▒ ▓██░ ▒░▒███   ░ ▓██▄   ",
        "▒██▀▀█▄  ▓▓█  ░██░  ▒   ██▒░ ▓██▓ ░   ░ ▐██▓░   ▓██▒  ▐▌██▒▒██   ██░░ ▓██▓ ░ ▒▓█  ▄   ▒   ██▒",
        "░██▓ ▒██▒▒▒█████▓ ▒██████▒▒  ▒██▒ ░   ░ ██▒▓░   ▒██░   ▓██░░ ████▓▒░  ▒██▒ ░ ░▒████▒▒██████▒▒",
        "░ ▒▓ ░▒▓░░▒▓▒ ▒ ▒ ▒ ▒▓▒ ▒ ░  ▒ ░░      ██▒▒▒    ░ ▒░   ▒ ▒ ░ ▒░▒░▒░   ▒ ░░   ░░ ▒░ ░▒ ▒▓▒ ▒ ░",
        "  ░▒ ░ ▒░░░▒░ ░ ░ ░ ░▒  ░ ░    ░     ▓██ ░▒░    ░ ░░   ░ ▒░  ░ ▒ ▒░     ░     ░ ░  ░░ ░▒  ░ ░",
        "  ░░   ░  ░░░ ░ ░ ░  ░  ░    ░       ▒ ▒ ░░        ░   ░ ░ ░ ░ ░ ▒    ░         ░   ░  ░  ░  ",
        "   ░        ░           ░            ░ ░                 ░     ░ ░              ░  ░      ░  ",
        "                                     ░ ░                                                     ",
    ];
    let mut options: Vec<&str> = iter::repeat("")
        .take(f.size().height as usize / 8)
        .collect();
    options.append(&mut vec![
        "",
        "",
        "",
        "N - Open new file     ",
        "O - Open existing file",
        "C - Open calendar     ",
        "Q or Esc - Exit       ",
    ]);
    let text: Vec<Line> = title
        .iter()
        .map(|x| Line::from(*x).red())
        .chain(options.iter().map(|x| Line::from(*x).blue()))
        .collect();
    let paragraph = Paragraph::new(text).alignment(Alignment::Center);

    f.render_widget(paragraph, f.size());
}

pub fn draw_editor(f: &mut Frame, editor: &mut Editor) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(1), Constraint::Length(22)])
        .split(f.size());

    match editor.state.clone() {
        crate::editor::EditorState::Edit => text_editor::draw_text_editor(f, &chunks[0], editor),
        crate::editor::EditorState::Search(search) => {
            let search_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(1), Constraint::Length(1)])
                .split(chunks[0]);
            text_editor::draw_text_editor(f, &search_chunks[0], editor);
            search::draw_search(f, &search_chunks[1], &search);
        }
        _ => (),
    }

    let side_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(10)])
        .split(chunks[1]);

    side_details::draw_side_details(
        f,
        &side_chunks[0],
        editor.creation_date,
        editor.last_edited,
        editor
            .text
            .lines
            .iter()
            .map(|line| {
                if line.trim() == "" {
                    0
                } else {
                    line.trim().split(" ").count()
                }
            })
            .sum(),
    );
    calendar::draw_calendar_month(f, &side_chunks[1]);
}
