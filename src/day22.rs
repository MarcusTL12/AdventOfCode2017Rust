use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

use num::Complex;

pub const PARTS: [fn(); 2] = [part1, part2];

fn load_input(filename: &str) -> HashMap<Complex<i32>, bool> {
    BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|l| l.unwrap())
        .enumerate()
        .fold(HashMap::new(), |mut ans, (y, l)| {
            for (k, v) in l.chars().enumerate().map(|(x, c)| {
                (
                    Complex {
                        re: x as i32,
                        im: -(y as i32),
                    },
                    match c {
                        '.' => false,
                        '#' => true,
                        _ => unreachable!(),
                    },
                )
            }) {
                ans.insert(k, v);
            }
            ans
        })
}

fn find_start<T>(map: &HashMap<Complex<i32>, T>) -> Complex<i32> {
    let (x1, x2) = match map.keys().map(|z| z.re).minmax() {
        itertools::MinMaxResult::MinMax(a, b) => (a, b),
        _ => unreachable!(),
    };
    let (y1, y2) = match map.keys().map(|z| z.im).minmax() {
        itertools::MinMaxResult::MinMax(a, b) => (a, b),
        _ => unreachable!(),
    };
    //
    let ans = Complex {
        re: x1 + (x2 - x1) / 2,
        im: y1 + (y2 - y1) / 2,
    };
    //
    ans
}

fn part1() {
    const IM: Complex<i32> = Complex { re: 0, im: 1 };
    //
    let mut map = load_input("inputfiles/day22/input.txt");
    //
    let mut pos = find_start(&map);
    let mut dir = IM;
    //
    let mut ans = 0;
    //
    for _ in 0..10_000 {
        dir *= match map.get(&pos) {
            Some(true) => -IM,
            _ => IM,
        };
        if let Some(x) = map.get_mut(&pos) {
            if !*x {
                ans += 1;
            }
            *x = !*x;
        } else {
            ans += 1;
            map.insert(pos, true);
        }
        pos += dir;
    }
    //
    println!("{}", ans);
}

enum Node {
    Clean,
    Weak,
    Infected,
    Flagged,
}

fn part2() {
    const ONE: Complex<i32> = Complex { re: 1, im: 0 };
    const IM: Complex<i32> = Complex { re: 0, im: 1 };
    //
    let mut map: HashMap<_, _> = load_input("inputfiles/day22/input.txt")
        .into_iter()
        .map(|(k, v)| {
            (
                k,
                match v {
                    false => Node::Clean,
                    true => Node::Infected,
                },
            )
        })
        .collect();
    //
    let mut pos = find_start(&map);
    let mut dir = IM;
    //
    let mut ans = 0;
    //
    for _ in 0..10_000_000 {
        dir *= match map.get(&pos) {
            Some(Node::Clean) | None => IM,
            Some(Node::Weak) => ONE,
            Some(Node::Infected) => -IM,
            Some(Node::Flagged) => -ONE,
        };
        if let Some(x) = map.get_mut(&pos) {
            *x = match x {
                Node::Clean => Node::Weak,
                Node::Weak => {
                    ans += 1;
                    Node::Infected
                }
                Node::Infected => Node::Flagged,
                Node::Flagged => Node::Clean,
            };
        } else {
            map.insert(pos, Node::Weak);
        }
        pos += dir;
    }
    //
    println!("{}", ans);
}
