use super::text::Text;

impl Text {
    pub fn copy(&mut self) {
        if let Some(selection_start) = self.selection_start {
			let mut start = self.cursor;
			let mut end = selection_start;
			if self.cursor > selection_start {
				start = selection_start;
				end = self.cursor;
			}

			if start.0 == end.0 {
				self.copy_buffer = Some(vec![
					String::from(&self.lines[start.0][start.1..end.1+1])
				]);
			} else {
				let mut copy_lines = Vec::new();
				copy_lines.push(String::from(&self.lines[start.0][start.1..]));
				for line_id in (start.0..end.0).skip(1) {
					copy_lines.push(self.lines[line_id].clone());
				}
				copy_lines.push(String::from(&self.lines[end.0][0..end.1]));
				self.copy_buffer = Some(copy_lines);
			}
        } else {
            self.copy_buffer = Some(vec![
				self.lines[self.cursor.0].clone(), 
				String::new(),
			]);
        }
    }

    pub fn paste(&mut self) {
		if self.selection_start.is_some() {
			self.remove_selected();
		}
		
        if let Some(copy_buffer) = &self.copy_buffer {
            if copy_buffer.len() == 1 {
                self.lines[self.cursor.0].insert_str(self.cursor.1, &copy_buffer[0]);
				self.cursor.1 += copy_buffer[0].len();
            } else {
                let line = String::from(&self.lines[self.cursor.0]);
                self.lines[self.cursor.0] 
					= String::from(&line[0..self.cursor.1]) + &copy_buffer[0];
                for copy_line in copy_buffer.iter().skip(1).rev() {
                    self.lines
                        .insert(self.cursor.0 + 1, String::from(copy_line));
                }
				self.cursor.0 += copy_buffer.len() - 1;
                self.lines[self.cursor.0] += &line[self.cursor.1..];
				self.cursor.1 = 0; //?
            }
        }
    }

	pub fn cut(&mut self) {
		self.copy();

		if self.selection_start.is_some() {
			self.remove_selected();
		} else {
			self.lines.remove(self.cursor.0);
		}
	}
}
