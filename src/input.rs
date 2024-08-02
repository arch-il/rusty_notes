use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::editor::{Editor, Search, State, Text};

pub fn take_input(editor: &mut Editor) {
    match event::read().unwrap() {
        Event::Key(key_event) => {
            if key_event.kind == KeyEventKind::Release {
                return;
            }

            match editor.state {
                State::Edit => editor_input(editor, &key_event),
                State::Search(_) => search_input(editor, &key_event),
                _ => (),
            }
        }
        _ => (),
    }
}

fn editor_input(editor: &mut Editor, key_event: &KeyEvent) {
    if key_event.modifiers.contains(KeyModifiers::CONTROL) {
        match key_event.code {
            KeyCode::Char('q') => editor.state = State::Exit,

            KeyCode::Char('f') => editor.state = State::Search(Search::new()),

            KeyCode::Up => editor.scroll_up(),
            KeyCode::Down => editor.scroll_down(),

            _ => text_input(&mut editor.text, key_event),
        }
    } else {
        match key_event.code {
            KeyCode::Esc => editor.state = State::Exit,

            _ => text_input(&mut editor.text, key_event),
        }
    }

    editor.focus_scroll_on_cursor();
}

fn search_input(editor: &mut Editor, key_event: &KeyEvent) {
    if let State::Search(ref mut search) = editor.state {
        match key_event.code {
            KeyCode::Esc => editor.state = State::Edit,
            KeyCode::Enter => (),

            _ => text_input(&mut search.text, key_event),
        }
    }
}

fn text_input(text: &mut Text, key_event: &KeyEvent) {
    let shift = key_event.modifiers.contains(KeyModifiers::SHIFT);

    if key_event.modifiers.contains(KeyModifiers::CONTROL) {
        match key_event.code {
            KeyCode::Char('c') => text.copy(),
            KeyCode::Char('p') => text.paste(),
            KeyCode::Char('x') => text.cut(),

            KeyCode::Backspace => text.backspace_word(),
            KeyCode::Delete => text.delete_word(),

            KeyCode::Left => text.move_left_word(shift),
            KeyCode::Right => text.move_right_word(shift),

            _ => (),
        }
    } else {
        match key_event.code {
            KeyCode::Char(c) => text.insert_char(c),

            KeyCode::Enter => text.enter(),
            KeyCode::Backspace => text.backspace(),
            KeyCode::Delete => text.delete(),

            KeyCode::Left => text.move_left(shift),
            KeyCode::Right => text.move_right(shift),
            KeyCode::Up => text.move_up(shift),
            KeyCode::Down => text.move_down(shift),

            _ => (),
        }
    }
}
