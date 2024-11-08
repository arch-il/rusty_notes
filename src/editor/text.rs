use super::Cursor;

#[derive(Clone, Debug, PartialEq)]
pub struct Text {
    pub lines: Vec<String>,
    pub cursor: Cursor,
    pub selection_start: Option<Cursor>,
    pub copy_buffer: Option<Vec<String>>,
    pub focus: bool,
}

impl Text {
    pub fn new() -> Text {
        Text {
            lines: vec![String::new()],
            cursor: Cursor(0, 0),
            selection_start: None,
            copy_buffer: None,
            focus: false,
        }
    }

    pub fn from_string(text: String) -> Text {
        let lines: Vec<String> = text.split('\n').map(|line| line.to_string()).collect();
        Text {
            lines,
            cursor: Cursor(0, 0),
            selection_start: None,
            copy_buffer: None,
            focus: false,
        }
    }

    pub fn insert_char(&mut self, c: char) {
        if self.selection_start.is_some() {
            self.remove_selected();
        }

        self.lines[self.cursor.0].insert(self.cursor.1, c);
        self.cursor.1 += 1;

        self.focus = true;
    }

    pub fn enter(&mut self) {
        if self.selection_start.is_some() {
            self.remove_selected();
        }

        let after_split = self.lines[self.cursor.0].split_at(self.cursor.1);
        let after_split = (String::from(after_split.0), String::from(after_split.1));
        self.lines[self.cursor.0] = after_split.0;
        self.cursor.0 += 1;
        self.cursor.1 = 0;
        self.lines.insert(self.cursor.0, after_split.1);

        self.focus = true;
    }

    pub fn backspace(&mut self) {
        if self.selection_start.is_some() {
            self.remove_selected();
            return;
        }

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

        self.focus = true;
    }

    pub fn backspace_word(&mut self) {
        if self.selection_start.is_some() {
            self.remove_selected();
            return;
        }

        if self.cursor.1 != 0 {
            let temp = self.get_biggest_space();
            self.lines[self.cursor.0].replace_range(temp..self.cursor.1, "");
            self.cursor.1 = temp;
        } else if self.cursor.0 != 0 {
            self.cursor.0 -= 1;
            self.cursor.1 = self.lines[self.cursor.0].len();
            self.lines[self.cursor.0] =
                self.lines[self.cursor.0].clone() + &self.lines[self.cursor.0 + 1];
            self.lines.remove(self.cursor.0 + 1);
        }

        self.focus = true;
    }

    pub fn delete(&mut self) {
        if self.selection_start.is_some() {
            self.remove_selected();
            return;
        }

        if self.cursor.1 < self.lines[self.cursor.0].len() {
            self.lines[self.cursor.0].remove(self.cursor.1);
        } else if self.cursor.0 < self.lines.len() - 1 {
            self.lines[self.cursor.0] =
                self.lines[self.cursor.0].clone() + &self.lines[self.cursor.0 + 1];
            self.lines.remove(self.cursor.0 + 1);
        }
    }

    pub fn delete_word(&mut self) {
        if self.selection_start.is_some() {
            self.remove_selected();
            return;
        }

        if self.cursor.1 < self.lines[self.cursor.0].len() {
            let temp = self.get_smallest_space();
            self.lines[self.cursor.0].replace_range(self.cursor.1..temp, "");
        } else if self.cursor.0 < self.lines.len() - 1 {
            self.lines[self.cursor.0] =
                self.lines[self.cursor.0].clone() + &self.lines[self.cursor.0 + 1];
            self.lines.remove(self.cursor.0 + 1);
        }
    }

    pub fn remove_selected(&mut self) {
        if let Some(selection_start) = self.selection_start {
            self.selection_start = None;

            let mut start = selection_start;
            let mut end = self.cursor;

            if self.cursor < selection_start {
                start = self.cursor;
                end = selection_start;
            }

            self.cursor = start;

            if start.0 == end.0 {
                let line = &self.lines[start.0];
                self.lines[start.0] = String::from(&line[0..start.1]) + &line[end.1 + 1..];
            } else {
                self.lines[end.0] =
                    String::from(&self.lines[start.0][0..start.1]) + &self.lines[end.0][end.1..];
                self.lines.drain(start.0..end.0);
            }

            self.focus = true;
        }
    }
}
