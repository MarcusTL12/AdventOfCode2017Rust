use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use arrayvec::ArrayVec;

use itertools::Itertools;

use regex::Regex;

use lazy_static::*;

pub const PARTS: [fn(); 2] = [part1, part2];

lazy_static! {
    static ref REG: Regex = Regex::new(concat!(
        r"p=<(-?\d+),(-?\d+),(-?\d+)>, ",
        r"v=<(-?\d+),(-?\d+),(-?\d+)>, ",
        r"a=<(-?\d+),(-?\d+),(-?\d+)>"
    ))
    .unwrap();
}

fn do_time_step([p, v, a]: &mut [[i32; 3]; 3]) {
    for (v, a) in v.iter_mut().zip(a.iter()) {
        *v += a;
    }
    for (p, v) in p.iter_mut().zip(v.iter()) {
        *p += v;
    }
}

fn simulate_all(particles: &mut [[[i32; 3]; 3]]) {
    for p in particles.iter_mut() {
        do_time_step(p);
    }
}

fn load_input(filename: &str) -> Vec<[[i32; 3]; 3]> {
    BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            if let Some(c) = REG.captures(&l) {
                let vals = (1..10)
                    .map(|i| c[i].parse().unwrap())
                    .collect::<ArrayVec<[i32; 9]>>()
                    .into_inner()
                    .unwrap();
                //
                (0..3)
                    .map(|i| {
                        vals[i * 3..i * 3 + 3]
                            .iter()
                            .cloned()
                            .collect::<ArrayVec<[_; 3]>>()
                            .into_inner()
                            .unwrap()
                    })
                    .collect::<ArrayVec<[_; 3]>>()
                    .into_inner()
                    .unwrap()
            } else {
                unreachable!("{}", l)
            }
        })
        .collect()
}

fn part1() {
    let inp = load_input("inputfiles/day20/input.txt");
    //
    let ans = inp
        .iter()
        .map(|[_, _, a]| a.iter().map(|&x| x.abs()).sum::<i32>())
        .enumerate()
        .min_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap()
        .0;
    //
    println!("{}", ans);
}

fn remove_collisions(particles: &mut Vec<[[i32; 3]; 3]>) -> bool {
    let mut to_be_removed = HashSet::new();
    //
    for i in 0..particles.len() {
        for j in i + 1..particles.len() {
            if particles[i][0] == particles[j][0] {
                to_be_removed.insert(i);
                to_be_removed.insert(j);
            }
        }
    }
    //
    let ans = to_be_removed.len() > 0;
    //
    for i in to_be_removed.into_iter().sorted().rev() {
        particles.swap_remove(i);
    }
    //
    ans
}

fn part2() {
    let mut particles = load_input("inputfiles/day20/input.txt");
    //
    for _ in 0..39 {
        simulate_all(&mut particles);
        remove_collisions(&mut particles);
    }
    //
    println!("{}", particles.len());
}
