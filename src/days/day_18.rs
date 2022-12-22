use once_cell::sync::Lazy;
use std::{collections::HashSet, fs};

static INPUT: Lazy<String> =
    Lazy::new(|| fs::read_to_string("inputs/day_18.txt").expect("Unable to read from input file."));

pub fn star_one() -> u32 {
    let mut lava: HashSet<(isize, isize, isize)> = HashSet::new();
    INPUT.lines().fold(0, |mut surface_area, line| {
        let coordinates = line.split(',').collect::<Vec<_>>();
        let x = coordinates[0].parse::<_>().unwrap();
        let y = coordinates[1].parse::<_>().unwrap();
        let z = coordinates[2].parse::<_>().unwrap();
        lava.insert((x, y, z));
        surface_area += 6;
        let neighbors = vec![
            (x + 1, y, z),
            (x - 1, y, z),
            (x, y + 1, z),
            (x, y - 1, z),
            (x, y, z + 1),
            (x, y, z - 1),
        ];

        neighbors.iter().for_each(|neighbor| {
            if lava.contains(neighbor) {
                surface_area -= 2
            }
        });
        surface_area
    })
}

#[derive(Debug, Clone, PartialEq)]
enum Space {
    Air,
    Water,
    Lava,
}

pub fn star_two() -> u32 {
    let mut space = vec![vec![vec![Space::Air; 24]; 24]; 24];

    INPUT.lines().for_each(|line| {
        let coordinates = line.split(',').collect::<Vec<_>>();
        let x: usize = coordinates[0].parse::<usize>().unwrap() + 2;
        let y: usize = coordinates[1].parse::<usize>().unwrap() + 2;
        let z: usize = coordinates[2].parse::<usize>().unwrap() + 2;
        space[x][y][z] = Space::Lava;
    });

    // Fill with Water
    let mut surface_area = 0;

    space[1][1][1] = Space::Water;
    let mut successors: Vec<(usize, usize, usize)> = vec![(1, 1, 1)];

    while !successors.is_empty() {
        let mut next = vec![];
        successors.into_iter().for_each(|(x, y, z)| {
            let neighbors = vec![
                (x + 1, y, z),
                (x - 1, y, z),
                (x, y + 1, z),
                (x, y - 1, z),
                (x, y, z + 1),
                (x, y, z - 1),
            ];
            neighbors.into_iter().for_each(|(nx, ny, nz)| {
                if nx > 0 && ny > 0 && nz > 0 && nx < 23 && ny < 23 && nz < 23 {
                    match &space[nx][ny][nz] {
                        Space::Air => {
                            space[nx][ny][nz] = Space::Water;
                            next.push((nx, ny, nz));
                        }
                        Space::Lava => {
                            surface_area += 1;
                        }
                        Space::Water => {}
                    }
                }
            });
        });
        successors = next;
    }
    surface_area
}
