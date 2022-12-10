use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
	// read data from file using a buffered reader
	let data = "data/input";
	let file = File::open(data).expect("Error reading file");
	let buffer_reader = BufReader::new(file);

	let mut calories = 0;
	let mut totals = Vec::new();

	// iterate over the file lines
	for line in buffer_reader.lines() {
		let line = line.expect("Unable to read line");

		if line.is_empty() {
			// if the line is empty we have reached the end of the elf's inventory so add the total calories to the
			// list of totals and reset it for the next elf
			totals.push(calories);
			calories = 0;
		} else {
			// otherwise parse and add the line to the total
			calories += line.parse::<i32>().expect("Unable to parse line");
		}
	}

	// print the maximum value from the vector
	println!("Most calories: {}", totals.iter().max().unwrap());

	let mut first = 0;
	let mut second = 0;
	let mut third = 0;

	// Extract the three largest totals from the vector
	for total in totals.iter() {
		if total > &third {
			if total > &second {
				if total > &first {
					third = second;
					second = first;
					first = *total;
				} else {
					third = second;
					second = *total;
				}
			} else {
				third = *total;
			}
		}
	}

	println!(
		"Top three: {} + {} + {} = {}",
		first,
		second,
		third,
		first + second + third
	);
}
