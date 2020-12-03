use std::{cmp::Ordering, fs};
use regex::Regex;

fn main() {
    println!("Day 1: {}", solve_1());
    println!("Day 2: {}", solve_2());
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
    struct Lines{
        min:i32,
        max: i32, 
        ch: char,
        pass: String
    };
    let parse_line = |line|{
        let cap = re.captures(line).expect("Line did not match");
        Lines{
            min: cap[1].parse::<i32>().unwrap(),
            max: cap[2].parse::<i32>().unwrap(),
            ch: cap[3].chars().next().unwrap(),
            pass: cap[4].to_string(),
        }
    };

    let parsed_lines :Vec<_>= contents.lines().map(&parse_line).collect();

    let part1 = parsed_lines.iter().filter(|line|{
        let count = line.pass.chars().filter(|c| *c == line.ch).count() as i32;
        line.min <= count && count <= line.max
    }).count();

    let part2 = parsed_lines.iter().filter(|line|{
        let first = line.pass.chars().nth(line.min as usize - 1).unwrap();
        let second = line.pass.chars().nth(line.max as usize - 1).unwrap();
        (first == line.ch) != (second == line.ch)
    }).count();


    format!("{} {}", part1,part2) 
}