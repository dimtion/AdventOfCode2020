use std::collections::HashSet;
use std::{error::Error, fs::File, io::BufRead, io::BufReader};

use itertools::iproduct;

fn load_expenses() -> Result<HashSet<i32>, Box<dyn Error>> {
    let buf = BufReader::new(File::open("data/day1.txt")?);
    Ok(buf
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect())
}

#[test]
fn part1() -> Result<(), Box<dyn Error>> {
    let expenses = load_expenses()?;
    let solution = expenses
        .iter()
        .find_map(|&a| expenses.get(&(2020 - a)).map(|&b| (a, b)));
    if let Some((a, b)) = solution {
        println!("a: {}, b: {}, a * b: {}", a, b, a * b);
    } else {
        println!("No solution found")
    }
    Ok(())
}

#[test]
fn part2() -> Result<(), Box<dyn Error>> {
    let expenses = load_expenses()?;
    let solution = iproduct!(&expenses, &expenses)
        .find_map(|(&a, &b)| expenses.get(&(2020 - a - b)).map(|&c| (a, b, c)));

    if let Some((a, b, c)) = solution {
        println!("a: {}, b: {}, c: {}, a * b * c: {}", a, b, c, a * b * c);
    } else {
        println!("No solution found")
    }
    Ok(())
}
