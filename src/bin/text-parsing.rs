use std::{
	io::{BufRead, BufReader},
	num::ParseIntError,
	str::FromStr,
};

#[allow(dead_code)]
#[derive(Debug)]
struct Data {
	name: String,
	number: u64,
}

impl FromStr for Data {
	type Err = ParseIntError;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (name_str, number_str) = s.split_once('=').expect("str shoud contain 1 '='");
		Ok(Self {
			name: name_str.into(),
			number: number_str.parse()?,
		})
	}
}

fn main() {
	for line in BufReader::new(std::fs::File::open("test_input.txt").unwrap()).lines() {
		let data: Data = line.unwrap().parse().unwrap();
		println!("{:?}", data);
	}
}
