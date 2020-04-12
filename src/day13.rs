use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use arrayvec::ArrayVec;

pub const PARTS: [fn(); 2] = [part1, part2];

fn part1() {
    let ans = BufReader::new(File::open("inputfiles/day13/input.txt").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            if let [a, b] = l
                .split(": ")
                .map(|x| x.parse().unwrap())
                .collect::<ArrayVec<[u32; 2]>>()
                .as_slice()
            {
                (*a, *b)
            } else {
                unreachable!()
            }
        })
        .fold(0, |s, (d, r)| s + ((d % ((r - 1) * 2) == 0) as u32) * d * r);
    //
    println!("{}", ans);
}

fn part2() {
    let inp: Vec<_> =
        BufReader::new(File::open("inputfiles/day13/input.txt").unwrap())
            .lines()
            .map(|l| l.unwrap())
            .map(|l| {
                if let [a, b] = l
                    .split(": ")
                    .map(|x| x.parse().unwrap())
                    .collect::<ArrayVec<[u32; 2]>>()
                    .as_slice()
                {
                    (*a, *b)
                } else {
                    unreachable!()
                }
            })
            .collect();
    //
    let ans = (0..)
        .skip_while(|&i| inp.iter().any(|(d, r)| (d + i) % ((r - 1) * 2) == 0))
        .next()
        .unwrap();
    //
    println!("{}", ans);
}
