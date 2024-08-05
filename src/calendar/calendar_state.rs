use super::CalendarPosition;

#[derive(Debug)]
pub enum CalendarState {
    Browse(CalendarPosition),
    Exit,
}
