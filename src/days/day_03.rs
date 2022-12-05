use itertools::Itertools;
use once_cell::sync::Lazy;
use rayon::prelude::*;

use std::{collections::HashSet, fs};

static INPUT: Lazy<String> =
    Lazy::new(|| fs::read_to_string("inputs/day_03.txt").expect("Unable to read from input file."));

pub fn star_one() -> u32 {
    INPUT
        .par_lines()
        .map(|line| {
            let (one, two) = line.split_at(line.len() / 2);
            let one: HashSet<u8> = HashSet::from_iter(one.bytes());
            for item in two.bytes() {
                if one.contains(&item) {
                    return match item {
                        b'a'..=b'z' => {(item - b'a') as u32 + 1}
                        b'A'..=b'Z' => {(item - b'A') as u32 + 27}
                        _ => 0
                    }
                }
            }
            unreachable!()
        })
        .sum()
}

pub fn star_two() -> u32 {
    INPUT
        .lines()
        .chunks(3)
        .into_iter()
        .map(|mut sacks| {
            let mut first: HashSet<char> = HashSet::from_iter(sacks.next().unwrap().chars());
            sacks.for_each(|sack| {
                first.retain(|item| sack.contains(*item))
            });

            let badge = *first.iter().next().unwrap() as u8;
            match badge {
                b'a'..=b'z' => {(badge - b'a') as u32 + 1}
                b'A'..=b'Z' => {(badge - b'A') as u32 + 27}
                _ => 0
            }
        })
        .sum()
}
