use std::{collections::HashSet, io::Write, iter};

use itertools::Itertools;

use bitvec::prelude::*;

use num::Complex;

pub const PARTS: [fn(); 2] = [part1, part2];

fn circ_ind(i: i32, len: usize) -> usize {
    (i % (len as i32) + if i < 0 { len as i32 } else { 0 }) as usize
}

fn knot_hash<I: Iterator<Item = u8> + Clone>(data: I, target: &mut Vec<u8>) {
    const SUFFIX: [u8; 5] = [17, 31, 73, 47, 23];
    //
    target.clear();
    //
    target.extend(
        iter::repeat(data.chain(SUFFIX.iter().cloned()))
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
            .map(|c| c.fold(0, |a, &b| a ^ b)),
    );
}

fn part1() {
    const INP: &[u8] = b"uugsqrei-";
    //
    let (_, _, ans) = (0..128).fold(
        (Vec::from(INP), Vec::new(), 0),
        |(mut buf, mut buf2, mut s), i| {
            buf.truncate(INP.len());
            write!(&mut buf, "{}", i).unwrap();
            //
            buf2.clear();
            knot_hash(buf.iter().cloned(), &mut buf2);
            //
            let v = BitVec::<Msb0, u8>::from_vec(buf2);
            s += v.iter().filter(|x| **x).count();
            //
            (buf, v.into_vec(), s)
        },
    );
    //
    println!("{}", ans);
}

fn dfs(
    grid: &Vec<BitVec<Msb0, u8>>,
    visited: &mut HashSet<Complex<i32>>,
    pos: Complex<i32>,
) {
    const DIRS: [Complex<i32>; 4] = [
        Complex { re: 1, im: 0 },
        Complex { re: -1, im: 0 },
        Complex { re: 0, im: 1 },
        Complex { re: 0, im: -1 },
    ];
    //
    for npos in DIRS.iter().map(|d| pos + d).filter(|p| {
        p.re >= 0
            && p.re < grid[0].len() as i32
            && p.im >= 0
            && p.im < grid.len() as i32
    }) {
        if grid[npos.im as usize][npos.re as usize] && !visited.contains(&npos)
        {
            visited.insert(npos);
            dfs(grid, visited, npos);
        }
    }
}

fn part2() {
    const INP: &[u8] = b"uugsqrei-";
    //
    let (_, _, grid) = (0..128).fold(
        (
            Vec::from(INP),
            Vec::with_capacity(32),
            Vec::with_capacity(32),
        ),
        |(mut buf, mut buf2, mut grid), i| {
            buf.truncate(INP.len());
            write!(&mut buf, "{}", i).unwrap();
            //
            buf2.clear();
            knot_hash(buf.iter().cloned(), &mut buf2);
            //
            let v = BitVec::<Msb0, u8>::from_vec(buf2);
            //
            grid.push(v.clone());
            //
            (buf, v.into_vec(), grid)
        },
    );
    //
    let mut visited = HashSet::new();
    //
    let mut regions = 0;
    //
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let pos = Complex {
                re: j as i32,
                im: i as i32,
            };
            if grid[i][j] && !visited.contains(&pos) {
                regions += 1;
                dfs(&grid, &mut visited, pos);
            }
        }
    }
    //
    println!("{}", regions);
}
