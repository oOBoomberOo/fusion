use super::prelude::{File, Index, IndexList, Pid, Project, Strategy, Timeline};
use std::collections::HashMap;
use std::path::Path;

/// Workspace interface
pub trait Workspace {
	type Project: Project;
	type File: File;

	fn projects(&self) -> &[Self::Project];

	/// Conflict handling strategy
	///
	/// Note that the strategy should not be determine from the content of the file but rather the *location* of the file.
	/// This is for keeping the handling strategy consistent across all project.
	fn strategy(&self, index: &Index) -> Strategy;

	fn file(path: &Path, pid: Pid) -> Option<Self::File>;

	fn formatter(pid: &Pid, filename: &str) -> String {
		format!("{}_{}", filename, pid.value())
	}

	fn resolve(&self) -> Timeline<Self>
	where
		Self: Sized,
	{
		let preview = preview(self);
		let projects = project_paths(self);

		let strategy = indexes(self)
			.map(|index| {
				let strategy = preview
					.get_different_pid(index)
					.map_or(Strategy::Replace, |_| self.strategy(index));
				(index, strategy)
			})
			.collect();

		Timeline::new(strategy, projects)
	}
}

/// Get an iterator over Workspace's projects
fn projects<W: Workspace>(workspace: &W) -> impl Iterator<Item = &W::Project> {
	workspace.projects().iter()
}

/// Get an iterator over every indexes in all projects
fn indexes<W: Workspace>(workspace: &W) -> impl Iterator<Item = &Index> {
	projects(workspace).flat_map(Project::indexes)
}

/// Get HashMap of 'Pid → Project's path'
fn project_paths<W: Workspace>(workspace: &W) -> HashMap<Pid, &Path> {
	projects(workspace).map(|p| (p.pid(), p.root())).collect()
}

/// Get IndexList of all indexes
fn preview<W: Workspace>(workspace: &W) -> IndexList {
	indexes(workspace).collect()
}
