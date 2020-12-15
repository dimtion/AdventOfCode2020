use std::{collections::HashMap, error::Error, fs::File, io::Read};

#[test]
fn part1() -> Result<(), Box<dyn Error>> {
    let seed = load_seed();
    let res = compute_result(seed, 2020);

    assert_eq!(res, 1111);
    println!("Result: {}", res);
    Ok(())
}

#[test]
fn part2() -> Result<(), Box<dyn Error>> {
    let seed = load_seed();
    let res = compute_result(seed, 30000000);

    assert_eq!(res, 48568);
    println!("Result: {}", res);
    Ok(())
}

fn load_seed() -> Vec<usize> {
    let mut file = File::open("data/day15.txt").unwrap();
    let mut raw_input = String::new();
    file.read_to_string(&mut raw_input).unwrap();

    raw_input
        .split(",")
        .map(|n| n.parse::<usize>().unwrap())
        .collect()
}

fn compute_result(seed: Vec<usize>, n: usize) -> usize {
    let mut last_spoken: HashMap<_, _> =
        seed.iter().enumerate().map(|(i, &n)| (n, i + 1)).collect();

    let mut current = *seed.last().unwrap();
    for i in seed.len()..n {
        let idx = last_spoken.entry(current).or_insert(i);
        current = i - *idx;
        *idx = i;
    }

    current
}
