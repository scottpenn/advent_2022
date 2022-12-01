#![feature(test)]
extern crate test;

use advent::days::day_01;
use test::Bencher;

#[bench]
fn bench_star_one(b: &mut Bencher) {
    b.iter(|| day_01::star_one());
}

#[bench]
fn bench_star_two(b: &mut Bencher) {
    b.iter(|| day_01::star_two());
}
