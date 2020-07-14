type Criteria = fusion::prelude::Criteria<Error>;
type Composite = fusion::prelude::Composite<Error>;

#[derive(Debug, PartialEq)]
enum Error {
	PackMeta,
	DataFolder,
	TickingTags,
}

fn pack_meta() -> Criteria {
	Criteria::new(|path| {
		let meta = path.join("pack.mcmeta");
		if meta.is_file() {
			Ok(())
		} else {
			Err(Error::PackMeta)
		}
	})
}

fn data_folder() -> Criteria {
	Criteria::new(|path| {
		let data = path.join("data");
		if data.is_dir() {
			Ok(())
		} else {
			Err(Error::DataFolder)
		}
	})
}

fn ticking_tags() -> Criteria {
	Criteria::new(|path| {
		let ticks = path.join("data/minecraft/tags/functions/tick.json");
		if ticks.is_file() {
			Ok(())
		} else {
			Err(Error::TickingTags)
		}
	})
}

fn create_composite() -> Composite {
	Composite::new()
		.with(pack_meta())
		.with(data_folder())
		.with(ticking_tags())
}

#[test]
fn valid_dir() {
	let composite = create_composite();
	composite.check("tests/criteria/valid_dir").unwrap();
}

#[test]
fn invalid_dir_1() {
	let composite = create_composite();
	let result = composite.check("tests/criteria/invalid_dir_1").unwrap_err();
	assert_eq!(result, Error::PackMeta);
}

#[test]
fn invalid_dir_2() {
	let composite = create_composite();
	let result = composite.check("tests/criteria/invalid_dir_2").unwrap_err();
	assert_eq!(result, Error::DataFolder);
}

#[test]
fn invalid_dir_3() {
	let composite = create_composite();
	let result = composite.check("tests/criteria/invalid_dir_3").unwrap_err();
	assert_eq!(result, Error::TickingTags);
}

#[test]
#[should_panic]
fn non_existing_dir() {
	let composite = create_composite();
	composite.check("tests/criteria/non_existing_dir").unwrap();
}
