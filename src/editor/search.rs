use super::text::Text;


#[derive(Clone, Debug, PartialEq)]
pub struct Search {
	pub text: Text,
}

impl Search {
	pub fn new() -> Search {
		Search {
			text: Text::new(),
		}
	}
}