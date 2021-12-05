use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use arrayvec::ArrayVec;
use primes::is_prime;

pub const PARTS: [fn(); 2] = [part1, part2];

#[derive(Debug, Clone, Copy)]
enum Arg {
    Val(i64),
    Reg(usize),
}

#[derive(Debug, Clone, Copy)]
enum Ins {
    Set(Arg, Arg),
    Sub(Arg, Arg),
    Mul(Arg, Arg),
    Jnz(Arg, Arg),
}

fn decode_arg(x: &str) -> Arg {
    if let Ok(n) = x.parse() {
        Arg::Val(n)
    } else if let Some(x) = x.chars().next() {
        Arg::Reg(x as usize - b'a' as usize)
    } else {
        unreachable!()
    }
}

fn compile_code(filename: &str) -> Vec<Ins> {
    BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|l| l.unwrap())
        .map(
            |l| match l.split(' ').collect::<ArrayVec<_, 3>>().as_slice() {
                ["set", x, y] => Ins::Set(decode_arg(x), decode_arg(y)),
                ["sub", x, y] => Ins::Sub(decode_arg(x), decode_arg(y)),
                ["mul", x, y] => Ins::Mul(decode_arg(x), decode_arg(y)),
                ["jnz", x, y] => Ins::Jnz(decode_arg(x), decode_arg(y)),
                _ => unreachable!(),
            },
        )
        .collect()
}

fn getval(registers: &[i64], i: Arg) -> i64 {
    match i {
        Arg::Val(x) => x,
        Arg::Reg(x) => registers[x],
    }
}

fn part1() {
    let inp = compile_code("inputfiles/day23/input.txt");
    //
    let mut registers = [0; 8];
    //
    let mut i = 0;
    //
    let mut ans = 0;
    //
    while i >= 0 && i < inp.len() as i64 {
        match inp[i as usize] {
            Ins::Set(Arg::Reg(x), y) => registers[x] = getval(&registers, y),
            Ins::Sub(Arg::Reg(x), y) => registers[x] -= getval(&registers, y),
            Ins::Mul(Arg::Reg(x), y) => {
                ans += 1;
                registers[x] *= getval(&registers, y)
            }
            Ins::Jnz(x, y) => {
                let x = getval(&registers, x);
                let y = getval(&registers, y);
                //
                if x != 0 {
                    i += y - 1;
                }
            }
            _ => unreachable!(),
        }
        i += 1;
    }
    //
    println!("{}", ans);
}

fn part2() {
    let ans = (106700..=123700).step_by(17).filter(|&x| !is_prime(x)).count();

    println!("{}", ans);
}

fn _done_by_hand() {
    let mut b = 106700;
    let c = 123700;
    let mut d;
    let mut e;
    let mut f;
    let mut h = 0;
    //
    loop {
        f = 1;
        d = 2;
        //
        loop {
            e = 2;
            loop {
                if d * e == b {
                    f = 0;
                }
                //
                e += 1;
                //
                if e == b {
                    break;
                }
            }
            //
            d += 1;
            //
            if d == b {
                break;
            }
        }
        //
        if f == 0 {
            h += 1;
        }
        //
        if b == c {
            break;
        }
        //
        b += 17;
    }
    //
    println!("{}", h);
}
