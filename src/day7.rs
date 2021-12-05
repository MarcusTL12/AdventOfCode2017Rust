use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

use arrayvec::ArrayVec;

pub const PARTS: [fn(); 2] = [part1, part2];

fn findroot(filename: &str) -> String {
    let mut inside = HashSet::new();
    //
    let all: HashSet<_> = BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            match l.split(" -> ").collect::<ArrayVec<_, 2>>().as_slice() {
                [a] => a,
                [a, b] => {
                    inside.extend(b.split(", ").map(|s| s.to_owned()));
                    a
                }
                _ => unreachable!(),
            }
            .split(' ')
            .next()
            .unwrap()
            .to_owned()
        })
        .collect();
    //
    all.difference(&inside).into_iter().next().unwrap().clone()
}

fn part1() {
    println!("{}", findroot("inputfiles/day7/input.txt"));
}

fn findweight(
    connections: &HashMap<String, Vec<String>>,
    weights: &HashMap<String, u32>,
    accum_weights: &mut HashMap<String, u32>,
    node: &String,
) -> u32 {
    if let Some(x) = accum_weights.get(node) {
        *x
    } else {
        let ans = weights[node]
            + if let Some(v) = connections.get(node) {
                v.iter()
                    .map(|s| findweight(connections, weights, accum_weights, s))
                    .sum()
            } else {
                0
            };
        accum_weights.insert(node.clone(), ans);
        ans
    }
}

fn find_odd_one(
    connections: &HashMap<String, Vec<String>>,
    accum_weights: &HashMap<String, u32>,
    node: &String,
) -> Option<String> {
    let most_common = connections
        .get(node)?
        .iter()
        .map(|x| accum_weights[x])
        .fold(HashMap::new(), |mut map, x| {
            if let Some(x) = map.get_mut(&x) {
                *x += 1;
            } else {
                map.insert(x, 1);
            }
            map
        })
        .into_iter()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap()
        .0;
    //
    let odd = if let Some(odd) = connections
        .get(node)?
        .iter()
        .filter(|&x| accum_weights[x] != most_common)
        .next()
    {
        find_odd_one(connections, accum_weights, odd)
    } else {
        Some(node.clone())
    };
    //
    odd
}

fn part2() {
    let mut connections = HashMap::new();
    let filename = "inputfiles/day7/input.txt";
    //
    let weights: HashMap<_, u32> =
        BufReader::new(File::open(filename).unwrap())
            .lines()
            .map(|l| l.unwrap())
            .map(|l| {
                let temp = match l
                    .split(" -> ")
                    .collect::<ArrayVec<_, 2>>()
                    .as_slice()
                {
                    [a] => a,
                    [a, b] => {
                        connections.insert(
                            a.split(' ').next().unwrap().to_owned(),
                            b.split(", ")
                                .map(|s| s.to_owned())
                                .collect::<Vec<_>>(),
                        );
                        a
                    }
                    _ => unreachable!(),
                }
                .split(' ')
                .collect::<ArrayVec<_, 2>>();
                let a = temp[0].to_owned();
                let b = temp[1]
                    .chars()
                    .skip(1)
                    .take_while(|x| x.is_numeric())
                    .collect::<String>()
                    .parse()
                    .unwrap();
                (a, b)
            })
            .collect();
    //
    let root = findroot(filename);
    //
    let mut accum_weights = HashMap::new();
    //
    for (node, _) in weights.iter() {
        findweight(&connections, &weights, &mut accum_weights, node);
    }
    //
    let odd = find_odd_one(&connections, &accum_weights, &root).unwrap();
    //
    let most_common = connections[&root]
        .iter()
        .map(|x| accum_weights[x])
        .fold(HashMap::new(), |mut map, x| {
            if let Some(x) = map.get_mut(&x) {
                *x += 1;
            } else {
                map.insert(x, 1);
            }
            map
        })
        .into_iter()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap()
        .0;
    //
    let odd_weight = connections[&root]
        .iter()
        .map(|x| accum_weights[x])
        .filter(|&x| x != most_common)
        .next()
        .unwrap();
    //
    println!("{}", odd);
    println!(
        "{}",
        weights[&odd] as i32 + (most_common as i32 - odd_weight as i32)
    );
}
