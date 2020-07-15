use super::prelude::{Index, Strategy};

pub trait Logger {
	fn add(&mut self, index: &Index);

	fn replace(&mut self, conflict: &Index, with: &Index);
	fn merge(&mut self, conflict: &Index, with: &Index);
	fn rename(&mut self, conflict: &Index, index: &Index);
}

pub(crate) trait LoggerExt: Logger {
	fn log_conflicted(&mut self, level: Strategy, conflict: &Index, with: &Index) {
		match level {
			Strategy::Replace => self.replace(conflict, with),
			Strategy::Merge => self.merge(conflict, with),
			Strategy::Rename => self.rename(conflict, with),
		}
	}
}

impl<T> LoggerExt for T where T: Logger {}
