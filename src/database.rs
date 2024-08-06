use chrono::{DateTime, Local};
use rusqlite::Connection;

use crate::note::Note;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> Self {
        // let conn = Connection::open_in_memory().expect("Failed to create connection");
        let conn = Connection::open("./notes/database.db3").expect("Failed to connect to database");
        // conn.execute(
        //     "CREATE TABLE note(
        //         id            INTEGER PRIMARY KEY,
        //         text          TEXT NOT NULL,
        //         creation_date DATETIME NOT NULL,
        //         last_edited   DATETIME NOT NULL
        //     )",
        //     (),
        // )
        // .expect("Failed to create table");

        Self { conn }
    }

    pub fn insert_note(&self, note: &Note) {
        self.conn
            .execute(
                "INSERT INTO note (text, creation_date, last_edited) VALUES (?1, ?2, ?3)",
                (&note.text, &note.creation_date, &note.last_edited),
            )
            .expect("Failed to insert note");
    }

    pub fn get_all_notes(&self) -> Vec<Note> {
        let mut stmt = self
            .conn
            .prepare("SELECT * FROM note")
            .expect("Failed to read notes");
        stmt.query_map([], |row| {
            Ok(Note {
                id: row.get(0)?,
                text: row.get(1)?,
                creation_date: row.get(2)?,
                last_edited: row.get(3)?,
            })
        })
        .unwrap()
        .map(|note| note.unwrap())
        .collect()
    }

    pub fn get_or_create_note(&self, date: &DateTime<Local>) -> Note {
        if let Some(note) = self
            .get_all_notes()
            .iter()
            .find(|note| &note.creation_date == date)
        {
            return note.clone();
        }
        let note = Note {
            id: 0,
            text: String::from(" "),
            creation_date: date.clone(),
            last_edited: date.clone(),
        };
        self.insert_note(&note);
        note
    }
}
