use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::{
    calendar::CalendarState,
    editor::{Editor, EditorState, Search, Text},
    title_screen::TitleScreenState,
};

pub fn take_title_screen_input(state: &mut TitleScreenState) {
    match event::read().unwrap() {
        Event::Key(key_event) => {
            if key_event.kind == KeyEventKind::Release {
                return;
            }

            match key_event.code {
                KeyCode::Char('n') => *state = TitleScreenState::OpenNew,
                KeyCode::Char('o') => *state = TitleScreenState::OpenExisting,
                KeyCode::Char('c') => *state = TitleScreenState::Calendar,
                KeyCode::Char('q') => *state = TitleScreenState::Exit,
                KeyCode::Esc => *state = TitleScreenState::Exit,
                _ => (),
            }
        }
        _ => (),
    }
}

pub fn take_editor_input(editor: &mut Editor) {
    match event::read().unwrap() {
        Event::Key(key_event) => {
            if key_event.kind == KeyEventKind::Release {
                return;
            }

            match editor.state {
                EditorState::Edit => text_editor_input(editor, &key_event),
                EditorState::Search(_) => search_input(editor, &key_event),
                _ => (),
            }
        }
        _ => (),
    }
}

fn text_editor_input(editor: &mut Editor, key_event: &KeyEvent) {
    if key_event.modifiers.contains(KeyModifiers::CONTROL) {
        match key_event.code {
            KeyCode::Char('q') => editor.state = EditorState::Exit,

            KeyCode::Char('f') => editor.state = EditorState::Search(Search::new()),

            KeyCode::Up => editor.scroll_up(),
            KeyCode::Down => editor.scroll_down(),

            _ => text_input(&mut editor.text, key_event),
        }
    } else {
        match key_event.code {
            KeyCode::Esc => editor.state = EditorState::Exit,

            _ => text_input(&mut editor.text, key_event),
        }
    }

    editor.focus_scroll_on_cursor();
}

fn search_input(editor: &mut Editor, key_event: &KeyEvent) {
    if let EditorState::Search(ref mut search) = editor.state {
        match key_event.code {
            KeyCode::Esc => editor.state = EditorState::Edit,
            KeyCode::Enter => (),

            _ => text_input(&mut search.text, key_event),
        }
    }
}

pub fn calendar_input(state: &mut CalendarState) {
    match event::read().unwrap() {
        Event::Key(key_event) => {
            if key_event.kind == KeyEventKind::Release {
                return;
            }

            match key_event.code {
                KeyCode::Esc => *state = CalendarState::Exit,
                KeyCode::Char('q') => *state = CalendarState::Exit,

                _ => (),
            }
        }

        _ => (),
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
