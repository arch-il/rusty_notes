use chrono::{DateTime, Local};

use super::CalendarPosition;

#[derive(Debug)]
pub enum CalendarState {
    Browse(CalendarPosition),
    Open(DateTime<Local>),
    Exit,
}
