use std::{error::Error, fs::File, io::Read};

type Layout = Vec<Vec<Seat>>;
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Seat {
    Free,
    Occ,
    Floor,
}

#[test]
fn part1() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("data/day11.txt").unwrap();
    let mut raw_input = String::new();
    file.read_to_string(&mut raw_input).unwrap();
    let layout: Vec<Vec<_>> = raw_input
        .lines()
        .map(|line| {
            line.bytes()
                .map(|c| match c {
                    b'L' => Seat::Free,
                    b'.' => Seat::Floor,
                    b'#' => Seat::Occ,
                    _ => panic!(" TOTO"),
                })
                .collect()
        })
        .collect();

    let mut old = Vec::new();
    let mut new = layout.clone();
    while old != new {
        std::mem::swap(&mut old, &mut new);
        simulate1(&mut new, &old);
    }

    let c: usize = new
        .iter()
        .map(|r| r.iter().filter(|c| **c == Seat::Occ).count())
        .sum();
    println!("C: {}", c);
    assert_eq!(c, 2424);
    Ok(())
}

fn count_occ1(layout: &Layout, i: usize, j: usize) -> usize {
    vec![
        (0, -1),
        (0, 1),
        (-1, 0),
        (-1, -1),
        (-1, 1),
        (1, 0),
        (1, -1),
        (1, 1),
    ]
    .iter()
    .filter(|(k, l)| {
        let i = i as isize + k;
        let j = j as isize + l;
        if inside(&layout, i, j) {
            layout[i as usize][j as usize] == Seat::Occ
        } else {
            false
        }
    })
    .count()
}

fn simulate1(new: &mut Layout, layout: &Layout) {
    *new = layout.clone();
    for i in 0..(layout.len()) {
        for j in 0..(layout[0].len()) {
            match layout[i][j] {
                Seat::Free => {
                    if count_occ1(layout, i, j) == 0 {
                        new[i][j] = Seat::Occ;
                    }
                }
                Seat::Occ => {
                    if count_occ1(layout, i, j) >= 4 {
                        new[i][j] = Seat::Free;
                    }
                }
                Seat::Floor => {}
            }
        }
    }
}

#[test]
fn part2() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("data/day11.txt").unwrap();
    let mut raw_input = String::new();
    file.read_to_string(&mut raw_input).unwrap();
    let layout: Vec<Vec<_>> = raw_input
        .lines()
        .map(|line| {
            line.bytes()
                .map(|c| match c {
                    b'L' => Seat::Free,
                    b'.' => Seat::Floor,
                    b'#' => Seat::Occ,
                    _ => panic!(" TOTO"),
                })
                .collect()
        })
        .collect();

    let mut old = vec![vec![Seat::Free; layout[0].len()]; layout.len()];
    let mut new = layout.clone();
    while old != new {
        std::mem::swap(&mut old, &mut new);
        simulate2(&mut new, &old);
    }

    let c: usize = new
        .iter()
        .map(|r| r.iter().filter(|c| **c == Seat::Occ).count())
        .sum();
    println!("C {}", c);
    assert_eq!(c, 2208);
    Ok(())
}

fn inside(layout: &Layout, i: isize, j: isize) -> bool {
    i >= 0 && i < layout.len() as isize && j >= 0 && j < layout[0].len() as isize
}

fn count_occ2(layout: &Layout, i: usize, j: usize) -> usize {
    let directions = vec![
        (0, -1),
        (0, 1),
        (-1, 0),
        (-1, -1),
        (-1, 1),
        (1, 0),
        (1, -1),
        (1, 1),
    ];

    let mut c = 0;
    for (k0, l0) in directions {
        let mut k = i as isize + k0;
        let mut l = j as isize + l0;

        while inside(&layout, k, l) && layout[k as usize][l as usize] == Seat::Floor {
            k += k0;
            l += l0;
        }
        if inside(&layout, k, l) && layout[k as usize][l as usize] == Seat::Occ {
            c += 1;
        }
    }
    c
}

fn simulate2(new: &mut Layout, layout: &Layout) {
    for i in 0..(layout.len()) {
        for j in 0..(layout[0].len()) {
            new[i][j] = match layout[i][j] {
                Seat::Free if count_occ2(&layout, i, j) == 0 => Seat::Occ,
                Seat::Occ if count_occ2(&layout, i, j) >= 5 => Seat::Free,
                s => s,
            }
        }
    }
}
