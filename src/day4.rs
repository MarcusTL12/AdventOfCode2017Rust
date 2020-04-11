use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

pub const PARTS: [fn(); 2] = [part1, part2];

fn part1() {
    let ans = BufReader::new(File::open("inputfiles/day4/input.txt").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            l.split(' ')
                .scan((HashSet::new(), true), |(set, okay), word| {
                    if *okay {
                        if set.contains(word) {
                            *okay = false;
                        } else {
                            set.insert(word);
                        }
                    }
                    Some(*okay)
                })
                .all(|x| x)
        })
        .filter(|&x| x)
        .count();
    //
    println!("{}", ans);
}

fn part2() {
    let ans = BufReader::new(File::open("inputfiles/day4/input.txt").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            l.split(' ')
                .scan((HashSet::new(), true), |(set, okay), word| {
                    if *okay {
                        if set.contains(
                            &word.chars().sorted().collect::<Vec<_>>(),
                        ) {
                            *okay = false;
                        } else {
                            set.insert(
                                word.chars().sorted().collect::<Vec<_>>(),
                            );
                        }
                    }
                    Some(*okay)
                })
                .all(|x| x)
        })
        .filter(|&x| x)
        .count();
    //
    println!("{}", ans);
}
