use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
	// read data from file using a buffered reader
	let data = "data/input";
	let file = File::open(data).expect("Error reading file");
	let buffer_reader = BufReader::new(file);

	// read the file into a 2d vector
	let trees: Vec<Vec<u32>> = buffer_reader
		.lines()
		.map(|line| {
			line.unwrap()
				.chars()
				.collect::<Vec<char>>()
				.iter()
				.map(|c| c.to_string().parse::<u32>().unwrap())
				.collect()
		})
		.collect();

	// account for the trees on the edge
	let mut visible = (trees.len() * 2) + (trees[0].len() * 2) - 4;
	let mut highest_scenic = 0;

	// only check trees that are not on the edge
	for y in 1..trees.len() - 1 {
		for x in 1..trees[0].len() - 1 {
			let current = trees[y][x];
			let mut hidden: bool = false;

			let mut scenic = 1;
			let mut clear = 0;

			// find the number of visible trees to the left
			for left in (0..x).rev() {
				if trees[y][left] < current {
					clear += 1;
				} else {
					clear += 1;
					break;
				}
			}

			scenic *= clear;
			clear = 0;

			// find the number of visible trees to the right
			for right in x + 1..trees[0].len() {
				if trees[y][right] < current {
					clear += 1;
				} else {
					clear += 1;
					break;
				}
			}

			scenic *= clear;
			clear = 0;

			// find the number of visible trees up
			for up in (0..y).rev() {
				if trees[up][x] < current {
					clear += 1;
				} else {
					clear += 1;
					break;
				}
			}

			scenic *= clear;
			clear = 0;

			// find the number of visible trees down
			for down in y + 1..trees.len() {
				if trees[down][x] < current {
					clear += 1;
				} else {
					clear += 1;
					break;
				}
			}

			scenic *= clear;

			if scenic > highest_scenic {
				highest_scenic = scenic;
			}

			// check whether the tree is visible from the left
			for left in 0..x {
				if trees[y][left] >= current {
					hidden = true;
					break;
				}
			}

			if hidden {
				hidden = false;

				// check whether the tree is visible from the right
				for right in x + 1..trees[0].len() {
					if trees[y][right] >= current {
						hidden = true;
						break;
					}
				}

				if hidden {
					hidden = false;

					// check whether the tree is visible from the top
					for up in 0..y {
						if trees[up][x] >= current {
							hidden = true;
							break;
						}
					}

					if hidden {
						hidden = false;

						// check whether the tree is visible from the bottom
						for down in y + 1..trees.len() {
							if trees[down][x] >= current {
								hidden = true;
								break;
							}
						}
					}
				}
			}

			if !hidden {
				visible += 1;
			}
		}
	}

	println!("There are {} trees are visible", visible);
	println!("The maximum scenic value is {} ", highest_scenic);
}
