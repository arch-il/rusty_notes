pub use cursor::Cursor;

mod cursor;

pub struct Editor {
	pub lines: Vec<String>,
	pub current_file: Option<String>,
	pub cursor: Cursor,
}

#[allow(dead_code)] //?
impl Editor {
	pub fn new(lines: Vec<String>) -> Editor {
		Editor { 
			lines,
			current_file: None,
			cursor: Cursor(0, 0),
		}
	}

	pub fn from_string(text: String) -> Editor {
		let lines = text
			.split('\n')
			.map(|line| line.to_string())
			.collect::<Vec<_>>();
		Editor {
			lines,
			current_file: None,
			cursor: Cursor(0, 0),
		}
	}

	pub fn default() -> Editor {
		Editor {
			lines: Vec::new(),
			current_file: None,
			cursor: Cursor(0, 0),
		}
	}

	pub fn insert_char(&mut self, c: char) {
		self.lines[self.cursor.0].insert(self.cursor.1, c);
		self.cursor.1 += 1;
	}
}