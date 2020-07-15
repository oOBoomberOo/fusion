use superfusion::prelude::{Index, IndexList, Pid};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct Project {
	pid: Pid,
	indexes: HashSet<Index>,
	root: PathBuf,
}

impl Project {
	pub fn new(path: impl Into<PathBuf>, pid: Pid) -> Self {
		let root = path.into();
		let indexes = Self::generate_indexes(&root, pid);

		Self { root, pid, indexes }
	}

	fn generate_indexes(root: &Path, pid: Pid) -> HashSet<Index> {
		WalkDir::new(root)
			.into_iter()
			.filter_map(|entry| entry.ok())
			.map(|entry| entry.into_path())
			.filter(|p| p.is_file())
			.filter_map(|p| p.strip_prefix(root).map(|p| p.to_path_buf()).ok())
			.map(|path| Index::new(pid, path))
			.collect()
	}
}

impl superfusion::project::Project for Project {
	fn root(&self) -> &Path {
		&self.root
	}
	fn pid(&self) -> Pid {
		self.pid
	}
	fn indexes(&self) -> IndexList {
		self.indexes.iter().collect()
	}
}
