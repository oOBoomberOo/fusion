use super::prelude::{File, Index, IndexList, Pid, Project, Strategy, Timeline};
use log::*;
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
		let result = format!("{}_{}", filename, pid.value());
		debug!(
			"Formatted filename {} and pid {} into {}",
			filename, pid, result
		);
		result
	}

	fn resolve(&self) -> Timeline<Self>
	where
		Self: Sized,
	{
		let preview = preview(self);
		debug!("Generated index preview");
		let projects = project_paths(self);
		debug!("Generated mapping between Pid and Project Path");

		let strategy = indexes(self)
			.map(|index| {
				let strategy = match preview.get_different_pid(index) {
					Some(conflict) => {
						let strategy = self.strategy(index);
						debug!(
							"Found conflicting index at {}, choose {:?} for index: {}",
							conflict, strategy, index
						);
						strategy
					}
					None => {
						debug!("No conflicting index found, choose {:?}", Strategy::Replace);
						Strategy::Replace
					}
				};

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
