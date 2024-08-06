use chrono::{DateTime, Local};

#[derive(Debug, Clone)]
pub struct Note {
    pub id: i32,
    pub text: String,
    pub creation_date: DateTime<Local>,
    pub last_edited: DateTime<Local>,
}
