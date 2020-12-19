use std::{error::Error, fs::File, io::Read};

#[test]
fn part1() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("data/day18.txt").unwrap();
    let mut raw_input = String::new();
    file.read_to_string(&mut raw_input).unwrap();

    let mut sum = 0;

    for (i, line) in raw_input.lines().enumerate() {
        let chars: Vec<char> = line.chars().collect();
        let (_, c) = calc(&chars[..]);
        dbg!(i, c);
        sum += c;
        // break;
    }
    dbg!(sum);

    Ok(())
}

fn calc(s: &[char]) -> (usize, i128) {
    let mut result = 0;
    let mut next_op = '+';
    let mut i = 0;
    while i < s.len() {
        // for (i, &c) in s.iter().enumerate() {
        let c = s[i];
        // dbg!(c, result);
        match c {
            ' ' => {}
            '*' => {
                next_op = '*';
            }
            '+' => {
                next_op = '+';
            }
            '(' => match next_op {
                '+' => {
                    // println!("int {}", result);
                    let (k, ret) = calc(&s[(i + 1)..]);
                    result += ret;
                    i += k;
                }
                '*' => {
                    // println!("int {}", result);
                    let (k, ret) = calc(&s[(i + 1)..]);
                    result *= ret;
                    i += k;
                }
                _ => panic!("fds"),
            },
            ')' => {
                // println!("ret {}", result);
                return (i + 1, result);
            }
            x => match next_op {
                '+' => result += String::from(x).parse::<i128>().unwrap(),
                '*' => result *= String::from(x).parse::<i128>().unwrap(),
                _ => panic!("fds"),
            },
        };
        i += 1;
    }
    return (i, result);
}

#[test]
fn part2() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("data/day18.txt").unwrap();
    let mut raw_input = String::new();
    file.read_to_string(&mut raw_input).unwrap();

    let mut sum = 0;

    for (i, line) in raw_input.lines().enumerate() {
        let chars: Vec<char> = line.chars().filter(|&c| c != ' ').collect();
        let (_, c) = calc2(&chars[..], false);
        dbg!(i, c);
        sum += c;
        // break;
    }
    dbg!(sum);

    Ok(())
}

fn calc2(s: &[char], virt: bool) -> (usize, i128) {
    let mut result = 0;
    let mut next_op = '+';
    let mut i = 0;
    while i < s.len() {
        let c = s[i];
        match c {
            ' ' => {}
            '*' => {
                next_op = '*';
                if virt {
                    println!("rev {}", result);
                    return (i, result);
                }
            }
            '+' => {
                next_op = '+';
                // println!("inv {}", result);
                // let (k, ret) = calc2(&s[(i + 1)..], true);
                // result += ret;
                // i += k;
            }
            '(' => match next_op {
                '+' => {
                    println!("in+ {}", result);
                    let (k, ret) = calc2(&s[(i + 1)..], false);
                    result += ret;
                    i += k;
                }
                '*' => {
                    println!("in* {}", result);
                    let (k, ret) = calc2(&s[(i + 1)..], false);
                    result *= ret;
                    i += k;
                }
                _ => panic!("fds"),
            },
            ')' => {
                if virt {
                    println!("rev) {}", result);
                    return (i, result);
                } else {
                    println!("ret) {}", result);
                    return (i + 1, result);
                }
            }
            x => match next_op {
                '+' => result += String::from(x).parse::<i128>().unwrap(),
                '*' => {
                    if s.len() > i + 1 && s[i + 1] == '+' {
                        let (k, ret) = calc2(&s[(i)..], true);
                        result *= ret;
                        i += k;
                    } else {
                        result *= String::from(x).parse::<i128>().unwrap()
                    }
                }
                _ => panic!("fds"),
            },
        };
        i += 1;
    }
    return (i, result);
}

// 15969850874067
// 277606452576468