use std::{cmp::Ordering, fs};
fn main() {
    println!("Day 1: {}", solve_1());

}

fn solve_1() -> String {
    let contents = fs::read_to_string("input/1.txt")
        .expect("Unable to read file");

    let mut ints : Vec<_> = contents.lines().map(|int| int.parse::<i32>().expect("Unable to parse integer")).collect();
    ints.sort();
    let mut start = ints.iter();
    let mut end = ints.iter().rev();
    let mut s = start.next().expect("Empty file");
    let mut e = end.next().expect("Empty file");
    loop {
        match (s + e).cmp(&2020) {
            Ordering::Equal => return format!("{}", s * e),
            Ordering::Greater => e = end.next().unwrap(),
            Ordering::Less => s = start.next().unwrap(),
        }
    }


}
