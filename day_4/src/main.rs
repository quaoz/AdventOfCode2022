use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
	// read data from file using a buffered reader
	let data = "data/input";
	let file = File::open(data).expect("Error reading file");
	let buffer_reader = BufReader::new(file);

	let mut contained = 0;
	let mut overlap = 0;

	for line in buffer_reader.lines() {
		let line = line.expect("Unable to read line");

		// split the line on '-' and ','
		let split: Vec<&str> = line.split(|c| c == '-' || c == ',').collect();

		// parse the numbers
		let numbers: Vec<i32> = split
			.into_iter()
			.map(|n| str::parse::<i32>(n).unwrap())
			.collect();

		// check is either set of numbers contains the other
		if (numbers[0] <= numbers[2] && numbers[1] >= numbers[3])
			|| (numbers[2] <= numbers[0] && numbers[3] >= numbers[1])
		{
			contained += 1;
		}

		// check for overlap
		if (numbers[0] <= numbers[2] && numbers[1] >= numbers[2])
			|| (numbers[0] <= numbers[3] && numbers[1] >= numbers[3])
			|| (numbers[2] <= numbers[1] && numbers[3] >= numbers[1])
			|| (numbers[2] <= numbers[0] && numbers[3] >= numbers[0])
		{
			overlap += 1;
		}
	}

	println!("There are {contained} pairs which fully contain the other and {overlap} pairs which overlap");
}
