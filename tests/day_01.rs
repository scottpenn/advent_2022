#![feature(test)]
extern crate test;

use advent::days::day_01::{star_one, star_two};

#[test]
fn test_star_one() {
    assert_eq!(68923, star_one());
}

#[test]
fn test_star_two() {
    assert_eq!(200044, star_two());
}