use super::prelude::Index;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Strategy<F> {
	Merge(F),
	Rename(F),
	Replace(F),
}

pub trait File {
	fn relation(&self) -> Vec<Relation>;

	/// Absolute path to the given file
	fn path(&self) -> &Path;

	/// Conflict handling strategy
	///
	/// Note that the strategy should not be determine from the content of the file but rather the *location* of the file.
	/// This is for keeping the handling strategy consistent across all project.
	fn strategy(self) -> Strategy<Self> where Self: Sized {
		Strategy::Replace(self)
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Relation(pub Index);

impl Relation {
	pub fn new(depend: Index) -> Self {
		Self(depend)
	}
}
