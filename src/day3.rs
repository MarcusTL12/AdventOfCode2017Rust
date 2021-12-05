use std::{collections::HashMap, iter};

use num::Complex;

pub const PARTS: [fn(); 2] = [part1, part2];

fn part1() {
    const INPUT: usize = 289326;
    //
    let ans = (0..)
        .scan(
            (
                Complex { re: 0, im: 0 },
                Complex { re: 1, im: 0 },
                1,
                1,
                false,
            ),
            |(pos, dir, curlen, remaining, parity), _| {
                let thispos = *pos;
                *pos += *dir;
                *remaining -= 1;
                if *remaining == 0 {
                    if *parity {
                        *curlen += 1;
                        *remaining = *curlen;
                        *parity = false;
                    } else {
                        *remaining = *curlen;
                        *parity = true;
                    }
                    *dir *= Complex { re: 0, im: 1 };
                }
                Some(thispos)
            },
        )
        .skip(INPUT - 1)
        .next()
        .and_then(|x: Complex<i32>| Some(x.re.abs() + x.im.abs()))
        .unwrap();
    //
    println!("{}", ans);
}

fn part2() {
    const INPUT: u32 = 289326;
    const DIRS: [Complex<i32>; 8] = [
        Complex { re: 1, im: 0 },
        Complex { re: 1, im: 1 },
        Complex { re: 0, im: 1 },
        Complex { re: -1, im: 1 },
        Complex { re: -1, im: 0 },
        Complex { re: -1, im: -1 },
        Complex { re: 0, im: -1 },
        Complex { re: 1, im: -1 },
    ];
    //
    let ans = (0..)
        .scan(
            (
                Complex { re: 0, im: 0 },
                Complex { re: 1, im: 0 },
                1,
                1,
                false,
            ),
            |(pos, dir, curlen, remaining, parity), _| {
                let thispos = *pos;
                *pos += *dir;
                *remaining -= 1;
                if *remaining == 0 {
                    if *parity {
                        *curlen += 1;
                        *remaining = *curlen;
                        *parity = false;
                    } else {
                        *remaining = *curlen;
                        *parity = true;
                    }
                    *dir *= Complex { re: 0, im: 1 };
                }
                Some(thispos)
            },
        )
        .skip(1)
        .scan(
            iter::once((Complex { re: 0, im: 0 }, 1))
                .collect::<HashMap<_, _>>(),
            |buffer, pos| {
                let val: u32 =
                    DIRS.iter().filter_map(|d| buffer.get(&(d + pos))).sum();
                //
                buffer.insert(pos, val);
                //
                Some(val)
            },
        )
        .skip_while(|&x| x < INPUT)
        .next()
        .unwrap();
    //
    println!("{}", ans);
}
