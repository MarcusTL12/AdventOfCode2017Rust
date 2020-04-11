use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub const PARTS: [fn(); 2] = [part1, part2];

fn part1() {
    let ans = BufReader::new(File::open("inputfiles/day9/input.txt").unwrap())
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .fold(
            (0, 0, false, false),
            |(mut depth, mut sum, mut garbage, mut ignore), c| {
                if !garbage {
                    match c {
                        '{' => {
                            depth += 1;
                            sum += depth;
                        }
                        '}' => depth -= 1,
                        '<' => garbage = true,
                        ',' => (),
                        _ => unreachable!("{}", c),
                    }
                } else {
                    if !ignore {
                        match c {
                            '!' => ignore = true,
                            '>' => garbage = false,
                            _ => (),
                        }
                    } else {
                        ignore = false;
                    }
                }
                (depth, sum, garbage, ignore)
            },
        )
        .1;
    //
    println!("{}", ans);
}

fn part2() {
    let ans = BufReader::new(File::open("inputfiles/day9/input.txt").unwrap())
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .fold(
            (0, false, false),
            |(mut sum, mut garbage, mut ignore), c| {
                if !garbage {
                    match c {
                        '{' => (),
                        '}' => (),
                        '<' => garbage = true,
                        ',' => (),
                        _ => unreachable!("{}", c),
                    }
                } else {
                    if !ignore {
                        match c {
                            '!' => ignore = true,
                            '>' => garbage = false,
                            _ => sum += 1,
                        }
                    } else {
                        ignore = false;
                    }
                }
                (sum, garbage, ignore)
            },
        )
        .0;
    //
    println!("{}", ans);
}
