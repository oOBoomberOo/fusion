//! Define a set of criteria required for the directory to be considered a 'project'

use std::fmt;
use std::path::Path;

/// Represent a single criteria
pub struct Criteria<R> {
	f: Box<dyn Fn(&Path) -> Result<(), R>>,
}

impl<R> Criteria<R> {
	/// Create a new criteria with the given closure.
	///
	/// The closure contain one input argument that is the root of the project path. You can use this information to validate the project directory.
	///
	/// ```
	/// # use fusion::criteria::Criteria;
	/// # use std::path::PathBuf;
	/// let criteria = Criteria::new(|root| {
	///     let extension = root.extension().and_then(|os| os.to_str());
	///     match extension {
	///         Some("json") | Some("mcmeta") => Ok(()),
	///         _ => Err("Invalid extension"),
	///     }
	/// });
	///
	/// let root = PathBuf::from("path/to/project.json");
	/// let result = criteria.check(&root);
	/// assert!(result.is_ok());
	///
	/// let root = PathBuf::from("path/to/project.mcmeta");
	/// let result = criteria.check(&root);
	/// assert!(result.is_ok());
	///
	/// let root = PathBuf::from("path/to/pack.toml");
	/// let result = criteria.check(&root);
	/// assert!(result.is_err());
	/// ```
	pub fn new<F>(f: F) -> Self
	where
		F: Fn(&Path) -> Result<(), R> + 'static,
	{
		let f = Box::new(f);
		Self { f }
	}

	/// Create a new criteria with the given closure.
	///
	/// Unlike [Criteria::new()](struct.Criteria.html#method.new), This method take a closure that return a boolean value. If the closure returns false `Err(or)` value will be use and `Ok(())` otherwise.
	///
	/// ```
	/// # use fusion::criteria::Criteria;
	/// # use std::path::PathBuf;
	/// let criteria = Criteria::with_bool(|root| root.to_string_lossy().ends_with("json"), ());
	///
	/// let root = PathBuf::from("path/to/project.json");
	/// let result = criteria.check(&root);
	/// assert!(result.is_ok());
	/// ```
	pub fn with_bool<F>(f: F, or: R) -> Self
	where
		F: Fn(&Path) -> bool + 'static,
		R: Clone + 'static,
	{
		Self::new(move |root| if f(root) { Ok(()) } else { Err(or.clone()) })
	}

	pub fn check(&self, root: &Path) -> Result<(), R> {
		(self.f)(root)
	}
}

impl<R> fmt::Debug for Criteria<R> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Criteria<{}>", std::any::type_name::<R>())
	}
}

/// A composition of multiple criteria. All criteria must be satisfied to pass the test.
#[derive(Debug, Default)]
pub struct Composite<R> {
	criteria: Vec<Criteria<R>>,
}

impl<R> Composite<R> {
	pub fn new() -> Self {
		Self {
			criteria: Vec::new(),
		}
	}

	/// Insert a criteria into this composite
	/// ```
	/// # type Composite = fusion::criteria::Composite<()>;
	/// # type Criteria = fusion::criteria::Criteria<()>;
	/// let foo = Criteria::new(|_| todo!());
	/// let bar = Criteria::new(|_| todo!());
	///
	/// let composite = Composite::new()
	///     .with(foo)
	///     .with(bar);
	///
	/// assert_eq!(composite.len(), 2);
	/// ```
	pub fn with(mut self, criteria: Criteria<R>) -> Self {
		self.criteria.push(criteria);
		self
	}

	pub fn check<P: AsRef<Path>>(&self, root: P) -> Result<(), R> {
		let root = root.as_ref();
		self.criteria
			.iter()
			.try_for_each(|criteria| criteria.check(root))
	}

	pub fn len(&self) -> usize {
		self.criteria.len()
	}

	pub fn is_empty(&self) -> bool {
		self.criteria.is_empty()
	}
}
