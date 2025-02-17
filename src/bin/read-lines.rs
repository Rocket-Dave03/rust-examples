fn main() {
	for line in std::io::stdin().lines() {
		println!("You wrote '{}'", line.unwrap());
	}
}
