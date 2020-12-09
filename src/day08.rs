use std::{convert::TryInto, error::Error, fs::File, io::BufRead, io::BufReader};

#[derive(Debug, Copy, Clone)]
enum Instr {
    Acc(i64),
    Jmp(i64),
    Nop(i64),
}

type Program = Vec<Instr>;

impl From<String> for Instr {
    fn from(line: String) -> Self {
        let inst: Vec<_> = line.split(" ").collect();
        let v = inst[1].parse().unwrap();
        match inst[0] {
            "acc" => Self::Acc(v),
            "jmp" => Self::Jmp(v),
            "nop" => Self::Nop(v),
            i => panic!("Invalid instruction {}", i),
        }
    }
}

fn load_program() -> Program {
    let buf = BufReader::new(File::open("data/day08.txt").unwrap());
    buf.lines()
        .map(|line| {
            let line = line.unwrap();
            line.into()
        })
        .collect()
}

fn execute(program: Program) -> (bool, i64) {
    let mut acc = 0;
    let mut executed = vec![0; program.len()];
    let mut pointer = 0;

    while pointer < program.len() {
        if executed[pointer] == 1 {
            return (false, acc);
        } else {
            executed[pointer] += 1;
        }
        let mut jmp = 1;
        let inst = &program[pointer];
        match inst {
            Instr::Acc(c) => acc += c,
            Instr::Jmp(c) => jmp = *c,
            Instr::Nop(_) => (),
        }
        pointer = (pointer as i64 + jmp).try_into().unwrap();
    }
    (true, acc)
}

#[test]
fn part1() -> Result<(), Box<dyn Error>> {
    let program = load_program();
    let (_, acc) = execute(program);
    println!("Acc: {}", acc);
    Ok(())
}

#[test]
fn part2() -> Result<(), Box<dyn Error>> {
    let program = load_program();

    for i in 0..program.len() {
        let mut program = program.clone();
        match program[i] {
            Instr::Jmp(c) => program[i] = Instr::Nop(c),
            Instr::Nop(c) => program[i] = Instr::Jmp(c),
            _ => continue,
        };
        match execute(program) {
            (true, acc) => {
                println!("Acc {}", acc);
                break;
            }
            (false, _) => (),
        }
    }
    Ok(())
}
