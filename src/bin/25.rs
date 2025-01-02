pub fn part_one(input: &str) -> Option<usize> {
    let mut keys = Vec::new();
    let mut locks = Vec::new();

    for key_or_lock in input.split("\n\n") {
        let lock = key_or_lock
            .lines()
            .next()
            .unwrap()
            .chars()
            .all(|x| x == '#');

        let mut columns = [0; 5];
        for (idx, c) in key_or_lock.lines().flat_map(|x| x.chars().enumerate()) {
            if c == '#' {
                columns[idx] += 1;
            }
        }

        // remove top or bottom row from count
        for c in columns.iter_mut() {
            *c -= 1;
        }

        if lock {
            locks.push(columns);
        } else {
            keys.push(columns);
        }
    }

    let mut count = 0;
    for k in keys.iter() {
        for l in locks.iter() {
            if k.iter().zip(l.iter()).all(|(k, l)| k + l <= 5) {
                count += 1;
            }
        }
    }

    count.into()
}

pub fn part_two(_input: &str) -> Option<String> {
    "Happy christmas!".to_string().into()
}

aoc::solution!(25);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&aoc::template::read_file("examples", 25)), Some(3));
    }
}
