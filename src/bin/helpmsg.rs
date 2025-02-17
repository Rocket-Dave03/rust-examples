/// Display the list of binaries posible
fn get_bins() -> Vec<String> {
	std::fs::read_dir("src/bin")
		.unwrap()
		.filter_map(|e| e.ok())
		.map(|e| e.path())
		.filter(|e| e.is_file())
		.filter(|f| f.extension().is_some_and(|ext| ext == "rs"))
		.filter(|f| f.as_os_str() != file!())
		.map(|f| {
			f.file_stem()
				.unwrap()
				.to_owned()
				.into_string()
				.expect("Expected example name to containe valid utf-8 chars")
		})
		.collect()
}

fn main() {
	println!("\n\nWelcome to the rust examples library!");
	println!(
		"Contained within this repo are many examples of how simple tasks could be achived in rust"
	);
	println!("\nTo run an example simply run the command 'cargo run --bin <example name>' with <example name> replaced by the name of the example");

	println!("\nThe following examples are available");
	for example in get_bins() {
		println!("\t{}", example);
	}
}
