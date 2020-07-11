use std::path::Path;
use std::io;
use std::fs::DirEntry;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;
pub type Projects<P> = Vec<P>;

#[derive(Debug, Error)]
pub enum Error {
	#[error("An unrelated project")]
	UnrelatedProject,
	#[error(transparent)]
	Io(#[from] io::Error),
	#[error("{0}")]
	Custom(String)
}

pub trait Project {
	fn from_entry(entry: DirEntry) -> Result<Self> where Self: Sized;
	fn path(&self) -> &Path;
}

pub fn projects_from_dir<P: Project>(dir: &Path) -> Result<Projects<P>> {
	dir.read_dir()?
		.filter_map(|e| e.ok())
		.map(P::from_entry)
		.filter(insignificant_error)
		.collect()
}

fn insignificant_error<T>(item: &Result<T>) -> bool {
	match item {
		Err(Error::UnrelatedProject) => false,
		_ => true
	}
}