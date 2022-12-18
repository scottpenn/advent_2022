use once_cell::sync::Lazy;
use pathfinding::prelude::bfs;
use std::fs;

static INPUT: Lazy<String> =
    Lazy::new(|| fs::read_to_string("inputs/day_12.txt").expect("Unable to read from input file."));

pub fn star_one() -> usize {
    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);
    let grid: Vec<Vec<u8>> = INPUT
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.bytes()
                .enumerate()
                .map(|(col, elevation)| match elevation {
                    b'S' => {
                        start = (row, col);
                        0
                    }
                    b'E' => {
                        end = (row, col);
                        b'z' - b'a'
                    }
                    letter => letter - b'a',
                })
                .collect()
        })
        .collect();
    let path = bfs(
        &start,
        |&(r, c)| {
            let mut neighbors = Vec::new();
            if r > 0 {
                neighbors.push((r - 1, c))
            }
            if r < (grid.len() - 1) {
                neighbors.push((r + 1, c))
            }
            if c > 0 {
                neighbors.push((r, c - 1))
            }
            if c < (grid[0].len() - 1) {
                neighbors.push((r, c + 1))
            }
            neighbors
                .into_iter()
                .filter(|neighbor| (grid[neighbor.0][neighbor.1] as i8 - grid[r][c] as i8) <= 1)
                .collect::<Vec<(usize, usize)>>()
        },
        |&node| node == end,
    )
    .unwrap();
    path.len() - 1
}

pub fn star_two() -> usize {
    let mut start: (usize, usize) = (0, 0);
    let grid: Vec<Vec<u8>> = INPUT
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.bytes()
                .enumerate()
                .map(|(col, elevation)| match elevation {
                    b'S' => {
                        b'z' - b'a'
                    }
                    b'E' => {
                        start = (row, col);
                        0
                    }
                    letter => b'z' - letter,
                })
                .collect()
        })
        .collect();
    let path = bfs(
        &start,
        |&(r, c)| {
            let mut neighbors = Vec::new();
            if r > 0 {
                neighbors.push((r - 1, c))
            }
            if r < (grid.len() - 1) {
                neighbors.push((r + 1, c))
            }
            if c > 0 {
                neighbors.push((r, c - 1))
            }
            if c < (grid[0].len() - 1) {
                neighbors.push((r, c + 1))
            }
            neighbors
                .into_iter()
                .filter(|neighbor| (grid[neighbor.0][neighbor.1] as i8 - grid[r][c] as i8) <= 1)
                .collect::<Vec<(usize, usize)>>()
        },
        |&(r, c)| grid[r][c] == (b'z' - b'a'),
    )
    .unwrap();
    path.len() - 1
}
