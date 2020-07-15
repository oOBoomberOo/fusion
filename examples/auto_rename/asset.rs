use super::Error;
use fusion::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Asset {
	pid: Pid,
	path: PathBuf,
	data: Data,
}

impl Asset {
	pub fn new(path: impl Into<PathBuf>, pid: Pid) -> Result<Self, Error> {
		let path = path.into();
		let reader = File::open(&path)?;
		let data: Data = serde_json::from_reader(reader)?;
		let result = Self { pid, path, data };
		Ok(result)
	}
}

impl fusion::file::File for Asset {
	type Error = Error;
	fn relation(&self) -> Vec<Relation> {
		self.data
			.import
			.as_deref()
			.map(into_index)
			.map(|i| i.with_pid(self.pid))
			.map(Relation::new)
			.map_or(vec![], |r| vec![r])
	}
	fn path(&self) -> &Path {
		&self.path
	}
	fn data(&self) -> Vec<u8> {
		serde_json::to_vec(&self.data).unwrap_or_default()
	}
	fn modify_relation(mut self, _from: &Index, to: &Index) -> Self
	where
		Self: Sized,
	{
		self.data.import = Some(from_index(to));
		self
	}
	fn merge(self, other: Self) -> Result<Self, Self::Error>
	where
		Self: Sized,
	{
		let data = self.data.merge(other.data);
		let path = other.path;
		let pid = other.pid;
		let result = Self { data, path, pid };
		Ok(result)
	}
}

#[derive(Debug, Deserialize, Serialize)]
struct Data {
	#[serde(skip_serializing_if = "Option::is_none")]
	import: Option<String>,
	data: Vec<usize>,
}

impl Data {
	fn merge(self, other: Self) -> Self {
		let import = self.import.or(other.import);
		let mut data = self.data;
		data.extend(other.data);
		Self { import, data }
	}
}

fn into_index(value: &str) -> Index {
	let path = PathBuf::from(value).with_extension("json");
	Index::new(Pid::new(0), path)
}

fn from_index(value: &Index) -> String {
	value
		.path()
		.with_extension("")
		.to_str()
		.map(|v| v.to_string())
		.unwrap_or_default()
}
