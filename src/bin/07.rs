use itertools::Itertools;

fn calibrate_equation(
    fns: &[fn(a: usize, b: usize) -> usize],
    target: usize,
    nums: &[usize],
    acc: usize,
) -> bool {
    if nums.is_empty() {
        return target == acc;
    }

    if acc > target {
        return false;
    }

    fns.iter()
        .any(|f| calibrate_equation(fns, target, &nums[1..], f(acc, nums[0])))
}

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|l| {
            l.split([' ', ':'])
                .filter_map(|x| x.parse().ok())
                .collect_vec()
        })
        .collect_vec()
}

fn sum(a: usize, b: usize) -> usize {
    a + b
}

fn prod(a: usize, b: usize) -> usize {
    a * b
}

fn concat(a: usize, b: usize) -> usize {
    a * 10_usize.pow(b.ilog10() + 1) + b
}

pub fn part_one(input: &str) -> Option<usize> {
    parse_input(input)
        .into_iter()
        .filter(|n| calibrate_equation(&[sum, prod], n[0], &n[2..], n[1]))
        .map(|n| n[0])
        .sum::<usize>()
        .into()
}

pub fn part_two(input: &str) -> Option<usize> {
    parse_input(input)
        .into_iter()
        .filter(|n| calibrate_equation(&[sum, prod, concat], n[0], &n[2..], n[1]))
        .map(|n| n[0])
        .sum::<usize>()
        .into()
}

aoc::solution!(7);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(&aoc::template::read_file("examples", 7)),
            Some(3749)
        );
    }
    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(&aoc::template::read_file("examples", 7)),
            Some(11387)
        );
    }
}
