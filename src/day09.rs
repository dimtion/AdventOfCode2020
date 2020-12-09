use std::{error::Error, fs::File, io::Read};

#[test]
fn part1() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("data/day09.txt").unwrap();
    let mut raw_input = String::new();
    file.read_to_string(&mut raw_input).unwrap();
    let cypher: Vec<u64> = raw_input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    for pointer in 25..cypher.len() {
        let mut ok = false;
        'a: for i in (pointer - 25)..pointer {
            for j in (i + 1)..pointer {
                if cypher[i] + cypher[j] == cypher[pointer] {
                    ok = true;
                    break 'a;
                }
            }
        }
        if !ok {
            println!("N: {}", cypher[pointer]);
        }
    }

    Ok(())
}

#[test]
fn part2() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("data/day09.txt").unwrap();
    let mut raw_input = String::new();
    file.read_to_string(&mut raw_input).unwrap();
    let cypher: Vec<u64> = raw_input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    let invalid = 400480901;

    let (n, size) = (2..cypher.len())
        .find_map(|size| {
            for i in 0..(cypher.len() - size) {
                let slice = &cypher[i..i + size];
                let sum: u64 = slice.iter().sum();
                if sum == invalid {
                    return Some((
                        slice.iter().min().unwrap() + slice.iter().max().unwrap(),
                        size,
                    ));
                }
            }
            None
        })
        .unwrap();

    println!("N: {} S: {}", n, size);

    Ok(())
}
