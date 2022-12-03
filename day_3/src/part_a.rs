use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn main() {
	// read data from file using a buffered reader
	let data = "data/input";
	let file = File::open(data).expect("Error reading file");
	let buffer_reader = BufReader::new(file);

	let mut sum= 0;

	for line in buffer_reader.lines() {
		let line = line.expect("Unable to read line");

		// split the line in half
		let (first, second) = line.split_at(line.len() / 2);
		let mut duplicate = ' ';

		// search for the duplicate char
		for char in first.chars() {
			if second.contains(char) {
				duplicate = char;
				break;
			}
		}

		// as u32 results in a..z being 97..122 and A..Z being 65..90
		let mut priority = duplicate as u32 - 64;

		if priority <= 26 {
			priority += 26;
		} else {
			priority -= 32;
		}

		sum += priority;
	}

	println!("Sum: {}", sum)
}
