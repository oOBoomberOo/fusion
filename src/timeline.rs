use super::fs;
use super::prelude::{Error, File, Index, IndexMapping, Pid, Strategy, Workspace};
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

	fn exporter<P: Into<PathBuf>>(&self, root: P) -> Exporter<W> {
		let oid = self.output_id();
		let root = root.into();
		let output_project = std::iter::once((oid, root.clone()));

		let projects = self
			.projects
			.iter()
			.map(|(&pid, path)| (pid, path.to_path_buf()))
			.chain(output_project)
			.collect();
		Exporter::new(root, oid, projects)
	}

	/// Save the merged project into the given `path`
	pub fn export_to<P>(self, path: P) -> Result<(), Error>
	where
		P: Into<PathBuf>,
	{
		let exporter = self.exporter(path);
		let mapping = self.mapping()?;

		for (index, strategy) in self.indexes() {
			if let Some(file) = exporter.file(&index) {
				let file = mapping.apply_mapping(file);
				match strategy {
					Strategy::Merge => exporter.merge(file, index)?,
					Strategy::Rename => exporter.rename(file, index)?,
					Strategy::Replace => exporter.add(file, index)?,
				}
			}
		}

		Ok(())
	}

	fn indexes(&self) -> impl Iterator<Item = (&Index, Strategy)> {
		self.strategy.iter().map(|(&a, &b)| (a, b))
	}
}

/// A struct that handle communication with the filesystem.
///
/// This is use to actually write the in-memory data into the filesystem.
struct Exporter<W> {
	root: PathBuf,
	output_id: Pid,
	projects: HashMap<Pid, PathBuf>,
	_workspace: std::marker::PhantomData<W>,
}

impl<W> Exporter<W>
where
	W: Workspace,
{
	fn new(root: impl Into<PathBuf>, output_id: Pid, projects: HashMap<Pid, PathBuf>) -> Self {
		let root = root.into();
		Self {
			root,
			output_id,
			projects,
			_workspace: std::marker::PhantomData,
		}
	}

	fn file(&self, index: &Index) -> Option<W::File> {
		let pid = index.pid();
		let root = self.projects.get(pid)?;
		let path = index.prefix(root);
		W::file(&path, *pid)
	}

	fn path(&self, index: &Index) -> PathBuf {
		index.prefix(&self.root)
	}

	fn write(&self, index: &Index, content: Vec<u8>) -> Result<(), Error> {
		let path = self.path(index);
		fs::prepare_parent(&path)?;
		fs::write(path, content)?;
		Ok(())
	}

	/// Add Index to the project
	fn add(&self, file: W::File, index: &Index) -> Result<(), Error> {
		let content = file.data();
		self.write(index, content)
	}

	/// Rename Index and then add it to the project
	fn rename(&self, file: W::File, index: &Index) -> Result<(), Error> {
		let renamed = index.rename(W::formatter)?;
		let content = file.data();
		self.write(&renamed, content)
	}

	/// Merge Index
	fn merge(&self, file: W::File, index: &Index) -> Result<(), Error> {
		let output_index = index.with_pid(self.output_id);
		let file = match self.file(&output_index) {
			Some(conflict) => conflict.merge(file)?,
			None => file,
		};
		let content = file.data();
		self.write(index, content)
	}
}
