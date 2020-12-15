use std::{error::Error, fs::File, io::Read};

#[test]
fn part1() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("data/day13.txt").unwrap();
    let mut raw_input = String::new();
    file.read_to_string(&mut raw_input).unwrap();
    let mut lines = raw_input.lines();
    let start: i64 = lines.next().unwrap().parse().unwrap();

    let busses: Vec<i64> = lines
        .next()
        .unwrap()
        .split(',')
        .filter(|&e| e != "x")
        .map(|e| e.parse().unwrap())
        .collect();

    let mut min = std::i64::MAX;
    let mut min_id = 0;
    for bus in busses {
        let d = bus - (start % bus);
        if d < min {
            min = d;
            min_id = bus;
        }
    }

    println!("{}", min * min_id);
    Ok(())
}

#[test]
fn part2() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("data/day13.txt").unwrap();
    let mut raw_input = String::new();
    file.read_to_string(&mut raw_input).unwrap();
    let mut lines = raw_input.lines();
    lines.next();

    // let mut buses: Vec<(i64, i64)> = lines.next().unwrap().split(',').enumerate().filter(|&(_, e)| e != "x").map(|(i, e)| (i as i64, e.parse().unwrap())).collect();
    // let mut buses: Vec<(i64, i64)> = lines.next().unwrap().split(',').enumerate().filter(|&(_, e)| e != "x").map(|(i, e)| (i as i64, e.parse().unwrap())).collect();
    let buses: Vec<usize> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|l| match l.parse() {
            Ok(n) => n,
            Err(_) => 1,
        })
        .collect();
    // buses.sort_by(|(_, ni), (_, nj)| ni.cmp(nj).reverse());

    let global_coprime: usize = buses.iter().product();

    let mut factors = vec![];

    for (i, bus) in buses.iter().rev().enumerate() {
        let current_factor = global_coprime / bus;
        let mut j = 1;
        while (current_factor * j) % bus != i % bus {
            j += 1;
        }
        factors.push(current_factor * j);
    }

    let xi = factors.iter().sum::<usize>() % global_coprime - buses.len() + 1;

    println!("{}", xi);
    Ok(())
}
