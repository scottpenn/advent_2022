#![feature(test)]
extern crate test;

use advent::days::day_01::{star_one, star_two};
use test::Bencher;

#[bench]
fn bench_star_one(b: &mut Bencher) {
    b.iter(|| star_one());
}

#[bench]
fn bench_star_two(b: &mut Bencher) {
    b.iter(|| star_two());
}
