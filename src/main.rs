use std::{fs, io};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};

mod editor;
mod terminal;
mod ui;

use crate::{
    editor::Editor,
    ui::ui
};

fn main() -> io::Result<()> {
    let mut terminal = terminal::init()?;
    
    
    let file_name = "notes/note.md";
    let text = fs::read_to_string(file_name)?;
    let mut app = Editor::from_string(text);

    loop {
        terminal.draw(|f| { ui(f, &app) })?;

        match event::read()? {
            Event::Key(key_event) => {
                if key_event.kind == KeyEventKind::Press {
                    match key_event.code {
                        KeyCode::Char(c) => app.insert_char(c),
                        KeyCode::Left => app.move_left(),
                        KeyCode::Right => app.move_right(),
                        KeyCode::Up => app.move_up(),
                        KeyCode::Down => app.move_down(),

                        KeyCode::Esc => break,
                        _ => (),
                    }
                }
            } 
            _ => (),
        }
    }

    terminal::restore()?;

    Ok(())
}
