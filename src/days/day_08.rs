use itertools::Itertools;
use once_cell::sync::Lazy;
use std::{collections::HashSet, fs};

static INPUT: Lazy<String> =
    Lazy::new(|| fs::read_to_string("inputs/day_08.txt").expect("Unable to read from input file."));

pub fn star_one() -> usize {
    let grid: Vec<Vec<u32>> = INPUT
        .lines()
        .map(|line| {
            line.chars()
                .map(|tree| tree.to_digit(10).unwrap() + 1)
                .collect()
        })
        .collect();

    let mut visible_trees: HashSet<(usize, usize)> = HashSet::new();

    // West East
    for row in 0..grid.len() {
        // From the west
        let mut tallest_tree = 0;
        for (col, tree) in grid[row].iter().enumerate() {
            if tree > &tallest_tree {
                visible_trees.insert((row, col));
                tallest_tree = *tree;
            } else {
                continue;
            }
        }
        // From the east
        tallest_tree = 0;
        for (col, tree) in grid[row].iter().enumerate().rev() {
            if tree > &tallest_tree {
                visible_trees.insert((row, col));
                tallest_tree = *tree;
            } else {
                continue;
            }
        }
    }
    // North South
    for col in 0..grid.len() {
        // From the north
        let mut tallest_tree = 0;
        for (row, trees) in grid.iter().enumerate() {
            if trees[col] > tallest_tree {
                visible_trees.insert((row, col));
                tallest_tree = trees[col];
            } else {
                continue;
            }
        }
        // From the south
        tallest_tree = 0;
        for (row, trees) in grid.iter().enumerate().rev() {
            if trees[col] > tallest_tree {
                visible_trees.insert((row, col));
                tallest_tree = trees[col];
            } else {
                continue;
            }
        }
    }

    visible_trees.len()
}

pub fn star_two() -> u32 {
    let grid: Vec<Vec<u32>> = INPUT
    .lines()
    .map(|line| {
        line.chars()
            .map(|tree| tree.to_digit(10).unwrap() + 1)
            .collect()
    })
    .collect();

    (0..grid.len()).map(|row| (0..grid.len()).map(move |col| (row, col)).map(|(row, col)| {
        let tree_height = grid[row][col];

        let mut west = grid[row][..col].iter().rev();
        let mut west_count = west.take_while_ref(|tree| {
            tree_height > **tree
        }).count();
        if let Some(_) = west.next() {west_count += 1}

        let mut east = grid[row][col..].iter().skip(1);
        let mut east_count = east.take_while_ref(|tree| {
            tree_height > **tree
        }).count();
        if let Some(_) = east.next() {east_count += 1}

        let mut north = grid[..row].iter().rev();
        let mut north_count = north.take_while_ref(|trees| {
            tree_height > trees[col]
        }).count();
        if let Some(_) = north.next() {north_count += 1}

        let mut south = grid[row..].iter().skip(1);
        let mut south_count = south.take_while_ref(|trees| {
            tree_height > trees[col]
        }).count();
        if let Some(_) = south.next() {south_count += 1}

        (west_count * east_count * north_count * south_count) as u32
    }).collect::<Vec<u32>>()).flatten().max().unwrap()
}