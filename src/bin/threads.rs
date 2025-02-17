use std::time::Duration;

fn main() {
	std::thread::spawn(|| loop {
		println!("WEEEEE!");
	});
	std::thread::sleep(Duration::from_secs(2));
	println!("Exiting");
}
