use once_cell::sync::Lazy;
use std::{
    collections::{HashMap, VecDeque},
    fs,
};

static INPUT: Lazy<String> =
    Lazy::new(|| fs::read_to_string("inputs/day_17.txt").expect("Unable to read from input file."));

#[derive(Debug)]
enum Shape {
    Line,
    Plus,
    Corner,
    Stake,
    Square,
}

fn init_rocks(shape: &Shape, h: usize) -> Vec<(usize, usize)> {
    match shape {
        Shape::Line => {
            vec![(2, h + 3), (3, h + 3), (4, h + 3), (5, h + 3)]
        }
        Shape::Plus => {
            vec![(3, h + 5), (2, h + 4), (3, h + 4), (4, h + 4), (3, h + 3)]
        }
        Shape::Corner => {
            vec![(4, h + 5), (4, h + 4), (2, h + 3), (3, h + 3), (4, h + 3)]
        }
        Shape::Stake => {
            vec![(2, h + 6), (2, h + 5), (2, h + 4), (2, h + 3)]
        }
        Shape::Square => {
            vec![(2, h + 4), (3, h + 4), (2, h + 3), (3, h + 3)]
        }
    }
}

pub fn star_one() -> usize {
    let mut height = 0;
    let mut max_height = 0;
    let mut rock_count = 0;
    let target_count = 2022;

    let mut chamber: VecDeque<Vec<bool>> = VecDeque::from(vec![vec![false; 7]; 30]);

    let mut shape = Shape::Line;
    let mut falling = init_rocks(&shape, height);

    let mut winds = INPUT.chars().cycle();

    while rock_count < target_count {
        let wind = winds.next().unwrap();
        // Apply gust of wind
        match wind {
            '<' => {
                if falling.iter().all(|(x, y)| *x > 0 && !chamber[*y][*x - 1]) {
                    falling = falling
                        .clone()
                        .into_iter()
                        .map(|(x, y)| (x - 1, y))
                        .collect();
                }
            }
            '>' => {
                if falling.iter().all(|(x, y)| *x < 6 && !chamber[*y][*x + 1]) {
                    falling = falling
                        .clone()
                        .into_iter()
                        .map(|(x, y)| (x + 1, y))
                        .collect();
                }
            }
            _ => unreachable!(),
        };
        // Move rocks down one level
        if falling.iter().all(|(x, y)| *y > 0 && !chamber[*y - 1][*x]) {
            falling = falling
                .clone()
                .into_iter()
                .map(|(x, y)| (x, y - 1))
                .collect();
        } else {
            // Rocks settle
            falling.iter().for_each(|(x, y)| {
                chamber[*y][*x] = true;
            });
            rock_count += 1;
            let height_change = height.max(falling[0].1 + 1) - height;
            max_height += height_change;
            height += height_change;
            while height > 20 {
                chamber.pop_front();
                chamber.push_back(vec![false; 7]);
                height -= 1;
            }
            match shape {
                Shape::Line => {
                    falling = init_rocks(&Shape::Plus, height);
                    shape = Shape::Plus;
                }
                Shape::Plus => {
                    falling = init_rocks(&Shape::Corner, height);
                    shape = Shape::Corner;
                }
                Shape::Corner => {
                    falling = init_rocks(&Shape::Stake, height);
                    shape = Shape::Stake;
                }
                Shape::Stake => {
                    falling = init_rocks(&Shape::Square, height);
                    shape = Shape::Square;
                }
                Shape::Square => {
                    falling = init_rocks(&Shape::Line, height);
                    shape = Shape::Line;
                }
            }
        }
    }
    max_height
}

pub fn star_two() -> u64 {
    let mut height: usize = 0;
    let mut max_height: u64 = 0;
    let mut rock_count: u64 = 0;
    let target_count: u64 = 1_000_000_000_000;

    let mut chamber: VecDeque<Vec<bool>> = VecDeque::from(vec![vec![false; 7]; 50]);
    let mut states: HashMap<(usize, VecDeque<Vec<bool>>), (u64, u64)> = HashMap::new();
    states.insert((0, chamber.clone()), (max_height, rock_count));

    let mut shape = Shape::Line;
    let mut falling = init_rocks(&shape, height);

    let mut winds = INPUT.chars().enumerate().cycle();

    let mut cycle_found = false;
    while rock_count < target_count {
        let (i, wind) = winds.next().unwrap();
        // Apply gust of wind
        match wind {
            '<' => {
                if falling.iter().all(|(x, y)| *x > 0 && !chamber[*y][*x - 1]) {
                    falling = falling
                        .clone()
                        .into_iter()
                        .map(|(x, y)| (x - 1, y))
                        .collect();
                }
            }
            '>' => {
                if falling.iter().all(|(x, y)| *x < 6 && !chamber[*y][*x + 1]) {
                    falling = falling
                        .clone()
                        .into_iter()
                        .map(|(x, y)| (x + 1, y))
                        .collect();
                }
            }
            _ => unreachable!(),
        };
        // Move rocks down one level
        if falling.iter().all(|(x, y)| *y > 0 && !chamber[*y - 1][*x]) {
            falling = falling
                .clone()
                .into_iter()
                .map(|(x, y)| (x, y - 1))
                .collect();
        } else {
            // Rocks settle
            falling.iter().for_each(|(x, y)| {
                chamber[*y][*x] = true;
            });
            rock_count += 1;
            let height_change = height.max(falling[0].1 + 1) - height;
            max_height += height_change as u64;
            height += height_change;
            while height > 40 {
                chamber.pop_front();
                chamber.push_back(vec![false; 7]);
                height -= 1;
            }
            match shape {
                Shape::Line => {
                    falling = init_rocks(&Shape::Plus, height);
                    shape = Shape::Plus;
                }
                Shape::Plus => {
                    falling = init_rocks(&Shape::Corner, height);
                    shape = Shape::Corner;
                }
                Shape::Corner => {
                    falling = init_rocks(&Shape::Stake, height);
                    shape = Shape::Stake;
                }
                Shape::Stake => {
                    falling = init_rocks(&Shape::Square, height);
                    shape = Shape::Square;
                }
                Shape::Square => {
                    falling = init_rocks(&Shape::Line, height);
                    shape = Shape::Line;
                }
            }
            // Check for cycle
            if cycle_found {
                continue;
            }
            if let std::collections::hash_map::Entry::Vacant(e) = states.entry((i, chamber.clone()))
            {
                e.insert((max_height, rock_count));
            } else {
                let (h, r) = states.get(&(i, chamber.clone())).unwrap();
                let cycle_height = max_height - h;
                let cycle_rocks = rock_count - r;
                let cycles = (target_count - rock_count) / cycle_rocks;
                max_height += cycle_height * cycles;
                rock_count += cycle_rocks * cycles;
                cycle_found = true;
            }
        }
    }
    max_height
}
