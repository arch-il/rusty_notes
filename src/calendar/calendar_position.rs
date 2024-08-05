use chrono::{DateTime, Local};

#[derive(Debug)]
pub struct CalendarPosition {
    pub date: DateTime<Local>,
    pub editing: CurrentlyEditing,
}

#[derive(Debug)]
pub enum CurrentlyEditing {
    Year,
    Month,
    Day,
}

impl CalendarPosition {
    pub fn new() -> Self {
        Self {
            date: Local::now(),
            editing: CurrentlyEditing::Month,
        }
    }
}
