use chrono::{DateTime, Local, TimeZone};

#[derive(Debug, PartialEq)]
pub enum TitleScreenState {
    None,
    EntryPicker(EntryPicker),
    OpenOldEntry(DateTime<Local>),
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

    pub fn get_date(&self) -> Option<DateTime<Local>> {
        let date = Local
            .with_ymd_and_hms(
                self.input[4..6].parse().unwrap(),
                self.input[2..4].parse().unwrap(),
                self.input[0..2].parse().unwrap(),
                0,
                0,
                0,
            )
            .unwrap();

        if date < Local::now() {
            return Some(date);
        }
        None
    }
}
