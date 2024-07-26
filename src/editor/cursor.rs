use std::cmp::Ordering;

use super::Editor;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cursor(pub usize, pub usize);

impl Editor {
    pub fn move_left(&mut self, shift: bool) {
        if !shift {
            if let Some(start) = self.selection_start {
                self.selection_start = None;
                self.cursor = if start < self.cursor {
                    start
                } else {
                    self.cursor
                };
                return;
            }
        } else if self.selection_start.is_none() {
            self.selection_start = Some(self.cursor);
        }

        if self.cursor.1 != 0 {
            self.cursor.1 -= 1;
        } else if self.cursor.0 != 0 {
            self.cursor.0 -= 1;
            self.cursor.1 = self.lines[self.cursor.0].len();
        }
    }

    pub fn move_right(&mut self, shift: bool) {
        if !shift {
            if let Some(start) = self.selection_start {
                self.selection_start = None;
                self.cursor = if start < self.cursor {
                    self.cursor
                } else {
                    start
                };
                return;
            }
        } else if self.selection_start.is_none() {
            self.selection_start = Some(self.cursor);
        }

        if self.cursor.1 < self.lines[self.cursor.0].len() {
            self.cursor.1 += 1;
        } else if self.cursor.0 < self.lines.len() - 1 {
            self.cursor.0 += 1;
            self.cursor.1 = 0;
        }
    }

    pub fn move_up(&mut self, shift: bool) {
        if !shift {
            if let Some(start) = self.selection_start {
                self.selection_start = None;
                self.cursor = if start < self.cursor {
                    start
                } else {
                    self.cursor
                };
            }
        } else if self.selection_start.is_none() {
            self.selection_start = Some(self.cursor);
        }

        if self.cursor.0 != 0 {
            self.cursor.0 -= 1;
            let line_len = self.lines[self.cursor.0].len();
            if self.cursor.1 > line_len {
                self.cursor.1 = line_len;
            }
        }
    }

    pub fn move_down(&mut self, shift: bool) {
        if !shift {
            if let Some(start) = self.selection_start {
                self.selection_start = None;
                self.cursor = if start < self.cursor {
                    self.cursor
                } else {
                    start
                };
            }
        } else if self.selection_start.is_none() {
            self.selection_start = Some(self.cursor);
        }

        if self.cursor.0 < self.lines.len() - 1 {
            self.cursor.0 += 1;
            let line_len = self.lines[self.cursor.0].len();
            if self.cursor.1 > line_len {
                self.cursor.1 = line_len;
            }
        }
    }
}

impl PartialOrd for Cursor {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.0.cmp(&other.0) {
            Ordering::Greater => Some(Ordering::Greater),
            Ordering::Less => Some(Ordering::Less),
            Ordering::Equal => Some(self.1.cmp(&other.1)),
        }
    }
}
