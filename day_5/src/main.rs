use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::usize;

fn main() {
	// true for part one, false for part two
	let mode: bool = false;

	// read data from file using a buffered reader
	let data = "data/input";
	let file = File::open(data).expect("Error reading file");
	let mut lines = BufReader::new(file).lines();

	let mut stacks: Vec<VecDeque<char>> = vec![VecDeque::new(); 9];

	for _ in 0..8 {
		let line = lines.next().unwrap().expect("Unable to read line");

		// reads the initial state
		let split: Vec<char> = line
			.chars()
			.enumerate()
			.filter(|(i, _)| (i + 3) % 4 == 0)
			.map(|(_, c)| c)
			.collect();

		for (j, char) in split.iter().enumerate() {
			if char != &' ' {
				stacks[j].push_back(*char);
			}
		}
	}

	for line in lines.skip(2) {
		let line = line.expect("Unable to read line");
		let split: Vec<&str> = line.split_whitespace().collect();

		let count = split[1].parse::<usize>().expect("Unable to parse number");
		let source = split[3].parse::<usize>().expect("Unable to parse number") - 1;
		let target = split[5].parse::<usize>().expect("Unable to parse number") - 1;

		// for part two we push them to an intermediate stack first to preserve the order
		let mut stack: VecDeque<char> = VecDeque::new();

		for _ in 0..count {
			let front = stacks[source].pop_front().unwrap();
			if mode {
				stacks[target].push_front(front);
			} else {
				stack.push_front(front)
			}
		}

		if !mode {
			for item in stack {
				stacks[target].push_front(item);
			}
		}
	}

	print!("Top layer: ");
	for stack in stacks {
		print!("{}", stack.front().unwrap())
	}
}
