use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs::File,
    io::Read,
};

use regex::Regex;

#[test]
fn part1() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("data/day16.txt").unwrap();
    let mut raw_input = String::new();
    file.read_to_string(&mut raw_input).unwrap();

    let mut lines = raw_input.lines();
    let mut line = lines.next().unwrap();

    let mut rules: Vec<(String, u64, u64, u64, u64)> = Vec::new();
    let re = Regex::new(r"([^:]): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
    while line != "" {
        let r = re.captures_iter(line).next().unwrap();
        rules.push((
            r[1].to_owned(),
            r[2].parse().unwrap(),
            r[3].parse().unwrap(),
            r[4].parse().unwrap(),
            r[5].parse().unwrap(),
        ));
        line = lines.next().unwrap();
    }

    lines.next();
    lines.next(); // my ticket

    lines.next(); // my ticket
    lines.next(); // Nearby

    let mut tickets = Vec::new();
    while let Some(line) = lines.next() {
        tickets.push(
            line.split(",")
                .map(|n| n.parse().unwrap())
                .collect::<Vec<u64>>(),
        );
    }

    let mut s = 0;
    for ticket in tickets {
        for v in ticket {
            let mut is_valid = false;
            for rule in &rules {
                if (rule.1 <= v && v <= rule.2) || (rule.3 <= v && v <= rule.4) {
                    is_valid = true;
                }
            }
            if !is_valid {
                s += v;
            }
        }
    }

    dbg!(s);
    Ok(())
}

#[test]
fn part2() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("data/day16.txt").unwrap();
    let mut raw_input = String::new();
    file.read_to_string(&mut raw_input).unwrap();

    let mut lines = raw_input.lines();
    let mut line = lines.next().unwrap();

    let mut rules: Vec<(String, u64, u64, u64, u64)> = Vec::new();
    let re = Regex::new(r"([^:]+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
    while line != "" {
        let r = re.captures_iter(line).next().unwrap();
        rules.push((
            r[1].to_owned(),
            r[2].parse().unwrap(),
            r[3].parse().unwrap(),
            r[4].parse().unwrap(),
            r[5].parse().unwrap(),
        ));
        line = lines.next().unwrap();
    }

    lines.next();

    let my_ticket: Vec<u64> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();
    lines.next();
    lines.next(); // Nearby

    let mut tickets = Vec::new();
    while let Some(line) = lines.next() {
        tickets.push(
            line.split(",")
                .map(|n| n.parse().unwrap())
                .collect::<Vec<u64>>(),
        );
    }

    let mut rules_order = (0..20)
        .map(|_| {
            rules
                .iter()
                .map(|(name, _, _, _, _)| name.clone())
                .collect::<HashSet<String>>()
        })
        .collect::<Vec<_>>();

    let valid_tickets = tickets.into_iter().filter(|ticket| {
        ticket.iter().all(|&v| {
            rules
                .iter()
                .any(|rule| (rule.1 <= v && v <= rule.2) || (rule.3 <= v && v <= rule.4))
        })
    });

    for ticket in valid_tickets {
        for (i, &v) in ticket.iter().enumerate() {
            for rule in &rules {
                if (rule.1 <= v && v <= rule.2) || (rule.3 <= v && v <= rule.4) {
                } else {
                    rules_order[i].remove(&rule.0);
                }
            }
        }
    }

    // cleanup duplicates
    for _ in 0..10 {
        let rr = rules_order.clone();
        for (i, r) in rr.iter().enumerate() {
            if r.len() == 1 {
                let rn = r.iter().next().unwrap();
                for (j, rule) in rules_order.iter_mut().enumerate() {
                    if i != j {
                        rule.remove(rn);
                    }
                }
            }
        }
    }

    let to_mul: HashSet<usize> = rules_order
        .iter()
        .enumerate()
        .filter_map(|(i, n)| {
            if n.iter().next().unwrap().starts_with("departure") {
                Some(i)
            } else {
                None
            }
        })
        .collect();

    let result: u64 = my_ticket
        .iter()
        .enumerate()
        .filter_map(|(i, &n)| if to_mul.contains(&i) { Some(n) } else { None })
        .product();

    dbg!(result);

    Ok(())
}
