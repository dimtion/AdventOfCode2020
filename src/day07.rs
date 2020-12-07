use std::{collections::HashMap, error::Error, fs::File, io::Read};

use regex::Regex;

fn get_bags() -> HashMap<String, Vec<(String, i64)>> {
    let mut bags = HashMap::new();
    let mut file = File::open("data/day07.txt").unwrap();
    let mut raw_input = String::new();
    file.read_to_string(&mut raw_input).unwrap();
    let re2 =
        Regex::new(r"^(?P<k>\w+\s\w+) bags contain (no other bags)?((?P<d0>\d+) (?P<s0>\w+ \w+) bags?, )?((?P<d1>\d+) (?P<s1>\w+ \w+) bags?, )?((?P<d2>\d+) (?P<s2>\w+ \w+) bags?, )?((?P<d3>\d+) (?P<s3>\w+ \w+) bags?, )?((?P<d4>\d+) (?P<s4>\w+ \w+) bags?)?\.").unwrap();
    for line in raw_input.lines() {
        re2.captures_iter(line).for_each(|cap| {
            bags.insert(
                cap[1].to_string(),
                [
                    ("s0", "d0"),
                    ("s1", "d1"),
                    ("s2", "d2"),
                    ("s3", "d3"),
                    ("s4", "d4"),
                ]
                .iter()
                .filter_map(|(s, d)| {
                    cap.name(s).map(|c| {
                        (
                            c.as_str().to_owned(),
                            cap.name(d).unwrap().as_str().parse().unwrap(),
                        )
                    })
                })
                .collect(),
            );
        });
    }
    bags
}

#[test]
fn part1() -> Result<(), Box<dyn Error>> {
    let bags = get_bags();
    let mut count = 0;
    for bag in bags.keys().filter(|&b| !"shiny gold".eq(b)) {
        let mut to_visit = vec![bag];
        while !to_visit.is_empty() {
            let current_bag = to_visit.pop().unwrap();
            if "shiny gold".eq(current_bag) {
                count += 1;
                break;
            }
            for (b, _) in bags.get(current_bag).unwrap() {
                to_visit.push(b);
            }
        }
    }
    println!("Bags {}", count);
    Ok(())
}

#[test]
fn part2() -> Result<(), Box<dyn Error>> {
    let bags = get_bags();
    let mut count = 0;
    let mut to_visit = vec![("shiny gold", 1)];
    while !to_visit.is_empty() {
        let (bag, c) = to_visit.pop().unwrap();
        count += c;
        bags.get(bag).map(|childs| {
            for (b, m) in childs {
                to_visit.push((b, c * m));
            }
        });
    }
    println!("Bags {}", count - 1);
    Ok(())
}
