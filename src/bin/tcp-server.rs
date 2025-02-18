use std::io::{self, BufRead, BufReader, Write};

fn main() -> io::Result<()> {
	let listener = std::net::TcpListener::bind("127.0.0.1:9999")?;

	for connection in listener.incoming() {
		match connection {
			Ok(mut connection) => {
				println!("Got connection from {}", connection.peer_addr()?);
				let mut msg = String::new();
				BufReader::new(&connection).read_line(&mut msg)?;
				println!("Recieved messge '{}'", msg.trim_end());

				connection.write_all("Msg from server\n".as_bytes())?
			}
			Err(e) => eprintln!("Couldn't get connection: {e}"),
		}
	}
	return Ok(());
}
