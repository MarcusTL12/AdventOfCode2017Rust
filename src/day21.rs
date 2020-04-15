use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use arrayvec::ArrayVec;

use lazy_static::*;

pub const PARTS: [fn(); 2] = [part1, part2];

fn decode(l: &str) -> ArrayVec<[ArrayVec<[bool; 4]>; 4]> {
    l.split('/')
        .map(|r| {
            r.chars()
                .map(|c| match c {
                    '.' => false,
                    '#' => true,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

fn _render_square(sq: &ArrayVec<[ArrayVec<[bool; 4]>; 4]>) {
    for r in sq {
        for &c in r {
            print!("{}", if c { '#' } else { '.' });
        }
        println!();
    }
}

fn _render_square_vec(sq: &Vec<Vec<bool>>) {
    for r in sq {
        for &c in r {
            print!("{}", if c { '#' } else { '.' });
        }
        println!();
    }
}

fn symmetries(
    sq: &ArrayVec<[ArrayVec<[bool; 4]>; 4]>,
) -> [ArrayVec<[ArrayVec<[bool; 4]>; 4]>; 8] {
    let l = sq.len();
    [
        sq.clone(),
        (0..l)
            .map(|i| (0..l).map(|j| sq[l - j - 1][i]).collect())
            .collect(),
        (0..l)
            .map(|i| (0..l).map(|j| sq[l - i - 1][l - j - 1]).collect())
            .collect(),
        (0..l)
            .map(|i| (0..l).map(|j| sq[j][l - i - 1]).collect())
            .collect(),
        (0..l)
            .map(|i| (0..l).map(|j| sq[i][l - j - 1]).collect())
            .collect(),
        (0..l)
            .map(|i| (0..l).map(|j| sq[l - i - 1][j]).collect())
            .collect(),
        (0..l).map(|i| (0..l).map(|j| sq[j][i]).collect()).collect(),
        (0..l)
            .map(|i| (0..l).map(|j| sq[l - j - 1][l - i - 1]).collect())
            .collect(),
    ]
}

fn load_input(
    filename: &str,
) -> HashMap<
    ArrayVec<[ArrayVec<[bool; 4]>; 4]>,
    ArrayVec<[ArrayVec<[bool; 4]>; 4]>,
> {
    BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|l| l.unwrap())
        .flat_map(|l| {
            if let [a, b] =
                l.split(" => ").collect::<ArrayVec<[_; 2]>>().as_slice()
            {
                let a = decode(a);
                //
                let b = decode(b);
                //
                let s = symmetries(&a);
                //
                (0..8).scan(s, move |s, i| Some((s[i].clone(), b.clone())))
            } else {
                unreachable!()
            }
        })
        .collect()
}

fn make_next(
    rules: &HashMap<
        ArrayVec<[ArrayVec<[bool; 4]>; 4]>,
        ArrayVec<[ArrayVec<[bool; 4]>; 4]>,
    >,
    sq: &Vec<Vec<bool>>,
) -> Vec<Vec<bool>> {
    let l = sq.len();
    //
    if l % 2 == 0 {
        let mut n_sq = vec![vec![false; (l * 3) / 2]; (l * 3) / 2];
        //
        for i in 0..l / 2 {
            for j in 0..l / 2 {
                let sub_square: ArrayVec<[ArrayVec<[_; 4]>; 4]> = (0..2)
                    .map(|y| (0..2).map(|x| sq[2 * i + y][2 * j + x]).collect())
                    .collect();
                //
                if let Some(n_sub_square) = rules.get(&sub_square) {
                    for y in 0..3 {
                        for x in 0..3 {
                            n_sq[3 * i + y][3 * j + x] = n_sub_square[y][x];
                        }
                    }
                } else {
                    println!("Did not find:");
                    _render_square(&sub_square);
                    println!("in rules!");
                    unreachable!()
                }
            }
        }
        //
        n_sq
    } else if l % 3 == 0 {
        let mut n_sq = vec![vec![false; (l * 4) / 3]; (l * 4) / 3];
        //
        for i in 0..l / 3 {
            for j in 0..l / 3 {
                let sub_square: ArrayVec<[ArrayVec<[_; 4]>; 4]> = (0..3)
                    .map(|y| (0..3).map(|x| sq[3 * i + y][3 * j + x]).collect())
                    .collect();
                //
                if let Some(n_sub_square) = rules.get(&sub_square) {
                    for y in 0..4 {
                        for x in 0..4 {
                            n_sq[4 * i + y][4 * j + x] = n_sub_square[y][x];
                        }
                    }
                } else {
                    println!("Did not find:");
                    _render_square(&sub_square);
                    println!("in rules!");
                    unreachable!()
                }
            }
        }
        //
        n_sq
    } else {
        unreachable!()
    }
}

lazy_static! {
    static ref INIT_SQUARE: Vec<Vec<bool>> = vec![
        vec![false, true, false],
        vec![false, false, true],
        vec![true; 3]
    ];
}

fn part1() {
    let inp = load_input("inputfiles/day21/input.txt");
    //
    let mut square = INIT_SQUARE.clone();
    //
    for _ in 0..5 {
        square = make_next(&inp, &square);
    }
    //
    let ans = square
        .into_iter()
        .flat_map(|row| row.into_iter())
        .filter(|&x| x)
        .count();
    //
    println!("{}", ans);
}

fn part2() {
    let inp = load_input("inputfiles/day21/input.txt");
    //
    let mut square = INIT_SQUARE.clone();
    //
    for _ in 0..18 {
        square = make_next(&inp, &square);
    }
    //
    let ans = square
        .into_iter()
        .flat_map(|row| row.into_iter())
        .filter(|&x| x)
        .count();
    //
    println!("{}", ans);
}
