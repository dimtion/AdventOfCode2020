use regex::Regex;
use std::{error::Error, fs::File, io::BufRead, io::BufReader};

#[test]
fn part1() -> Result<(), Box<dyn Error>> {
    let buf = BufReader::new(File::open("data/day2.txt")?);
    let re = Regex::new(r"^(\d+)-(\d+)\s(\w):\s(.+)$")?;
    let valid = buf
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| {
            let cap = re.captures_iter(&line).next().unwrap();
            let min: usize = cap[1].parse().unwrap();
            let max: usize = cap[2].parse().unwrap();
            let letter: char = cap[3].parse().unwrap();
            let text = &cap[4];
            let count = text.chars().filter(|&c| c == letter).count();
            min <= count && count <= max
        })
        .count();
    println!("Valid passwords: {}", &valid);
    Ok(())
}

#[test]
fn part2() -> Result<(), Box<dyn Error>> {
    let buf = BufReader::new(File::open("data/day2.txt")?);
    let re = Regex::new(r"^(\d+)-(\d+)\s(\w):\s(.+)$")?;
    let valid = buf
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| {
            let cap = re.captures_iter(&line).next().unwrap();
            let a: usize = cap[1].parse().unwrap();
            let b: usize = cap[2].parse().unwrap();
            let letter: char = cap[3].parse().unwrap();
            let text: Vec<char> = cap[4].chars().collect();
            (text[a - 1] == letter || text[b - 1] == letter) && text[a - 1] != text[b - 1]
        })
        .count();
    println!("Valid: {}", &valid);
    Ok(())
}
