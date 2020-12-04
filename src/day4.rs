use std::{error::Error, fs::File, io::BufRead, io::BufReader};

use regex::Regex;
#[derive(Default, Debug)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
    eyr: Option<String>,
    ecl: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
}

impl Passport {
    fn is_valid1(&self) -> bool {
        if self.byr.is_none() {
            return false;
        }
        if self.iyr.is_none() {
            return false;
        }
        if self.pid.is_none() {
            return false;
        }
        // pass cid
        if self.eyr.is_none() {
            return false;
        }
        if self.ecl.is_none() {
            return false;
        }
        if self.hgt.is_none() {
            return false;
        }
        if self.hcl.is_none() {
            return false;
        }
        true
    }

    fn is_valid2(&self) -> bool {
        if let Some(byr) = &self.byr {
            if let Ok(byr) = byr.parse::<i32>() {
                if !(byr >= 1920 && byr <= 2002) {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            return false;
        }

        if let Some(iyr) = &self.iyr {
            if let Ok(iyr) = iyr.parse::<u32>() {
                if !(iyr >= 2010 && iyr <= 2020) {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            return false;
        }

        if let Some(pid) = &self.pid {
            let re = Regex::new(r"^\d{9}$").unwrap();
            if !re.is_match(pid) {
                return false;
            }
        } else {
            return false;
        }

        // pass cid

        if let Some(eyr) = &self.eyr {
            if let Ok(eyr) = eyr.parse::<i32>() {
                if !(eyr >= 2020 && eyr <= 2030) {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            return false;
        }

        if let Some(ecl) = &self.ecl {
            match ecl.as_ref() {
                "amb" => (),
                "blu" => (),
                "brn" => (),
                "gry" => (),
                "grn" => (),
                "hzl" => (),
                "oth" => (),
                _ => return false,
            }
        } else {
            return false;
        }

        if let Some(hgt) = &self.hgt {
            let re = Regex::new(r"^(\d+)(cm|in)$").unwrap();
            let mut m = re.captures_iter(hgt.as_ref());
            if let Some(m) = m.next() {
                let n = if let Ok(x) = m[1].parse::<u32>() {
                    x
                } else {
                    return false;
                };
                match &m[2] {
                    "in" => {
                        if !(n >= 59 && n <= 76) {
                            return false;
                        }
                    }
                    "cm" => {
                        if !(n >= 150 && n <= 193) {
                            return false;
                        }
                    }
                    x => panic!("invalid inch {}", x),
                }
            } else {
                return false;
            }
        } else {
            return false;
        }

        if let Some(hcl) = &self.hcl {
            let re = Regex::new(r"^\#[\da-f]{6}$").unwrap();
            if !re.is_match(hcl) {
                return false;
            }
        } else {
            return false;
        }
        true
    }
}

fn get_passports() -> impl Iterator<Item = Passport> {
    let buf = BufReader::new(File::open("data/day4.txt").unwrap());
    let re = Regex::new(r"(\w+):([^\s]+)").unwrap();
    buf.lines()
        .map(|l| l.unwrap())
        .scan(Passport::default(), move |passport, line| {
            if line == "" {
                let mut pass = Passport::default();
                std::mem::swap(passport, &mut pass);
                return Some(Some(pass));
            }
            re.captures_iter(&line).for_each(|e| {
                match &e[1] {
                    "byr" => passport.byr = Some(e[2].to_owned()),
                    "iyr" => passport.iyr = Some(e[2].to_owned()),
                    "pid" => passport.pid = Some(e[2].to_owned()),
                    "cid" => passport.cid = Some(e[2].to_owned()),
                    "eyr" => passport.eyr = Some(e[2].to_owned()),
                    "ecl" => passport.ecl = Some(e[2].to_owned()),
                    "hgt" => passport.hgt = Some(e[2].to_owned()),
                    "hcl" => passport.hcl = Some(e[2].to_owned()),
                    x => panic!(format!("ERROR: {}", x)),
                };
            });
            Some(None)
        })
        .filter_map(|p| p)
}

#[test]
fn part1() {
    let valid = get_passports().filter(Passport::is_valid1).count();
    println!("Valid: {}", valid);
}

#[test]
fn part2() {
    let valid = get_passports().filter(Passport::is_valid2).count();
    println!("Valid: {}", valid);
}
