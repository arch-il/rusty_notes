use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};

use crate::editor::{Editor, State};

pub fn take_input(editor: &mut Editor) {
	match event::read().unwrap() {
		Event::Key(key_event) => {
			if key_event.kind == KeyEventKind::Release {
				return;
			}
			
			let shift = key_event.modifiers.contains(KeyModifiers::SHIFT);
			
			if key_event.modifiers.contains(KeyModifiers::CONTROL) {
				ctrl_input(editor, &key_event.code, shift);
			} else {
				normal_input(editor, &key_event.code, shift);
			}

		} 
		_ => (),
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

		KeyCode::Left => editor.move_left_word(shift),
		KeyCode::Right => editor.move_right_word(shift),
		KeyCode::Up => editor.scroll_up(),
		KeyCode::Down => editor.scroll_down(),

		_ => (),
	}
}