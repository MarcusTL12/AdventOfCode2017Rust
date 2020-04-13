use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use num::Complex;

use arrayvec::ArrayVec;

pub const PARTS: [fn(); 2] = [part1, part2];

#[derive(Debug)]
enum Cell {
    Empty,
    Vertical,
    Horizontal,
    Cross,
    Letter(char),
}

fn load_input(filename: &str) -> Vec<Vec<Cell>> {
    BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    ' ' => Cell::Empty,
                    '|' => Cell::Vertical,
                    '-' => Cell::Horizontal,
                    '+' => Cell::Cross,
                    _ => Cell::Letter(c),
                })
                .collect()
        })
        .collect()
}

fn _render_map(map: &Vec<Vec<Cell>>) {
    for row in map {
        for cell in row {
            print!(
                "{}",
                match cell {
                    Cell::Empty => '.',
                    Cell::Vertical => '|',
                    Cell::Horizontal => '-',
                    Cell::Cross => '+',
                    Cell::Letter(c) => *c,
                }
            );
        }
        println!();
    }
}

fn get_cell(map: &Vec<Vec<Cell>>, pos: Complex<i32>) -> &Cell {
    if let Some(row) = map.get(pos.im as usize) {
        if let Some(cell) = row.get(pos.re as usize) {
            cell
        } else {
            &Cell::Empty
        }
    } else {
        &Cell::Empty
    }
}

fn part1() {
    const ONE: Complex<i32> = Complex { re: 1, im: 0 };
    const IM: Complex<i32> = Complex { re: 0, im: 1 };
    //
    let map = load_input("inputfiles/day19/input.txt");
    //
    let entrance = map[0]
        .iter()
        .enumerate()
        .find(|(_, c)| matches!(c, Cell::Vertical))
        .unwrap()
        .0;
    //
    let mut pos = Complex {
        re: entrance as i32,
        im: 0,
    };
    let mut dir = Complex { re: 0, im: 1 };
    //
    let mut ans = String::new();
    //
    loop {
        pos += dir;
        match get_cell(&map, pos) {
            Cell::Empty => break,
            Cell::Vertical | Cell::Horizontal => (),
            Cell::Letter(c) => ans.push(*c),
            Cell::Cross => {
                dir *= match [dir * IM, dir * (-IM)]
                    .iter()
                    .map(|&d| get_cell(&map, d + pos))
                    .collect::<ArrayVec<[_; 2]>>()
                    .as_slice()
                {
                    [Cell::Horizontal, _] if dir == IM || dir == -IM => IM,
                    [Cell::Vertical, _] if dir == ONE || dir == -ONE => IM,
                    [Cell::Letter(_), _] => IM,
                    [_, Cell::Horizontal] if dir == IM || dir == -IM => -IM,
                    [_, Cell::Vertical] if dir == ONE || dir == -ONE => -IM,
                    [_, Cell::Letter(_)] => -IM,
                    a => unreachable!("{:?}", a),
                };
            }
        }
    }
    //
    println!("{}", ans);
}

fn part2() {
    const ONE: Complex<i32> = Complex { re: 1, im: 0 };
    const IM: Complex<i32> = Complex { re: 0, im: 1 };
    //
    let map = load_input("inputfiles/day19/input.txt");
    //
    let entrance = map[0]
        .iter()
        .enumerate()
        .find(|(_, c)| matches!(c, Cell::Vertical))
        .unwrap()
        .0;
    //
    let mut pos = Complex {
        re: entrance as i32,
        im: 0,
    };
    let mut dir = Complex { re: 0, im: 1 };
    //
    let mut ans = 0;
    //
    loop {
        pos += dir;
        ans += 1;
        match get_cell(&map, pos) {
            Cell::Empty => break,
            Cell::Vertical | Cell::Horizontal | Cell::Letter(_) => (),
            Cell::Cross => {
                dir *= match [dir * IM, dir * (-IM)]
                    .iter()
                    .map(|&d| get_cell(&map, d + pos))
                    .collect::<ArrayVec<[_; 2]>>()
                    .as_slice()
                {
                    [Cell::Horizontal, _] if dir == IM || dir == -IM => IM,
                    [Cell::Vertical, _] if dir == ONE || dir == -ONE => IM,
                    [Cell::Letter(_), _] => IM,
                    [_, Cell::Horizontal] if dir == IM || dir == -IM => -IM,
                    [_, Cell::Vertical] if dir == ONE || dir == -ONE => -IM,
                    [_, Cell::Letter(_)] => -IM,
                    a => unreachable!("{:?}", a),
                };
            }
        }
    }
    //
    println!("{}", ans);
}
