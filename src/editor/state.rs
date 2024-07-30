use super::Search;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum State {
	Edit,
	Exit,
	Search(Search),
}
