fn main(){
	use std::thread::sleep;
	use std::time::{Duration, Instant};

	let now = Instant::now();
	println!("Latest instant {:?}", now.elapsed());
	sleep(Duration::from_secs(2));
	println!("Latest instant {:?}", now.elapsed());
}
