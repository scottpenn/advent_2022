#![feature(test)]
extern crate test;

use test::Bencher;

use advent::days::{
    day_01, 
    day_02,
    day_03,
};

// Day 01
#[bench]
fn day_01_star_one(b: &mut Bencher) {
    b.iter(|| day_01::star_one());
}

#[bench]
fn day_01_star_two(b: &mut Bencher) {
    b.iter(|| day_01::star_two());
}

// Day 02
#[bench]
fn day_02_star_one(b: &mut Bencher) {
    b.iter(|| day_02::star_one());
}

#[bench]
fn day_02_star_two(b: &mut Bencher) {
    b.iter(|| day_02::star_two());
}

// Day 03
#[bench]
fn day_03_star_one(b: &mut Bencher) {
    b.iter(|| day_03::star_one());
}

#[bench]
fn day_03_star_two(b: &mut Bencher) {
    b.iter(|| day_03::star_two());
}
