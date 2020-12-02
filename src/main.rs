use std::{cmp::Ordering, fs};
fn main() {
    println!("Day 1: {}", solve_1());
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
