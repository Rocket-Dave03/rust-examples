use std::{
	sync::{Arc, Mutex},
	time::Duration,
};

fn thread(data: Arc<Mutex<u64>>) {
	while *data.lock().unwrap() != 0 {
		println!(
			"Printing from {}",
			std::thread::current().name().unwrap_or("<no thread name>")
		);
	}
}

fn main() {
	let ref_a = Arc::new(Mutex::new(1u64));
	let ref_b = ref_a.clone();
	let ref_c = ref_a.clone();

	let (handle_a, handle_b) = (
		std::thread::Builder::new()
			.name("thread a".to_string())
			.spawn(|| {
				thread(ref_b);
			})
			.unwrap(),
		std::thread::Builder::new()
			.name("thread b".to_string())
			.spawn(|| {
				thread(ref_c);
			})
			.unwrap(),
	);

	std::thread::sleep(Duration::from_secs(5));
	*ref_a.lock().unwrap() = 0;
	handle_a.join().unwrap();
	handle_b.join().unwrap();
}
