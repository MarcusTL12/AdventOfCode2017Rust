use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

use arrayvec::ArrayVec;

pub const PARTS: [fn(); 2] = [part1, part2];

fn part1() {
    let ans = BufReader::new(File::open("inputfiles/day16/input.txt").unwrap())
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .fold(
            VecDeque::from((b'a'..=b'p').collect::<Vec<_>>()),
            |mut buf, c| {
                match &c[0..1] {
                    "s" => {
                        for _ in 0..c[1..].parse().unwrap() {
                            let temp = buf.pop_back().unwrap();
                            buf.push_front(temp);
                        }
                    }
                    "x" => {
                        if let [a, b] = c[1..]
                            .split('/')
                            .map(|n| n.parse().unwrap())
                            .collect::<ArrayVec<[_; 2]>>()
                            .as_slice()
                        {
                            buf.swap(*a, *b);
                        }
                    }
                    "p" => {
                        if let [a, _, b] = c[1..]
                            .chars()
                            .collect::<ArrayVec<[_; 3]>>()
                            .as_slice()
                        {
                            let a = buf
                                .iter()
                                .enumerate()
                                .find(|(_, &x)| x == *a as u8)
                                .unwrap()
                                .0;
                            let b = buf
                                .iter()
                                .enumerate()
                                .find(|(_, &x)| x == *b as u8)
                                .unwrap()
                                .0;
                            //
                            buf.swap(a, b);
                        }
                    }
                    _ => unreachable!(),
                }
                buf
            },
        )
        .iter()
        .map(|&c| c as char)
        .collect::<String>();
    //
    println!("{}", ans);
}

#[derive(Debug)]
enum Move {
    S(usize),
    X(usize, usize),
    P(u8, u8),
}

fn compile_moves(filename: &str) -> Vec<Move> {
    BufReader::new(File::open(filename).unwrap())
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|c| match &c[0..1] {
            "s" => Move::S(c[1..].parse().unwrap()),
            "x" => {
                if let [a, b] = c[1..]
                    .split('/')
                    .map(|n| n.parse().unwrap())
                    .collect::<ArrayVec<[_; 2]>>()
                    .as_slice()
                {
                    Move::X(*a, *b)
                } else {
                    unreachable!()
                }
            }
            "p" => {
                if let [a, _, b] =
                    c[1..].chars().collect::<ArrayVec<[_; 3]>>().as_slice()
                {
                    Move::P(*a as u8, *b as u8)
                } else {
                    unreachable!()
                }
            }
            _ => unreachable!(),
        })
        .collect()
}

fn part2() {
    let moves = compile_moves("inputfiles/day16/input.txt");
    //
    let it = (0..).scan(
        VecDeque::from((b'a'..=b'p').collect::<Vec<_>>()),
        |buf, _| {
            Some(
                moves
                    .iter()
                    .fold(buf, |buf, m| {
                        match m {
                            Move::S(i) => {
                                for _ in 0..*i {
                                    let temp = buf.pop_back().unwrap();
                                    buf.push_front(temp);
                                }
                            }
                            Move::X(a, b) => buf.swap(*a, *b),
                            Move::P(a, b) => {
                                let a = buf
                                    .iter()
                                    .enumerate()
                                    .find(|(_, &x)| x == *a)
                                    .unwrap()
                                    .0;
                                let b = buf
                                    .iter()
                                    .enumerate()
                                    .find(|(_, &x)| x == *b)
                                    .unwrap()
                                    .0;
                                //
                                buf.swap(a, b);
                            }
                        }
                        buf
                    })
                    .iter()
                    .cloned()
                    .collect::<ArrayVec<[_; 16]>>()
                    .into_inner()
                    .unwrap(),
            )
        },
    );
    //
    let mut seen = HashSet::new();
    let mut memo = vec![b"abcdefghijklmnop".to_owned()];
    seen.insert(b"abcdefghijklmnop".to_owned());
    //
    let mut n = 0;
    //
    for (i, conf) in it.clone().enumerate() {
        if seen.contains(&conf) {
            n = i + 1;
            break;
        } else {
            memo.push(conf);
            seen.insert(conf);
        }
    }
    //
    let ans = memo[1000_000_000 % n]
        .iter()
        .map(|&c| c as char)
        .collect::<String>();
    //
    println!("{}", ans);
}
