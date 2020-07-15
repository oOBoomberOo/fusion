use super::Error;
use std::path::Path;

type Result<T> = std::result::Result<T, Error>;

pub fn write<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> Result<()> {
	let path = path.as_ref();
	std::fs::write(&path, contents).map_err(|io| Error::write(path, io))
}

pub fn parent(path: &Path) -> Result<&Path> {
	path.parent().ok_or_else(|| Error::parent(path))
}

pub fn file_stem(path: &Path) -> Result<&str> {
	path.file_stem()
		.and_then(|s| s.to_str())
		.ok_or_else(|| Error::filename(path))
}

pub fn create_dir_all<P: AsRef<Path>>(path: P) -> Result<()> {
	std::fs::create_dir_all(&path).map_err(|io| Error::dir_all(path.as_ref(), io))
}

pub fn prepare_parent(path: &Path) -> Result<()> {
	if let Some(parent) = path.parent() {
		create_dir_all(parent)?;
	}

	Ok(())
}
