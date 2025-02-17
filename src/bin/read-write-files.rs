use std::{
	fs,
	io::{BufRead, BufReader, BufWriter, Write},
};

fn main() {
	let file_a = BufReader::new(fs::File::open("test_input.txt").expect("Unable to open file"));
	let mut file_b = BufWriter::new(
		fs::OpenOptions::new()
			.create(true)
			.write(true)
			.open("test_output.txt")
			.expect("Unable to open file"),
	);

	for line in file_a.lines() {
		let mut line = line.unwrap();
		println!("Read Line: '{}'", line);
		line += "\n"; // Needed as .lines() gives string with newlines removed
		file_b.write(line.as_bytes()).unwrap();
	}
}
