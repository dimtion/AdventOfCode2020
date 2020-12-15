use std::{collections::HashMap, error::Error, fs::File, io::Read};

use regex::Regex;

#[test]
fn part1() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("data/day14.txt").unwrap();
    let mut raw_input = String::new();
    file.read_to_string(&mut raw_input).unwrap();
    let lines = raw_input.lines();

    let mut mem = vec![0; 164000];
    let rem = Regex::new(r"mask = (\w+)").unwrap();
    let ren = Regex::new(r"mem\[(\w+)\] = (\w+)").unwrap();
    let mut mask = 0;
    let mut mask_inv = 0;
    for line in lines {
        if rem.is_match(line) {
            mask = 0;
            mask_inv = 0;
            rem.captures_iter(line).next().unwrap()[1]
                .chars()
                .for_each(|c| {
                    mask <<= 1;
                    mask_inv <<= 1;
                    match c {
                        'X' => {
                            mask += 1;
                            mask_inv += 0
                        }
                        '1' => {
                            mask += 0;
                            mask_inv += 1
                        }
                        '0' => {
                            mask += 0;
                            mask_inv += 0
                        }
                        _ => panic!("FDS"),
                    }
                });
        } else if ren.is_match(line) {
            let cap = ren.captures_iter(line).next().unwrap();
            let loc: usize = cap[1].parse().unwrap();
            let val: u64 = cap[2].parse().unwrap();

            let v = (mask & val) | mask_inv;

            mem[loc] = v;
        }
    }

    let s: u64 = mem.iter().sum();
    println!("mem {}", s);

    Ok(())
}

#[test]
fn part2() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("data/day14.txt").unwrap();
    let mut raw_input = String::new();
    file.read_to_string(&mut raw_input).unwrap();
    let lines = raw_input.lines();

    let mut mem = HashMap::new();
    let rem = Regex::new(r"mask = (\w+)").unwrap();
    let ren = Regex::new(r"mem\[(\w+)\] = (\w+)").unwrap();
    let mut mask = Vec::new();
    for line in lines {
        if rem.is_match(line) {
            mask = rem.captures_iter(line).next().unwrap()[1].chars().collect();
        } else if ren.is_match(line) {
            let cap = ren.captures_iter(line).next().unwrap();
            let mut loc: usize = cap[1].parse().unwrap();
            let val: u64 = cap[2].parse().unwrap();

            let mut loc2 = Vec::new();
            while loc > 0 {
                loc2.push((loc % 2) as u8);
                loc /= 2;
            }
            loc2.reverse();

            for l in get_mems(mask.clone(), loc2) {
                mem.insert(l, val);
            }
        }
    }

    let s: u64 = mem.values().sum();
    println!("mem {}", s);

    Ok(())
}

fn get_mems(mask: Vec<char>, mem: Vec<u8>) -> Vec<usize> {
    if mask.len() == 0 {
        return vec![0];
    }
    let mut mask = mask.clone();
    let mut mem = mem.clone();

    let m = mask.pop().unwrap();
    let v = mem.pop().unwrap_or(0);
    let mut resp = Vec::new();

    match m {
        '1' => {
            for addr in get_mems(mask, mem) {
                resp.push((addr << 1) + 1);
            }
        }
        '0' => {
            for addr in get_mems(mask, mem) {
                resp.push((addr << 1) + v as usize);
            }
        }
        'X' => {
            for addr in get_mems(mask, mem) {
                resp.push((addr << 1) + 1);
                resp.push((addr << 1) + 0);
            }
        }
        _ => panic!("toto"),
    }

    resp
}
