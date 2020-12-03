use std::{error::Error, fs::File, io::BufRead, io::BufReader};

type Trees = Vec<Vec<u8>>;

fn get_map() -> Result<Trees, Box<dyn Error>> {
    let buf = BufReader::new(File::open("data/day3.txt")?);
    Ok(buf
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| match c {
                    '#' => 1,
                    '.' => 0,
                    _ => panic!("Invalid char"),
                })
                .collect()
        })
        .collect())
}

fn count_trees(map: &Trees, k: usize, l: usize) -> u64 {
    let r = map[0].len();
    let mut count = 0;
    let mut i = 0;
    let mut j = 0;
    while i < map.len() {
        count += map[i][j % r] as u64;
        j += k;
        i += l;
    }
    count
}

#[test]
fn part1() -> Result<(), Box<dyn Error>> {
    let map = get_map()?;
    let trees = count_trees(&map, 3, 1);
    println!("Trees: {}", trees);
    Ok(())
}

#[test]
fn part2() -> Result<(), Box<dyn Error>> {
    let map = get_map()?;
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let res: u64 = slopes
        .iter()
        .map(|(k, l)| count_trees(&map, *k, *l))
        .product();

    println!("result: {}", res);
    Ok(())
}
