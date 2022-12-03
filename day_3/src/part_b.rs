use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn main() {
	// read data from file using a buffered reader
	let data = "data/input";
	let file = File::open(data).expect("Error reading file");
	let mut lines = BufReader::new(file).lines();
	
	let mut sum = 0;

	let mut line_1 = lines.next();
	let mut line_2 = lines.next();
	let mut line_3 = lines.next();

	// read three lines at a time
	while line_1.is_some() && line_2.is_some() && line_3.is_some() {
		let mut group = ' ';

		// unwrap the lines
		let bag_1 = line_1.unwrap().expect("Unable to read line");
		let bag_2 = line_2.unwrap().expect("Unable to read line");
		let bag_3 = line_3.unwrap().expect("Unable to read line");

		// search for the shared character
		for char in bag_1.chars() {
			if bag_2.contains(char) && bag_3.contains(char) {
				group = char;
				break;
			}
		}

		// as u32 results in a..z being 97..122 and A..Z being 65..90
		let mut priority = group as u32 - 64;

		if priority <= 26 {
			priority += 26;
		} else {
			priority -= 32;
		}

		sum += priority;

		// get the next lines
		line_1 = lines.next();
		line_2 = lines.next();
		line_3 = lines.next();
	}

	println!("Sum: {}", sum)
}
