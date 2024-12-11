use std::mem::swap;

use hashbrown::HashMap;
use itertools::Itertools;

fn blink(map: &HashMap<usize, usize>, nmap: &mut HashMap<usize, usize>) {
    for (&k, &v) in map.iter() {
        match k {
            0 => *nmap.entry(1).or_default() += v,
            x if (x.ilog10() + 1) % 2 == 0 => {
                let xlen = x.ilog10() + 1;
                let xhalf = 10_usize.pow(xlen / 2);

                *nmap.entry(x / xhalf).or_default() += v;
                *nmap.entry(x % xhalf).or_default() += v;
            }
            x => *nmap.entry(x * 2024).or_default() += v,
        }
    }
}

fn solve(input: &str, blinks: usize) -> usize {
    let v = input
        .split(" ")
        .filter_map(|x| x.parse().ok())
        .collect_vec();

    let mut map = HashMap::<usize, usize>::new();

    for vv in v {
        *map.entry(vv).or_default() += 1;
    }

    let mut nmap = HashMap::<usize, usize>::new();

    for _ in 0..blinks {
        blink(&map, &mut nmap);
        map.clear();
        swap(&mut map, &mut nmap);
    }

    map.values().sum()
}

pub fn part_one(input: &str) -> Option<usize> {
    solve(input, 25).into()
}

pub fn part_two(input: &str) -> Option<usize> {
    solve(input, 75).into()
}

aoc::solution!(11);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(&aoc::template::read_file("examples", 11)),
            Some(55312)
        );
    }
    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(&aoc::template::read_file("examples", 11)),
            Some(65601038650482)
        );
    }
}
