use once_cell::sync::Lazy;
use std::fs;

static INPUT: Lazy<String> =
    Lazy::new(|| fs::read_to_string("inputs/day_02.txt").expect("Unable to read from input file."));

pub fn star_one() -> u32 {
    INPUT.lines().map(|line| {
        match line {
            // Win
            "C X" => 1 + 6,
            "A Y" => 2 + 6,
            "B Z" => 3 + 6,
            // Lose
            "B X" => 1 + 0,
            "C Y" => 2 + 0,
            "A Z" => 3 + 0,
            // Tie
            "A X" => 1 + 3,
            "B Y" => 2 + 3,
            "C Z" => 3 + 3,
            _ => unreachable!("No other pairs are possible.")
        }
    }).sum()
}

pub fn star_two() -> u32 {
    INPUT.lines().map(|line| {
        match line {
            // Win
            "A Z" => 2 + 6,
            "B Z" => 3 + 6,
            "C Z" => 1 + 6,
            // Lose
            "A X" => 3 + 0,
            "B X" => 1 + 0,
            "C X" => 2 + 0,
            // Tie
            "A Y" => 1 + 3,
            "B Y" => 2 + 3,
            "C Y" => 3 + 3,
            _ => unreachable!("No other pairs are possible.")
        }
    }).sum()
}
