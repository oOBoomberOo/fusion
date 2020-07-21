use super::asset::Asset;
use super::project::Project;
use anyhow::Result;
use glob::Pattern;
use lazy_static::lazy_static;
use std::path::{Path, PathBuf};
use superfusion::prelude::Pid;
use superfusion::prelude::*;

lazy_static! {
	static ref PACK_META: Pattern = Pattern::new("pack.mcmeta").unwrap();
	static ref RENAMEABLE: Pattern = Pattern::new("**/*.json").unwrap();
	static ref MERGEABLE: Pattern = Pattern::new("**/*.txt").unwrap();
}

pub struct Workspace {
	projects: Vec<Project>,
}

impl Workspace {
	pub fn from_directory(root: impl Into<PathBuf>) -> Result<Self> {
		let root = root.into();
		let projects = root
			.read_dir()?
			.filter_map(workspace_entry)
			.enumerate()
			.map(create_project)
			.collect();

		let result = Self { projects };
		Ok(result)
	}
}

fn workspace_entry(entry: std::io::Result<std::fs::DirEntry>) -> Option<PathBuf> {
	let entry = entry.ok()?;
	let path = entry.path();
	Some(path)
}

fn create_project((i, path): (usize, PathBuf)) -> Project {
	let pid = Pid::new(i);
	Project::new(path, pid)
}

impl superfusion::workspace::Workspace for Workspace {
	type Project = Project;
	type File = Asset;

	fn projects(&self) -> &[Self::Project] {
		&self.projects
	}

	fn file(path: &Path, pid: Pid) -> Option<Self::File> {
		Asset::new(path, pid).ok()
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
