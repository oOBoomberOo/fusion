use anyhow::Result;
use superfusion::prelude::Workspace as _;
use log::LevelFilter;
use std::io::Write;
use std::path::Path;
use thiserror::Error;

mod asset;
mod logger;
mod project;
mod workspace;

use logger::MyLogger;
use workspace::Workspace;

fn logger() {
	let mut builder = pretty_env_logger::formatted_builder();
	builder
		.format(|buf, record| writeln!(buf, "{}", record.args()))
		.format_indent(Some(2))
		.filter(None, LevelFilter::Trace)
		.init();
}

fn main() -> Result<()> {
	logger();

	clear_output("./output")?;
	let workspace = Workspace::from_directory("./projects")?;
	let mut logger = MyLogger::from_workspace(&workspace);
	let timeline = workspace.resolve(&mut logger);
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
