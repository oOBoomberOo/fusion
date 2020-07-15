use superfusion::criteria::Composite;

fn create_composite() -> Composite {
	Composite::new()
		.with(|path| path.join("pack.mcmeta").is_file())
		.with(|path| path.join("data").is_dir())
		.with(|path| {
			path.join("data/minecraft/tags/functions/tick.json")
				.is_file()
		})
}

#[test]
fn valid_dir() {
	let composite = create_composite();
	let result = composite.check("tests/criteria/valid_dir");
	assert!(result);
}

#[test]
fn invalid_dir_1() {
	let composite = create_composite();
	let should_fail = !composite.check("tests/criteria/invalid_dir_1");
	assert!(should_fail);
}

#[test]
fn invalid_dir_2() {
	let composite = create_composite();
	let should_fail = !composite.check("tests/criteria/invalid_dir_2");
	assert!(should_fail);
}

#[test]
fn invalid_dir_3() {
	let composite = create_composite();
	let should_fail = !composite.check("tests/criteria/invalid_dir_3");
	assert!(should_fail);
}

#[test]
fn non_existing_dir() {
	let composite = create_composite();
	let should_fail = !composite.check("tests/criteria/non_existing_dir");
	assert!(should_fail)
}
