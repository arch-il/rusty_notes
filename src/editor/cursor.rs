use super::Editor;

pub struct Cursor(pub usize, pub usize);

impl Editor {
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
		} else if self.cursor.0 < self.lines.len()-1 {
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