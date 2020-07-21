use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use superfusion::prelude::*;

#[derive(Debug)]
pub enum Asset {
	Json(Json),
	Text(Text),
}

impl Asset {
	pub fn new(path: impl Into<PathBuf>, pid: Pid) -> Result<Self, super::Error> {
		let path = path.into();
		let mut reader = File::open(&path)?;

		let result = match path.extension().and_then(|os| os.to_str()) {
			Some("json") => {
				let data: Data = serde_json::from_reader(reader)?;
				let json = Json { pid, data };
				Asset::Json(json)
			}
			_ => {
				let mut data = String::new();
				reader.read_to_string(&mut data)?;
				let text = Text { data };
				Asset::Text(text)
			}
		};

		Ok(result)
	}
}

impl superfusion::file::File for Asset {
	fn relation(&self) -> Vec<Relation> {
		match self {
			Asset::Json(json) => json.relation(),
			Asset::Text(text) => text.relation(),
		}
	}
	fn data(self) -> Vec<u8> {
		match self {
			Asset::Json(json) => json.data(),
			Asset::Text(text) => text.data(),
		}
	}
	fn modify_relation(self, from: &Index, to: &Index) -> Self
	where
		Self: Sized,
	{
		match self {
			Asset::Json(json) => Asset::Json(json.modify_relation(from, to)),
			Asset::Text(text) => Asset::Text(text.modify_relation(from, to)),
		}
	}
	fn merge(self, other: Self) -> Result<Self, Error>
	where
		Self: Sized,
	{
		use Asset::*;
		let result = match (self, other) {
			(Json(a), Json(b)) => Asset::Json(a.merge(b)?),
			(Text(a), Text(b)) => Asset::Text(a.merge(b)?),
			_ => unreachable!(),
		};
		Ok(result)
	}
}

#[derive(Debug)]
pub struct Json {
	pid: Pid,
	data: Data,
}

impl superfusion::file::File for Json {
	fn relation(&self) -> Vec<Relation> {
		self.data
			.import
			.as_deref()
			.map(into_index)
			.map(|i| i.with_pid(self.pid))
			.map(Relation::new)
			.map_or(vec![], |r| vec![r])
	}
	fn data(self) -> Vec<u8> {
		serde_json::to_vec(&self.data).unwrap_or_default()
	}
	fn modify_relation(mut self, _from: &Index, to: &Index) -> Self
	where
		Self: Sized,
	{
		self.data.import = Some(from_index(to));
		self
	}
	fn merge(self, other: Self) -> Result<Self, Error>
	where
		Self: Sized,
	{
		let data = self.data.merge(other.data);
		let pid = other.pid;
		let result = Self { data, pid };
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

#[derive(Debug)]
pub struct Text {
	data: String,
}

impl superfusion::prelude::File for Text {
	fn relation(&self) -> Vec<Relation> {
		vec![]
	}
	fn data(self) -> Vec<u8> {
		self.data.as_bytes().to_vec()
	}
	fn modify_relation(self, _from: &Index, _to: &Index) -> Self
	where
		Self: Sized,
	{
		self
	}
	fn merge(self, other: Self) -> Result<Self, Error>
	where
		Self: Sized,
	{
		let data = self.data + &other.data;
		Ok(Self { data })
	}
}
