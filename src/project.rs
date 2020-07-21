use super::prelude::IndexList;
use std::fmt;
use std::path::Path;

/// A unique Project ID that can be easily copy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pid(usize);

impl Pid {
	pub fn new(id: usize) -> Self {
		Self(id)
	}

	pub fn value(&self) -> usize {
		self.0
	}
}

impl fmt::Display for Pid {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "#{}", self.0)
	}
}

/// Conflict handling strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Strategy {
	/// This strategy will cause [File::merge](../file/trait.File.html#method.merge) method to be call.
	Merge,
	/// This strategy will cause the file to be rename to some unique name and [File::modify_relation](../file/trait.File.html#method.modify_relation) method to be call on related files.
	Rename,
	/// This strategy will cause the file to override the conflicted file entirely.
	Replace,
}

/// Project interface representing a single project directory.
pub trait Project {
	/// Path to the root of the project directory
	fn root(&self) -> &Path;
	/// Pid of this project
	fn pid(&self) -> Pid;

	/// Return [IndexList](../index/struct.IndexList.html) of all indexes inside this project.
	fn indexes(&self) -> IndexList;
}
