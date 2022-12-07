#![feature(test)]
extern crate test;

use advent::days::{
    day_01,
    day_02,
    day_03,
    day_04,
    day_05,
    day_06,
};

// Day 01
#[test]
fn day_01_star_one() {
    dbg!(day_01::star_one());
    assert_eq!(68923, day_01::star_one());
}

#[test]
fn day_01_star_two() {
    dbg!(day_01::star_two());
    assert_eq!(200044, day_01::star_two());
}

// Day 02
#[test]
fn day_02_star_one() {
    dbg!(day_02::star_one());
    assert_eq!(11873, day_02::star_one());
}

#[test]
fn day_02_star_two() {
    dbg!(day_02::star_two());
    assert_eq!(12014, day_02::star_two());
}

// Day 03
#[test]
fn day_03_star_one() {
    assert_eq!(7848, day_03::star_one());
}

#[test]
fn day_03_star_two() {
    assert_eq!(2616, day_03::star_two());
}

// Day 04
#[test]
fn day_04_star_one() {
    assert_eq!(483, day_04::star_one());
}

#[test]
fn day_04_star_two() {
    assert_eq!(874, day_04::star_two());
}

// Day 05
#[test]
fn day_05_star_one() {
    assert_eq!("TQRFCBSJJ".to_string(), day_05::star_one());
}

#[test]
fn day_05_star_two() {
    assert_eq!("RMHFJNVFP".to_string(), day_05::star_two());
}

// Day 06
#[test]
fn day_06_star_one() {
    assert_eq!(1876, day_06::star_one());
}

#[test]
fn day_06_star_two() {
    assert_eq!(2202, day_06::star_two());
}