#![feature(test)]
extern crate test;

use test::Bencher;

use advent::days::{
    day_01, 
    day_02,
    day_03,
    day_04,
    day_05,
    day_06,
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

// Day 04
#[bench]
fn day_04_star_one(b: &mut Bencher) {
    b.iter(|| day_04::star_one());
}

#[bench]
fn day_04_star_two(b: &mut Bencher) {
    b.iter(|| day_04::star_two());
}

// Day 05
#[bench]
fn day_05_star_one(b: &mut Bencher) {
    b.iter(|| day_05::star_one());
}

#[bench]
fn day_05_star_two(b: &mut Bencher) {
    b.iter(|| day_05::star_two());
}

// Day 06
#[bench]
fn day_06_star_one(b: &mut Bencher) {
    b.iter(|| day_06::star_one());
}

#[bench]
fn day_06_star_two(b: &mut Bencher) {
    b.iter(|| day_06::star_two());
}