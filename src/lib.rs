pub mod criteria;
pub mod file;
pub mod index;
pub mod logger;
pub mod project;
pub mod timeline;
pub mod workspace;

mod error;
/// Modified `std::fs` module that contain a little more context in Error.
mod fs;
pub use error::Error;

/// Re-export necessary data
pub mod prelude {
	pub use crate::file::{File, Relation};
	pub use crate::index::{Index, IndexList, IndexMapping};
	pub use crate::logger::Logger;
	pub(crate) use crate::logger::LoggerExt;
	pub use crate::project::{Pid, Project, Strategy};
	pub use crate::timeline::Timeline;
	pub use crate::workspace::Workspace;
	pub use crate::Error;
}
