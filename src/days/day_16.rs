use itertools::Itertools;
use once_cell::sync::Lazy;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

static INPUT: Lazy<String> =
    Lazy::new(|| fs::read_to_string("inputs/day_16.txt").expect("Unable to read from input file."));

pub fn star_one() -> i32 {
    let mut valves = HashMap::new();
    let mut flows = HashMap::new();
    let mut tunnels = HashMap::new();
    INPUT.lines().for_each(|line| {
        let split_input: Vec<&str> = line.split(' ').collect();
        let valve = split_input[1];
        let flow = split_input[4]
            .split_once('=')
            .unwrap()
            .1
            .split_once(';')
            .unwrap()
            .0
            .parse::<i32>()
            .unwrap();
        let mut next_tunnels = split_input[9..]
            .iter()
            .map(|t| *t.split_terminator(',').collect::<Vec<_>>().first().unwrap())
            .collect::<Vec<_>>();
        next_tunnels.push(valve);
        valves.insert(valve, false);
        flows.insert(valve, flow);
        tunnels.insert(valve, next_tunnels);
    });

    let mut time = 0;

    // State = (valve, open valves, flow_per_second, total flow)
    let mut states = vec![("AA", HashSet::<&str>::new(), 0, 0)];

    while time < 30 {
        states = states
            .into_iter()
            .flat_map(|(valve, open, fps, flow)| {
                tunnels
                    .get(valve)
                    .unwrap()
                    .into_iter()
                    .map(|tunnel| {
                        if *tunnel == valve && flows[valve] > 0 && !open.contains(&valve) {
                            let mut open = open.clone();
                            open.insert(valve);
                            (valve, open, fps + flows[valve], flow + fps)
                        } else {
                            (*tunnel, open.clone(), fps, flow + fps)
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .sorted_by(|a, b| (b.2).cmp(&(a.2)))
            .take(200)
            .collect::<Vec<_>>();
        time += 1;
    }

    *states.iter().map(|(_, _, _, flow)| flow).max().unwrap()
}

pub fn star_two() -> i32 {
    let mut valves = HashMap::new();
    let mut flows = HashMap::new();
    let mut tunnels = HashMap::new();
    INPUT.lines().for_each(|line| {
        let split_input: Vec<&str> = line.split(' ').collect();
        let valve = split_input[1];
        let flow = split_input[4]
            .split_once('=')
            .unwrap()
            .1
            .split_once(';')
            .unwrap()
            .0
            .parse::<i32>()
            .unwrap();
        let mut next_tunnels = split_input[9..]
            .iter()
            .map(|t| *t.split_terminator(',').collect::<Vec<_>>().first().unwrap())
            .collect::<Vec<_>>();
        next_tunnels.push(valve);
        valves.insert(valve, false);
        flows.insert(valve, flow);
        tunnels.insert(valve, next_tunnels);
    });

    let mut time: i32 = 0;

    // State = (valve, elephant, open valves, flow_per_second, total flow)
    let mut states = vec![("AA", "AA", HashSet::<&str>::new(), 0, 0)];

    while time < 26 {
        states = states
            .into_iter()
            .flat_map(|(valve, e_valve, open, fps, flow)| {
                tunnels
                    .get(valve)
                    .unwrap()
                    .iter()
                    .cartesian_product(tunnels.get(e_valve).unwrap().iter())
                    .map(|next| match next {
                        (a, b) if *a == valve || *b == e_valve => {
                            let mut open = open.clone();
                            let mut new_fps = fps;
                            if *a == valve && flows[a] > 0 && !open.contains(a) {
                                open.insert(a);
                                new_fps += flows[a];
                            }
                            if *b == e_valve && flows[b] > 0 && !open.contains(b) {
                                open.insert(b);
                                new_fps += flows[b];
                            }
                            (*a, *b, open, new_fps, flow + fps)
                        }
                        (a, b) => (*a, *b, open.clone(), fps, flow + fps),
                    })
                    .collect::<Vec<_>>()
            })
            .sorted_by(|a, b| {
                (b.3 + b.4).cmp(&(a.3 + a.4))
            })
            .take(50000)
            .collect::<Vec<_>>();
        time += 1;
    }
    dbg!(states.len());

    *states.iter().map(|(_, _, _, _, flow)| flow).max().unwrap()
}
