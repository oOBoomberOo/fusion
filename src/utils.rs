use std::io::{Error, ErrorKind, Result};
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UtilError {
	#[error("Cannot find parent of this path")]
	Parent,
	#[error("Cannot resolve filename")]
	FileName
}

pub fn parent(path: &Path) -> Result<&Path> {
	path.parent()
		.ok_or_else(|| Error::new(ErrorKind::Other, UtilError::Parent))
}

pub fn file_stem(path: &Path) -> Result<&str> {
	path.file_stem()
		.and_then(|s| s.to_str())
		.ok_or_else(|| Error::new(ErrorKind::Other, UtilError::FileName))
}
