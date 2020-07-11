use super::prelude::Namespace;

#[derive(Debug)]
pub struct Conflict {
	conflicts: Vec<Namespace>
}

impl Conflict {
    pub fn new() -> Self {
		let conflicts = vec![];
		Self { conflicts }
	}

	pub fn with(&mut self, value: Namespace) {
		self.conflicts.push(value);
	}

	pub fn resolve(self) {
		todo!()
	}
}