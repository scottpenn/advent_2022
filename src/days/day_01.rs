use std::fs;
use once_cell::sync::Lazy;

static INPUT: Lazy<String> = Lazy::new(|| {
    fs::read_to_string("inputs/day_01").expect("Unable to read from input file.")
});

fn get_calories() -> Vec<u32> {
    INPUT
        .split("\n\n")
        .map(|list| list.lines().map(|food| food.parse::<u32>().unwrap()).sum())
        .collect()
}


pub fn star_one() -> u32 {
    get_calories().into_iter().max().unwrap()
}

pub fn star_two() -> u32 {
    let mut calories = get_calories();
    calories.sort();
    calories.iter().rev().take(3).sum::<u32>()
}

#[cfg(test)]
mod tests_01 {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(68923, star_one());
    }

    #[test]
    fn test_star_two() {
        assert_eq!(200044, star_two());
    }
}
