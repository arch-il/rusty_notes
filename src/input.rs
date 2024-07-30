use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::editor::{Editor, Search, State};

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
	let shift = key_event.modifiers.contains(KeyModifiers::SHIFT);
			
	if key_event.modifiers.contains(KeyModifiers::CONTROL) {
		ctrl_input(editor, &key_event.code, shift);
	} else {
		normal_input(editor, &key_event.code, shift);
	}
}

fn normal_input(editor: &mut Editor, key_code: &KeyCode, shift: bool) {
	match key_code {
		KeyCode::Char(c) => editor.insert_char(*c),

		KeyCode::Enter => editor.enter(),
		KeyCode::Backspace => editor.backspace(),
		KeyCode::Delete => editor.delete(),
		
		KeyCode::Left => editor.move_left(shift),
		KeyCode::Right => editor.move_right(shift),
		KeyCode::Up => editor.move_up(shift),
		KeyCode::Down => editor.move_down(shift),

		KeyCode::Esc => editor.state = State::Exit,
		_ => (),
	}
}

fn ctrl_input(editor: &mut Editor, key_code: &KeyCode, shift: bool) {
	match key_code {
		KeyCode::Char('q') => editor.state = State::Exit,

		KeyCode::Char('c') => editor.copy(),
		KeyCode::Char('p') => editor.paste(),
		KeyCode::Char('x') => editor.cut(),

		KeyCode::Char('/') => editor.state = State::Search(Search::new()),

		KeyCode::Left => editor.move_left_word(shift),
		KeyCode::Right => editor.move_right_word(shift),
		KeyCode::Up => editor.scroll_up(),
		KeyCode::Down => editor.scroll_down(),

		_ => (),
	}
}

fn search_input(editor: &mut Editor, key_event: &KeyEvent) {
	match key_event.code {
		KeyCode::Esc => editor.state = State::Edit,
		_ => (),
	}
}
