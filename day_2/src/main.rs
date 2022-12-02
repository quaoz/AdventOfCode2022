use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
	let mode: bool = false;

	// read data from file using a buffered reader
	let data = "data/input";
	let file = File::open(data).expect("Error reading file");
	let buffer_reader = BufReader::new(file);

	let mut score = 0;

	for line in buffer_reader.lines() {
		// split the line
		let line = line.expect("Unable to read line");
		let v: Vec<&str> = line.split(' ').collect();

		// extract the moves as numeric values
		let opponent = to_value(v.get(0).unwrap());
		let you = to_value(v.get(1).unwrap());

		if mode {
			score += part_one(&opponent, &you);
		} else {
			score += part_two(&opponent, &you);
		}
	}

	println!("Score: {}", score)
}

// calculates the score using the assumptions in part one
fn part_one(opponent: &i32, you: &i32) -> i32 {
	// add on the score based on the shape you play
	let mut score = *you;

	if opponent == you {
		// draw condition
		score += 3;
	} else if you - opponent % 3 == 1 {
		// win conditions:
		//
		// op	 mod3  you   you - mod3
		// 1	 1	   2	 1
		// 2	 2	   3	 1
		// 3	 0	   1	 1
		score += 6
	}

	score
}

// calculates the score using the assumptions in part two
fn part_two(opponent: &i32, you: &i32) -> i32 {
	return if you == &2 {
		// draw condition so same shape as opponent
		opponent + 3
	} else if you == &3 {
		// on win: points = you + 6
		// 				  = (1 + opponent % 3) + 6
		//				  = opponent % 3 + 6
		opponent % 3 + 7
	} else {
		// loss condition
		if opponent == &1 {
			3
		} else if opponent == &2 {
			1
		} else {
			2
		}
	}
}

// Converts the letters to numeric values
fn to_value(s: &str) -> i32 {
	return if s == "A" || s == "X" {
		1 // rock
	} else if s == "B" || s == "Y" {
		2 // paper
	} else {
		3 // scissors
	}
}
