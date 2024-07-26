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

    pub fn enter(&mut self) {
        let after_split = self.lines[self.cursor.0].split_at(self.cursor.1);
        let after_split = (String::from(after_split.0), String::from(after_split.1));
        self.lines[self.cursor.0] = String::from(after_split.0);
        self.cursor.0 += 1;
        self.cursor.1 = 0;
        self.lines
            .insert(self.cursor.0, String::from(after_split.1));
    }

    pub fn backspace(&mut self) {
        if self.cursor.1 != 0 {
            self.lines[self.cursor.0].remove(self.cursor.1 - 1);
            self.cursor.1 -= 1;
        } else if self.cursor.0 != 0 {
            self.cursor.0 -= 1;
            self.cursor.1 = self.lines[self.cursor.0].len();
            self.lines[self.cursor.0] =
                self.lines[self.cursor.0].clone() + &self.lines[self.cursor.0 + 1];
            self.lines.remove(self.cursor.0 + 1);
        }
    }

	pub fn delete(&mut self) {
		if self.cursor.1 < self.lines[self.cursor.0].len() {
			self.lines[self.cursor.0].remove(self.cursor.1);
		} else if self.cursor.0 < self.lines.len() - 1 {
			self.lines[self.cursor.0] =
                self.lines[self.cursor.0].clone() + &self.lines[self.cursor.0 + 1];
			self.lines.remove(self.cursor.0 + 1);
		}
	}
}
