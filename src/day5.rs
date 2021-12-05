use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub const PARTS: [fn(); 2] = [part1, part2];

fn part1() {
    let inp: Vec<i32> =
        BufReader::new(File::open("inputfiles/day5/input.txt").unwrap())
            .lines()
            .map(|l| l.unwrap())
            .map(|l| l.parse().unwrap())
            .collect();
    //
    let ans = (0..)
        .scan((inp, 0), |(mem, i), _| {
            if *i >= 0 && *i < mem.len() as i32 {
                let jump = mem[*i as usize];
                mem[*i as usize] += 1;
                *i += jump;
                Some(())
            } else {
                None
            }
        })
        .count();
    //
    println!("{}", ans);
}

fn part2() {
    let inp: Vec<i32> =
        BufReader::new(File::open("inputfiles/day5/input.txt").unwrap())
            .lines()
            .map(|l| l.unwrap())
            .map(|l| l.parse().unwrap())
            .collect();
    //
    let ans = (0..)
        .scan((inp, 0), |(mem, i), _| {
            if *i >= 0 && *i < mem.len() as i32 {
                let jump = mem[*i as usize];
                mem[*i as usize] += if jump >= 3 { -1 } else { 1 };
                *i += jump;
                Some(())
            } else {
                None
            }
        })
        .count();
    //
    println!("{}", ans);
}
