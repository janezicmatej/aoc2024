use hashbrown::{HashMap, HashSet};

fn parse_input(input: &str) -> (HashSet<&str>, HashSet<&str>) {
    let (upatterns, udesigns) = input.split_once("\n\n").unwrap();

    let patterns = upatterns.split(", ").collect();
    let designs = udesigns.split('\n').collect();

    (patterns, designs)
}

fn count_pattern_designs<'a>(
    memo: &mut HashMap<&'a str, usize>,
    patterns: &HashSet<&str>,
    pattern: &'a str,
) -> usize {
    if pattern.is_empty() {
        return 1;
    }

    if !memo.contains_key(pattern) {
        let mut count = 0;
        for p in patterns.iter() {
            if let Some(stripped) = pattern.strip_prefix(p) {
                count += count_pattern_designs(memo, patterns, stripped)
            }
        }
        memo.insert(pattern, count);
    }

    memo[pattern]
}

pub fn part_one(input: &str) -> Option<usize> {
    let (patterns, designs) = parse_input(input);

    let mut memo = HashMap::new();

    for d in designs.iter().copied() {
        count_pattern_designs(&mut memo, &patterns, d);
    }

    designs.into_iter().filter(|x| memo[x] > 0).count().into()
}

pub fn part_two(input: &str) -> Option<usize> {
    let (patterns, designs) = parse_input(input);

    let mut memo = HashMap::new();

    for d in designs.iter().copied() {
        count_pattern_designs(&mut memo, &patterns, d);
    }

    designs.into_iter().map(|x| memo[x]).sum::<usize>().into()
}

aoc::solution!(19);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&aoc::template::read_file("examples", 19)), Some(6));
    }
    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(&aoc::template::read_file("examples", 19)),
            Some(16)
        );
    }
}
