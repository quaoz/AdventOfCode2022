use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
	// read data from file using a buffered reader
	let data = "data/input";
	let file = File::open(data).expect("Error reading file");
	let buffer_reader = BufReader::new(file);

	let mut head = (0, 0);
	let mut tail = (0, 0);

	let knot_count = 8;
	let mut knots: Vec<(i32, i32)> = Vec::new();

	for _ in 0..knot_count {
		knots.push((0, 0));
	}

	let mut tail_positions: HashSet<(i32, i32)> = HashSet::new();
	tail_positions.insert(tail);

	// iterate over the file lines
	for line in buffer_reader.lines() {
		let line = line.expect("Unable to read line");

		// extract the direction and distance
		let operation: Vec<&str> = line.split(' ').collect();
		let direction = operation[0];
		let distance = operation[1].parse::<i32>().unwrap();

		// move the knots one square at a time
		for _ in 0..distance {
			// move the head
			if direction == "L" {
				head.0 -= 1;
			} else if direction == "R" {
				head.0 += 1;
			} else if direction == "U" {
				head.1 -= 1;
			} else {
				head.1 += 1;
			}

			// move the first knot
			knots[0] = move_knot(head, knots[0]);

			// update the positions of all the knots between the head and tail
			for i in 0..knot_count - 1 {
				knots[i + 1] = move_knot(knots[i], knots[i + 1])
			}

			// move the tail and add it's position to the set
			tail = move_knot(knots[knot_count - 1], tail);
			tail_positions.insert(tail);
		}
	}

	println!("Tail was in {} different positions", tail_positions.len())
}

fn move_knot(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
	let mut position = tail;

	// are they touching? x and y within one square of each other
	if (head.0 - tail.0 as i32).abs() > 1 || (head.1 - tail.1 as i32).abs() > 1 {
		// same x value so move in y
		if head.1 > tail.1 {
			// head is above tail so move up
			position.1 += 1;
		} else if head.1 < tail.1 {
			// head is bellow tail so move up
			position.1 -= 1;
		}

		// same y value so move in x
		if head.0 > tail.0 {
			// head is to the right of tail so move right
			position.0 += 1;
		} else if head.0 < tail.0 {
			// head is to the left of tail so move left
			position.0 -= 1;
		}
	}

	position
}
