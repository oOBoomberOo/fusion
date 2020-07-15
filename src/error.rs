use std::path::PathBuf;
use thiserror::Error;

/// Internal Error
#[derive(Debug, Error)]
pub enum Error {
	#[error("Unable to write data to {path}")]
	Write {
		path: PathBuf,
		#[source]
		source: std::io::Error,
	},

	#[error("Unable to get the parent of this path {path}")]
	Parent { path: PathBuf },

	#[error("Unable to get filename of this path {path}")]
	NoFileName { path: PathBuf },

	#[error("Unable to create directory from this path {path}")]
	CreateDirAll {
		path: PathBuf,
		#[source]
		source: std::io::Error,
	},

	#[error(transparent)]
	Custom(Box<dyn std::error::Error + Sync + Send>),
}

impl Error {
	pub fn write(path: impl Into<PathBuf>, source: std::io::Error) -> Self {
		Error::Write {
			path: path.into(),
			source,
		}
	}

	pub fn parent(path: impl Into<PathBuf>) -> Self {
		Error::Parent { path: path.into() }
	}

	pub fn filename(path: impl Into<PathBuf>) -> Self {
		Error::NoFileName { path: path.into() }
	}

	pub fn dir_all(path: impl Into<PathBuf>, source: std::io::Error) -> Self {
		Error::CreateDirAll {
			path: path.into(),
			source,
		}
	}

	pub fn custom(error: impl std::error::Error + Sync + Send + 'static) -> Self {
		let boxed_error = Box::new(error);
		Error::Custom(boxed_error)
	}
}
