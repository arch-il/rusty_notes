use ratatui::{
    style::Stylize,
    symbols::border,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::editor::{Cursor, Editor};

pub fn ui(f: &mut Frame, app: &Editor) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_set(border::ROUNDED)
        .title(if let Some(file_name) = &app.current_file {
            &file_name
        } else {
            ""
        });

        
        let mut lines = app
        .lines
        .clone()
        .iter()
        .map(|line| Line::from(format!("{} ", line)))
        .collect::<Vec<_>>();
    
    highlight_cursor(&mut lines, &app.cursor);
    
    add_line_numbers(&mut lines);

    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, f.size());
}

fn highlight_cursor(lines: &mut Vec<Line>, cursor: &Cursor) {
    let cursor_line = lines[cursor.0].to_string();
    let left = cursor_line.split_at(cursor.1).0;
    let cursor_str = cursor_line.chars().nth(cursor.1).unwrap().to_string();
    let right = cursor_line.split_at(cursor.1 + 1).1;

    lines[cursor.0] = Line::from(vec![
        Span::raw(String::from(left)),
        Span::raw(cursor_str).black().on_white(),
        Span::raw(String::from(right)),
    ]);
}

fn add_line_numbers(lines: &mut Vec<Line>) {
    let max_width = lines.len().to_string().len() + 1;
    *lines = lines
        .iter_mut()
        .enumerate()
        .map(|(i, line)| {
            let mut temp = vec![Span::raw(format!("{:>max_width$} ", i + 1)).green()];
            temp.append(&mut line.spans);
            Line::from(temp)
        })
        .collect();
}
