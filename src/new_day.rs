use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let day = std::env::args().nth(1).expect("No day specified.");

    //Create input file
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(format!("inputs/day_{day:02}.txt"))?;

    //Append to days module
    OpenOptions::new()
    .append(true)
    .open("src/days/mod.rs")?
    .write_all(format!("pub mod day_{day:02};\n").as_bytes())?;

    //Create solution file
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(format!("src/days/day_{day:02}.rs"))?
        .write_all(
            format!(
                "
use std::fs;
use once_cell::sync::Lazy;

static INPUT: Lazy<String> = Lazy::new(|| {{
    fs::read_to_string(\"inputs/day_{day:02}.txt\").expect(\"Unable to read from input file.\")
}});

pub fn star_one() -> u32 {{
    0
}}

pub fn star_two() -> u32 {{
    0
}}
            "
            )
            .as_bytes(),
        )?;

    //Append to benches file
    OpenOptions::new()
        .append(true)
        .open("benches/benches.rs")?
        .write_all(
            format!(
                "

// Day {day}
use advent::days::day_{day};

#[bench]
fn day_{day}_star_one(b: &mut Bencher) {{
    b.iter(|| day_{day}::star_one());
}}

#[bench]
fn day_{day}_star_two(b: &mut Bencher) {{
    b.iter(|| day_{day}::star_two());
}}
",
                day = format!("{day:02}")
            )
            .as_bytes(),
        )?;

    //Append to tests file
    OpenOptions::new()
        .append(true)
        .open("tests/tests.rs")?
        .write_all(
            format!(
                "

// Day {day}
use advent::days::day_{day};

#[test]
fn day_{day}_star_one() {{
    assert_eq!(0, day_{day}::star_one());
}}

#[test]
fn day_{day}_star_two() {{
    assert_eq!(0, day_{day}::star_two());
}}
        ",
                day = format!("{day:02}")
            )
            .as_bytes(),
        )?;

    Ok(())
}
