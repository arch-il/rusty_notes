use crate::{editor::Editor, title_screen::TitleScreenState};

#[derive(Debug)]
pub enum State {
	TitleScreen(TitleScreenState),
	Editor(Editor),
	Exit,
}

impl PartialEq for State {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::Editor(_), Self::Editor(_)) => true,
			_ => core::mem::discriminant(self) == core::mem::discriminant(other),
		}
	}
}