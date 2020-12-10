use std::{error::Error, fs::File, io::Read, time::Instant};

use itertools::Itertools;

#[test]
fn part1() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("data/day10.txt").unwrap();
    let mut raw_input = String::new();
    file.read_to_string(&mut raw_input).unwrap();
    let mut adapters: Vec<u32> = raw_input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    adapters.sort();

    let mut jumps = 0;
    let mut jumps3 = 0;
    let mut current = 0;
    for i in 0..adapters.len() {
        if adapters[i] - current == 3 {
            jumps3 += 1;
        } else if adapters[i] - current == 1 {
            jumps += 1;
        } else {
            println!("{} {}", adapters[i], current);
        }
        current = adapters[i];
    }

    jumps3 += 1;

    println!("{} {}", jumps, jumps3);
    println!("{}", jumps * jumps3);

    Ok(())
}

#[inline]
fn solutions(i: usize, adapters: &Vec<u8>, mem: &mut Vec<usize>) {
    if mem[i] == 0 {
        mem[i] = (i + 1..=i + 3)
            .filter(|&j| j < adapters.len() && adapters[j] - adapters[i] <= 3)
            .map(|j| {
                solutions(j, adapters, mem);
                mem[j]
            })
            .sum();
    }
}

#[test]
fn part2() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("data/day10.txt").unwrap();
    let mut raw_input = String::new();
    file.read_to_string(&mut raw_input).unwrap();
    let adapters: Vec<u8> = raw_input
        .lines()
        .map(|line| line.parse().unwrap())
        .sorted()
        .collect();

    let start = Instant::now();
    let mut mem = vec![0; adapters.len()];
    mem[adapters.len() - 1] = 1;
    solutions(0, &adapters, &mut mem);
    solutions(1, &adapters, &mut mem);
    solutions(2, &adapters, &mut mem);
    let sol = mem[0] + mem[1] + mem[2];

    let dur = start.elapsed();

    println!("S: {} E: {} ns", sol, dur.as_nanos());
    assert_eq!(sol, 24803586664192);

    Ok(())
}
