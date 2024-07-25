pub struct Cursor(pub usize, pub usize);

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

	pub fn move_left(&mut self) {
		if self.cursor.1 != 0 {
			self.cursor.1 -= 1;
		} else if self.cursor.0 != 0 {
			self.cursor.0 -= 1;
			self.cursor.1 = self.lines[self.cursor.0].len();
		}
	}

	pub fn move_right(&mut self) {
		if self.cursor.1 < self.lines[self.cursor.0].len() {
			self.cursor.1 += 1;
		} else if self.cursor.0 < self.lines.len() {
			self.cursor.0 += 1;
			self.cursor.1 = 0;
		}
	}

	pub fn move_up(&mut self) {
		if self.cursor.0 != 0 {
			self.cursor.0 -= 1;
			let line_len = self.lines[self.cursor.0].len();
			if self.cursor.1 > line_len {
				self.cursor.1 = line_len;
			}
		}
	}

	pub fn move_down(&mut self) {
		if self.cursor.0 < self.lines.len()-1 {
			self.cursor.0 += 1;
			let line_len = self.lines[self.cursor.0].len();
			if self.cursor.1 > line_len {
				self.cursor.1 = line_len;
			}
		}
	}
}