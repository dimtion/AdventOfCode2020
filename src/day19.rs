use std::{collections::HashMap, error::Error, fs::File, io::Read, str::Lines};

use regex::Regex;

#[derive(Debug)]
enum Rule {
    Char(char),
    Rules(Vec<usize>, Vec<usize>),
}

fn build_rules(mut raw_input: &mut String) -> (HashMap<usize, Rule>, Lines) {
    let mut file = File::open("data/day19.txt").unwrap();
    file.read_to_string(&mut raw_input).unwrap();

    let mut rules = HashMap::new();

    let mut lines = raw_input.lines();
    let re = Regex::new(r"^(?P<idx>\d+): (.(?P<char>\w).|(?P<d0>\d+)( (?P<d1>\d+))?( \| (?P<d2>\d+)( (?P<d3>\d+))?)?)$").unwrap();
    loop {
        let line = lines.next().unwrap();
        if line.len() == 0 {
            break;
        }
        let cap = re.captures_iter(line).next().unwrap();
        let idx: usize = cap.name("idx").unwrap().as_str().parse().unwrap();
        if let Some(c) = cap.name("char") {
            rules.insert(idx, Rule::Char(c.as_str().chars().next().unwrap()));
        } else {
            let mut ra = Vec::new();
            let mut rb = Vec::new();
            if let Some(d0) = cap.name("d0") {
                ra.push(d0.as_str().parse().unwrap());
            }
            if let Some(d1) = cap.name("d1") {
                ra.push(d1.as_str().parse().unwrap());
            }

            if let Some(d2) = cap.name("d2") {
                rb.push(d2.as_str().parse().unwrap());
            }
            if let Some(d3) = cap.name("d3") {
                rb.push(d3.as_str().parse().unwrap());
            }

            rules.insert(idx, Rule::Rules(ra, rb));
        }
    }
    (rules, lines)
}

fn build_re(i: &usize, r: &HashMap<usize, Rule>, depth: usize) -> String {
    let rule = r.get(i).unwrap();
    let mut s = String::new();
    if depth > 14 {
        return s;
    }

    match rule {
        Rule::Char(c) => s.push(*c),
        Rule::Rules(a, b) => {
            s.push('(');
            for x in a {
                s.push_str(build_re(x, r, depth + 1).as_str())
            }

            if b.len() > 0 {
                s.push('|');
                for x in b {
                    s.push_str(build_re(x, r, depth + 1).as_str())
                }
            }
            s.push(')');
        }
    };

    s
}

#[test]
fn part1() -> Result<(), Box<dyn Error>> {
    let mut raw_input = String::new();
    let (rules, lines) = build_rules(&mut raw_input);
    let re = format!("^{}$", build_re(&0, &rules, 0));
    let re = Regex::new(&re).unwrap();

    let count = lines.filter(|&line| re.is_match(line)).count();
    dbg!(count);
    assert_eq!(113, count);

    Ok(())
}


#[test]
fn part2() -> Result<(), Box<dyn Error>> {
    let mut raw_input = String::new();
    let (mut rules, lines) = build_rules(&mut raw_input);

    // override rules
    rules.insert(8, Rule::Rules(vec![42], vec![42, 8]));
    rules.insert(11, Rule::Rules(vec![42, 31], vec![42, 11, 31]));

    let re = format!("^{}$", build_re(&0, &rules, 0));
    let re = Regex::new(&re).unwrap();

    let count = lines.filter(|&line| re.is_match(line)).count();
    dbg!(count);
    assert_eq!(253, count);

    Ok(())
}
