use itertools::Itertools;

fn parse_input(input: &str) -> impl Iterator<Item = Vec<isize>> + '_ {
    input.lines().map(|l| {
        l.split(' ')
            .filter_map(|x| x.parse::<isize>().ok())
            .collect_vec()
    })
}

fn check_report(mut r: impl Iterator<Item = isize>) -> bool {
    let mut low = 0;
    let mut high = 0;

    let mut prev = r.next().expect("Invalid input file?");

    for n in r {
        let diff = n - prev;
        prev = n;

        if diff == 0 {
            return false;
        };

        if !(-3..=3).contains(&diff) {
            return false;
        }

        if diff > 0 {
            high += 1;
        }

        if diff < 0 {
            low += 1;
        }
    }

    !(low > 0 && high > 0)
}

pub fn part_one(input: &str) -> Option<usize> {
    parse_input(input)
        .map(|r| check_report(r.into_iter()))
        .filter(|&x| x)
        .count()
        .into()
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        parse_input(input)
            .map(|r| {
                (0..r.len()).any(|i| {
                    check_report(
                        // skip nth idx
                        r.iter()
                            .enumerate()
                            .filter(|&(j, _)| j != i)
                            .map(|(_, &v)| v),
                    )
                })
            })
            .filter(|x| *x)
            .count(),
    )
}

aoc::solution!(2);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&aoc::template::read_file("examples", 2)), Some(2));
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&aoc::template::read_file("examples", 2)), Some(4));
    }
}
