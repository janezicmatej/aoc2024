use hashbrown::HashSet;
use itertools::Itertools;

fn parse_input(input: &str) -> (usize, Vec<usize>) {
    let mut input_iter = input.lines();

    let ua = input_iter.next().unwrap();
    let uprogram = input_iter.last().unwrap();

    let a = ua.strip_prefix("Register A: ").unwrap().parse().unwrap();
    let program = uprogram
        .strip_prefix("Program: ")
        .unwrap()
        .split(',')
        .filter_map(|x| x.parse().ok())
        .collect();

    (a, program)
}

fn quick_exec(a: usize) -> usize {
    // hardcoded for my input
    // below operation was produced by reducing below steps
    //
    // B = A % 8
    // B = B ^ 7
    // C = A >> B
    // B = B ^ 7
    // A = A >> 3
    // B = B ^ C
    // out(B % 8)
    // jmp(0)
    //
    // remaining mutation after returning is A >> 3
    ((a % 8) ^ (a >> ((a % 8) ^ 7))) % 8
}

pub fn part_one(input: &str) -> Option<String> {
    let (mut a, _) = parse_input(input);
    let mut v = vec![];

    while a != 0 {
        v.push(quick_exec(a));
        a >>= 3;
    }

    v.iter().join(",").into()
}

pub fn part_two(input: &str) -> Option<usize> {
    let (_, program) = parse_input(input);

    let mut n_candidates = HashSet::new();
    let mut candidates = HashSet::new();
    candidates.insert(0);

    for n in program.iter().copied().rev() {
        for candidate in candidates.iter() {
            n_candidates.extend(
                (0..8)
                    .map(|shift| (candidate << 3) + shift)
                    .filter(|&m| quick_exec(m) == n),
            );
        }
        candidates.clear();
        candidates.extend(n_candidates.drain())
    }

    candidates.iter().min().copied()
}

aoc::solution!(17);
