use hashbrown::HashMap;
use num_integer::Integer;
use regex::Regex;

fn parse_input(input: &str) -> Vec<((isize, isize), (isize, isize))> {
    let r = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

    let mut parsed = Vec::new();

    for line in input.lines() {
        let c = r.captures(line).unwrap();
        let j: isize = c.get(1).unwrap().as_str().parse().unwrap();
        let i: isize = c.get(2).unwrap().as_str().parse().unwrap();
        let y: isize = c.get(3).unwrap().as_str().parse().unwrap();
        let x: isize = c.get(4).unwrap().as_str().parse().unwrap();

        parsed.push(((i, j), (x, y)));
    }

    parsed
}

pub fn part_one(input: &str) -> Option<usize> {
    let parsed = parse_input(input);
    let mut r = [0; 4];

    let h = 103;
    let w = 101;

    for ((pi, pj), (vi, vj)) in parsed {
        let ni = (pi + vi * 100).rem_euclid(h);
        let nj = (pj + vj * 100).rem_euclid(w);

        if ni < h / 2 && nj > w / 2 {
            r[0] += 1;
        }

        if ni < h / 2 && nj < w / 2 {
            r[1] += 1;
        }

        if ni > h / 2 && nj < w / 2 {
            r[2] += 1;
        }

        if ni > h / 2 && nj > w / 2 {
            r[3] += 1;
        }
    }

    r.into_iter().product::<usize>().into()
}

pub fn part_two(input: &str) -> Option<usize> {
    let pv = parse_input(input);

    let h = 103;
    let w = 101;

    let mut n = 1;

    // hardcoded side lengths of the "easter egg"
    let th = 33;
    let tw = 31;

    for ((pi, pj), (vi, vj)) in pv.iter().copied() {
        let mut m = 1;

        loop {
            let ni = (pi + vi * m).rem_euclid(h);
            let nj = (pj + vj * m).rem_euclid(w);
            if (ni, nj) == (pi, pj) {
                break;
            }
            m += 1;
        }

        n = n.lcm(&m);
    }

    let mut sh = HashMap::new();
    let mut sw = HashMap::new();

    for m in 0..n {
        sh.clear();
        sw.clear();

        for ((pi, pj), (vi, vj)) in pv.iter().copied() {
            let ni = (pi + vi * m).rem_euclid(h);
            let nj = (pj + vj * m).rem_euclid(w);

            *sw.entry(ni).or_default() += 1_usize;
            *sh.entry(nj).or_default() += 1_usize;
        }

        let ch = sh.values().filter(|&&x: &&usize| x >= th).count();
        let cw = sw.values().filter(|&&x: &&usize| x >= tw).count();

        if ch >= 2 && cw >= 2 {
            return Some(n as usize);
        }
    }

    None
}

aoc::solution!(14);
