use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
	let mut lines = std::io::stdin().lines();
	let line = lines.next().ok_or("Unable to read stdin")?;
	println!("You wrote '{}'", line.unwrap());
	return Ok(());
}
