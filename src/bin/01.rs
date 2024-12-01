use hashbrown::HashMap;
use itertools::Itertools;

fn parse_input(input: &str) -> impl Iterator<Item = (isize, isize)> + '_ {
    input.lines().map(|l| {
        l.split(' ')
            .filter_map(|x| x.parse::<isize>().ok())
            .collect_tuple()
            .unwrap()
    })
}

pub fn part_one(input: &str) -> Option<isize> {
    let mut left = vec![];
    let mut right = vec![];

    for (a, b) in parse_input(input) {
        left.push(a);
        right.push(b);
    }

    left.sort();
    right.sort();

    Some(
        left.into_iter()
            .zip(right)
            .map(|(x, y)| (x - y).abs())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<isize> {
    let mut map = HashMap::new();

    for (a, b) in parse_input(input) {
        map.entry(a).or_insert((0, 0)).0 += 1;
        map.entry(b).or_insert((0, 0)).1 += 1;
    }

    Some(map.into_iter().map(|(k, v)| k * v.0 * v.1).sum())
}

aoc::solution!(1);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&aoc::template::read_file("examples", 1)), Some(11));
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&aoc::template::read_file("examples", 1)), Some(31));
    }
}
