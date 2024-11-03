use chrono::{Local, NaiveDate};

#[derive(Clone, Debug, PartialEq)]
pub enum TitleScreenState {
    Options,
    Stats,
    EntryPicker(EntryPicker),
    OpenOldEntry(NaiveDate),
    OpenTodaysEntry,
    Calendar,
    Exit,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EntryPicker {
    pub input: String,
    pub cursor: usize,
}

impl EntryPicker {
    pub fn new() -> EntryPicker {
        EntryPicker {
            input: Local::now().format("%d%m%y").to_string(),
            cursor: 0,
        }
    }

    pub fn insert_char(&mut self, c: char) {
        if !c.is_ascii_digit() {
            return;
        }
        self.input
            .replace_range(self.cursor..self.cursor + 1, &c.to_string());
        self.move_right();
    }

    pub fn move_left(&mut self) {
        if self.cursor == 0 {
            return;
        }
        self.cursor -= 1;
    }

    pub fn move_right(&mut self) {
        if self.cursor == 5 {
            return;
        }
        self.cursor += 1;
    }

    pub fn get_date(&self) -> Option<NaiveDate> {
        let date = NaiveDate::from_ymd_opt(
            2000 + self.input[4..6].parse::<i32>().unwrap(),
            self.input[2..4].parse().unwrap(),
            self.input[0..2].parse().unwrap(),
        )
        .unwrap();

        if date <= Local::now().date_naive() {
            return Some(date);
        }
        None
    }
}
