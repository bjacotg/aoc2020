use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::{cmp::Ordering, fs};

fn main() {
    println!("Day 1: {}", solve_1());
    println!("Day 2: {}", solve_2());
    println!("Day 3: {}", solve_3());
    println!("Day 4: {}", solve_4());
    println!("Day 5: {}", solve_5());
    println!("Day 6: {}", solve_6());
}

fn solve_1() -> String {
    let contents = fs::read_to_string("input/1.txt").expect("Unable to read file");

    let mut ints: Vec<_> = contents
        .lines()
        .map(|int| int.parse::<i32>().expect("Unable to parse integer"))
        .collect();
    ints.sort();
    let find_sum = |ints: &[i32], target| {
        let mut iter = ints.iter();
        let mut s = iter.next()?;
        let mut e = iter.next_back()?;
        loop {
            match (s + e).cmp(&target) {
                Ordering::Equal => break Some(s * e),
                Ordering::Greater => e = iter.next_back()?,
                Ordering::Less => s = iter.next()?,
            }
        }
    };
    let part1 = find_sum(&ints, 2020).expect("Could not find values for part 1");

    let part2 = {
        let mut iter = ints.iter().enumerate();
        loop {
            let (idx, value) = iter.next().expect("Could not find solution for part 2");
            if let Some(pdt) = find_sum(&ints[idx + 1..], 2020 - value) {
                break pdt * value;
            }
        }
    };

    format!("{:?} {:?}", part1, part2)
}

fn solve_2() -> String {
    let contents = fs::read_to_string("input/2.txt").expect("Unable to read file");
    let re = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").expect("Bad regex");
    struct Lines {
        min: i32,
        max: i32,
        ch: char,
        pass: String,
    };
    let parse_line = |line| {
        let cap = re.captures(line).expect("Line did not match");
        Lines {
            min: cap[1].parse::<i32>().unwrap(),
            max: cap[2].parse::<i32>().unwrap(),
            ch: cap[3].chars().next().unwrap(),
            pass: cap[4].to_string(),
        }
    };

    let parsed_lines: Vec<_> = contents.lines().map(&parse_line).collect();

    let part1 = parsed_lines
        .iter()
        .filter(|line| {
            let count = line.pass.chars().filter(|c| *c == line.ch).count() as i32;
            line.min <= count && count <= line.max
        })
        .count();

    let part2 = parsed_lines
        .iter()
        .filter(|line| {
            let first = line.pass.chars().nth(line.min as usize - 1).unwrap();
            let second = line.pass.chars().nth(line.max as usize - 1).unwrap();
            (first == line.ch) != (second == line.ch)
        })
        .count();

    format!("{} {}", part1, part2)
}

fn solve_3() -> String {
    let contents = fs::read_to_string("input/3.txt").expect("Unable to read file");
    let trees_on_path = |down: usize, right: usize| {
        contents
            .lines()
            .enumerate()
            .filter(|(nb, line)| {
                let position = (right * nb / down) % line.len();
                nb % down == 0 && line.chars().nth(position).unwrap() == '#'
            })
            .count()
    };

    let part1 = trees_on_path(1, 3);
    let part2 = trees_on_path(1, 1)
        * trees_on_path(1, 3)
        * trees_on_path(1, 5)
        * trees_on_path(1, 7)
        * trees_on_path(2, 1);

    format!("{} {}", part1, part2)
}

fn solve_4() -> String {
    let contents = fs::read_to_string("input/4.txt").expect("Unable to read file");
    let necessary_tags = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    type Passport<'a> = HashMap<&'a str, &'a str>;
    let passports: Vec<_> = contents
        .split("\n\n")
        .map(|raw_passport| {
            raw_passport
                .split_whitespace()
                .map(|token| (&token[0..3], &token[4..]))
                .collect::<Passport>()
        })
        .collect();
    let part1 = passports
        .iter()
        .filter(|passport| necessary_tags.iter().all(|tag| passport.contains_key(tag)))
        .count();

    let hair_regex = Regex::new(r"^#[0-9a-f]{6}$").expect("Cannot parse hair regex");
    let passport_id = Regex::new(r"^[0-9]{9}$").expect("Cannot parse passport regex");
    let part2 = passports
        .iter()
        .filter(|passport: &&Passport| {
            let check_date = |tag, min, max| {
                passport
                    .get(tag)
                    .and_then(|val| val.parse::<i32>().ok())
                    .map(|val| min <= val && val <= max)
                    .unwrap_or(false)
            };
            let byr_correct = check_date("byr", 1920, 2002);
            let iyr_correct = check_date("iyr", 2010, 2020);
            let eyr_correct = check_date("eyr", 2020, 2030);
            let hgt_correct = {
                let check_height = |height: &str, unit: &str, min: i32, max: i32| {
                    height
                        .strip_suffix(unit)
                        .and_then(|h| h.parse::<i32>().ok())
                        .map(|h| min <= h && h <= max)
                        .unwrap_or(false)
                };
                passport
                    .get("hgt")
                    .map(|h| check_height(h, "cm", 150, 193) || check_height(h, "in", 59, 76))
                    .unwrap_or(false)
            };

            let hcl_correct = passport
                .get("hcl")
                .map(|h| hair_regex.is_match(h))
                .unwrap_or(false);

            let possible_eye = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
            let ecl_correct = passport
                .get("ecl")
                .map(|e| possible_eye.contains(e))
                .unwrap_or(false);

            let pid_correct = passport
                .get("pid")
                .map(|p| passport_id.is_match(p))
                .unwrap_or(false);

            byr_correct
                && iyr_correct
                && eyr_correct
                && hgt_correct
                && hcl_correct
                && ecl_correct
                && pid_correct
        })
        .count();

    format!("{} {}", part1, part2)
}
fn solve_5() -> String {
    let contents = fs::read_to_string("input/5.txt").expect("Unable to read file");
    let parse_seat = |seat: &str| {
        seat.chars().fold(0, |acc, x| {
            if x == 'R' || x == 'B' {
                2 * acc + 1
            } else {
                2 * acc
            }
        })
    };
    let seat_code: Vec<_> = contents.lines().map(&parse_seat).collect();
    let part1 = seat_code.iter().max().unwrap();
    let part2 = {
        let max = *seat_code.iter().max().unwrap();
        let min = *seat_code.iter().min().unwrap();
        let sum: i32 = seat_code.iter().sum();
        max * (max + 1) / 2 - sum - min * (min - 1) / 2
    };

    format!("{} {}", part1, part2)
}

fn solve_6() -> String {
    let contents = fs::read_to_string("input/6.txt").expect("Unable to read file");
    type Questions = HashSet<char>;
    type Group = Vec<Questions>;
    let groups: Vec<_> = contents
        .split("\n\n")
        .map(|g| {
            g.lines()
                .map(|q| q.chars().collect::<Questions>())
                .collect::<Group>()
        })
        .collect();

    let part1: i32 = groups
        .iter()
        .map(|g| {
            g.iter()
                .fold(Questions::new(), |acc, next| {
                    acc.union(&next).copied().collect::<Questions>()
                })
                .len() as i32
        })
        .sum();
        
    let all = "qwertyuioplkjhgfdsazxcvbnm".chars().collect::<Questions>();
    let part2: i32 = groups
        .iter()
        .map(|g| {
            g.iter()
                .fold(all.clone(), |acc, next| {
                    acc.intersection(&next).copied().collect::<Questions>()
                })
                .len() as i32
        })
        .sum();

    format!("{} {}", part1, part2)
}
