use super::prelude::{IndexList, Project};
use log::*;

/// Default implementation of Workspace's resolver
mod default;
use default::sort_index;

/// Workspace that hold all the projects
pub trait Workspace {
	type Project: Project;

	fn projects(&self) -> &[Self::Project];

	/// Compute Index union of all projects except from the given `project`
	///
	/// This is useful for checking if the index is going to be conflicted with another project but it should not be use for accurate measure.
	fn overlay_preview(&self, project: &Self::Project) -> IndexList {
		self.projects()
			.iter()
			.filter(|p| p.name() != project.name())
			.map(Project::indexes)
			.fold(IndexList::default(), |a, b| a.union(&b))
	}

	/// Resolve project conflict and output it into `exporter`
	///
	/// NOTE: Maybe separate resolving and exporting process?
	fn resolve(self, exporter: &Self::Project) -> Result<(), <Self::Project as Project>::Error>
	where
		Self: Sized,
	{
		let wid = exporter.pid();

		trace!("Resolving conflicts");
		for project in self.projects() {
			trace!("Project: {} {}", project.name(), project.pid());
			let preview = self.overlay_preview(&project);
			let index_group = sort_index(project, preview, wid)?;
			index_group.apply(exporter, project)?;
		}

		Ok(())
	}
}
