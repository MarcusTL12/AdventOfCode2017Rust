use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

use arrayvec::ArrayVec;

pub const PARTS: [fn(); 2] = [part1, part2];

#[derive(Debug, Clone, Copy)]
enum Arg {
    Val(i64),
    Reg(usize),
}

#[derive(Debug, Clone, Copy)]
enum Ins {
    Snd(Arg),
    Set(Arg, Arg),
    Add(Arg, Arg),
    Mul(Arg, Arg),
    Mod(Arg, Arg),
    Rcv(Arg),
    Jgz(Arg, Arg),
}

fn decode_arg(registers: &mut HashMap<char, usize>, x: &str) -> Arg {
    if let Ok(n) = x.parse() {
        Arg::Val(n)
    } else if let Some(x) = x.chars().next().and_then(|c| registers.get(&c)) {
        Arg::Reg(*x)
    } else if let Some(c) = x.chars().next() {
        let n = registers.len();
        registers.insert(c, n);
        Arg::Reg(n)
    } else {
        unreachable!()
    }
}

fn compile_code(filename: &str) -> (Vec<Ins>, usize, usize) {
    let mut registers = HashMap::new();
    //
    (
        BufReader::new(File::open(filename).unwrap())
            .lines()
            .map(|l| l.unwrap())
            .map(|l| {
                match l.split(' ').collect::<ArrayVec<[_; 3]>>().as_slice() {
                    ["snd", x] => Ins::Snd(decode_arg(&mut registers, x)),
                    ["set", x, y] => Ins::Set(
                        decode_arg(&mut registers, x),
                        decode_arg(&mut registers, y),
                    ),
                    ["add", x, y] => Ins::Add(
                        decode_arg(&mut registers, x),
                        decode_arg(&mut registers, y),
                    ),
                    ["mul", x, y] => Ins::Mul(
                        decode_arg(&mut registers, x),
                        decode_arg(&mut registers, y),
                    ),
                    ["mod", x, y] => Ins::Mod(
                        decode_arg(&mut registers, x),
                        decode_arg(&mut registers, y),
                    ),
                    ["rcv", x] => Ins::Rcv(decode_arg(&mut registers, x)),
                    ["jgz", x, y] => Ins::Jgz(
                        decode_arg(&mut registers, x),
                        decode_arg(&mut registers, y),
                    ),
                    _ => unreachable!(),
                }
            })
            .collect(),
        registers.len(),
        registers[&'p'],
    )
}

fn get_val(registers: &[i64], arg: Arg) -> i64 {
    match arg {
        Arg::Val(x) => x,
        Arg::Reg(i) => registers[i],
    }
}

fn run_program1(program: &[Ins], registers: &mut [i64]) -> i64 {
    let mut i = 0;
    let mut last_freq = 0;
    //
    while i >= 0 && i < program.len() as i64 {
        match program[i as usize] {
            Ins::Snd(x) => last_freq = get_val(registers, x),
            Ins::Rcv(x) => {
                if get_val(registers, x) != 0 {
                    break;
                }
            }
            Ins::Set(Arg::Reg(x), y) => registers[x] = get_val(registers, y),
            Ins::Add(Arg::Reg(x), y) => registers[x] += get_val(registers, y),
            Ins::Mul(Arg::Reg(x), y) => registers[x] *= get_val(registers, y),
            Ins::Mod(Arg::Reg(x), y) => registers[x] %= get_val(registers, y),
            Ins::Jgz(x, y) => {
                if get_val(registers, x) > 0 {
                    i += get_val(registers, y) - 1;
                }
            }
            _ => unreachable!(),
        }
        i += 1;
    }
    //
    last_freq
}

fn part1() {
    let (program, amt_reg, _) = compile_code("inputfiles/day18/input.txt");
    //
    let ans = run_program1(&program, &mut vec![0; amt_reg]);
    //
    println!("{}", ans);
}

fn run_program2(
    program: &[Ins],
    i: &mut i64,
    registers: &mut [i64],
    in_strm: &mut VecDeque<i64>,
    out_strm: &mut VecDeque<i64>,
) -> bool {
    while *i >= 0 && *i < program.len() as i64 {
        match program[*i as usize] {
            Ins::Snd(x) => out_strm.push_back(get_val(registers, x)),
            Ins::Rcv(Arg::Reg(x)) => {
                if let Some(y) = in_strm.pop_front() {
                    registers[x] = y;
                } else {
                    break;
                }
            }
            Ins::Set(Arg::Reg(x), y) => registers[x] = get_val(registers, y),
            Ins::Add(Arg::Reg(x), y) => registers[x] += get_val(registers, y),
            Ins::Mul(Arg::Reg(x), y) => registers[x] *= get_val(registers, y),
            Ins::Mod(Arg::Reg(x), y) => registers[x] %= get_val(registers, y),
            Ins::Jgz(x, y) => {
                if get_val(registers, x) > 0 {
                    *i += get_val(registers, y) - 1;
                }
            }
            _ => unreachable!(),
        }
        *i += 1;
    }
    out_strm.len() > 0
}

fn part2() {
    let (program, amt_reg, p) = compile_code("inputfiles/day18/input.txt");
    //
    let mut a_reg = vec![0; amt_reg];
    a_reg[p] = 1;
    let mut a_i = 0;
    let mut a_queue = VecDeque::new();
    //
    let mut b_reg = vec![0; amt_reg];
    let mut b_i = 0;
    let mut b_queue = VecDeque::new();
    //
    let mut ans = 0;
    //
    run_program2(&program, &mut b_i, &mut b_reg, &mut b_queue, &mut a_queue);
    //
    while run_program2(
        &program,
        &mut a_i,
        &mut a_reg,
        &mut a_queue,
        &mut b_queue,
    ) {
        ans += b_queue.len();
        run_program2(
            &program,
            &mut b_i,
            &mut b_reg,
            &mut b_queue,
            &mut a_queue,
        );
    }
    //
    println!("{}", ans);
}
