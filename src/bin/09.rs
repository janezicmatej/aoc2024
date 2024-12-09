use std::ops::Range;

use itertools::Itertools;

fn map_disk(input: &str) -> Vec<Option<usize>> {
    let disk_map = input.chars().filter_map(|x| x.to_digit(10)).collect_vec();

    let mut disk = Vec::with_capacity(disk_map.len() * 10);
    for (i, n) in disk_map.iter().copied().enumerate() {
        let id = if i % 2 == 0 { Some(i / 2) } else { None };
        for _ in 0..n {
            disk.push(id);
        }
    }

    disk
}

fn disk_checksum(disk: &[Option<usize>]) -> usize {
    disk.iter()
        .enumerate()
        .filter(|x| x.1.is_some())
        .map(|(i, x)| i * x.unwrap())
        .sum()
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut disk = map_disk(input);

    let mut left_ptr = 0;
    let mut right_ptr = disk.len() - 1;

    while left_ptr < right_ptr {
        if disk[left_ptr].is_some() {
            left_ptr += 1;
            continue;
        }
        if disk[right_ptr].is_none() {
            right_ptr -= 1;
            continue;
        }
        disk.swap(left_ptr, right_ptr);
        left_ptr += 1;
        right_ptr -= 1;
    }

    disk_checksum(&disk).into()
}

fn next_move_candidate(disk: &[Option<usize>], mut start: usize) -> Option<Range<usize>> {
    while start > 0 && disk[start].is_none() {
        start -= 1;
    }

    if start == 0 && disk[start].is_none() {
        return None;
    }

    let mut end = start;

    while end > 0
        && disk
            .get(end)
            .copied()
            .flatten()
            .filter(|&x| Some(x) == disk[start])
            .is_some()
    {
        end -= 1;
    }

    Some(end + 1..start + 1)
}

fn find_empty_space(
    disk: &[Option<usize>],
    size: usize,
    index_limit: usize,
) -> Option<Range<usize>> {
    let mut i = 0;

    loop {
        while i < index_limit && disk[i].is_some() {
            i += 1;
        }
        if i == index_limit {
            break;
        }
        let mut j = i;
        while j < index_limit && disk[j].is_none() {
            j += 1;
        }
        if j - i >= size {
            return Some(i..i + size);
        }
        i = j
    }

    None
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut disk = map_disk(input);

    let mut n = disk.len() - 1;

    while n > 0 {
        let c = next_move_candidate(&disk, n);
        if c.is_none() {
            break;
        }
        let c = c.unwrap();
        if c.start == 0 {
            break;
        }
        n = c.start - 1;

        let t = find_empty_space(&disk, c.len(), c.start);
        if t.is_none() {
            continue;
        }
        let t = t.unwrap();
        c.zip(t).map(|(cc, tt)| disk.swap(cc, tt)).count();
    }

    disk_checksum(&disk).into()
}

aoc::solution!(9);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(&aoc::template::read_file("examples", 9)),
            Some(1928)
        );
    }
    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(&aoc::template::read_file("examples", 9)),
            Some(2858)
        );
    }
}
