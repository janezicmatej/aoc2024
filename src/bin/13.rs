use regex::Regex;

fn parse_input(input: &str) -> Vec<[(isize, isize); 3]> {
    let r = Regex::new(r".*X(?:=|\+)(\d*), Y(?:=|\+)(\d*)").unwrap();

    let mut parsed = Vec::new();

    for machine in input.split("\n\n") {
        let mut p = [(0, 0); 3];
        for (idx, line) in machine.lines().enumerate() {
            let c = r.captures(line).unwrap();
            let x: isize = c.get(1).unwrap().as_str().parse().unwrap();
            let y: isize = c.get(2).unwrap().as_str().parse().unwrap();

            p[idx] = (x, y);
        }
        parsed.push(p)
    }

    parsed
}

fn solve(m: ((isize, isize), (isize, isize)), v: (isize, isize)) -> Option<isize> {
    let det = m.0 .0 * m.1 .1 - m.0 .1 * m.1 .0;

    if det == 0 {
        return None;
    }

    let x = (v.0 * m.1 .1 - m.0 .1 * v.1) as f64 / det as f64;
    let y = (m.0 .0 * v.1 - v.0 * m.1 .0) as f64 / det as f64;

    if x.trunc() != x || y.trunc() != y {
        return None;
    }

    Some(x as isize * 3 + y as isize)
}

pub fn part_one(input: &str) -> Option<isize> {
    parse_input(input)
        .into_iter()
        .filter_map(|[(a1, b1), (a2, b2), t]| solve(((a1, a2), (b1, b2)), t))
        .sum::<isize>()
        .into()
}

pub fn part_two(input: &str) -> Option<isize> {
    let prize_add = 10_000_000_000_000;

    parse_input(input)
        .into_iter()
        .filter_map(|[(a0, b0), (a1, b1), (t0, t1)]| {
            solve(((a0, a1), (b0, b1)), (t0 + prize_add, t1 + prize_add))
        })
        .sum::<isize>()
        .into()
}

aoc::solution!(13);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(&aoc::template::read_file("examples", 13)),
            Some(480)
        );
    }
    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(&aoc::template::read_file("examples", 13)),
            Some(875318608908)
        );
    }
}
