use std::cmp::Ordering;

use super::text::Text;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cursor(pub usize, pub usize);

impl Text {
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

        self.focus = true;
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

        self.focus = true;
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

        self.focus = true;
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

        self.focus = true;
    }

    pub fn move_left_word(&mut self, shift: bool) {
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
            self.cursor.1 = self.get_biggest_space();
        } else if self.cursor.0 != 0 {
            self.cursor.0 -= 1;
            self.cursor.1 = self.lines[self.cursor.0].len();
            self.cursor.1 = self.get_biggest_space();
            self.focus = true;
        }
    }

    pub fn get_biggest_space(&mut self) -> usize {
        let mut biggest = None;
        for (i, c) in self.lines[self.cursor.0].chars().enumerate() {
            if c != ' ' {
                continue;
            }
            if i > self.cursor.1 - 2 {
                break;
            }
            biggest = Some(i);
        }
        if let Some(biggest) = biggest {
            biggest + 1
        } else {
            0
        }
    }

    pub fn move_right_word(&mut self, shift: bool) {
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
            self.cursor.1 = self.get_smallest_space();
            self.focus = true;
            //? self.cursor.1 += 1;
        } else if self.cursor.0 < self.lines.len() - 1 {
            self.cursor.0 += 1;
            self.cursor.1 = 0;
            self.cursor.1 = self.get_smallest_space();
            self.focus = true;
        }
    }

    pub fn get_smallest_space(&mut self) -> usize {
        let mut smallest = None;
        let size = self.lines[self.cursor.0].len();
        for (i, c) in self.lines[self.cursor.0].chars().rev().enumerate() {
            let i = size - i - 1;
            if c != ' ' {
                continue;
            }
            if i < self.cursor.1 + 1 {
                break;
            }
            smallest = Some(i);
        }
        if let Some(biggest) = smallest {
            biggest
        } else {
            size
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
