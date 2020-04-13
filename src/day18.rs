use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub const PARTS: [fn(); 2] = [part1, part2];

fn compile_code(filename: &str) {
    let temp =
        BufReader::new(File::open("inputfiles/day16/input.txt").unwrap())
            .lines()
            .map(|l| l.unwrap());
}

fn part1() {}

fn part2() {}
