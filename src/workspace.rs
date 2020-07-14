use super::prelude::{Exporter, ExporterExt, File, IndexList, Project, ProjectExt, Strategy};
use log::*;

pub trait Workspace {
	type Project: Project;

	fn projects(&self) -> &[Self::Project];

	/// Compute index union of all projects except from the given `project`
	///
	/// This is useful for checking if the index is going to be conflicted with another project but it should not be use for accurate measure.
	fn overlay_preview(&self, project: &Self::Project) -> IndexList {
		self.projects()
			.iter()
			.filter(|p| p.name() != project.name())
			.map(Project::indexes)
			.fold(IndexList::default(), |a, b| a.union(&b))
	}

	// TODO: Handle file reference when renaming
	fn resolve<T>(self, workspace: &T) -> Result<(), <T as Exporter>::Error>
	where
		T: Exporter<Item = <Self::Project as Project>::Item> + ExporterExt,
		Self: Sized
	{
		info!("Resolving conflicts");
		for project in self.projects() {
			info!("Project: {}", project.name());
			let preview = self.overlay_preview(&project);

			let files = project
				.indexes()
				.into_iter()
				.filter_map(|i| project.file_and_index(i));

			for (index, file) in files {
				info!("Working on {}", index);
				debug!("At path: {}", file.path().display());
				let result = if preview.get(index).is_some() {
					match file.strategy() {
						Strategy::Replace(file) => workspace.replace_ext(file, index),
						Strategy::Rename(file) => workspace.rename_ext(file, index),
						Strategy::Merge(file) => workspace.merge_ext(file, index),
					}
				} else {
					workspace.add_ext(file, index)
				};
				result?;
			}
		}

		Ok(())
	}
}
