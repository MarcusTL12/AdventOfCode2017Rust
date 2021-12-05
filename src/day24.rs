use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader},
};

use arrayvec::ArrayVec;

pub const PARTS: [fn(); 2] = [part1, part2];

type Int = u32;

fn load_input(filename: &str) -> Vec<[Int; 2]> {
    BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            l.split('/')
                .map(|n| n.parse().unwrap())
                .collect::<ArrayVec<_, 2>>()
                .into_inner()
                .unwrap()
        })
        .collect()
}

fn next_num(prev_num: Int, rest: &[[Int; 2]]) -> Option<Int> {
    if rest.len() == 0 {
        Some(prev_num)
    } else {
        if prev_num == rest[0][0] {
            next_num(rest[0][1], &rest[1..])
        } else if prev_num == rest[0][1] {
            next_num(rest[0][0], &rest[1..])
        } else {
            None
        }
    }
}

fn last_num(bridge: &[[Int; 2]]) -> Option<Int> {
    next_num(0, bridge)
}

fn bridge_strength(bridge: &[[Int; 2]]) -> Int {
    bridge.iter().flat_map(|x| x.iter()).sum()
}

fn solve(pieces: &mut Vec<[Int; 2]>, bridge: &mut Vec<[Int; 2]>) -> Int {
    match (0..pieces.len())
        .filter_map(|i| {
            last_num(bridge)
                .and_then(|x| next_num(x, &[pieces[i]]))
                .and_then(|_| Some(i))
        })
        .collect::<Vec<_>>()
        .into_iter()
        .map(|i| {
            bridge.push(pieces.remove(i));
            //
            let ans = solve(pieces, bridge);
            //
            pieces.insert(i, bridge.pop().unwrap());
            //
            ans
        })
        .max()
    {
        Some(x) => x,
        None => bridge_strength(bridge),
    }
}

fn part1() {
    let mut inp = load_input("inputfiles/day24/input.txt");
    //
    let ans = solve(&mut inp, &mut Vec::new());
    //
    println!("{}", ans);
}

fn solve2(
    pieces: &mut Vec<[Int; 2]>,
    bridge: &mut Vec<[Int; 2]>,
) -> (usize, Int) {
    match (0..pieces.len())
        .filter_map(|i| {
            last_num(bridge)
                .and_then(|x| next_num(x, &[pieces[i]]))
                .and_then(|_| Some(i))
        })
        .collect::<Vec<_>>()
        .into_iter()
        .map(|i| {
            bridge.push(pieces.remove(i));
            //
            let ans = solve2(pieces, bridge);
            //
            pieces.insert(i, bridge.pop().unwrap());
            //
            ans
        })
        .max_by(|(a1, a2), (b1, b2)| {
            let x = a1.cmp(b1);
            match x {
                Ordering::Equal => a2.cmp(b2),
                _ => x,
            }
        }) {
        Some(x) => x,
        None => (bridge.len(), bridge_strength(bridge)),
    }
}

fn part2() {
    let mut inp = load_input("inputfiles/day24/input.txt");
    //
    let ans = solve2(&mut inp, &mut Vec::new()).1;
    //
    println!("{}", ans);
}
