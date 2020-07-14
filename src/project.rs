use super::prelude::{File, Index, IndexList};
use std::fmt;

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
		write!(f, "{}", self.0)
	}
}

pub trait Project {
	type Item: File;

	fn name(&self) -> &str;
	fn pid(&self) -> Pid;
	fn indexes(&self) -> IndexList;
	fn file(&self, index: &Index) -> Option<Self::Item>;
}

pub trait ProjectExt: Project {
	fn file_and_index<'a>(&self, index: &'a Index) -> Option<(&'a Index, Self::Item)> {
		let file = self.file(index)?;
		let result = (index, file);
		Some(result)
	}
}

impl<T> ProjectExt for T where T: Project {}
