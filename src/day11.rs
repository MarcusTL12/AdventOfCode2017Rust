use std::{
    cmp::min,
    fs::File,
    io::{BufRead, BufReader},
};

use num::Complex;

pub const PARTS: [fn(); 2] = [part1, part2];

fn dist(pos: Complex<i32>) -> usize {
    let dist = if num::signum(pos.re) == num::signum(pos.im) {
        (pos.re + pos.im).abs()
    } else {
        min(pos.re.abs(), pos.im.abs()) + (pos.re + pos.im).abs()
    };
    dist as usize
}

fn part1() {
    let pos = BufReader::new(File::open("inputfiles/day11/input.txt").unwrap())
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|d| match d {
            "n" => Complex { re: 1, im: 0 },
            "s" => Complex { re: -1, im: 0 },
            "ne" => Complex { re: 0, im: 1 },
            "sw" => Complex { re: 0, im: -1 },
            "nw" => Complex { re: 1, im: -1 },
            "se" => Complex { re: -1, im: 1 },
            _ => unreachable!(),
        })
        .sum();
    //
    let dist = dist(pos);
    //
    println!("{}", dist);
}

fn part2() {
    let ans = BufReader::new(File::open("inputfiles/day11/input.txt").unwrap())
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .scan(Complex { re: 0, im: 0 }, |pos, d| {
            *pos += match d {
                "n" => Complex { re: 1, im: 0 },
                "s" => Complex { re: -1, im: 0 },
                "ne" => Complex { re: 0, im: 1 },
                "sw" => Complex { re: 0, im: -1 },
                "nw" => Complex { re: 1, im: -1 },
                "se" => Complex { re: -1, im: 1 },
                _ => unreachable!(),
            };
            Some(dist(*pos))
        })
        .max()
        .unwrap();
    //
    println!("{}", ans);
}
