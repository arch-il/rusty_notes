use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::{
    calendar::CalendarState,
    editor::{Editor, EditorState, Search, Text},
    title_screen::{EntryPicker, TitleScreenState},
};

pub fn take_title_screen_input(state: &mut TitleScreenState) {
    if let Event::Key(key_event) = event::read().unwrap() {
        if key_event.kind == KeyEventKind::Release {
            return;
        }

        if let TitleScreenState::EntryPicker(ref mut entry_picker) = state {
            match key_event.code {
                KeyCode::Left => entry_picker.move_left(),
                KeyCode::Char('h') => entry_picker.move_left(),
                KeyCode::Right => entry_picker.move_right(),
                KeyCode::Char('l') => entry_picker.move_right(),

                KeyCode::Enter => {
                    if let Some(date) = entry_picker.get_date() {
                        *state = TitleScreenState::OpenOldEntry(date);
                    }
                }

                KeyCode::Esc => *state = TitleScreenState::Options,
                KeyCode::Char('q') => *state = TitleScreenState::Options,

                KeyCode::Char(c) => entry_picker.insert_char(c),

                _ => (),
            }
        } else {
            match key_event.code {
                KeyCode::Char('t') => *state = TitleScreenState::OpenTodaysEntry,
                KeyCode::Char('o') => *state = TitleScreenState::EntryPicker(EntryPicker::new()),
                KeyCode::Char('s') => {
                    if state == &TitleScreenState::Options {
                        *state = TitleScreenState::Stats;
                    } else if state == &TitleScreenState::Stats {
                        *state = TitleScreenState::Options;
                    }
                }
                KeyCode::Char('c') => *state = TitleScreenState::Calendar,
                KeyCode::Char('q') => *state = TitleScreenState::Exit,
                KeyCode::Esc => *state = TitleScreenState::Exit,
                _ => (),
            }
        }
    }
}

pub fn take_editor_input(editor: &mut Editor) {
    if let Event::Key(key_event) = event::read().unwrap() {
        if key_event.kind == KeyEventKind::Release {
            return;
        }
        if key_event.modifiers.contains(KeyModifiers::CONTROL) {
            match key_event.code {
                KeyCode::Char('w') => editor.write = true,
                KeyCode::Char('j') => editor.side_panel = !editor.side_panel,

                _ => (),
            }
        }
        match editor.state {
            EditorState::Edit => text_editor_input(editor, &key_event),
            EditorState::Search(_) => search_input(editor, &key_event),
            _ => (),
        }
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

pub fn take_calendar_input(state: &mut CalendarState) {
    if let Event::Key(key_event) = event::read().unwrap() {
        if key_event.kind == KeyEventKind::Release {
            return;
        }
        if let CalendarState::Browse(ref mut cal_position) = state {
            if cal_position.open {
                *state = CalendarState::Open(cal_position.date);
                return;
            }
            match key_event.code {
                KeyCode::Left => cal_position.move_left(),
                KeyCode::Char('h') => cal_position.move_left(),
                KeyCode::Right => cal_position.move_right(),
                KeyCode::Char('l') => cal_position.move_right(),
                KeyCode::Up => cal_position.move_up(),
                KeyCode::Char('k') => cal_position.move_up(),
                KeyCode::Down => cal_position.move_down(),
                KeyCode::Char('j') => cal_position.move_down(),

                KeyCode::Enter => cal_position.choose_selection(),
                KeyCode::Backspace => cal_position.backtrace_selection(),

                KeyCode::Esc => *state = CalendarState::Exit,
                KeyCode::Char('q') => *state = CalendarState::Exit,

                _ => (),
            }
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
