use std::{
    cmp::{max, min},
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

pub const PARTS: [fn(); 2] = [part1, part2];

fn part1() {
    let ans: u32 =
        BufReader::new(File::open("inputfiles/day2/input.txt").unwrap())
            .lines()
            .map(|l| l.unwrap())
            .map(|l| {
                match l
                    .split_whitespace()
                    .map(|s| s.parse::<u32>().unwrap())
                    .minmax()
                {
                    itertools::MinMaxResult::MinMax(a, b) => b - a,
                    _ => unreachable!(),
                }
            })
            .sum();
    //
    println!("{}", ans);
}

fn part2() {
    let ans: u32 =
        BufReader::new(File::open("inputfiles/day2/input.txt").unwrap())
            .lines()
            .map(|l| l.unwrap())
            .map(|l| {
                l.split_whitespace()
                    .map(|s| s.parse::<u32>().unwrap())
                    .tuple_combinations()
                    .filter(|(a, b)| a % b == 0 || b % a == 0)
                    .next()
                    .and_then(|(a, b)| Some(max(a, b) / min(a, b)))
                    .unwrap()
            })
            .sum();
    //
    println!("{}", ans);
}
