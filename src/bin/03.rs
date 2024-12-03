use regex::Regex;

fn solve(input: &str, flip: bool) -> usize {
    let r = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    let mut enabled = true;
    let mut res = 0;

    for c in r.captures_iter(input) {
        let full = c.get(0).unwrap().as_str();
        match &full[..=2] {
            "mul" => {
                if enabled {
                    let a: usize = c.get(1).unwrap().as_str().parse().unwrap();
                    let b: usize = c.get(2).unwrap().as_str().parse().unwrap();
                    res += a * b;
                }
            }
            "do(" => enabled = true,
            "don" => enabled = !flip,
            _ => unreachable!(),
        }
    }

    res
}

pub fn part_one(input: &str) -> Option<usize> {
    solve(input, false).into()
}

pub fn part_two(input: &str) -> Option<usize> {
    solve(input, true).into()
}

aoc::solution!(3);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(&aoc::template::read_file("examples", 3)),
            Some(161)
        );
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&aoc::template::read_file("examples", 3)), Some(48));
    }
}
