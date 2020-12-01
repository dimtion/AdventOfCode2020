use std::{error::Error, fs::File, io::BufRead, io::BufReader};

use itertools::iproduct;

fn load_expenses() -> Result<Vec<i32>, Box<dyn Error>> {
    let buf = BufReader::new(File::open("data/day1.txt")?);
    Ok(buf
        .lines()
        .map(|line| line.unwrap().trim().parse().unwrap())
        .collect()
    )
}

#[test]
fn part1() -> Result<(), Box<dyn Error>> {
    let expenses = load_expenses()?;
    if let Some((a, b)) = iproduct!(&expenses, &expenses).find(|(&a, &b)| a + b == 2020) {
        println!("a: {}, b: {}, a * b: {}", a, b, a * b);
    } else {
        println!("No solution found")
    }
    Ok(())
}

#[test]
fn part2() -> Result<(), Box<dyn Error>> {
    let expenses = load_expenses()?;

    if let Some((a, b, c)) = iproduct!(&expenses, &expenses, &expenses).find(|(&a, &b, &c)| a + b + c == 2020) {
        println!("a: {}, b: {}, c: {}, a * b * c: {}", a, b, c, a * b * c);
    } else {
        println!("No solution found")
    }
    Ok(())
}