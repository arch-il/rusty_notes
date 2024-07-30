use super::Editor;

const SCROLL_DISTANCE: usize = 5;

impl Editor {
    pub fn focus_scroll_on_cursor(&mut self) {
        if !self.text.focus { return; }

        self.text.focus = false;

        if self.text.cursor.0 < self.scroll_offset.0 as usize + SCROLL_DISTANCE {
            if self.text.cursor.0 < SCROLL_DISTANCE {
                self.scroll_offset.0 = 0;
            } else {
                self.scroll_offset.0 = (self.text.cursor.0 - SCROLL_DISTANCE) as u16;
            }
        } else if self.text.cursor.0 as u16
            > self.scroll_offset.0 + self.screen_size.0 - SCROLL_DISTANCE as u16
        {
            self.scroll_offset.0 
				= (self.text.cursor.0 + SCROLL_DISTANCE) as u16 - self.screen_size.0;
        }

        if self.text.cursor.1 + 4 < self.scroll_offset.1 as usize + SCROLL_DISTANCE
            && self.text.cursor.1 >= SCROLL_DISTANCE - 4
        {
            self.scroll_offset.1 = (self.text.cursor.1 + 4 - SCROLL_DISTANCE) as u16;
        } else if self.text.cursor.1 >= (self.screen_size.1 + self.scroll_offset.1) as usize {
            self.scroll_offset.1 = self.text.cursor.1 as u16 - self.screen_size.1 + 1;
        }
    }

    pub fn scroll_up(&mut self) {
        if self.scroll_offset.0 == 0 {
            return;
        }
        self.scroll_offset.0 -= 1;
    }

    pub fn scroll_down(&mut self) {
        if self.scroll_offset.0 + 1 >= self.text.lines.len() as u16 {
            return;
        }
        self.scroll_offset.0 += 1;
    }
}
