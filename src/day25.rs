use std::{collections::VecDeque, fs};

use lazy_static::*;

use regex::Regex;

use arrayvec::ArrayVec;

pub const PARTS: [fn(); 2] = [part1, part2];

lazy_static! {
    static ref REG1: Regex = Regex::new(r"Begin in state (\w).").unwrap();
    static ref REG2: Regex =
        Regex::new(r"Perform a diagnostic checksum after (\d+) steps.")
            .unwrap();
    static ref REG3: Regex = Regex::new(concat!(
        r"In state \w:",
        r"\s*If the current value is 0:",
        r"\s*- Write the value (\d).",
        r"\s*- Move one slot to the (\w+).",
        r"\s*- Continue with state (\w).",
        r"\s*If the current value is 1:",
        r"\s*- Write the value (\d).",
        r"\s*- Move one slot to the (\w+).",
        r"\s*- Continue with state (\w).",
    ))
    .unwrap();
}

fn load_input(filename: &str) -> (usize, usize, Vec<[(bool, bool, usize); 2]>) {
    let file = fs::read_to_string(filename).unwrap();
    //
    let init_state = REG1
        .captures(&file)
        .and_then(|c| c[1].chars().next())
        .and_then(|c| Some(c as usize - b'A' as usize))
        .unwrap();
    //
    let steps = REG2
        .captures(&file)
        .and_then(|c| match c[1].parse() {
            Ok(x) => Some(x),
            _ => None,
        })
        .unwrap();
    //
    let states = REG3
        .captures_iter(&file)
        .map(|c| {
            [[1, 2, 3], [4, 5, 6]]
                .iter()
                .cloned()
                .map(|[x, y, z]| {
                    (
                        match c[x].chars().next() {
                            Some('1') => true,
                            Some('0') => false,
                            _ => unreachable!(),
                        },
                        match &c[y] {
                            "right" => true,
                            "left" => false,
                            _ => unreachable!(),
                        },
                        c[z].chars()
                            .next()
                            .and_then(|c| Some(c as usize - b'A' as usize))
                            .unwrap(),
                    )
                })
                .collect::<ArrayVec<[_; 2]>>()
                .into_inner()
                .unwrap()
        })
        .collect();
    //
    (init_state, steps, states)
}

fn part1() {
    let (mut state, steps, states) = load_input("inputfiles/day25/input.txt");
    //
    let mut cursor = 0;
    let mut tape = VecDeque::new();
    tape.push_back(false);
    //
    for _ in 0..steps {
        let (nval, rl, nstate) = states[state][tape[cursor] as usize];
        //
        tape[cursor] = nval;
        state = nstate;
        //
        match rl {
            true => {
                cursor += 1;
                if cursor == tape.len() {
                    tape.push_back(false);
                }
            }
            false => {
                if cursor == 0 {
                    tape.push_front(false);
                } else {
                    cursor -= 1;
                }
            }
        }
    }
    //
    let ans = tape.into_iter().filter(|&x| x).count();
    //
    println!("{}", ans);
}

fn part2() {
    println!("Done!");
}
