/// Specify criteria that can be considered a 'project'
pub mod criteria;
/// File interface
pub mod file;
/// Internal representation of a single file inside the project
pub mod index;
/// Project interface
pub mod project;
pub mod timeline;
/// Workspace interface
pub mod workspace;

mod error;
/// Modified `std::fs` module that contain a little more context in Error.
mod fs;
pub use error::Error;

/// Re-export necessary data
pub mod prelude {
	pub use crate::file::{File, Relation};
	pub use crate::index::{Index, IndexList, IndexMapping};
	pub use crate::project::{Pid, Project, Strategy};
	pub use crate::timeline::Timeline;
	pub use crate::workspace::Workspace;
	pub use crate::Error;
}
