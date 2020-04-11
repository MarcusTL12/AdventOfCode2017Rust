use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use arrayvec::ArrayVec;

pub const PARTS: [fn(); 2] = [part1, part2];

fn part1() {
    let ans = BufReader::new(File::open("inputfiles/day8/input.txt").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .fold(HashMap::new(), |mut registers, l| {
            match l.split(' ').collect::<ArrayVec<[_; 7]>>().as_slice() {
                [a, inc, n, "if", b, cond, m] => {
                    let b = registers.get(*b).cloned().unwrap_or(0);
                    let m = m.parse::<i32>().unwrap();
                    if match *cond {
                        ">" => b > m,
                        "<" => b < m,
                        ">=" => b >= m,
                        "==" => b == m,
                        "<=" => b <= m,
                        "!=" => b != m,
                        _ => unreachable!(),
                    } {
                        let n = n.parse::<i32>().unwrap();
                        //
                        let sign = match *inc {
                            "inc" => 1,
                            "dec" => -1,
                            _ => unreachable!(),
                        };
                        //
                        *if let Some(x) = registers.get_mut(*a) {
                            x
                        } else {
                            registers.insert((*a).to_owned(), 0);
                            registers.get_mut(*a).unwrap()
                        } += sign * n;
                    }
                }
                _ => unreachable!(),
            }
            registers
        })
        .values()
        .cloned()
        .max()
        .unwrap();
    //
    println!("{}", ans);
}

fn part2() {
    let ans = BufReader::new(File::open("inputfiles/day8/input.txt").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .fold((HashMap::new(), 0), |(mut registers, mut maxval), l| {
            match l.split(' ').collect::<ArrayVec<[_; 7]>>().as_slice() {
                [a, inc, n, "if", b, cond, m] => {
                    let b = registers.get(*b).cloned().unwrap_or(0);
                    let m = m.parse::<i32>().unwrap();
                    if match *cond {
                        ">" => b > m,
                        "<" => b < m,
                        ">=" => b >= m,
                        "==" => b == m,
                        "<=" => b <= m,
                        "!=" => b != m,
                        _ => unreachable!(),
                    } {
                        let n = n.parse::<i32>().unwrap();
                        //
                        let sign = match *inc {
                            "inc" => 1,
                            "dec" => -1,
                            _ => unreachable!(),
                        };
                        //
                        let x = if let Some(x) = registers.get_mut(*a) {
                            x
                        } else {
                            registers.insert((*a).to_owned(), 0);
                            registers.get_mut(*a).unwrap()
                        };
                        //
                        *x += sign * n;
                        //
                        maxval = std::cmp::max(maxval, *x);
                    }
                }
                _ => unreachable!(),
            }
            (registers, maxval)
        })
        .1;
    //
    println!("{}", ans);
}
