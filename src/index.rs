use super::fs;
use super::prelude::{Error, File, Pid, Relation};
use std::collections::hash_set::{IntoIter, Iter};
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::iter::FromIterator;
use std::path::{Path, PathBuf};

/// A relative path to the project that also carry information about where the path is from.
#[derive(Clone, PartialEq, Eq, Hash)]
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

	/// Compare **only** the `path` component of this struct
	///
	/// ```
	/// # use fusion::prelude::{Index, Pid};
	/// let foo = Index::new(Pid::new(0), "./path/to/index");
	/// let bar = Index::new(Pid::new(1), "./path/to/index");
	/// assert!(foo.is_similar(&bar));
	///
	/// let baz = Index::new(Pid::new(1), "./not/a/path/to/index");
	/// assert!(!foo.is_similar(&baz));
	/// ```
	pub fn is_similar(&self, other: &Self) -> bool {
		self.path == other.path
	}

	/// This method will attempt to create a file name that is unique across the entire workspace.
	/// It's usually called when workspace found a conflicting file.
	///
	/// # Note
	/// 1. `format` is a 'formatter function', this function must return a unique formatted string base on its `Pid` and filename.
	/// 2. Changing file extension is not permitted.
	///
	/// # Error
	/// This function can fail in the following errors:
	/// - Logical parent does not exists. (Path like `./` does not have logical parent)
	/// - Unable to get file stem of this path. (Check [Path::file_stem()](std::path::Path::file_stem))
	pub fn rename<F>(&self, format: F) -> Result<Self, Error>
	where
		F: Fn(&Pid, &str) -> String,
	{
		let path = self.path();
		let parent = fs::parent(path)?;
		let stem = fs::file_stem(path).map(|name| format(&self.pid, name))?;

		let mut new_path = parent.join(stem);
		if let Some(extension) = self.path.extension() {
			new_path.set_extension(extension);
		}

		let result = Self::new(self.pid, new_path);
		Ok(result)
	}

	/// Transforming the Index into full path again
	pub fn prefix(&self, root: &Path) -> PathBuf {
		root.join(&self.path)
	}

	/// Change Pid of the Index
	pub fn with_pid(&self, pid: Pid) -> Self {
		Self::new(pid, &self.path)
	}
}

impl fmt::Debug for Index {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Index {{ {} @ {} }}", self.path.display(), self.pid)
	}
}

impl fmt::Display for Index {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} ({})", self.path.display(), self.pid)
	}
}

/// List of Index's references
///
/// It use `HashSet` internally but provide a bit of abstraction that allow looking up Index base on "path similarity"
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

	/// Get index loosely base on the relative path similar to [IndexList::get()](#method.get) but the result index **must** not contain the same Pid as the given index.
	///
	/// ```
	/// # use fusion::prelude::{IndexList, Index, Pid};
	/// let alpha = Pid::new(0);
	/// let beta = Pid::new(1);
	///
	/// let foo = Index::new(alpha, "example/path");
	/// let bar = Index::new(beta, "example/path");
	///
	/// let mut list = IndexList::default();
	/// list.add(&foo);
	///
	/// assert_eq!(list.get_different_pid(&foo), None);
	/// assert_eq!(list.get_different_pid(&bar), Some(&Index::new(alpha, "example/path")));
	/// ```
	pub fn get_different_pid(&self, index: &Index) -> Option<&Index> {
		self.indexes()
			.find(|i| i.is_similar(index) && i.pid() != index.pid())
	}

	/// Get index loosely base on the relative path
	///
	/// ```
	/// # use fusion::prelude::{IndexList, Index, Pid};
	/// let alpha = Pid::new(0);
	/// let beta = Pid::new(1);
	///
	/// let foo = Index::new(alpha, "example/path");
	/// let bar = Index::new(beta, "example/path");
	///
	/// let mut list = IndexList::default();
	/// list.add(&foo);
	///
	/// assert_eq!(list.get(&bar), Some(&Index::new(alpha, "example/path")));
	/// ```
	pub fn get(&self, index: &Index) -> Option<&Index> {
		self.indexes().find(|i| i.is_similar(index))
	}

	/// Get index with exactly the same `Pid` and `Path`
	///
	/// This method is really useless since you need to already know the index to run it but it can be useful to check if the index exists, I suppose.
	pub fn get_exact(&self, index: &Index) -> Option<&Index> {
		self.indexes.get(index).copied()
	}

	/// Get access to the internal's `HashSet`
	pub fn inner(&self) -> &HashSet<&Index> {
		&self.indexes
	}

	/// Create iterator over Index's reference
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

/// List of mapping from one Index to another
///
/// This is used for each file to lookup the Index's path that it referenced to.
/// Usually the Index's path can be compute from the given Index itself but do to 'renaming strategy' feature, there is a need for this lookup.
///
/// You also cannot modify any value after this struct has been created to prevent any misuse of it.
#[derive(Debug, Default, Clone)]
pub struct IndexMapping<'a> {
	map: HashMap<&'a Index, Index>,
}

impl<'a> IndexMapping<'a> {
	pub fn new(map: HashMap<&'a Index, Index>) -> Self {
		Self { map }
	}

	pub fn get(&self, index: &'a Index) -> Option<&Index> {
		self.map.get(index)
	}

	pub fn apply_mapping<F: File>(&self, file: F) -> F {
		let modify_if_exists = |acc: F, ref from| match self.get(from) {
			Some(to) => acc.modify_relation(from, to),
			None => acc,
		};

		file.relation()
			.into_iter()
			.map(Relation::index)
			.fold(file, modify_if_exists)
	}
}

impl<'a> FromIterator<(&'a Index, Index)> for IndexMapping<'a> {
	fn from_iter<T: IntoIterator<Item = (&'a Index, Index)>>(iter: T) -> Self {
		let map = iter.into_iter().collect();
		Self::new(map)
	}
}

#[cfg(test)]
#[allow(clippy::blacklisted_name)]
mod tests {
	use super::*;

	fn formatter(pid: &Pid, filename: &str) -> String {
		format!("{}_{}", filename, pid.value())
	}

	#[test]
	fn is_similar_index() {
		let foo = Index::new(Pid::new(0), "./foo/bar");
		let bar = Index::new(Pid::new(1), "./foo/bar");

		assert!(foo.is_similar(&bar))
	}

	#[test]
	fn rename_index() {
		let index = Index::new(Pid::new(42), "./foo");
		let result = index.rename(formatter).unwrap();
		let expect = Index::new(Pid::new(42), "./foo_42");

		assert_eq!(result, expect);
	}

	#[test]
	fn rename_index_2() {
		let index = Index::new(Pid::new(1), "./foo/bar.json");
		let result = index.rename(formatter).unwrap();
		let expect = Index::new(Pid::new(1), "./foo/bar_1.json");

		assert_eq!(result, expect);
	}

	#[test]
	fn rename_index_3() {
		let index = Index::new(Pid::new(1000), "./bar/");
		let result = index.rename(formatter).unwrap();
		let expect = Index::new(Pid::new(1000), "./bar_1000");

		assert_eq!(result, expect);
	}
}
