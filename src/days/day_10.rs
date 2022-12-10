
use std::fs;
use once_cell::sync::Lazy;

static INPUT: Lazy<String> = Lazy::new(|| {
    fs::read_to_string("inputs/day_10.txt").expect("Unable to read from input file.")
});

pub fn star_one() -> i32 {
    let mut cycle = 1;
    let mut x = 1;
    let test_cycles = [20, 60, 100, 140, 180, 220];
    let mut signal_strengths = Vec::new();
    INPUT.lines().for_each(|line| {
        if let Some((_, n)) = line.split_once(' ') {
            if test_cycles.contains(&cycle) {signal_strengths.push(cycle * x);}
            cycle += 1;
            if test_cycles.contains(&cycle) {signal_strengths.push(cycle * x);}
            cycle += 1;
            x += n.parse::<i32>().unwrap();
        } else {
            if test_cycles.contains(&cycle) {signal_strengths.push(cycle * x);}
            cycle += 1
        }
    });
    signal_strengths.iter().sum()
}

pub fn star_two() -> String {
    let mut crt = String::new();
    let mut cycle = 1;
    let mut x = 1;
    INPUT.lines().for_each(|line| {
        if let Some((_, n)) = line.split_once(' ') {
            crt.push_str(if ((cycle - 1) % 40 - x) <= 1 && ((cycle - 1) % 40 - x) >= -1 {"#"} else {" "});
            cycle += 1;
            crt.push_str(if ((cycle - 1) % 40 - x) <= 1 && ((cycle - 1) % 40 - x) >= -1 {"#"} else {" "});
            cycle += 1;
            x += n.parse::<i32>().unwrap();
        } else {
            crt.push_str(if ((cycle - 1) % 40 - x) <= 1 && ((cycle - 1) % 40 - x) >= -1 {"#"} else {" "});
            cycle += 1
        }
        if (cycle - 1) % 40 == 0 {crt.push_str("\n")}
    });
    crt
}
            