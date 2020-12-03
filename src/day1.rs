use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|l| l.trim().parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

pub fn solve_day1_general(input: &Vec<u64>, sum: u64) -> u64 {
    let mut hm: HashMap<u64, u64> = HashMap::with_capacity(input.len());
    for &i in input.iter() {
        if i >= sum {
            continue;
        }
        let key = i * (sum - i);
        if hm.contains_key(&key) {
            return key;
        } else {
            hm.insert(key, i);
        }
    }
    0
}

#[aoc(day1, part1)]
pub fn solve_day1_part1(input: &Vec<u64>) -> u64 {
    solve_day1_general(input, 2020)
}

#[aoc(day1, part2)]
pub fn solve_day1_part2(input: &Vec<u64>) -> u64 {
    for &i in input.iter() {
        if i >= 2020 {
            continue;
        }
        let prod = i * solve_day1_general(input, 2020 - i);
        if prod > 0 {
            return prod;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator() {
        assert_eq!(input_generator("1\n2\n3\n4\n5\n"), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn d1p1_no_solution() {
        assert_eq!(solve_day1_part1(&vec![1, 2, 3]), 0);
    }

    #[test]
    fn d1p1_proper_solution() {
        assert_eq!(solve_day1_part1(&vec![1, 2019]), 2019);
    }

    #[test]
    fn d1p2_dumb_example() {
        assert_eq!(979 + 366 + 675, 2020);
        assert_eq!(979 * 366 * 675, 241861950);
    }

    #[test]
    fn d1p2_real_example() {
        assert_eq!(
            solve_day1_part2(&vec![1, 979, 2019, 366, 10, 675, 17, 3030]),
            241861950
        );

        assert_eq!(solve_day1_part2(&vec![979, 366, 675]), 241861950);
    }
}
