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
///
/// - `Strategy::Merge` will cause [File::merge](/fusion/file/trait.File.html#merge) method to be call.
/// - `Strategy::Rename` will cause the file to be rename to some unique name and [File::modify_relation](/fusion/file/trait.File.html#modify_relation) method to be call on related files.
/// - `Strategy::Replace` will cause the file to override the conflicted file entirely.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Strategy {
	Merge,
	Rename,
	Replace,
}

/// Project trait representing a single project directory.
pub trait Project {
	/// Path to the root of the project directory
	fn root(&self) -> &Path;
	fn pid(&self) -> Pid;

	/// Return [IndexList](/fusion/index/struct.IndexList.html) of all indexes inside this project.
	fn indexes(&self) -> IndexList;
}
