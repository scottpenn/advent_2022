
use std::fs;
use once_cell::sync::Lazy;

static INPUT: Lazy<String> = Lazy::new(|| {
    fs::read_to_string("inputs/day_08.txt").expect("Unable to read from input file.")
});

pub fn star_one() -> u32 {
    0
}

pub fn star_two() -> u32 {
    0
}
            