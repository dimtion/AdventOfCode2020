use itertools::{sorted, Itertools};
use regex::Regex;
use std::{error::Error, fs::File, io::BufRead, io::BufReader};

fn get_seats() -> impl Iterator<Item = i32> {
    let buf = BufReader::new(File::open("data/day05.txt").unwrap());
    buf.lines().map(|line| {
        let line = line.unwrap();
        let mut max_r = 128;
        let mut min_r = 0;
        let mut max_c = 8;
        let mut min_c = 0;
        for c in line.chars() {
            match c {
                'F' => max_r = (min_r + max_r) / 2,
                'B' => min_r = (min_r + max_r) / 2,
                'L' => max_c = (min_c + max_c) / 2,
                'R' => min_c = (min_c + max_c) / 2,
                _ => panic!("Car"),
            }
        }
        min_r * 8 + min_c
    })
}

#[test]
fn part1() -> Result<(), Box<dyn Error>> {
    let seat = get_seats().max().unwrap();
    println!("Seat {}", seat);
    Ok(())
}

#[test]
fn part2() -> Result<(), Box<dyn Error>> {
    let missing = get_seats()
        .sorted()
        .tuple_windows()
        .find(|&(a, b)| a + 1 != b)
        .unwrap()
        .0
        + 1;
    println!("Missing seat: {}", missing);
    Ok(())
}
