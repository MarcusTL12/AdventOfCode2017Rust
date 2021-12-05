use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

pub const PARTS: [fn(); 2] = [part1, part2];

fn part1() {
    let mut first = None;
    let mut last = 0;
    //
    let ans = BufReader::new(File::open("inputfiles/day1/input.txt").unwrap())
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .map(|c| {
            let val = c as u32 - b'0' as u32;
            if first.is_none() {
                first = Some(val);
            }
            last = val;
            val
        })
        .tuple_windows()
        .fold(0, |sum, (a, b)| if a == b { sum + a } else { sum })
        + if let Some(first) = first {
            if first == last {
                first
            } else {
                0
            }
        } else {
            unreachable!()
        };
    //
    println!("{}", ans);
}

fn part2() {
    let inp: Vec<_> =
        BufReader::new(File::open("inputfiles/day1/input.txt").unwrap())
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .chars()
            .map(|c| c as u32 - b'0' as u32)
            .collect();
    //
    let ans: u32 = inp
        .iter()
        .enumerate()
        .filter_map(|(i, &v)| {
            if v == inp[(i + inp.len() / 2) % inp.len()] {
                Some(v)
            } else {
                None
            }
        })
        .sum();
    //
    println!("{}", ans);
}
