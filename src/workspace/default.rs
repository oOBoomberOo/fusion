use crate::prelude::{Index, IndexList, IndexMapping, Pid, Project, ProjectExt, Strategy};
use log::*;
use std::collections::HashMap;

pub(crate) fn sort_index<'a, P>(
	project: &'a P,
	preview: IndexList,
	pid: Pid,
) -> std::io::Result<IndexGroup<'a>>
where
	P: Project,
{
	let mut result = IndexGroup::new(pid);

	for index in project.indexes() {
		trace!("Inspecting {}...", index);
		if preview.get(index).is_some() {
			trace!("{} already exists", index);
			let strategy = project.strategy(index);
			trace!("Strategy → {:?}", strategy);
			match strategy {
				Strategy::Rename => result.rename::<P>(index)?,
				Strategy::Merge => result.merge(index),
				Strategy::Replace => result.add(index),
			};
		} else {
			trace!("{} doesn't exists", index);
			result.add(index);
		}
	}

	Ok(result)
}

pub(crate) struct IndexGroup<'a> {
	rename: HashMap<&'a Index, Index>,
	merge: HashMap<&'a Index, Index>,
	add: HashMap<&'a Index, Index>,
	pid: Pid,
}

impl<'a> IndexGroup<'a> {
	fn new(pid: Pid) -> Self {
		Self {
			rename: HashMap::new(),
			merge: HashMap::new(),
			add: HashMap::new(),
			pid,
		}
	}

	fn rename<P>(&mut self, from: &'a Index) -> std::io::Result<()>
	where
		P: Project,
	{
		let to = from.rename(P::formatter)?.with_pid(self.pid);
		self.rename.insert(from, to);
		Ok(())
	}

	fn merge(&mut self, idx: &'a Index) {
		self.merge.insert(idx, idx.clone().with_pid(self.pid));
	}

	fn add(&mut self, idx: &'a Index) {
		self.add.insert(idx, idx.clone().with_pid(self.pid));
	}

	fn to_mapping(&self) -> IndexMapping {
		self.rename
			.iter()
			.chain(self.merge.iter())
			.chain(self.add.iter())
			.map(|(&a, b)| (a, b.clone()))
			.collect()
	}

	pub(crate) fn apply<P>(&self, exporter: &P, project: &P) -> Result<(), P::Error>
	where
		P: Project,
	{
		info!(
			"Applying change from project: '{} {}'",
			project.name(),
			project.pid()
		);
		let mapping = self.to_mapping();

		for index in self.rename.keys() {
			if let Some(file) = project.file(index) {
				let file = mapping.apply_mapping(file);
				exporter.rename(file, index)?;
			}
		}

		for index in self.merge.keys() {
			if let Some(file) = project.file(index) {
				let file = mapping.apply_mapping(file);
				exporter.merge(file, index)?;
			}
		}

		for index in self.add.keys() {
			if let Some(file) = project.file(index) {
				let file = mapping.apply_mapping(file);
				exporter.add(file, index)?;
			}
		}

		Ok(())
	}
}
