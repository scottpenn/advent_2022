use once_cell::sync::Lazy;

use std::{collections::HashMap, fs, str};

static INPUT: Lazy<String> =
    Lazy::new(|| fs::read_to_string("inputs/day_07.txt").expect("Unable to read from input file."));

fn read_commands() -> HashMap<String, u64> {
    let mut sizes: HashMap<String, u64> = HashMap::new();
    let mut dirs: Vec<&str> = Vec::new();

    INPUT.lines().for_each(|line| {
        match line.chars().nth(0).unwrap() {
            '$' => {
                match &line[2..=3] {
                    "cd" => {
                        match &line[5..] {
                            // Pop directory from stack for cd ..
                            ".." => {
                                dirs.pop();
                            }
                            // Push directory to stack for cd [dir]
                            dir => {
                                dirs.push(dir);
                            }
                        }
                    }
                    // Nothing to implement for the ls command
                    _ => {}
                }
            }
            // Work with the output of ls.
            _ => {
                match line.split_once(" ").unwrap().0 {
                    // Directory names can be skipped
                    "dir" => {}
                    // Update sizes for every directory in the stack.
                    size => {
                        let size = size.parse::<u64>().unwrap();
                        for i in 1..=dirs.len() {
                            let dir = dirs[..i].join("/");
                            sizes.entry(dir).and_modify(|e| *e += size).or_insert(size);
                        }
                    }
                }
            }
        }
    });
    sizes
}

pub fn star_one() -> u64 {
    read_commands().values().filter(|size| **size <= 100_000).sum()
}

pub fn star_two() -> u64 {
    let sizes = read_commands();

    let disk_space_remaining = 70_000_000 - sizes.get("/").unwrap();
    let disk_space_needed = 30_000_000 - disk_space_remaining;

    *sizes.values().filter(|size| **size >= disk_space_needed).min().unwrap()
}
