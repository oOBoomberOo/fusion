use std::path::{PathBuf, Path};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Index {
	path: PathBuf
}

impl Index {
    pub fn new(path: impl Into<PathBuf>) -> Self {
		let path = path.into();
		Self { path }
	}

	pub fn path(&self) -> &Path {
		&self.path
	}
}

impl From<&Path> for Index {
	fn from(path: &Path) -> Self {
		Self::new(path)
	}
}
