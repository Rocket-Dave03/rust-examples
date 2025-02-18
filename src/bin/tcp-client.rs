use std::io::{self, BufRead, BufReader, Write};

fn main() -> io::Result<()> {
	let mut socket = std::net::TcpStream::connect("127.0.0.1:9999")?;

	println!("Sending message");
	socket.write_all("TCP Socket test message\n".as_bytes())?;
	println!("Sucseeefuly sent message");

	let mut msg = String::new();
	BufReader::new(&socket).read_line(&mut msg)?;
	println!("Got server reply: '{}'", msg.trim_end());

	return Ok(());
}
