use once_cell::sync::Lazy;
use std::{cell::Cell, collections::HashSet, fs};

static INPUT: Lazy<String> =
    Lazy::new(|| fs::read_to_string("inputs/day_09.txt").expect("Unable to read from input file."));

pub fn star_one() -> usize {
    let mut visited = HashSet::new();
    let mut head = (0, 0);
    let mut tail = (0, 0);
    visited.insert(tail);
    INPUT
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .for_each(|(direction, steps)| {
            for _step in 0..steps.parse::<u32>().unwrap() {
                head = match direction {
                    "U" => (head.0, head.1 + 1),
                    "D" => (head.0, head.1 - 1),
                    "L" => (head.0 - 1, head.1),
                    "R" => (head.0 + 1, head.1),
                    _ => {
                        unreachable!()
                    }
                };

                match (head.0 - tail.0, head.1 - tail.1) {
                    // U
                    (0, 2) => tail = (tail.0, tail.1 + 1),
                    (-1, 2) => tail = (tail.0 - 1, tail.1 + 1),
                    (1, 2) => tail = (tail.0 + 1, tail.1 + 1),
                    // D
                    (0, -2) => tail = (tail.0, tail.1 - 1),
                    (-1, -2) => tail = (tail.0 - 1, tail.1 - 1),
                    (1, -2) => tail = (tail.0 + 1, tail.1 - 1),
                    // L
                    (-2, 0) => tail = (tail.0 - 1, tail.1),
                    (-2, -1) => tail = (tail.0 - 1, tail.1 - 1),
                    (-2, 1) => tail = (tail.0 - 1, tail.1 + 1),
                    // R
                    (2, 0) => tail = (tail.0 + 1, tail.1),
                    (2, -1) => tail = (tail.0 + 1, tail.1 - 1),
                    (2, 1) => tail = (tail.0 + 1, tail.1 + 1),
                    _ => (),
                }
                visited.insert(tail);
            }
        });
    visited.len()
}

pub fn star_two() -> usize {
    let mut visited = HashSet::new();
    visited.insert((0, 0));
    let mut rope = vec![(0, 0); 10];
    let knots = Cell::from_mut(&mut rope[..]).as_slice_of_cells();
    INPUT
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .for_each(|(direction, steps)| {
            for _step in 0..steps.parse::<u32>().unwrap() {
                // Move head
                {
                    let head = knots.first().unwrap();
                    head.set(match direction {
                        "U" => (head.get().0, head.get().1 + 1),
                        "D" => (head.get().0, head.get().1 - 1),
                        "L" => (head.get().0 - 1, head.get().1),
                        "R" => (head.get().0 + 1, head.get().1),
                        _ => {
                            unreachable!()
                        }
                    });
                }
                // Move knots in chain reaction
                for window in knots.windows(2) {
                    let [head, tail] = window else {unreachable!()};

                    match (head.get().0 - tail.get().0, head.get().1 - tail.get().1) {
                        // U
                        (0, 2) => tail.set((tail.get().0, tail.get().1 + 1)),
                        (-1, 2) => tail.set((tail.get().0 - 1, tail.get().1 + 1)),
                        (1, 2) => tail.set((tail.get().0 + 1, tail.get().1 + 1)),
                        // D
                        (0, -2) => tail.set((tail.get().0, tail.get().1 - 1)),
                        (-1, -2) => tail.set((tail.get().0 - 1, tail.get().1 - 1)),
                        (1, -2) => tail.set((tail.get().0 + 1, tail.get().1 - 1)),
                        // L
                        (-2, 0) => tail.set((tail.get().0 - 1, tail.get().1)),
                        (-2, -1) => tail.set((tail.get().0 - 1, tail.get().1 - 1)),
                        (-2, 1) => tail.set((tail.get().0 - 1, tail.get().1 + 1)),
                        // R
                        (2, 0) => tail.set((tail.get().0 + 1, tail.get().1)),
                        (2, -1) => tail.set((tail.get().0 + 1, tail.get().1 - 1)),
                        (2, 1) => tail.set((tail.get().0 + 1, tail.get().1 + 1)),
                        // Diagonal
                        (2, 2) => tail.set((tail.get().0 + 1, tail.get().1 + 1)),
                        (-2, 2) => tail.set((tail.get().0 - 1, tail.get().1 + 1)),
                        (2, -2) => tail.set((tail.get().0 + 1, tail.get().1 - 1)),
                        (-2, -2) => tail.set((tail.get().0 - 1, tail.get().1 - 1)),
                        _ => (),
                    }
                }
                // Insert position of last knot
                visited.insert(knots.last().unwrap().get());
            }
        });
    visited.len()
}
