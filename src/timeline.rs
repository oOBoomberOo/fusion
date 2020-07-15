use super::fs;
use super::prelude::{Error, File, Index, IndexMapping, Pid, Strategy, Workspace};
use std::collections::{HashSet, HashMap};
use std::path::{Path, PathBuf};

pub struct Timeline<'a, W> {
	strategy: HashMap<&'a Index, Strategy>,
	projects: HashMap<Pid, &'a Path>,
	_workspace: std::marker::PhantomData<W>,
}

impl<'a, W> Timeline<'a, W>
where
	W: Workspace,
{
	pub fn new(strategy: HashMap<&'a Index, Strategy>, projects: HashMap<Pid, &'a Path>) -> Self {
		Self {
			strategy,
			projects,
			_workspace: std::marker::PhantomData,
		}
	}

	pub fn output_id(&self) -> Pid {
		Pid::new(self.projects.len())
	}

	pub fn mapping(&self) -> Result<IndexMapping, Error> {
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

	pub fn exporter<P: Into<PathBuf>>(&self, root: P) -> Exporter<W> {
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

	pub fn export_to<P>(self, path: P) -> Result<(), Error>
	where
		P: Into<PathBuf>,
	{
		let mut exporter = self.exporter(path);
		let mapping = self.mapping()?;

		for (index, strategy) in self.indexes() {
			let output_path = exporter.path(index);
			let already_exists = output_path.is_file();

			if let Some(file) = exporter.file(&index) {
				let file = mapping.apply_mapping(file);
				match strategy {
					Strategy::Merge if already_exists => exporter.merge(file, index)?,
					Strategy::Rename if already_exists => exporter.rename(file, index)?,
					_ => exporter.add(file, index)?,
				}
			}
		}

		Ok(())
	}

	pub fn indexes(&self) -> impl Iterator<Item = (&Index, Strategy)> {
		self.strategy.iter().map(|(&a, &b)| (a, b))
	}
}

pub struct Exporter<W> {
	root: PathBuf,
	output_id: Pid,
	projects: HashMap<Pid, PathBuf>,
	touched: HashSet<Index>,
	_workspace: std::marker::PhantomData<W>,
}

impl<W> Exporter<W>
where
	W: Workspace,
{
	pub fn new(root: impl Into<PathBuf>, output_id: Pid, projects: HashMap<Pid, PathBuf>) -> Self {
		let root = root.into();
		Self {
			root,
			output_id,
			projects,
			touched: HashSet::new(),
			_workspace: std::marker::PhantomData,
		}
	}

	pub fn get(&self, index: &Index) -> Index {
		let eindex = index.with_pid(self.output_id);
		if let Some(touched) = self.touched.get(&eindex) {
			touched.clone()
		} else {
			index.clone()
		}
	}

	pub fn file(&self, index: &Index) -> Option<W::File> {
		let pid = index.pid();
		let root = self.projects.get(pid)?;
		let path = index.prefix(root);
		W::file(&path, *pid)
	}

	fn path(&self, index: &Index) -> PathBuf {
		index.prefix(&self.root)
	}

	fn write(&mut self, index: &Index, content: Vec<u8>) -> Result<(), Error> {
		let path = self.path(index);
		fs::prepare_parent(&path)?;
		fs::write(path, content)?;

		let index = index.with_pid(self.output_id);
		self.touched.insert(index);
		Ok(())
	}

	/// Add Index to the project
	fn add(&mut self, file: W::File, index: &Index) -> Result<(), Error> {
		let content = file.data();
		self.write(index, content)
	}

	/// Rename Index and then add it to the project
	fn rename(&mut self, file: W::File, index: &Index) -> Result<(), Error> {
		let renamed = index.rename(W::formatter)?;
		let content = file.data();
		self.write(&renamed, content)
	}

	/// Merge Index
	fn merge(&mut self, file: W::File, index: &Index) -> Result<(), Error> {
		let file = match self.file(index) {
			Some(conflict) => conflict.merge(file)?,
			None => file,
		};
		let content = file.data();
		self.write(index, content)
	}
}
