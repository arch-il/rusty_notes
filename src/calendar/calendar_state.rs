use chrono::NaiveDate;

use super::CalendarPosition;

#[derive(Debug)]
pub enum CalendarState {
    Browse(CalendarPosition),
    Open(NaiveDate),
    Exit,
}
