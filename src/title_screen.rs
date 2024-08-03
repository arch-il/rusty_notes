#[derive(Debug, PartialEq)]
pub enum TitleScreenState {
	None,
	OpenExisting,
	OpenNew,
	Calendar,
	Exit,
}