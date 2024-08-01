use ratatui::{
    layout::{Margin, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::editor::{Cursor, Editor, State};

pub fn draw_editor(f: &mut Frame, rect: &Rect, editor: &mut Editor) {
    let editor_block = Block::default()
        .borders(Borders::ALL)
        .border_set(border::ROUNDED)
        .title(if let Some(file_name) = &editor.current_file {
            &file_name
        } else {
            ""
        });

    let mut lines = editor
        .text
        .lines
        .clone()
        .iter()
        .map(|line| Line::from(format!("{} ", line)))
        .collect::<Vec<_>>();

    if let Some(selection_start) = &editor.text.selection_start {
        highlight_selection(&mut lines, &editor.text.cursor, selection_start)
    } else {
        highlight_cursor(&mut lines, &editor.text.cursor);
    }

    if let State::Search(search) = &editor.state {
        highlight_search(&mut lines, &search.text.lines[0]);
    }

    add_line_numbers(&mut lines);

    let inner_rect = rect.inner(Margin {
        //?
        vertical: 1,
        horizontal: 1,
    });
    editor.screen_size = (
        inner_rect.height,
        inner_rect.width - 2 - lines.len().to_string().len() as u16,
    ); //? temp

    let paragraph = Paragraph::new(lines)
        .block(editor_block)
        .scroll(editor.scroll_offset);
    f.render_widget(paragraph, *rect);
}

fn highlight_cursor(lines: &mut Vec<Line>, cursor: &Cursor) {
    let cursor_line = lines[cursor.0].to_string();
    let left = String::from(&cursor_line[0..cursor.1]);
    let cursor_str = String::from(&cursor_line[cursor.1..cursor.1 + 1]);
    let right = String::from(&cursor_line[cursor.1 + 1..]);

    lines[cursor.0] = Line::from(vec![
        Span::raw(String::from(left)),
        Span::raw(cursor_str).black().on_white(),
        Span::raw(String::from(right)),
    ]);
}

fn highlight_selection(lines: &mut Vec<Line>, cursor: &Cursor, selection_start: &Cursor) {
    let mut start = cursor;
    let mut end = selection_start;
    if cursor > selection_start {
        start = selection_start;
        end = cursor;
    }

    if start.0 == end.0 {
        let cursor_line = lines[start.0].to_string();
        let left = String::from(&cursor_line[0..start.1]);
        let cursor_str = String::from(&cursor_line[start.1..end.1 + 1]);
        let right = String::from(&cursor_line[end.1 + 1..]);

        lines[start.0] = Line::from(vec![
            Span::raw(String::from(left)),
            Span::raw(cursor_str).black().on_light_blue(),
            Span::raw(String::from(right)),
        ]);
        return;
    }

    let start_line = lines[start.0].to_string();
    lines[start.0] = Line::from(vec![
        Span::raw(String::from(&start_line[0..start.1])),
        Span::raw(String::from(&start_line[start.1..]))
            .black()
            .on_light_blue(),
    ]);

    for line_id in (start.0..end.0).skip(1) {
        lines[line_id] = Line::from(Span::raw(lines[line_id].to_string()).black().on_light_blue());
    }

    let end_line = lines[end.0].to_string();
    lines[end.0] = Line::from(vec![
        Span::raw(String::from(&end_line[0..end.1]))
            .black()
            .on_light_blue(),
        Span::raw(String::from(&end_line[end.1..])),
    ]);
}

fn add_line_numbers(lines: &mut Vec<Line>) {
    let max_width = lines.len().to_string().len() + 1;
    *lines = lines
        .iter_mut()
        .enumerate()
        .map(|(i, line)| {
            let mut temp = vec![Span::raw(format!("{:>max_width$} ", i + 1)).bold().green()];
            temp.append(&mut line.spans);
            Line::from(temp)
        })
        .collect();
}

fn highlight_search(lines: &mut Vec<Line>, search: &str) {
    if search.len() == 0 {
        return;
    }
    for line in lines.iter_mut() {
        let mut spans = Vec::new();
        let line_str = line.to_string();
        let mut last_pos = 0;
        let mut found = false;
        while let Some(pos) = line_str[last_pos..].find(search) {
            if found {
                spans.push(Span::raw(String::from(
                    &line_str[(last_pos + search.len() - 1)..(last_pos + pos)],
                )));
            } else {
                spans.push(Span::raw(String::from(
                    &line_str[0..(last_pos + pos)],
                )));
            }
            spans.push(
                Span::raw(String::from(
                    &line_str[(last_pos + pos)..(last_pos + pos + search.len())],
                ))
                .black()
                .on_green(),
            );

            last_pos += pos + 1;
            found = true;
        }
        if found {
            spans.push(Span::raw(String::from(
                &line_str[(last_pos + search.len() - 1)..],
            )));
            line.spans = spans;
        }
    }
}
