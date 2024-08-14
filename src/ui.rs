use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Stylize,
    text::Line,
    widgets::Paragraph,
    Frame,
};

pub mod calendar;
mod entry_picker;
mod search;
mod side_details;
mod text_editor;

use crate::editor::Editor;
pub use entry_picker::draw_entry_picker;

pub fn draw_title_screen(f: &mut Frame) {
    // let block = Block::default()
    //     .borders(Borders::ALL)
    //     .border_set(border::ROUNDED)
    //     .title(Title::from("^_^").alignment(Alignment::Right))
    //     .green();
    const TITLE_SIZE: (u16, u16) = (10, 93);
    let title_text = vec![
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

    const OPTIONS_SIZE: (u16, u16) = (4, 22);
    let options_text = vec![
        "T - Open today's entry",
        "O - Open old entry    ",
        "C - Open calendar     ",
        "Q or Esc - Exit       ",
    ];

    let title_rect = Rect::new(
        (f.size().width - TITLE_SIZE.1) / 2,
        (f.size().height - TITLE_SIZE.0) / 5,
        TITLE_SIZE.1,
        TITLE_SIZE.0,
    );

    let options_rect = Rect::new(
        (f.size().width - OPTIONS_SIZE.1) / 2,
        (f.size().height - OPTIONS_SIZE.0) * 3 / 5,
        OPTIONS_SIZE.1,
        OPTIONS_SIZE.0,
    );

    let title = Paragraph::new(
        title_text
            .iter()
            .map(|x| Line::raw(String::from(*x)))
            .collect::<Vec<_>>(),
    )
    .red()
    .alignment(Alignment::Center);

    let options = Paragraph::new(
        options_text
            .iter()
            .map(|x| Line::raw(String::from(*x)))
            .collect::<Vec<_>>(),
    )
    .blue()
    .alignment(Alignment::Center);

    f.render_widget(title, title_rect);
    f.render_widget(options, options_rect);
}

pub fn draw_editor(f: &mut Frame, editor: &mut Editor) {
    if editor.side_panel {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(1), Constraint::Length(22)])
            .split(f.size());

        match editor.state.clone() {
            crate::editor::EditorState::Edit => {
                text_editor::draw_text_editor(f, &chunks[0], editor)
            }
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
    } else {
        match editor.state.clone() {
            crate::editor::EditorState::Edit => text_editor::draw_text_editor(f, &f.size(), editor),
            crate::editor::EditorState::Search(search) => {
                let search_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Min(1), Constraint::Length(1)])
                    .split(f.size());
                text_editor::draw_text_editor(f, &search_chunks[0], editor);
                search::draw_search(f, &search_chunks[1], &search);
            }
            _ => (),
        }
    }
}
