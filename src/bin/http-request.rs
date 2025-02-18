use reqwest::blocking as req;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
	let response = req::get("https://api.ipify.org")?;
	println!("Device ip: {}", response.text()?);

	return Ok(());
}
