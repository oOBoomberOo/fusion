use super::project::Project;
use super::Workspace;
use colored::*;
use superfusion::prelude::{Index, Logger, Pid, Project as _, Workspace as _};
use log::*;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct MyLogger {
	projects: HashMap<Pid, PathBuf>,
}

impl MyLogger {
	pub fn new(projects: &[Project]) -> Self {
		let projects = projects.iter().map(project_map).collect();
		Self { projects }
	}

	pub fn from_workspace(workspace: &Workspace) -> Self {
		Self::new(workspace.projects())
	}

	fn path(&self, index: &Index) -> PathBuf {
		let path = &self.projects[index.pid()];
		index.prefix(path)
	}
}

#[derive(Debug, Clone, Copy)]
enum Level {
	Add,
	Replace,
	Rename,
	Merge,
}

impl std::fmt::Display for Level {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let value = match self {
			Level::Add => "[+] Add".green(),
			Level::Replace => "[↺] Replace".red(),
			Level::Rename => "[⇢] Rename".magenta(),
			Level::Merge => "[↪] Merge".cyan(),
		};
		write!(f, "{:<11}", value.bold())
	}
}

fn project_map(project: &Project) -> (Pid, PathBuf) {
	let pid = project.pid();
	let path = project.root().to_path_buf();
	(pid, path)
}

impl Logger for MyLogger {
	fn add(&mut self, index: &Index) {
		let path = self.path(index);
		info!(
			"{level} {} to output directory",
			path.display().to_string().green(),
			level = Level::Add
		);
	}
	fn replace(&mut self, conflict: &Index, with: &Index) {
		let output_path = conflict.path();
		let new_path = self.path(with);
		info!(
			"{level} {} with {}",
			output_path.display().to_string().underline(),
			new_path.display().to_string().cyan(),
			level = Level::Replace
		);
	}
	fn merge(&mut self, conflict: &Index, with: &Index) {
		let output_path = conflict.path();
		let new_path = self.path(with);
		info!(
			"{level} {} with {}",
			output_path.display().to_string().red(),
			new_path.display().to_string().cyan(),
			level = Level::Merge
		);
	}
	fn rename(&mut self, _conflict: &Index, index: &Index) {
		let index_path = self.path(index);
		if let Ok(renamed_path) = index.rename(Workspace::formatter) {
			let renamed_path = renamed_path.path().display();
			info!(
				"{level} {} to {}",
				index_path.display().to_string().yellow(),
				renamed_path.to_string().cyan(),
				level = Level::Rename
			);
		}
	}
}
