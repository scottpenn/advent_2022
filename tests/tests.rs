#![feature(test)]
extern crate test;

// Day 01
use advent::days::day_01;

#[test]
fn day_01_star_one() {
    dbg!(day_01::star_one());
    assert_eq!(68923, day_01::star_one());
}

#[test]
fn day_01_star_two() {
    dbg!(day_01::star_two());
    assert_eq!(200044, day_01::star_two());
}

// Day 02
use advent::days::day_02;

#[test]
fn day_02_star_one() {
    dbg!(day_02::star_one());
    assert_eq!(11873, day_02::star_one());
}

#[test]
fn day_02_star_two() {
    dbg!(day_02::star_two());
    assert_eq!(12014, day_02::star_two());
}

// Day 03
use advent::days::day_03;

#[test]
fn day_03_star_one() {
    assert_eq!(7848, day_03::star_one());
}

#[test]
fn day_03_star_two() {
    assert_eq!(2616, day_03::star_two());
}

// Day 04
use advent::days::day_04;

#[test]
fn day_04_star_one() {
    assert_eq!(483, day_04::star_one());
}

#[test]
fn day_04_star_two() {
    assert_eq!(874, day_04::star_two());
}

// Day 05
use advent::days::day_05;

#[test]
fn day_05_star_one() {
    assert_eq!("TQRFCBSJJ".to_string(), day_05::star_one());
}

#[test]
fn day_05_star_two() {
    assert_eq!("RMHFJNVFP".to_string(), day_05::star_two());
}

// Day 06
use advent::days::day_06;

#[test]
fn day_06_star_one() {
    assert_eq!(1876, day_06::star_one());
}

#[test]
fn day_06_star_two() {
    assert_eq!(2202, day_06::star_two());
}

// Day 07
use advent::days::day_07;

#[test]
fn day_07_star_one() {
    assert_eq!(1642503, day_07::star_one());
}

#[test]
fn day_07_star_two() {
    assert_eq!(6999588, day_07::star_two());
}  

// Day 08
use advent::days::day_08;

#[test]
fn day_08_star_one() {
    assert_eq!(1695, day_08::star_one());
}

#[test]
fn day_08_star_two() {
    assert_eq!(287040, day_08::star_two());
}

// Day 09
use advent::days::day_09;

#[test]
fn day_09_star_one() {
    assert_eq!(6256, day_09::star_one());
}

#[test]
fn day_09_star_two() {
    assert_eq!(2665, day_09::star_two());
}
        

// Day 10
use advent::days::day_10;

#[test]
fn day_10_star_one() {
    assert_eq!(12540, day_10::star_one());
}

#[test]
fn day_10_star_two() {
    assert_eq!("#### ####  ##  #### #### #    #  # #### 
#    #    #  #    # #    #    #  # #    
###  ###  #      #  ###  #    #### ###  
#    #    #     #   #    #    #  # #    
#    #    #  # #    #    #    #  # #    
#    ####  ##  #### #### #### #  # #### 
".to_string(), day_10::star_two());
}
        

// Day 11
use advent::days::day_11;

#[test]
fn day_11_star_one() {
    assert_eq!(90882, day_11::star_one());
}

#[test]
fn day_11_star_two() {
    assert_eq!(30893109657, day_11::star_two());
}
        

// Day 12
use advent::days::day_12;

#[test]
fn day_12_star_one() {
    assert_eq!(350, day_12::star_one());
}

#[test]
fn day_12_star_two() {
    assert_eq!(349, day_12::star_two());
}
        

// Day 13
use advent::days::day_13;

#[test]
fn day_13_star_one() {
    assert_eq!(0, day_13::star_one());
}

#[test]
fn day_13_star_two() {
    assert_eq!(0, day_13::star_two());
}
        