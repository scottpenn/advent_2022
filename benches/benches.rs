#![feature(test)]
extern crate test;

use test::Bencher;

// Day 01
use advent::days::day_01;

#[bench]
fn day_01_star_one(b: &mut Bencher) {
    b.iter(|| day_01::star_one());
}

#[bench]
fn day_01_star_two(b: &mut Bencher) {
    b.iter(|| day_01::star_two());
}

// Day 02
use advent::days::day_02;

#[bench]
fn day_02_star_one(b: &mut Bencher) {
    b.iter(|| day_02::star_one());
}

#[bench]
fn day_02_star_two(b: &mut Bencher) {
    b.iter(|| day_02::star_two());
}

// Day 03
use advent::days::day_03;

#[bench]
fn day_03_star_one(b: &mut Bencher) {
    b.iter(|| day_03::star_one());
}

#[bench]
fn day_03_star_two(b: &mut Bencher) {
    b.iter(|| day_03::star_two());
}

// Day 04
use advent::days::day_04;

#[bench]
fn day_04_star_one(b: &mut Bencher) {
    b.iter(|| day_04::star_one());
}

#[bench]
fn day_04_star_two(b: &mut Bencher) {
    b.iter(|| day_04::star_two());
}

// Day 05
use advent::days::day_05;

#[bench]
fn day_05_star_one(b: &mut Bencher) {
    b.iter(|| day_05::star_one());
}

#[bench]
fn day_05_star_two(b: &mut Bencher) {
    b.iter(|| day_05::star_two());
}

// Day 06
use advent::days::day_06;

#[bench]
fn day_06_star_one(b: &mut Bencher) {
    b.iter(|| day_06::star_one());
}

#[bench]
fn day_06_star_two(b: &mut Bencher) {
    b.iter(|| day_06::star_two());
}

// Day 07
use advent::days::day_07;

#[bench]
fn day_07_star_one(b: &mut Bencher) {
    b.iter(|| day_07::star_one());
}

#[bench]
fn day_07_star_two(b: &mut Bencher) {
    b.iter(|| day_07::star_two());
}

// Day 08
use advent::days::day_08;

#[bench]
fn day_08_star_one(b: &mut Bencher) {
    b.iter(|| day_08::star_one());
}

#[bench]
fn day_08_star_two(b: &mut Bencher) {
    b.iter(|| day_08::star_two());
}
