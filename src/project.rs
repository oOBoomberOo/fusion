use super::prelude::{File, Index};

pub trait Project {
	type Item: File;

	fn file(&self, index: &Index) -> Option<Self::Item>;
}
