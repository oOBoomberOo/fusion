use super::prelude::{Error, Index};

/// Representing a file type within the project.
///
/// Note that at this step, the file's data should already be loaded into memory.  
/// The workspace will be calling this trait to ask for various modification and its job is to provide back the correct version.
pub trait File {
	/// Return a "relationship" to another files.
	///
	/// This is use to determine whether its content should be modify when the other index is renamed.
	fn relation(&self) -> Vec<Relation>;

	/// Return the file's data.
	///
	/// Note that the file's data *should* already be stored inside this struct and this method simply return that data.  
	/// This is to allow transforming relationship via [modify_relation()](#method.modify_relation) method without rewriting the file.
	fn data(self) -> Vec<u8>;

	/// Get call when an Index that have relation to this file get renamed.  
	/// This method should act appropriately and rename the reference to that index within this file as well.
	fn modify_relation(self, from: &Index, to: &Index) -> Self
	where
		Self: Sized;

	/// Define how the file can be merge together.
	///
	/// Note: `other` will always be newer than `self` so you should prioritize `other` more than `self`.
	fn merge(self, other: Self) -> Result<Self, Error>
	where
		Self: Sized,
	{
		Ok(other)
	}
}

/// Describing a relationship to another file.
///
/// This should only be generated inside [relation()](trait.File.html#method.relation).
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
