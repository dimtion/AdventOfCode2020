use std::{error::Error, fs::File, io::Read};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Action {
    N(i32),
    S(i32),
    E(i32),
    W(i32),
    L(i32),
    R(i32),
    F(i32),
}

impl From<String> for Action {
    fn from(mut s: String) -> Self {
        let c = s.remove(0);
        let d = s.parse().unwrap();
        match c {
            'N' => Self::N(d),
            'S' => Self::S(d),
            'E' => Self::E(d),
            'W' => Self::W(d),
            'L' => Self::L(d),
            'R' => Self::R(d),
            'F' => Self::F(d),
            _ => panic!("Invalid move"),
        }
    }
}

fn get_actions() -> Vec<Action> {
    let mut file = File::open("data/day12.txt").unwrap();
    let mut raw_input = String::new();
    file.read_to_string(&mut raw_input).unwrap();
    raw_input
        .lines()
        .map(|line| line.to_string().into())
        .collect()
}

fn rot_mat(angle: i32) -> (i32, i32) {
    match angle {
        0 => (1, 0),
        90 => (0, 1),
        180 => (-1, 0),
        270 => (0, -1),
        _ => panic!("invalid bearing"),
    }
}

#[test]
fn part1() -> Result<(), Box<dyn Error>> {
    let (mut x0, mut y0, mut bearing) = (0, 0, 0);
    for action in get_actions() {
        match action {
            Action::N(y) => y0 += y,
            Action::S(y) => y0 -= y,
            Action::E(x) => x0 += x,
            Action::W(x) => x0 -= x,
            Action::L(r) => bearing = (bearing - r + 360) % 360,
            Action::R(r) => bearing = (bearing + r + 360) % 360,
            Action::F(d) => {
                let (cos, sin) = rot_mat((360 - bearing) % 360);
                x0 += d * cos;
                y0 += d * sin;
            }
        }
    }
    println!("Solution {}", x0.abs() + y0.abs());
    Ok(())
}

#[test]
fn part2() -> Result<(), Box<dyn Error>> {
    let (mut x0, mut y0) = (0, 0);
    let (mut wx, mut wy) = (10, 1);
    for action in get_actions() {
        match action {
            Action::N(y) => wy += y,
            Action::S(y) => wy -= y,
            Action::E(x) => wx += x,
            Action::W(x) => wx -= x,
            Action::L(r) => {
                let (cos, sin) = rot_mat(r);
                let wx0 = wx;
                let wy0 = wy;
                wx = wx0 * cos - wy0 * sin;
                wy = wx0 * sin + wy0 * cos;
            }
            Action::R(r) => {
                let (cos, sin) = rot_mat((360 - r) % 360);
                let wx0 = wx;
                let wy0 = wy;
                wx = wx0 * cos - wy0 * sin;
                wy = wx0 * sin + wy0 * cos;
            }
            Action::F(d) => {
                x0 += wx * d;
                y0 += wy * d;
            }
        }
    }
    println!("Solution: {}", x0.abs() + y0.abs());

    Ok(())
}
