use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
	// read data from file using a buffered reader
	let data = "data/input";
	let file = File::open(data).expect("Error reading file");
	let buffer_reader = BufReader::new(file);

	let mut path: Vec<String> = Vec::new();
	let mut directories: HashMap<String, u32> = HashMap::new();

	// iterate over the file lines
	for line in buffer_reader.lines() {
		let line = line.expect("Unable to read line");

		let command: Vec<&str> = line.split(' ').collect();
		let identifier = command[0];
		let value = command[1];

		if identifier == "$" {
			if value == "cd" {
				let directory = command[2];

				// path stores the full path of the current directory
				if directory == ".." {
					path.pop();
				} else {
					if directory == "/" {
						path.push(String::from("/"))
					} else {
						path.push(String::from(directory) + "/");
					}

					// add every directory which is accessed to the map
					directories.insert(path.join(""), 0);
				}
			}
		} else {
			if identifier != "dir" {
				let size = identifier.parse::<u32>().unwrap();

				// add the file to the current directory and all its parents total sizes
				for i in 0..path.len() {
					let name: String = path[..i + 1].join("");
					*directories.entry(name).or_insert(0) += size;
				}
			}
		}
	}

	// find sum of directories smaller than 100,000
	let mut sum = 0;
	for (_, size) in &directories {
		if size <= &100_000 {
			sum += size;
		}
	}

	println!("Sum of directories under 100,000: {}", sum);

	let total_space = 70_000_000;
	let min_space = 30_000_000;

	// calculate the minimum size needed
	let min_remove = min_space - (total_space - directories.get("/").unwrap());
	let mut candidates: Vec<u32> = Vec::new();

	// find directories above the minimum size
	for (_, size) in directories {
		if size >= min_remove {
			candidates.push(size);
		}
	}

	println!(
		"Minimum directory for {} free {}",
		min_space,
		candidates.iter().min().unwrap()
	)
}
