use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use arrayvec::ArrayVec;

pub const PARTS: [fn(); 2] = [part1, part2];

fn find_connected(
    network: &Vec<Vec<usize>>,
    visited: &mut HashSet<usize>,
    from: usize,
) {
    for &node in &network[from] {
        if !visited.contains(&node) {
            visited.insert(node);
            find_connected(network, visited, node);
        }
    }
}

fn part1() {
    let inp: Vec<Vec<usize>> =
        BufReader::new(File::open("inputfiles/day12/input.txt").unwrap())
            .lines()
            .map(|l| l.unwrap())
            .map(|l| {
                if let [_, b] =
                    l.split(" <-> ").collect::<ArrayVec<[_; 2]>>().as_slice()
                {
                    b.split(", ").map(|n| n.parse().unwrap()).collect()
                } else {
                    unreachable!()
                }
            })
            .collect();
    //
    let mut connected = HashSet::new();
    //
    find_connected(&inp, &mut connected, 0);
    //
    println!("{}", connected.len());
}

fn part2() {
    let inp: Vec<Vec<usize>> =
        BufReader::new(File::open("inputfiles/day12/input.txt").unwrap())
            .lines()
            .map(|l| l.unwrap())
            .map(|l| {
                if let [_, b] =
                    l.split(" <-> ").collect::<ArrayVec<[_; 2]>>().as_slice()
                {
                    b.split(", ").map(|n| n.parse().unwrap()).collect()
                } else {
                    unreachable!()
                }
            })
            .collect();
    //
    let mut visited = HashSet::new();
    let mut groups = 0;
    //
    for i in 0..inp.len() {
        if !visited.contains(&i) {
            groups += 1;
            find_connected(&inp, &mut visited, i);
        }
    }
    //
    println!("{}", groups);
}
