use super::prelude::Pid;
use super::utils;
use std::collections::hash_set::{IntoIter, Iter};
use std::collections::HashSet;
use std::iter::FromIterator;
use std::path::{Path, PathBuf};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Index {
	pid: Pid,
	path: PathBuf,
}

impl Index {
	pub fn new(pid: Pid, path: impl Into<PathBuf>) -> Self {
		let path = path.into();
		Self { pid, path }
	}

	pub fn path(&self) -> &Path {
		&self.path
	}

	pub fn pid(&self) -> &Pid {
		&self.pid
	}

	pub fn is_similar(&self, other: &Self) -> bool {
		self.path == other.path
	}

	pub fn rename<F>(&self, format: F) -> std::io::Result<Self>
	where
		F: Fn(Pid, &str) -> String,
	{
		let path = self.path();
		let parent = utils::parent(path)?;
		let stem = utils::file_stem(path).map(|name| format(self.pid, name))?;

		let mut new_path = parent.join(stem);
		if let Some(extension) = self.path.extension() {
			new_path.set_extension(extension);
		}

		let result = Self::new(self.pid, new_path);
		Ok(result)
	}

	pub fn prefix(&self, path: &Path) -> PathBuf {
		path.join(&self.path)
	}
}

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}) {}", self.pid, self.path.display())
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct IndexList<'a> {
	indexes: HashSet<&'a Index>,
}

impl<'a> IndexList<'a> {
	fn indexes(&self) -> impl Iterator<Item = &Index> {
		self.indexes.iter().copied()
	}

	pub fn new(indexes: HashSet<&'a Index>) -> Self {
		Self { indexes }
	}

	/// Get index loosely base on the relative path
	///
	/// ```
	/// # use fusion::prelude::{IndexList, Index, Pid};
	///
	/// let alpha = Pid::new(0);
	/// let beta = Pid::new(1);
	///
	/// let path = PathBuf::from("example/path");
	/// let foo = Index::new(alpha, &path);
	/// let bar = Index::new(beta, &path);
	///
	/// let mut list = IndexList::default();
	/// list.add(foo);
	///
	/// assert_eq!(list.get(bar), Some(Index::new(alpha, &path)));
	/// ```
	pub fn get(&self, index: &Index) -> Option<&Index> {
		self.indexes().find(|i| i.is_similar(index))
	}

	/// Get index with exactly the same `Pid` and `Path`
	///
	/// This method is really useless since you'd need to already know the index to run it but it can be useful to check if the index exists, I suppose.
	pub fn get_exact(&self, index: &Index) -> Option<&Index> {
		self.indexes.get(index).copied()
	}

	pub fn add(&mut self, index: &'a Index) -> bool {
		self.indexes.insert(index)
	}

	pub fn remove(&mut self, index: &'a Index) -> bool {
		self.indexes.remove(index)
	}

	pub fn inner(&self) -> &HashSet<&Index> {
		&self.indexes
	}

	pub fn union(&self, with: &Self) -> Self {
		let indexes = &self.indexes | &with.indexes;
		Self::new(indexes)
	}

	pub fn iter(&self) -> Iter<&Index> {
		self.indexes.iter()
	}
}

impl<'a, 'b> IntoIterator for &'a IndexList<'b> {
	type Item = &'a &'b Index;
	type IntoIter = Iter<'a, &'b Index>;
	fn into_iter(self) -> Self::IntoIter {
		self.indexes.iter()
	}
}

impl<'a> IntoIterator for IndexList<'a> {
	type Item = &'a Index;
	type IntoIter = IntoIter<&'a Index>;

	fn into_iter(self) -> Self::IntoIter {
		self.indexes.into_iter()
	}
}

impl<'a> FromIterator<&'a Index> for IndexList<'a> {
	fn from_iter<T: IntoIterator<Item = &'a Index>>(iter: T) -> Self {
		let indexes = iter.into_iter().collect();
		Self::new(indexes)
	}
}
