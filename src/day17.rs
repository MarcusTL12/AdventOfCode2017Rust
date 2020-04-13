pub const PARTS: [fn(); 2] = [part1, part2];

fn part1() {
    const STEP_LENGTH: usize = 355;
    //
    let mut v = vec![0];
    let mut i = 0;
    //
    for j in 1..2018 {
        i = (i + STEP_LENGTH) % v.len() + 1;
        v.insert(i, j);
    }
    //
    println!("{}", v[(i + 1) % v.len()]);
}

fn part2() {
    const STEP_LENGTH: usize = 355;
    //
    let mut i = 0;
    let mut l = 1;
    let mut n = 0;
    //
    for j in 1..50_000_001 {
        i = (i + STEP_LENGTH) % l;
        if i == 0 {
            n = j;
        }
        i += 1;
        l += 1;
    }
    //
    println!("{}", n);
}
