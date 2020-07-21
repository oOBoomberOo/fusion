use std::fmt;
use std::path::Path;

/// A single 'criteria' is a function that take Path reference and return some boolean.
///
/// The input path will be the root of the project directory.
pub type Criteria = Box<dyn Fn(&Path) -> bool>;

/// A composition of multiple criteria. All criteria must be satisfied to pass the test.
///
/// This is useful for declaring a format that your project can take but it is not necessary to implement.
///
/// ```
/// # use superfusion::criteria::Composite;
/// let composite = Composite::new()
///     .with(|path| path.ends_with("test.txt"));
///
/// let should_pass = composite.check("data/test.txt");
/// assert!(should_pass);
///
/// let should_fail = !composite.check("data/test/functions");
/// assert!(should_fail);
/// ```
#[derive(Default)]
pub struct Composite {
	pub criteria: Vec<Criteria>,
}

impl Composite {
	pub fn new() -> Self {
		Self {
			criteria: Vec::new(),
		}
	}

	/// Insert a criteria into this composite
	/// ```
	/// # use superfusion::criteria::Composite;
	/// let composite = Composite::new()
	///     .with(|path| path.ends_with("txt"))
	///     .with(|path| path.is_file());
	///
	/// assert_eq!(composite.len(), 2);
	/// ```
	pub fn with<F>(mut self, criteria: F) -> Self
	where
		F: Fn(&Path) -> bool + 'static,
	{
		self.criteria.push(Box::new(criteria));
		self
	}

	pub fn check<P: AsRef<Path>>(&self, root: P) -> bool {
		let root = root.as_ref();
		self.criteria.iter().all(|f| f(root))
	}

	pub fn len(&self) -> usize {
		self.criteria.len()
	}

	pub fn is_empty(&self) -> bool {
		self.criteria.is_empty()
	}
}

impl fmt::Debug for Composite {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Composite")
			.field("criteria", &self.criteria.len())
			.finish()
	}
}
