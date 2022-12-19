use once_cell::sync::Lazy;
use std::{collections::HashMap, fs};

static INPUT: Lazy<String> =
    Lazy::new(|| fs::read_to_string("inputs/day_14.txt").expect("Unable to read from input file."));

#[derive(Debug, PartialEq)]
enum Space {
    AIR,
    ROCK,
    SAND,
}

pub fn star_one() -> u32 {
    let mut cave: HashMap<(usize, usize), Space> = HashMap::new();
    let mut lowest_point: usize = 0;
    INPUT.lines().for_each(|line| {
        let mut paths = line.split(" -> ");
        let mut start = paths.next().unwrap();
        paths.for_each(|end| {
            let (x1, y1) = start.split_once(',').unwrap();
            let (x2, y2) = end.split_once(',').unwrap();
            let x1 = x1.parse::<usize>().unwrap();
            let y1 = y1.parse::<usize>().unwrap();
            let x2 = x2.parse::<usize>().unwrap();
            let y2 = y2.parse::<usize>().unwrap();
            if x1 == x2 {
                for y in y1.min(y2)..=y1.max(y2) {
                    cave.insert((x1, y), Space::ROCK);
                }
            }
            if y1 == y2 {
                for x in x1.min(x2)..=x1.max(x2) {
                    cave.insert((x, y1), Space::ROCK);
                }
            }
            lowest_point = if y1 > lowest_point { y1 } else { lowest_point };
            lowest_point = if y2 > lowest_point { y2 } else { lowest_point };
            start = end;
        });
    });

    let mut sand_at_rest = 0;
    'outer: loop {
        let mut sand = (500, 0);
        'inner: loop {
            if sand.1 > lowest_point {
                break 'outer;
            } else if *cave.entry((sand.0, sand.1 + 1)).or_insert(Space::AIR) == Space::AIR {
                sand = (sand.0, sand.1 + 1)
            } else if *cave.entry((sand.0 - 1, sand.1 + 1)).or_insert(Space::AIR) == Space::AIR {
                sand = (sand.0 - 1, sand.1 + 1)
            } else if *cave.entry((sand.0 + 1, sand.1 + 1)).or_insert(Space::AIR) == Space::AIR {
                sand = (sand.0 + 1, sand.1 + 1)
            } else {
                cave.entry(sand)
                    .and_modify(|e| *e = Space::SAND)
                    .or_insert(Space::SAND);
                sand_at_rest += 1;
                break 'inner;
            }
        }
    }
    sand_at_rest
}

pub fn star_two() -> u32 {
    let mut cave: HashMap<(usize, usize), Space> = HashMap::new();
    let mut lowest_point: usize = 0;
    INPUT.lines().for_each(|line| {
        let mut paths = line.split(" -> ");
        let mut start = paths.next().unwrap();
        paths.for_each(|end| {
            let (x1, y1) = start.split_once(',').unwrap();
            let (x2, y2) = end.split_once(',').unwrap();
            let x1 = x1.parse::<usize>().unwrap();
            let y1 = y1.parse::<usize>().unwrap();
            let x2 = x2.parse::<usize>().unwrap();
            let y2 = y2.parse::<usize>().unwrap();
            if x1 == x2 {
                for y in y1.min(y2)..=y1.max(y2) {
                    cave.insert((x1, y), Space::ROCK);
                }
            }
            if y1 == y2 {
                for x in x1.min(x2)..=x1.max(x2) {
                    cave.insert((x, y1), Space::ROCK);
                }
            }
            lowest_point = if y1 > lowest_point { y1 } else { lowest_point };
            lowest_point = if y2 > lowest_point { y2 } else { lowest_point };
            start = end;
        });
    });

    let mut path = vec![(500, 0)];
    let mut sand_at_rest = 0;
    loop {
        if path.len() == 0 {
            break;
        }
        let mut sand = path.last().unwrap().clone();
        loop {
            if sand.1 == lowest_point + 1 {
                cave.entry(sand)
                    .and_modify(|e| *e = Space::SAND)
                    .or_insert(Space::SAND);
                sand_at_rest += 1;
                path.pop();
                break;
            } else if *cave.entry((sand.0, sand.1 + 1)).or_insert(Space::AIR) == Space::AIR {
                sand = (sand.0, sand.1 + 1);
                path.push(sand);
            } else if *cave.entry((sand.0 - 1, sand.1 + 1)).or_insert(Space::AIR) == Space::AIR {
                sand = (sand.0 - 1, sand.1 + 1);
                path.push(sand);
            } else if *cave.entry((sand.0 + 1, sand.1 + 1)).or_insert(Space::AIR) == Space::AIR {
                sand = (sand.0 + 1, sand.1 + 1);
                path.push(sand);
            } else {
                cave.entry(sand)
                    .and_modify(|e| *e = Space::SAND)
                    .or_insert(Space::SAND);
                sand_at_rest += 1;
                path.pop();
                break;
            }
        }
    }
    sand_at_rest
}
