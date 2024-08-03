use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{block::Title, Block, Borders, Paragraph},
    Frame,
};

mod calendar;
mod search;
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
    let options = vec![
        "",
        "",
        "",
        "O - Open existing file",
        "N - Open new file     ",
        "C - Open calendar     ",
        "X - Exit              ",
    ];
    let text = title
        .iter()
        .map(|x| Line::from(*x).red())
        .chain(options
            .iter()
            .map(|x| Line::from(*x).blue())
        )
        .collect::<Vec<_>>();
    let paragraph = Paragraph::new(text)
        .alignment(Alignment::Center);

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

    calendar::draw_calendar(f, &side_chunks[1]);
}
