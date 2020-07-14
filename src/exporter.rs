use super::prelude::{File, Index, Pid};
use std::path::{Path, PathBuf};
use log::*;

pub trait Exporter {
	type Error: From<std::io::Error>;
	type Item: File;

	fn root(&self) -> &Path;

	/// Add Index to the project
	#[allow(unused_variables)]
	fn add(&self, file: Self::Item, index: &Index) -> Result<Vec<u8>, Self::Error> {
		let path = file.path();
		let contents = std::fs::read(path)?;
		Ok(contents)
	}

	/// Add Renamed Index to the project
	///
	/// Rename index will have a Pid suffix of the project it came from when export.
	#[allow(unused_variables)]
	fn rename(&self, file: Self::Item, index: &Index) -> Result<Vec<u8>, Self::Error> {
		let path = file.path();
		let contents = std::fs::read(path)?;
		Ok(contents)
	}

	/// Replace the current Index
	#[allow(unused_variables)]
	fn replace(&self, file: Self::Item, index: &Index) -> Result<Vec<u8>, Self::Error> {
		let path = file.path();
		let contents = std::fs::read(path)?;
		Ok(contents)
	}

	/// Merge Index
	#[allow(unused_variables)]
	fn merge(&self, file: Self::Item, index: &Index) -> Result<Vec<u8>, Self::Error>;

	fn rename_format(pid: Pid, filename: &str) -> String {
		format!("{}_{}", filename, pid.value())
	}
}

pub trait ExporterExt: Exporter {
	fn path(&self, index: &Index) -> PathBuf {
		self.root().join(index.path())
	}

	fn write(&self, path: PathBuf, content: Vec<u8>) -> Result<(), Self::Error> {
		prepare_parent(&path)?;
		std::fs::write(&path, content)?;
		debug!("Successfully write data to {}", path.display());
		Ok(())
	}

	/// Add Index to the project
	fn add_ext(&self, file: Self::Item, index: &Index) -> Result<(), Self::Error> {
		let path = self.path(index);
		info!("Add {} to path {}", index, path.display());
		let contents = self.add(file, index)?;
		self.write(path, contents)
	}

	/// Add Renamed Index to the project
	///
	/// Rename index will have a Pid suffix of the project it came from when export.
	fn rename_ext(&self, file: Self::Item, index: &Index) -> Result<(), Self::Error> {
		let name = index.rename(Self::rename_format)?;
		let path = self.path(&name);
		info!("Rename {} to path {}", index, path.display());
		let contents = self.rename(file, index)?;
		self.write(path, contents)
	}

	/// Replace the current Index
	fn replace_ext(&self, file: Self::Item, index: &Index) -> Result<(), Self::Error> {
		let path = self.path(index);
		info!("Replace {} to path {}", index, path.display());
		let contents = self.replace(file, index)?;
		self.write(path, contents)
	}

	/// Merge Index
	fn merge_ext(&self, file: Self::Item, index: &Index) -> Result<(), Self::Error> {
		let path = self.path(index);
		info!("Merge {} to path {}", index, path.display());
		let contents = self.merge(file, index)?;
		self.write(path, contents)
	}
}

fn prepare_parent(path: &Path) -> std::io::Result<()> {
	if let Some(parent) = path.parent() {
		debug!("Prepare parent for {}", path.display());
		std::fs::create_dir_all(parent)?;
	}

	Ok(())
}

impl<T> ExporterExt for T where T: Exporter {}
