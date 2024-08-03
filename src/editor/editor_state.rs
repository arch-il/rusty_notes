use super::Search;

#[derive(Clone, Debug, PartialEq)]
pub enum EditorState {
	Edit,
	Exit,
	Search(Search),
}
