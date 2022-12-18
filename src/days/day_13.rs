use itertools::{EitherOrBoth::*, Itertools};
use once_cell::sync::Lazy;
use std::cmp::Ordering;
use std::{fs, str};

static INPUT: Lazy<String> =
    Lazy::new(|| fs::read_to_string("inputs/day_13.txt").expect("Unable to read from input file."));

fn compare(left: &[u8], right: &[u8]) -> Option<bool> {
    if left.len() == 0 && right.len() == 0 {
        return None;
    } else if left.len() == 0 {
        return Some(true);
    } else if right.len() == 0 {
        return Some(false);
    }

    match (left[0], right[0]) {
        // Compare two lists
        (b'[', b'[') => {
            let mut level: u32 = 0;
            let l = left[1..(left.len() - 1)].split(|c| match c {
                b'[' => {
                    level += 1;
                    false
                }
                b']' => {
                    level -= 1;
                    false
                }
                b',' => level == 0,
                _ => false,
            });
            let mut level: u32 = 0;
            let r = right[1..(right.len() - 1)].split(|c| match c {
                b'[' => {
                    level += 1;
                    false
                }
                b']' => {
                    level -= 1;
                    false
                }
                b',' => level == 0,
                _ => false,
            });
            for pair in l.zip_longest(r) {
                let result = match pair {
                    Both(l, r) => compare(l, r),
                    Left(_l) => Some(false),
                    Right(_r) => Some(true),
                };
                if result.is_some() {
                    return result;
                }
            }
            None
        }
        // One List
        (_l, b'[') => compare(&[[b'['], [b']']].join(left), right),
        (b'[', _r) => compare(left, &[[b'['], [b']']].join(right)),
        // Two integers
        (_l, _r) => {
            let l = str::from_utf8(left).unwrap().parse::<u32>().unwrap();
            let r = str::from_utf8(right).unwrap().parse::<u32>().unwrap();
            if l < r {
                Some(true)
            } else if l > r {
                Some(false)
            } else {
                None
            }
        }
    }
}

pub fn star_one() -> usize {
    INPUT
        .split("\n\n")
        .enumerate()
        .filter_map(|(i, pair)| {
            let mut packets = pair.lines();
            let left = packets.next().unwrap();
            let right = packets.next().unwrap();
            if let Some(result) = compare(left.as_bytes(), right.as_bytes()) {
                if result {
                    Some(i + 1)
                } else {
                    Some(0)
                }
            } else {
                None
            }
        })
        .sum()
}

pub fn star_two() -> usize {
    let mut dividers = INPUT.clone();
    dividers.push_str("\n\n[[2]]\n[[6]]");
    dividers
        .split("\n\n")
        .flat_map(|pair| pair.lines().collect::<Vec<&str>>())
        .sorted_by(|a, b| {
            match compare(a.as_bytes(), b.as_bytes()) {
                Some(true) => Ordering::Less,
                Some(false) => Ordering::Greater,
                _ => Ordering::Equal
            }
        }).enumerate().filter_map(|(i, packet)| {
            if packet == "[[2]]" {
                Some(i + 1)
            } else if packet == "[[6]]" {
                Some(i + 1)
            } else {
                None
            }
        }).product()
}
