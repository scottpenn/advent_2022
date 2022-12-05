use once_cell::sync::Lazy;

use std::{collections::VecDeque, fs};

static INPUT: Lazy<String> =
    Lazy::new(|| fs::read_to_string("inputs/day_05.txt").expect("Unable to read from input file."));

fn get_stacks() -> Vec<VecDeque<u8>> {
    let mut stacks = vec![VecDeque::new(); 9];
    INPUT.lines().take(8).for_each(|line| {
        line.as_bytes()
            .iter()
            .skip(1)
            .step_by(4)
            .enumerate()
            .for_each(|(i, cargo)| match cargo {
                b' ' => {}
                _ => {
                    stacks[i].push_front(*cargo);
                }
            });
    });
    stacks
}

pub fn star_one() -> String {
    let mut stacks = get_stacks();

    INPUT.lines().skip(10).for_each(|line| {
        let amount = line[5..]
            .split_once(" from ")
            .unwrap()
            .0
            .parse::<u32>()
            .unwrap();

        let source = line
            .chars()
            .rev()
            .skip(5)
            .next()
            .unwrap()
            .to_digit(10)
            .unwrap();

        let dest = line
            .chars()
            .last()
            .unwrap()
            .to_digit(10)
            .unwrap();

        for _ in 1..=amount {
            let cargo = stacks[source as usize - 1].pop_back().unwrap();
            stacks[dest as usize - 1].push_back(cargo);
        }
    });

    String::from_iter(
        stacks
            .into_iter()
            .map(|mut stack| stack.pop_back().unwrap() as char),
    )
}

pub fn star_two() -> String {
    let mut stacks = get_stacks();

    INPUT.lines().skip(10).for_each(|line| {
        let amount = line[5..]
            .split_once(" from ")
            .unwrap()
            .0
            .parse::<u32>()
            .unwrap();

        let source = line
            .chars()
            .rev()
            .skip(5)
            .next()
            .unwrap()
            .to_digit(10)
            .unwrap();

        let dest = line
            .chars()
            .last()
            .unwrap()
            .to_digit(10)
            .unwrap();

        let mut temp = Vec::new();
        for _ in 1..=amount {
            let cargo = stacks[source as usize - 1].pop_back().unwrap();
            temp.push(cargo);
        }
        for _ in 1..=amount {
            let cargo = temp.pop().unwrap();
            stacks[dest as usize - 1].push_back(cargo);
        }
    });

    String::from_iter(
        stacks
            .into_iter()
            .map(|mut stack| stack.pop_back().unwrap() as char),
    )
}
