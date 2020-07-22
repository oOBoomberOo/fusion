use super::fs;
use super::prelude::{Error, File, Index, IndexMapping, Pid, Strategy, Workspace};
use log::*;
use std::collections::hash_map::Iter;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// A handle containing information describing how to merge the projects together.
pub struct Timeline<'a, W> {
	strategy: HashMap<&'a Index, Strategy>,
	projects: HashMap<Pid, &'a Path>,
	_workspace: std::marker::PhantomData<W>,
}

impl<'a, W> Timeline<'a, W>
where
	W: Workspace,
{
	pub(crate) fn new(
		strategy: HashMap<&'a Index, Strategy>,
		projects: HashMap<Pid, &'a Path>,
	) -> Self {
		debug!(
			"Create new timeline with {} projects and {} strategies",
			projects.len(),
			strategy.len()
		);
		Self {
			strategy,
			projects,
			_workspace: std::marker::PhantomData,
		}
	}

	/// Output Project's [Pid](../project/struct.Pid.html)
	pub fn output_id(&self) -> Pid {
		Pid::new(self.projects.len())
	}

	pub fn strategy(&self) -> Iter<&Index, Strategy> {
		self.strategy.iter()
	}

	pub fn projects(&self) -> Iter<Pid, &Path> {
		self.projects.iter()
	}

	fn mapping(&self) -> Result<IndexMapping, Error> {
		let mut map = HashMap::new();
		let oid = self.output_id();

		for (index, strategy) in self.indexes() {
			match strategy {
				Strategy::Replace | Strategy::Merge => map.insert(index, index.with_pid(oid)),
				Strategy::Rename => {
					let renamed = index.rename(W::formatter)?.with_pid(oid);
					map.insert(index, renamed)
				}
			};
		}

		Ok(IndexMapping::new(map))
	}

	fn exporter<P: Into<PathBuf>>(&self, root: P, mapping: IndexMapping<'a>) -> Exporter<'a, W> {
		let oid = self.output_id();
		let root = root.into();
		let output_project = std::iter::once((oid, root.clone()));

		let projects = self
			.projects()
			.map(|(&pid, path)| (pid, path.to_path_buf()))
			.chain(output_project)
			.collect();
		Exporter::new(root, oid, projects, mapping)
	}

	/// Save the merged project into the given `path`
	pub fn export_to<P>(self, path: P) -> Result<(), Error>
	where
		P: Into<PathBuf>,
	{
		let mapping = self.mapping()?;
		let exporter = self.exporter(path, mapping);

		for (index, strategy) in self.indexes() {
			debug!("Export {} with {:?}", index, strategy);

			if let Some(file) = exporter.file(&index) {
				match strategy {
					Strategy::Merge => exporter.merge(file, index)?,
					Strategy::Rename | Strategy::Replace => exporter.write(file, index)?,
				}
			}
		}

		Ok(())
	}

	fn indexes(&self) -> impl Iterator<Item = (&Index, Strategy)> {
		self.strategy().map(|(&a, &b)| (a, b))
	}
}

/// A struct that handle communication with the filesystem.
///
/// This is use to actually write the in-memory data into the filesystem.
struct Exporter<'a, W> {
	root: PathBuf,
	output_id: Pid,
	projects: HashMap<Pid, PathBuf>,
	mapping: IndexMapping<'a>,
	_workspace: std::marker::PhantomData<W>,
}

impl<'a, W> Exporter<'a, W>
where
	W: Workspace,
{
	fn new(
		root: impl Into<PathBuf>,
		output_id: Pid,
		projects: HashMap<Pid, PathBuf>,
		mapping: IndexMapping<'a>,
	) -> Self {
		let root = root.into();

		debug!(
			"Create exporter to {} with pid {}",
			root.display(),
			output_id
		);

		Self {
			root,
			output_id,
			projects,
			mapping,
			_workspace: std::marker::PhantomData,
		}
	}

	fn file(&self, index: &Index) -> Option<W::File> {
		let pid = index.pid();
		let root = self.projects.get(pid)?;
		let path = index.prefix(root);
		debug!(
			"Looking up file with index {} at path {}",
			index,
			path.display()
		);
		W::file(&path, *pid)
	}

	fn path(&self, index: &Index) -> PathBuf {
		index.prefix(&self.root)
	}

	fn index(&'a self, index: &'a Index) -> Result<&'a Index, Error> {
		self.mapping
			.get(index)
			.ok_or_else(|| Error::unknown_index(index.clone()))
	}

	fn write(&self, file: W::File, index: &Index) -> Result<(), Error> {
		let output_index = self.index(index)?;
		let path = self.path(output_index);

		let file = self.mapping.apply_mapping(file);
		let content = file.data();

		debug!("Write file content from {} to {}", index, path.display());
		fs::prepare_parent(&path)?;
		fs::write(path, content)?;
		Ok(())
	}

	/// Merge Index
	fn merge(&self, file: W::File, index: &Index) -> Result<(), Error> {
		let output_index = index.with_pid(self.output_id);
		debug!("Try to merge file's content from {} with {}", index, output_index);
		let file = match self.file(&output_index) {
			Some(conflict) => conflict.merge(file)?,
			None => file,
		};
		self.write(file, index)
	}
}
