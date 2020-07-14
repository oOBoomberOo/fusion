use fusion::prelude::*;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[test]
#[allow(clippy::blacklisted_name)]
fn simple_relationship() {
	let data = PathBuf::from("tests/simple_relationship");
	let resource = Resource::new(&data);

	let foo = resource.file(&Index::new("foo")).unwrap();
	assert_eq!(foo.relation().len(), 1);
	assert_eq!(foo.path(), PathBuf::from("tests/simple_relationship/foo"));

	let bar = resource.file(&Index::new("bar")).unwrap();
	assert_eq!(bar.relation().len(), 0);
	assert_eq!(bar.path(), PathBuf::from("tests/simple_relationship/bar"));
	
	let nested = resource.file(&Index::new("nested/file")).unwrap();
	assert_eq!(nested.relation().len(), 1);
	assert_eq!(nested.path(), PathBuf::from("tests/simple_relationship/nested/file"));
}

struct Resource {
	map: HashSet<Index>,
	root: PathBuf,
}

impl Resource {
	fn new(root: &Path) -> Self {
		let filter_entry = |entry: walkdir::Result<walkdir::DirEntry>| {
			let entry = entry.ok()?;
			let path = entry.into_path();

			if path.is_dir() {
				return None;
			}

			let path = path.strip_prefix(root).ok()?;
			let result = Index::new(path);
			Some(result)
		};

		let map: HashSet<Index> = WalkDir::new(root)
			.contents_first(true)
			.into_iter()
			.filter_map(filter_entry)
			.collect();
		let root = root.to_path_buf();
		Self { map, root }
	}
}

impl Project for Resource {
	type Item = Asset;

	fn file(&self, index: &Index) -> Option<Self::Item> {
		let none_if_empty = |data: String| {
			if data.is_empty() {
				None
			} else {
				Some(data)
			}
		};

		if !self.map.contains(index) {
			return None;
		}

		let path = index.path();
		let full_path = self.root.join(path);
		let reference = std::fs::read_to_string(&full_path)
			.ok()
			.and_then(none_if_empty)
			.map(PathBuf::from)
			.map(Index::new);

		let result = Asset::new(full_path, reference);
		Some(result)
	}
}

#[derive(Debug)]
struct Asset {
	path: PathBuf,
	reference: Option<Index>,
}

impl Asset {
	fn new(path: PathBuf, reference: Option<Index>) -> Self {
		Self { path, reference }
	}
}

impl File for Asset {
	fn relation(&self) -> Vec<Relation> {
		self.reference
			.as_ref()
			.map(Relation::from)
			.map(|r| vec![r])
			.unwrap_or_default()
	}

	fn path(&self) -> &Path {
		&self.path
	}
}
