use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
	// read data from file using a buffered reader
	let data = "data/input";
	let file = File::open(data).expect("Error reading file");
	let buffer_reader = BufReader::new(file);

	let mut sum = 0;
	let mut cycle = 0;
	let mut register = 1;

	// iterate over the file lines
	for line in buffer_reader.lines() {
		let line = line.expect("Unable to read line");

		if line == "noop" {
			do_cycle(&mut cycle, register, &mut sum);
		} else {
			do_cycle(&mut cycle, register, &mut sum);
			do_cycle(&mut cycle, register, &mut sum);
			register += line.split(' ').last().unwrap().parse::<i32>().unwrap()
		}
	}

	println!("Sum of signal strengths: {}", sum)
}

fn do_cycle(cycle: &mut i32, register: i32, sum: &mut i32) {
	// increment the cycle by one
	*cycle += 1;

	// check whether it is one of the desired cycles
	if (*cycle + 20) % 40 == 0 {
		println!("Signal strength: {}", register * *cycle);
		*sum += register * *cycle;
	}
}
