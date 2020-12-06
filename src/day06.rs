use std::{error::Error, fs::File, io::BufRead, io::BufReader};

#[test]
fn part1() -> Result<(), Box<dyn Error>> {
    let buf = BufReader::new(File::open("data/day06.txt")?);
    let (tot, _) = buf
        .lines()
        .fold((0, [0; 26]), |(mut tot, mut group), line| {
            let line = line.unwrap();
            if line.is_empty() {
                tot += group.iter().sum::<i32>();
                group = [0; 26];
            } else {
                line.bytes().for_each(|c| {
                    group[(c - 97) as usize] |= 1;
                });
            }
            (tot, group)
        });

    println!("Total {}", tot);
    Ok(())
}

#[test]
fn part2() -> Result<(), Box<dyn Error>> {
    let buf = BufReader::new(File::open("data/day06.txt")?);
    let (tot, _, _) = buf.lines().fold(
        (0, [0; 26], 0),
        |(mut tot, mut group, mut group_len), line| {
            let line = line.unwrap();
            if line.is_empty() {
                tot += group.iter().filter(|&&c| c == group_len).count();
                group = [0; 26];
                group_len += 0;
            } else {
                group_len += 1;
                line.bytes().for_each(|c| {
                    group[(c - 97) as usize] += 1;
                });
            }
            (tot, group, group_len)
        },
    );

    println!("Total {}", tot);
    Ok(())
}
