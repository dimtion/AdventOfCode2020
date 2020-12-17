use std::{error::Error, fs::File, io::Read};

type Space4 = Vec<Vec<Vec<Vec<bool>>>>;
type Space3 = Vec<Vec<Vec<bool>>>;

#[test]
fn part1() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("data/day17.txt").unwrap();
    let mut raw_input = String::new();
    file.read_to_string(&mut raw_input).unwrap();

    let dim = 23;
    let mut space: Space3 = (0..dim)
        .map(|_| {
            (0..dim)
                .map(|_| (0..dim).map(|_| false).collect())
                .collect()
        })
        .collect();

    let offset = dim / 2;
    for (i, line) in raw_input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            space[offset][offset + i][offset + j] = match c {
                '#' => true,
                '.' => false,
                _ => panic!("fds"),
            }
        }
    }

    for _ in 0..6 {
        space = exec3(&space);
    }

    let count = space.iter().flatten().flatten().filter(|&&e| e).count();

    assert_eq!(353, count);
    dbg!(count);

    Ok(())
}

#[inline]
fn is_in(x: isize, y: isize, z: isize, space: &Space3) -> bool {
    0 <= x
        && x < space.len() as isize
        && 0 <= y
        && y < space[0].len() as isize
        && 0 <= z
        && z < space[0][0].len() as isize
}

fn get_neigh3() -> Vec<(isize, isize, isize)> {
    let mut n = Vec::new();
    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                if x != 0 || y != 0 || z != 0 {
                    n.push((x, y, z));
                }
            }
        }
    }
    n
}

fn exec3(space: &Space3) -> Space3 {
    let mut space2 = space.clone();
    let neigh = get_neigh3();
    for x in 0..(space.len()) {
        for y in 0..space[0].len() {
            for z in 0..space[0][0].len() {
                let x = x as isize;
                let y = y as isize;
                let z = z as isize;
                let mut neigh_c = 0;
                for (i, j, k) in &neigh {
                    if is_in(x + i, y + j, z + k, &space)
                        && space[(x + i) as usize][(y + j) as usize][(z + k) as usize]
                    {
                        neigh_c += 1;
                    }
                }

                if space[(x) as usize][(y) as usize][(z) as usize] {
                    if neigh_c != 2 && neigh_c != 3 {
                        space2[x as usize][y as usize][z as usize] = false;
                    }
                } else {
                    if neigh_c == 3 {
                        space2[x as usize][y as usize][z as usize] = true;
                    }
                }
            }
        }
    }
    space2
}

#[test]
fn part2() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("data/day17.txt").unwrap();
    let mut raw_input = String::new();
    file.read_to_string(&mut raw_input).unwrap();

    let turns = 6;
    let dim_x = 8 + 2 * turns + 1;
    let dim_y = 8 + 2 * turns + 1;
    let dim_z = turns * 2 + 1;
    let dim_w = turns * 2 + 1;
    let mut space: Space4 = (0..dim_x)
        .map(|_| {
            (0..dim_y)
                .map(|_| {
                    (0..dim_z)
                        .map(|_| (0..dim_w).map(|_| false).collect())
                        .collect()
                })
                .collect()
        })
        .collect();

    for (i, line) in raw_input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            space[(dim_x / 2) + i - 4][(dim_y / 2) + j - 4][dim_z / 2][dim_w / 2] = match c {
                '#' => true,
                '.' => false,
                _ => panic!("fds"),
            }
        }
    }

    for i in 0..turns {
        space = exec4(&space);
        dbg!(i);
    }

    let count = space
        .iter()
        .flatten()
        .flatten()
        .flatten()
        .filter(|&&e| e)
        .count();

    assert_eq!(2472, count);
    dbg!(count);

    Ok(())
}

#[inline]
fn is_in4(x: isize, y: isize, z: isize, w: isize, space: &Space4) -> bool {
    0 <= x
        && x < space.len() as isize
        && 0 <= y
        && y < space[0].len() as isize
        && 0 <= z
        && z < space[0][0].len() as isize
        && 0 <= w
        && w < space[0][0][0].len() as isize
}

fn get_neigh4() -> Vec<(isize, isize, isize, isize)> {
    let mut n = Vec::new();
    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                for w in -1..=1 {
                    if x != 0 || y != 0 || z != 0 || w != 0 {
                        n.push((x, y, z, w));
                    }
                }
            }
        }
    }
    n
}

fn exec4(space: &Space4) -> Space4 {
    let mut space2 = space.clone();
    let neigh = get_neigh4();
    for x in 0..(space.len()) {
        for y in 0..space[0].len() {
            for z in 0..space[0][0].len() {
                for w in 0..space[0][0][0].len() {
                    let x = x as isize;
                    let y = y as isize;
                    let z = z as isize;
                    let w = w as isize;
                    let mut neigh_c = 0;
                    for &(i, j, k, l) in neigh.iter() {
                        if is_in4(x + i, y + j, z + k, w + l, &space)
                            && space[(x + i) as usize][(y + j) as usize][(z + k) as usize]
                                [(w + l) as usize]
                        {
                            neigh_c += 1;
                        }
                    }

                    if space[(x) as usize][(y) as usize][(z) as usize][w as usize] {
                        if neigh_c != 2 && neigh_c != 3 {
                            space2[x as usize][y as usize][z as usize][w as usize] = false;
                        }
                    } else {
                        if neigh_c == 3 {
                            space2[x as usize][y as usize][z as usize][w as usize] = true;
                        }
                    }
                }
            }
        }
    }
    space2
}
