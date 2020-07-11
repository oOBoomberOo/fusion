//! Internal Filesystem for managing files
use super::project::{projects_from_dir};
use super::prelude::{Namespace};
use std::io;
use std::path::Path;
use std::collections::{HashMap, hash_map};
use tempfile::{TempDir, tempdir};
use thiserror::Error;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
	#[error(transparent)]
	Io(#[from] io::Error),
}

pub struct Filesystem<P> {
	root: TempDir,
	projects: HashMap<Namespace, P>
}

impl<P> Filesystem<P> {
    pub fn from_dir(dir: &Path) -> Result<Self> {
		todo!()
	}
}

pub trait File {
	fn path(&self) -> &Path;
}