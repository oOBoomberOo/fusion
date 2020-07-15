use super::prelude::{File, Index, IndexList};
use log::*;
use std::fmt;
use std::path::{Path, PathBuf};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Strategy {
	Merge,
	Rename,
	Replace,
}

pub trait Project {
	type Item: File<Error = Self::Error>;
	type Error: From<std::io::Error>;

	fn root(&self) -> &Path;
	fn name(&self) -> &str;
	fn pid(&self) -> Pid;
	fn indexes(&self) -> IndexList;
	fn file(&self, index: &Index) -> Option<Self::Item>;

	/// Conflict handling strategy
	///
	/// Note that the strategy should not be determine from the content of the file but rather the *location* of the file.
	/// This is for keeping the handling strategy consistent across all project.
	#[allow(unused_variables)]
	fn strategy(&self, index: &Index) -> Strategy {
		Strategy::Replace
	}

	fn formatter(pid: Pid, filename: &str) -> String {
		format!("{}_{}", filename, pid.value())
	}
}

pub trait ProjectExt: Project {
	fn file_and_index<'a>(&self, index: &'a Index) -> Option<(&'a Index, Self::Item)> {
		let file = self.file(index)?;
		let result = (index, file);
		Some(result)
	}

	fn path(&self, index: &Index) -> PathBuf {
		self.root().join(index.path())
	}

	fn write(&self, index: &Index, content: Vec<u8>) -> Result<(), Self::Error> {
		let path = self.path(index);
		prepare_parent(&path)?;
		std::fs::write(&path, content)?;
		debug!("Successfully write data to {}", path.display());
		Ok(())
	}

	/// Add Index to the project
	fn add(&self, file: Self::Item, index: &Index) -> Result<(), Self::Error> {
		info!("Adding {} to the export list", index);
		let content = file.data();
		self.write(index, content)
	}

	/// Add Renamed Index to the project
	fn rename(&self, file: Self::Item, index: &Index) -> Result<(), Self::Error> {
		let renamed = index.rename(Self::formatter)?;
		info!("Renaming {} to {}", index, renamed);
		let content = file.data();
		self.write(&renamed, content)
	}

	/// Merge Index
	fn merge(&self, file: Self::Item, index: &Index) -> Result<(), Self::Error> {
		info!("Merging {} with already existing file", index);
		let file = match self.file(index) {
			Some(conflict) => conflict.merge(file)?,
			None => file,
		};
		let content = file.data();
		self.write(index, content)
	}
}

impl<T> ProjectExt for T where T: Project {}

fn prepare_parent(path: &Path) -> std::io::Result<()> {
	if let Some(parent) = path.parent() {
		debug!("Prepare parent for {}", path.display());
		std::fs::create_dir_all(parent)?;
	}

	Ok(())
}
