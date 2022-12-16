use std::collections::VecDeque;
use std::fs;

fn main() {
	// use 4 for part one and 14 for part two
	let marker_size = 14;

	// read data from file using a buffered reader
	let data = "data/input";
	let content = fs::read_to_string(data).expect("Error reading file");

	let mut chars = content.chars();

	// fill the queue with the first four values
	for _ in 0..marker_size {
		queue.push_front(chars.next().unwrap());
	}

	for (i, char) in chars.enumerate() {
		// check for characters which appear more than once
		let has_duplicates = queue
			.iter()
			.any(|c| queue.iter().filter(|d| *d == c).count() > 1);

		// if a duplicate is found break
		if !has_duplicates {
			println!("Marker found at: {}", i + marker_size);
			break;
		}

		// otherwise push to next character
		queue.push_front(char);
		queue.pop_back();
	}
}
