#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Search {
	pub text: String,
}

impl Search {
	pub fn new() -> Search {
		Search {
			text: String::new(),
		}
	}
}