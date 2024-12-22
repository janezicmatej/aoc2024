use aoc::parsers::to_vec_map;
use hashbrown::{HashMap, HashSet};

fn pseudo_next(mut n: usize) -> usize {
    n = ((n << 6) ^ n) % 16777216;
    n = ((n >> 5) ^ n) % 16777216;
    n = ((n << 11) ^ n) % 16777216;
    n
}

struct MonkeyTrader {
    numbers: [usize; 2001],
    sequences: HashMap<[isize; 4], usize>,
}

impl MonkeyTrader {
    fn new(seed: usize) -> Self {
        let mut numbers = [0; 2001];
        numbers[0] = seed;

        let mut n = seed;
        for num in numbers.iter_mut().skip(1) {
            n = pseudo_next(n);
            *num = n;
        }

        let mut sequences = HashMap::new();
        for i in 4..2001 {
            let mut key = [0; 4];
            for j in 1..=4 {
                let d1 = numbers[i - 4 + j - 1] % 10;
                let d2 = numbers[i - 4 + j] % 10;
                let diff = d2 as isize - d1 as isize;
                key[j - 1] = diff;
            }
            if sequences.contains_key(&key) {
                continue;
            }
            *sequences.entry(key).or_default() = numbers[i] % 10;
        }

        MonkeyTrader { numbers, sequences }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    to_vec_map(input, '\n', MonkeyTrader::new)
        .into_iter()
        .map(|x| x.numbers[2000])
        .sum::<usize>()
        .into()
}

pub fn part_two(input: &str) -> Option<usize> {
    let traders = to_vec_map(input, '\n', MonkeyTrader::new);
    let key_union: HashSet<[isize; 4]> = traders
        .iter()
        .flat_map(|x| x.sequences.keys().cloned())
        .collect();

    key_union
        .into_iter()
        .map(|k| {
            traders
                .iter()
                .filter_map(|t| t.sequences.get(&k))
                .sum::<usize>()
        })
        .max()
}

aoc::solution!(22);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(&aoc::template::read_file("examples", 22)),
            Some(37990510)
        );
    }
    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(&aoc::template::read_file("examples", 22)),
            Some(23)
        );
    }
}
