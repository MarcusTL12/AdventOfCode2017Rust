use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    iter,
};

use itertools::Itertools;

pub const PARTS: [fn(); 2] = [part1, part2];

fn circ_ind(i: i32, len: usize) -> usize {
    (i % (len as i32) + if i < 0 { len as i32 } else { 0 }) as usize
}

fn part1() {
    let (ans, _) =
        BufReader::new(File::open("inputfiles/day10/input.txt").unwrap())
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .enumerate()
            .fold(
                ((0..256).collect::<Vec<_>>(), 0),
                |(mut seq, curpos), (skip, n)| {
                    let mut a = curpos;
                    let mut b = curpos + n as i32 - 1;
                    //
                    while b > a {
                        {
                            let a = circ_ind(a, seq.len());
                            let b = circ_ind(b, seq.len());
                            seq.swap(a, b);
                        }
                        a += 1;
                        b -= 1;
                    }
                    //
                    (seq, curpos + n as i32 + skip as i32)
                },
            );
    //
    println!("{}", ans[0] * ans[1]);
}

fn knot_hash<I: Iterator<Item = u8> + Clone>(data: I) -> String {
    const SUFFIX: [u8; 5] = [17, 31, 73, 47, 23];
    //
    iter::repeat(data.clone().chain(SUFFIX.iter().cloned()))
        .take(64)
        .flatten()
        .enumerate()
        .fold(
            ((0..256).map(|x| x as u8).collect::<Vec<_>>(), 0),
            |(mut seq, curpos), (skip, n)| {
                let mut a = curpos;
                let mut b = curpos + n as i32 - 1;
                //
                while b > a {
                    {
                        let a = circ_ind(a, seq.len());
                        let b = circ_ind(b, seq.len());
                        seq.swap(a, b);
                    }
                    a += 1;
                    b -= 1;
                }
                //
                (seq, curpos + n as i32 + skip as i32)
            },
        )
        .0
        .iter()
        .chunks(16)
        .into_iter()
        .map(|c| c.fold(0, |a, &b| a ^ b))
        .fold(Vec::with_capacity(32), |mut target, x| {
            write!(&mut target, "{:02x}", x).unwrap();
            target
        })
        .into_iter()
        .map(|x| x as char)
        .collect()
}

fn part2() {
    let ans = knot_hash(
        BufReader::new(File::open("inputfiles/day10/input.txt").unwrap())
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .chars()
            .map(|c| c as u8),
    );
    //
    println!("{}", ans);
}
