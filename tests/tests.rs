#![feature(test)]
extern crate test;

use advent::days::{
    day_01,
    day_02,
};

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