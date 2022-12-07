use once_cell::sync::Lazy;

use std::{collections::HashSet, fs};

static INPUT: Lazy<String> =
    Lazy::new(|| fs::read_to_string("inputs/day_06.txt").expect("Unable to read from input file."));

pub fn get_marker(n: usize) -> usize {
    INPUT
    .as_bytes()
    .windows(n)
    .take_while(|window| {
        let mut set = HashSet::new();
        window.iter().any(|signal| !set.insert(signal))
    })
    .count()
    + n
}

pub fn star_one() -> usize {
    get_marker(4)
}

pub fn star_two() -> usize {
    get_marker(14)
}
