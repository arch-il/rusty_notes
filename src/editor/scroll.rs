use super::Editor;

const SCROLL_DISTANCE: usize = 5;

impl Editor {
	pub fn focus_scroll_on_cursor(&mut self) {
		if self.cursor.0 < self.scroll_offset.0 as usize + SCROLL_DISTANCE{
			if self.cursor.0 < SCROLL_DISTANCE {
				self.scroll_offset.0 = 0;
			} else {
				self.scroll_offset.0 = (self.cursor.0 - SCROLL_DISTANCE) as u16;
			}
		}
	}

    pub fn scroll_up(&mut self) {
        if self.scroll_offset.0 != 0 {
			self.scroll_offset.0 -= 1;
		}
    }

    pub fn scroll_down(&mut self) {
        self.scroll_offset.0 += 1;
    }
}