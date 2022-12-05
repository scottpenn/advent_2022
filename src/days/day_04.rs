use once_cell::sync::Lazy;
use rayon::prelude::*;

use std::fs;

static INPUT: Lazy<String> =
    Lazy::new(|| fs::read_to_string("inputs/day_04.txt").expect("Unable to read from input file."));

pub fn star_one() -> u32 {
    INPUT
        .par_lines()
        .map(|line| {
            line.split(',')
                .map(|range| range.split('-').map(|n| n.parse::<u32>().unwrap()))
                .flatten()
                .collect::<Vec<u32>>()
        })
        .map(|ranges| match &ranges[..] {
            &[a, b, c, d] if a <= c && b >= d => 1,
            &[a, b, c, d] if c <= a && d >= b => 1,
            _ => 0,
        })
        .sum()
}

pub fn star_two() -> u32 {
    INPUT
        .par_lines()
        .map(|line| {
            line.split(',')
                .map(|range| range.split('-').map(|n| n.parse::<u32>().unwrap()))
                .flatten()
                .collect::<Vec<u32>>()
        })
        .map(|ranges| match &ranges[..] {
            &[a, _b, c, d] if a >= c && a <= d => 1,
            &[_a, b, c, d] if b >= c && b <= d => 1,
            &[a, b, c, _d] if c >= a && c <= b => 1,
            &[a, b, _c, d] if d >= a && d <= b => 1,
            _ => 0,
        })
        .sum()
}
