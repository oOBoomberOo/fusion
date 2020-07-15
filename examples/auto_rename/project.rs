use super::asset::Asset;
use super::Error;
use anyhow::Result;
use fusion::prelude::{Index, IndexList, Pid, Strategy};
use glob::Pattern;
use lazy_static::lazy_static;
use log::*;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

lazy_static! {
	static ref PACK_META: Pattern = Pattern::new("pack.mcmeta").unwrap();
	static ref RENAMEABLE: Pattern = Pattern::new("**/*.json").unwrap();
}

pub struct Project {
	name: String,
	pid: Pid,
	indexes: HashSet<Index>,
	root: PathBuf,
}

impl Project {
	pub fn new(path: impl Into<PathBuf>, pid: Pid) -> Self {
		let root = path.into();
		let name = root
			.file_name()
			.and_then(|s| s.to_str())
			.map(|s| s.to_string())
			.unwrap_or_default();
		let indexes = Self::generate_indexes(&root, pid);

		info!("Create new project at {} with id #{}", root.display(), pid);

		Self {
			root,
			name,
			pid,
			indexes,
		}
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

	pub fn clear_output(&self) -> Result<()> {
		if self.root.exists() {
			std::fs::remove_dir_all(&self.root)?;
		}

		Ok(())
	}
}

impl fusion::project::Project for Project {
	type Item = Asset;
	type Error = Error;
	fn root(&self) -> &Path {
		&self.root
	}
	fn name(&self) -> &str {
		&self.name
	}
	fn pid(&self) -> Pid {
		self.pid
	}
	fn indexes(&self) -> IndexList {
		self.indexes.iter().collect()
	}
	fn file(&self, index: &Index) -> Option<Self::Item> {
		let full_path = index.prefix(&self.root);
		info!("Looking up {}", full_path.display());
		let result = Asset::new(&full_path, self.pid());
		info!("Result: {:?}", result);
		result.ok()
	}
	fn strategy(&self, index: &Index) -> Strategy {
		let path = index.path();

		if PACK_META.matches_path(path) {
			Strategy::Replace
		} else if RENAMEABLE.matches_path(path) {
			Strategy::Rename
		} else {
			Strategy::Merge
		}
	}
}
