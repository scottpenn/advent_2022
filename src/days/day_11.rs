use once_cell::sync::Lazy;
use std::{collections::VecDeque, fs};

static INPUT: Lazy<String> =
    Lazy::new(|| fs::read_to_string("inputs/day_11.txt").expect("Unable to read from input file."));

pub fn star_one() -> usize {
    let mut items = Vec::new();
    let mut operations = Vec::new();
    let mut tests = Vec::new();
    let mut test_success = Vec::new();
    let mut test_failure = Vec::new();

    INPUT.split("\n\n").for_each(|monkey| {
        let mut info = monkey.lines().skip(1);
        // Parse item list
        items.push(
            info.next()
                .unwrap()
                .split("Starting items: ")
                .last()
                .unwrap()
                .split(", ")
                .map(|item| item.parse::<u32>().unwrap())
                .collect::<VecDeque<u32>>(),
        );
        // Parse operations
        operations.push(
            info.next()
                .unwrap()
                .split("Operation: new = old ")
                .last()
                .unwrap()
                .split_once(' ')
                .unwrap(),
        );
        // Parse tests
        tests.push(
            info.next()
                .unwrap()
                .split(' ')
                .last()
                .unwrap()
                .parse::<u32>()
                .unwrap(),
        );
        // Next monkey on test success
        test_success.push(
            info.next()
                .unwrap()
                .split(' ')
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap(),
        );
        // Next monkey on test failure
        test_failure.push(
            info.next()
                .unwrap()
                .split(' ')
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap(),
        );
    });

    let mut monkey_business = (0..20).fold(vec![0; 8], |monkeys, _round| {
        monkeys
            .iter()
            .enumerate()
            .map(|(i, monkey)| {
                let item_count = items[i].len();
                let worries = items[i]
                    .drain(..)
                    .map(|item| {
                        let worry = match operations[i] {
                            ("+", "old") => item + item,
                            ("*", "old") => item * item,
                            ("+", n) => item + n.parse::<u32>().unwrap(),
                            ("*", n) => item * n.parse::<u32>().unwrap(),
                            _ => unreachable!(),
                        };
                        worry / 3
                    })
                    .collect::<Vec<u32>>();
                worries.iter().for_each(|worry| {
                    if worry % tests[i] == 0 {
                        items[test_success[i]].push_back(*worry)
                    } else {
                        items[test_failure[i]].push_back(*worry)
                    }
                });
                *monkey + item_count
            })
            .collect()
    });
    monkey_business.sort();

    monkey_business.iter().rev().take(2).product()
}

pub fn star_two() -> u128 {
    let mut items = Vec::new();
    let mut operations = Vec::new();
    let mut tests = Vec::new();
    let mut test_success = Vec::new();
    let mut test_failure = Vec::new();

    INPUT.split("\n\n").for_each(|monkey| {
        let mut info = monkey.lines().skip(1);
        // Parse item list
        items.push(
            info.next()
                .unwrap()
                .split("Starting items: ")
                .last()
                .unwrap()
                .split(", ")
                .map(|item| item.parse::<u128>().unwrap())
                .collect::<VecDeque<u128>>(),
        );
        // Parse operations
        operations.push(
            info.next()
                .unwrap()
                .split("Operation: new = old ")
                .last()
                .unwrap()
                .split_once(' ')
                .unwrap(),
        );
        // Parse tests
        tests.push(
            info.next()
                .unwrap()
                .split(' ')
                .last()
                .unwrap()
                .parse::<u128>()
                .unwrap(),
        );
        // Next monkey on test success
        test_success.push(
            info.next()
                .unwrap()
                .split(' ')
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap(),
        );
        // Next monkey on test failure
        test_failure.push(
            info.next()
                .unwrap()
                .split(' ')
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap(),
        );
    });

    let lcm = tests.iter().product::<u128>();
    let mut monkey_business = (0..10_000).fold(vec![0; 8], |monkeys, _round| {
        monkeys
            .iter()
            .enumerate()
            .map(|(i, monkey)| {
                let item_count = items[i].len();
                let worries = items[i]
                    .drain(..)
                    .map(|item| {
                        let worry = match operations[i] {
                            ("+", "old") => item * 2,
                            ("*", "old") => item * item,
                            ("+", n) => item + n.parse::<u128>().unwrap(),
                            ("*", n) => item * n.parse::<u128>().unwrap(),
                            _ => unreachable!(),
                        };
                        worry % lcm
                    })
                    .collect::<Vec<u128>>();
                worries.iter().for_each(|worry| {
                    if worry % tests[i] == 0 {
                        items[test_success[i]].push_back(*worry)
                    } else {
                        items[test_failure[i]].push_back(*worry)
                    }
                });
                *monkey + item_count as u128
            })
            .collect()
    });
    monkey_business.sort();

    monkey_business.iter().rev().take(2).product()
}
