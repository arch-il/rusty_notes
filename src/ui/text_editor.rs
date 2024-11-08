use ratatui::{
    layout::{Alignment, Margin, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Span},
    widgets::{
        block::{Position, Title},
        Block, Borders, Paragraph, Wrap,
    },
    Frame,
};

use crate::editor::{Cursor, Editor, EditorState};

pub fn draw_text_editor(f: &mut Frame, rect: &Rect, editor: &mut Editor) {
    let editor_block = Block::default()
        .borders(Borders::ALL)
        .border_set(border::ROUNDED)
        .title(editor.creation_date.format("%d/%m/%Y").to_string())
        .title(Title::from("^_^").alignment(Alignment::Right))
        .title(
            Title::from(format!(
                "{}:{}",
                editor.text.cursor.0 + 1,
                editor.text.cursor.1 + 1
            ))
            .position(Position::Bottom)
            .alignment(Alignment::Right),
        );

    let mut lines: Vec<Line> = editor
        .text
        .lines
        .clone()
        .iter()
        .map(|line| Line::from(format!("{} ", line)))
        .collect();

    if let Some(selection_start) = &editor.text.selection_start {
        highlight_selection(&mut lines, &editor.text.cursor, selection_start)
    } else {
        highlight_cursor(&mut lines, &editor.text.cursor);
    }

    if let EditorState::Search(search) = &editor.state {
        highlight_search(&mut lines, &search.text.lines[0]);
    }

    //? Line numbers are removed.
    //? Reason: Note editor doesn't need line numbers.
    //? Uncomment this line to add them?
    // add_line_numbers(&mut lines);

    let inner_rect = rect.inner(Margin {
        vertical: 1,
        horizontal: 1,
    });
    editor.screen_size = (
        inner_rect.height,
        inner_rect.width - lines.len().to_string().len() as u16,
    ); //? temp

    let paragraph = Paragraph::new(lines)
        .block(editor_block)
        .scroll(editor.scroll_offset)
        .wrap(Wrap { trim: false });
    f.render_widget(paragraph, *rect);
}

fn highlight_cursor(lines: &mut [Line], cursor: &Cursor) {
    let cursor_line = lines[cursor.0].to_string();
    let left = String::from(&cursor_line[0..cursor.1]);
    let mut cursor_str = String::from(&cursor_line[cursor.1..cursor.1 + 1]);
    let right = String::from(&cursor_line[cursor.1 + 1..]);

    // Can't render space as cursor
    if cursor_str == *" " {
        cursor_str = String::from("_");
    }

    lines[cursor.0] = Line::from(vec![
        Span::raw(left),
        Span::raw(cursor_str).black().on_white(),
        Span::raw(right),
    ]);
}

fn highlight_selection(lines: &mut [Line], cursor: &Cursor, selection_start: &Cursor) {
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
            Span::raw(left),
            Span::raw(cursor_str).black().on_light_blue(),
            Span::raw(right),
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
        lines[line_id] = Line::from(
            Span::raw(lines[line_id].to_string())
                .black()
                .on_light_blue(),
        );
    }

    let end_line = lines[end.0].to_string();
    lines[end.0] = Line::from(vec![
        Span::raw(String::from(&end_line[0..end.1]))
            .black()
            .on_light_blue(),
        Span::raw(String::from(&end_line[end.1..])),
    ]);
}

//? Uncomment this to add line numbers
// fn add_line_numbers(lines: &mut Vec<Line>) {
//     let max_width = lines.len().to_string().len() + 1;
//     *lines = lines
//         .iter_mut()
//         .enumerate()
//         .map(|(i, line)| {
//             let mut temp = vec![Span::raw(format!("{:>max_width$} ", i + 1)).bold().green()];
//             temp.append(&mut line.spans);
//             Line::from(temp)
//         })
//         .collect();
// }

fn highlight_search(lines: &mut [Line], search: &str) {
    if search.is_empty() {
        return;
    }

    for line in lines.iter_mut() {
        let mut spans = Vec::new();
        let line_str = line.to_string();
        let mut last_pos = 0;
        let mut found = false;

        while let Some(pos) = line_str[last_pos..].find(search) {
            let mut start = 0;
            let end = last_pos + pos;
            if found {
                start = last_pos + search.len() - 1;
            }

            spans.extend(get_spans_in_range(line.spans.clone(), start, end));

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
            spans.extend(get_spans_in_range(
                line.spans.clone(),
                last_pos + search.len() - 1,
                line.to_string().len(),
            ));
            line.spans = spans;
        }
    }
}

fn get_spans_in_range(from_spans: Vec<Span>, start: usize, end: usize) -> Vec<Span> {
    let mut i = 0;
    let mut len = 0;
    let mut result = Vec::new();
    loop {
        if i >= from_spans.len() {
            break;
        }
        let span_len = from_spans[i].to_string().len();
        let mut from = None;
        let mut to = None;
        if len < start {
            if len + span_len > start {
                from = Some(start - len);
            } else {
                len += span_len;
                i += 1;
                continue;
            }
        }
        if len + span_len >= end {
            to = Some(end - len);
        }
        if let Some(to) = to {
            result.push(
                Span::raw(String::from(
                    &from_spans[i].to_string()[from.unwrap_or(0)..to],
                ))
                .style(from_spans[i].style),
            );
            break;
        } else if let Some(from) = from {
            result.push(
                Span::raw(String::from(&from_spans[i].to_string()[from..]))
                    .style(from_spans[i].style),
            );
        } else {
            result.push(from_spans[i].clone());
        }

        len += span_len;
        i += 1;
    }
    result
}
