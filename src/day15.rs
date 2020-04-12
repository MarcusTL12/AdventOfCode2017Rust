pub const PARTS: [fn(); 2] = [part1, part2];

const P: u64 = 2147483647;
const A_MUL: u64 = 16807;
const B_MUL: u64 = 48271;

const A_INIT: u64 = 591;
const B_INIT: u64 = 393;

fn generator(init: u64, mult: u64) -> impl Iterator<Item = u64> {
    (0..).scan(init, move |x, _| {
        *x = (*x * mult) % P;
        Some(*x)
    })
}

fn part1() {
    let a = generator(A_INIT, A_MUL);
    let b = generator(B_INIT, B_MUL);
    //
    let ans = a
        .zip(b)
        .take(40_000_000)
        .filter(|(a, b)| (a & 0xffff) == (b & 0xffff))
        .count();
    //
    println!("{}", ans);
}

fn part2() {
    let a = generator(A_INIT, A_MUL).filter(|x| x % 4 == 0);
    let b = generator(B_INIT, B_MUL).filter(|x| x % 8 == 0);
    //
    let ans = a
        .zip(b)
        .take(5_000_000)
        .filter(|(a, b)| (a & 0xffff) == (b & 0xffff))
        .count();
    //
    println!("{}", ans);
}
