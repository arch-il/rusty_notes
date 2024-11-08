use chrono::{Datelike, Local, NaiveDate};
use rusqlite::Connection;

use crate::note::Note;

pub struct Database {
    conn: Connection,
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}

impl Database {
    pub fn new() -> Self {
        let conn = Connection::open("./database.db3").expect("Failed to connect to database");

        // Try to create database
        // No need to check error
        let _ = conn.execute(
            "CREATE TABLE note(
                id              INTEGER PRIMARY KEY,
                text            TEXT NOT NULL,
                creation_date   DATE NOT NULL,
                last_edited     DATETIME NOT NULL
                )",
            (),
        );

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

    pub fn update_note(&self, note: &Note) {
        self.conn
            .execute(
                "UPDATE note
            SET text = ?2, creation_date = ?3, last_edited = ?4
            WHERE id = ?1",
                (&note.id, &note.text, &note.creation_date, &note.last_edited),
            )
            .expect("Failed to update note");
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
        .map(|note| note.expect("Failed to read Note from database"))
        .collect()
    }

    pub fn get_or_create_note(&self, creation_date: &NaiveDate) -> Note {
        if let Some(note) = self.get_all_notes().iter().find(|note| {
            note.creation_date.year() == creation_date.year()
                && note.creation_date.month() == creation_date.month()
                && note.creation_date.day() == creation_date.day()
        }) {
            return note.clone();
        }
        let note = Note {
            id: 0,
            text: String::from(" "),
            creation_date: *creation_date,
            last_edited: Local::now(),
        };
        self.insert_note(&note);
        self.get_or_create_note(creation_date)
    }

    pub fn insert_or_create_note(&self, note: &Note) {
        let note_id = self.get_or_create_note(&note.creation_date).id;
        self.update_note(&Note {
            id: note_id,
            text: note.text.clone(),
            creation_date: note.creation_date,
            last_edited: note.last_edited,
        });
    }
}
