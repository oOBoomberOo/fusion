use super::prelude::Index;
use std::path::Path;

pub trait File {
	fn relation(&self) -> Vec<Relation>;

	/// Relative path to the given file
	fn path(&self) -> &Path;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Relation(pub Index);

impl Relation {
	pub fn new(depend: Index) -> Self {
		Self(depend)
	}
}

impl From<&Path> for Relation {
	fn from(path: &Path) -> Self {
		Self::new(path.into())
	}
}

impl From<&Index> for Relation {
	fn from(index: &Index) -> Self {
		Self::new(index.clone())
	}
}
