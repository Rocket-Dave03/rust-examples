fn main() {
	let mut lines = std::io::stdin().lines();
	let line = lines.next().unwrap();
	println!("You wrote '{}'", line.unwrap());
}
