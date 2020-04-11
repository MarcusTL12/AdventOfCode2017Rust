use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

pub const PARTS: [fn(); 2] = [part1, part2];

fn redistribute(banks: &mut [u32]) {
    let maxind = banks
        .iter()
        .enumerate()
        .rev()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap()
        .0;
    //
    let mut blocks = banks[maxind];
    banks[maxind] = 0;
    let mut i = (maxind + 1) % banks.len();
    while blocks > 0 {
        banks[i] += 1;
        blocks -= 1;
        i = (i + 1) % banks.len();
    }
}

fn part1() {
    let inp: Vec<u32> =
        BufReader::new(File::open("inputfiles/day6/input.txt").unwrap())
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
    //
    let ans = (0..)
        .scan((HashSet::new(), inp), |(set, banks), _| {
            if set.contains(banks) {
                None
            } else {
                set.insert(banks.clone());
                redistribute(banks);
                Some(())
            }
        })
        .count();
    //
    println!("{}", ans);
}

fn part2() {
    let mut banks: Vec<u32> =
        BufReader::new(File::open("inputfiles/day6/input.txt").unwrap())
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
    //
    let mut ans = 0;
    //
    let mut map = HashMap::new();
    //
    for i in 0.. {
        if !map.contains_key(&banks) {
            map.insert(banks.clone(), i);
            redistribute(&mut banks);
        } else {
            ans = i - map[&banks];
            break;
        }
    }
    //
    println!("{}", ans);
}
