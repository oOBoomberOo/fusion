use super::prelude::{File, Index, IndexMapping, Pid, Relation};
use log::*;
use std::path::{Path, PathBuf};

/// Exporter handle the renaming, merging and replacing process of the output project.
pub trait Exporter {
	type Error: From<std::io::Error>;

	fn root(&self) -> &Path;

	/// This `Pid` will be use for creating `Index` and differentiate it from other Project.
	///
	/// It **must** be different from all other projects!
	fn pid(&self) -> Pid;

	fn formatter(pid: Pid, filename: &str) -> String {
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
	fn add<F: File>(&self, file: F, index: &Index) -> Result<(), Self::Error> {
		todo!()
	}

	/// Add Renamed Index to the project
	///
	/// Rename index will have a Pid suffix of the project it came from when export.
	fn rename<F: File>(&self, file: F, index: &Index) -> Result<(), Self::Error> {
		todo!()
	}

	/// Merge Index
	fn merge<F: File>(&self, file: F, index: &Index) -> Result<(), Self::Error> {
		todo!()
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
