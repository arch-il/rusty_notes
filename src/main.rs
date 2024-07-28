use std::{fs, io};

use input::take_input;

mod editor;
mod input;
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
    let mut editor = Editor::from_string(text);

    while !editor.exit {
        terminal.draw(|f| { ui(f, &mut editor) })?;

        take_input(&mut editor);
    }

    terminal::restore()?;

    Ok(())
}
