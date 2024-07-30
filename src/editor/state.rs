use super::Search;

#[derive(Clone, Debug, PartialEq)]
pub enum State {
	Edit,
	Exit,
	Search(Search),
}
