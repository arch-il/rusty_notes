use crate::editor::Editor;

#[derive(Debug)]
pub enum State {
	TitleScreen,
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