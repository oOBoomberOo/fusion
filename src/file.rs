use super::prelude::Index;
use std::path::Path;

/// Representing a file type within the project.
///
/// Note that at this step, the file's data should already be loaded into memory.  
/// The workspace will be calling this trait to ask for various modification and its job is to provide back the correct version.
pub trait File {
	type Error: From<std::io::Error>;

	fn relation(&self) -> Vec<Relation>;
	/// Absolute path to the given file
	fn path(&self) -> &Path;
	fn data(&self) -> Vec<u8>;

	fn modify_relation(self, from: &Index, to: &Index) -> Self
	where
		Self: Sized;

	fn merge(self, other: Self) -> Result<Self, Self::Error>
	where
		Self: Sized,
	{
		Ok(other)
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Relation(pub Index);

impl Relation {
	pub fn new(depend: Index) -> Self {
		Self(depend)
	}

	pub fn index(self) -> Index {
		self.0
	}
}
