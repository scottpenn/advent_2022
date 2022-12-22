use itertools::Itertools;
use once_cell::sync::Lazy;
use std::{collections::HashSet, fs};

static INPUT: Lazy<String> =
    Lazy::new(|| fs::read_to_string("inputs/day_15.txt").expect("Unable to read from input file."));

pub fn star_one() -> i64 {
    let target = 2_000_000;
    let mut beacons_at_target = HashSet::new();
    let mut min_x: Option<i64> = None;
    let mut max_x: Option<i64> = None;
    INPUT.lines().for_each(|line| {
        let coordinates: Vec<&str> = line.split('=').collect();
        let xs = coordinates[1]
            .split_once(',')
            .unwrap()
            .0
            .parse::<i64>()
            .unwrap();
        let ys = coordinates[2]
            .split_once(':')
            .unwrap()
            .0
            .parse::<i64>()
            .unwrap();
        let xb = coordinates[3]
            .split_once(',')
            .unwrap()
            .0
            .parse::<i64>()
            .unwrap();
        let yb = coordinates[4].parse::<i64>().unwrap();

        let distance = (xs - xb).abs() + (ys - yb).abs();
        let distance_to_target = (ys - target).abs();
        if distance >= distance_to_target {
            let slack = distance - distance_to_target;
            if let Some(x) = min_x {
                min_x = Some(x.min(xs - slack))
            } else {
                min_x = Some(xs - slack)
            }
            if let Some(x) = max_x {
                max_x = Some(x.max(xs + slack))
            } else {
                max_x = Some(xs + slack)
            }
        }
        if yb == target {
            beacons_at_target.insert(xb);
        }
    });
    let range = (max_x.unwrap() - min_x.unwrap()) + 1;
    range
        - beacons_at_target
            .into_iter()
            .filter(|x| *x <= max_x.unwrap() && *x >= min_x.unwrap())
            .count() as i64
}

pub fn star_two() -> i64 {
    let signals: Vec<_> = INPUT
        .lines()
        .map(|line| {
            let coordinates: Vec<&str> = line.split('=').collect();
            let xs = coordinates[1]
                .split_once(',')
                .unwrap()
                .0
                .parse::<i64>()
                .unwrap();
            let ys = coordinates[2]
                .split_once(':')
                .unwrap()
                .0
                .parse::<i64>()
                .unwrap();
            let xb = coordinates[3]
                .split_once(',')
                .unwrap()
                .0
                .parse::<i64>()
                .unwrap();
            let yb = coordinates[4].parse::<i64>().unwrap();
            let distance = (xs - xb).abs() + (ys - yb).abs();
            (xs, ys, distance)
        })
        .collect();
    let mut diagonals_up = Vec::new();
    let mut diagonals_down = Vec::new();
    signals.iter().enumerate().for_each(|(i, (x1, y1, d1))| {
        signals
            .iter()
            .cycle()
            .skip(i + 1)
            .take(signals.len())
            .for_each(|(x2, y2, d2)| {
                if ((x1 - x2).abs() + (y1 - y2).abs()) == d1 + d2 + 2 {
                    if x1 > x2 && y1 > y2 {
                        diagonals_up.push((*x1, y1 - (d1 + 1)));
                    } else if x1 < x2 && y1 < y2 {
                        diagonals_up.push((*x1, y1 + (d1 + 1)));
                    } else if x1 < x2 && y1 > y2 {
                        diagonals_down.push((*x1, y1 - (d1 + 1)));
                    } else if x1 > x2 && y1 < y2 {
                        diagonals_up.push((*x1, y1 + (d1 + 1)));
                    }
                }
            });
    });
    let holes = diagonals_up
        .into_iter()
        .cartesian_product(diagonals_down.into_iter())
        .filter_map(|((mut x1, mut y1), (mut x2, mut y2))| {
            if ((x1 - x2).abs() + (y1 - y2).abs()) % 2 == 1 {
                return None;
            }
            if x1 > x2 && y1 > y2 {
                while (x1 - x2).abs() != (y1 - y2).abs() {
                    x2 += 1;
                    y2 += 1;
                }
                Some((x2, y2))
            } else if x1 < x2 && y1 < y2 {
                while (x1 - x2).abs() != (y1 - y2).abs() {
                    x2 -= 1;
                    y2 -= 1;
                }
                Some((x2, y2))
            } else if x1 < x2 && y1 > y2 {
                while (x1 - x2).abs() != (y1 - y2).abs() {
                    x1 += 1;
                    y1 -= 1;
                }
                Some((x1, y1))
            } else {
                while (x1 - x2).abs() != (y1 - y2).abs() {
                    x1 -= 1;
                    y1 += 1;
                }
                Some((x1, y1))
            } 
        }).collect::<HashSet<_>>();

    holes.iter().for_each(|(x, y)| {
        dbg!(x);
        dbg!(y);
        dbg!(x * 4_000_000 + y);
    });
    0
}
