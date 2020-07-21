use anyhow::Result;
use std::path::Path;
use superfusion::prelude::Workspace as _;
use thiserror::Error;

mod asset;
mod project;
mod workspace;

use workspace::Workspace;

fn main() -> Result<()> {
	clear_output("./output")?;
	let workspace = Workspace::from_directory("./examples/auto_rename/projects")?;
	let timeline = workspace.resolve();
	timeline.export_to("./output")?;
	Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
	#[error(transparent)]
	Io(#[from] std::io::Error),
	#[error(transparent)]
	Serde(#[from] serde_json::Error),
}

fn clear_output(path: impl AsRef<Path>) -> Result<()> {
	let path = path.as_ref();
	if path.exists() {
		std::fs::remove_dir_all(path)?;
	}

	Ok(())
}
