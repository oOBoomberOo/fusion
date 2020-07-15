use anyhow::Result;
use fusion::prelude::Workspace as _;
use thiserror::Error;

mod asset;
mod project;
mod workspace;

use project::Project;
use workspace::Workspace;

fn main() -> Result<()> {
	pretty_env_logger::init();

	let (workspace, exporter_id) = Workspace::from_directory("./projects")?;
	let exporter = Project::new("./output", exporter_id);
	exporter.clear_output()?;
	workspace.resolve(&exporter)?;
	Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
	#[error(transparent)]
	Io(#[from] std::io::Error),
	#[error(transparent)]
	Serde(#[from] serde_json::Error),
}
