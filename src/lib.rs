pub mod project;
pub mod pattern;
pub mod conflict;
pub mod utils;
pub mod filesystem;
pub mod criteria;

pub mod prelude {
	pub use crate::utils::Namespace;
	pub use crate::conflict::Conflict;
	pub use crate::project::Project;
	pub use crate::criteria::{Criteria, Composite};
}