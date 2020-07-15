use std::io::{Error, ErrorKind, Result};
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UtilError {
	#[error("Cannot find parent of this path")]
	Parent,
	#[error("Cannot resolve filename")]
	FileName,
}

/// `io::Result` mapping over normal `Path::parent()` method that return `Option<&Path>`
///
/// This flow nicer together when use inside [Index::rename()](/fusion/index/struct.Index.html#rename).
pub fn parent(path: &Path) -> Result<&Path> {
	path.parent()
		.ok_or_else(|| Error::new(ErrorKind::Other, UtilError::Parent))
}

/// `io::Result` mapping over normal `Path::file_stem()` method that return `Option<&OsStr>`.  
/// This function also transform `&OsStr` into `&str` when possible and emit `UtilError` otherwise.
///
/// This flow nicer together when use inside [Index::rename()](/fusion/index/struct.Index.html#rename).
pub fn file_stem(path: &Path) -> Result<&str> {
	path.file_stem()
		.and_then(|s| s.to_str())
		.ok_or_else(|| Error::new(ErrorKind::Other, UtilError::FileName))
}
