use anyhow::{Result, Context};
use fusion::prelude::*;
use fusion::prelude::Workspace as _;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use log::*;

fn main() -> Result<()> {
	env_logger::init();

	let workspace = Workspace::from_directory("./projects")?;
	let exporter = Exporter::new("./output");
	exporter.clear_output()?;
	workspace.resolve(&exporter)?;
	Ok(())
}

struct Workspace {
	projects: Vec<Project>,
}

impl Workspace {
	fn from_directory(root: impl Into<PathBuf>) -> Result<Self> {
		let root = root.into();
		let projects = root.read_dir()
			.with_context(|| format!("Read directory: {}", root.display()))?
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

impl fusion::workspace::Workspace for Workspace {
	type Project = Project;
	fn projects(&self) -> &[Self::Project] {
		&self.projects
	}
}

struct Exporter {
	path: PathBuf,
}

impl Exporter {
    fn new(path: impl Into<PathBuf>) -> Self {
		Self { path: path.into() }
	}

	fn clear_output(&self) -> std::io::Result<()> {
		if self.path.exists() {
			std::fs::remove_dir_all(&self.path)?
		}

		Ok(())
	}
}

impl fusion::exporter::Exporter for Exporter {
	type Error = std::io::Error;
	type Item = Asset;

	fn root(&self) -> &std::path::Path {
		&self.path
	}

	fn merge(&self, _file: Self::Item, _index: &Index) -> Result<Vec<u8>, Self::Error> {
		unreachable!()
	}
}

struct Project {
	name: String,
	pid: Pid,
	indexes: HashSet<Index>,
	root: PathBuf,
}

impl Project {
	fn new(path: impl Into<PathBuf>, pid: Pid) -> Self {
		let root = path.into();
		let name = root
			.file_name()
			.and_then(|s| s.to_str())
			.map(|s| s.to_string())
			.unwrap_or_default();
		let indexes = Self::generate_indexes(&root, pid);
		info!("{:#?}", indexes);
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
}

impl fusion::project::Project for Project {
	type Item = Asset;
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
		let path = index.prefix(&self.root);

		if !path.is_file() {
			return None;
		}

		let result = Asset::new(path);
		Some(result)
	}
}

struct Asset {
	path: PathBuf,
}

impl Asset {
	fn new(path: impl Into<PathBuf>) -> Self {
		Self { path: path.into() }
	}
}

impl fusion::file::File for Asset {
	fn relation(&self) -> Vec<Relation> {
		vec![]
	}
	fn path(&self) -> &std::path::Path {
		&self.path
	}
	fn strategy(self) -> Strategy<Self>
	where
		Self: Sized,
	{
		Strategy::Rename(self)
	}
}
