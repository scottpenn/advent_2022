use itertools::Itertools;
use once_cell::sync::Lazy;

use std::{collections::HashSet, fs};

static INPUT: Lazy<String> =
    Lazy::new(|| fs::read_to_string("inputs/day_03.txt").expect("Unable to read from input file."));

pub fn star_one() -> u32 {
    INPUT
        .lines()
        .map(|line| {
            let (one, two) = line.split_at(line.len() / 2);
            let one: HashSet<u8> = HashSet::from_iter(one.bytes());
            for item in two.as_bytes() {
                if one.contains(item) {
                    if item.is_ascii_lowercase() {
                        return (item - b'a') as u32 + 1;
                    } else {
                        return (item - b'A') as u32 + 27;
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
            if badge.is_ascii_lowercase() {
                (badge - b'a') as u32 + 1
            } else {
                (badge as u8 - b'A') as u32 + 27
            }
        })
        .sum()
}
