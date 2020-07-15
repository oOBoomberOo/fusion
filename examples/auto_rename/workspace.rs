use super::project::Project;
use anyhow::Result;
use fusion::prelude::Pid;
use std::path::PathBuf;

pub struct Workspace {
	projects: Vec<Project>,
}

impl Workspace {
	pub fn from_directory(root: impl Into<PathBuf>) -> Result<(Self, Pid)> {
		let root = root.into();
		let projects = root
			.read_dir()?
			.filter_map(workspace_entry)
			.enumerate()
			.map(create_project)
			.collect();

		let result = Self { projects };
		let exporter_id = result.latest_pid();
		Ok((result, exporter_id))
	}

	pub fn latest_pid(&self) -> Pid {
		Pid::new(self.projects.len())
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
