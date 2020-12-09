use std::{cmp, error::Error, fs::File, io::Read};

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

#[test]
fn part2b() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("data/day09.txt").unwrap();
    let mut raw_input = String::new();
    file.read_to_string(&mut raw_input).unwrap();
    let cypher: Vec<u64> = raw_input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    let invalid = 400480901;
    let solution: u64 = 67587168;

    let mut i = 0;
    let mut j = 2;
    let mut sol = 0;
    let mut sum = cypher[0] + cypher[1];
    let mut min = cmp::min(cypher[0],cypher[1]);
    let mut max = cmp::max(cypher[0],cypher[1]);
    while i < cypher.len() - 2 {
        if sum == invalid {
            sol = min + max;
            break;
        } else if sum < invalid {
            sum += cypher[j];
            j += 1;
            min = cmp::min(min, cypher[j]);
            max = cmp::max(max, cypher[j]);
        } else {
            sum -= cypher[i];
            i += 1;
            if j - i < 2 {
                sum += cypher[j];
                j += 1;
            }
            min = *cypher[i..j].iter().min().unwrap();
            max = *cypher[i..j].iter().max().unwrap();
        }
    }

    assert_eq!(sol, solution);
    println!("S: {}", sol);

    Ok(())
}
