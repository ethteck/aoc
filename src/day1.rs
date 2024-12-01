use std::collections::HashSet;

use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> isize {
    let mut l1 = vec![];
    let mut l2 = vec![];
    for line in input.lines() {
        let (a, b) = line.split_once("   ").unwrap();
        l1.push(a.parse::<isize>().unwrap());
        l2.push(b.parse::<isize>().unwrap());
    }

    l1.sort();
    l2.sort();

    l1.iter().zip(l2.iter()).map(|(a, b)| (a - b).abs()).sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> isize {
    let mut l1 = HashSet::new();
    let mut l2 = vec![];

    for line in input.lines() {
        let (a, b) = line.split_once("   ").unwrap();
        l1.insert(a.parse::<isize>().unwrap());
        l2.push(b.parse::<isize>().unwrap());
    }

    l2.iter().filter(|a| l1.contains(a)).sum()
}
