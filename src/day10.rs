use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<u8> {
    input
        .lines()
        .map(|l| l.parse::<u8>().unwrap())
        .collect::<Vec<u8>>()
}

#[aoc(day10, part1)]
pub fn part1(input: &Vec<u8>) -> u16 {
    let mut sorted = input.clone();

    sorted.push(0); // starting point for the jolt socket

    sorted.sort_unstable();

    let (ones, threes) = sorted
        .iter()
        .skip(1)
        .enumerate()
        .map(|(i, n)| n - sorted[i])
        .fold((0, 1), |acc: (u16, u16), n| match n {
            1 => (acc.0 + 1, acc.1),
            3 => (acc.0, acc.1 + 1),
            _ => acc,
        });

    ones * threes
}

#[aoc(day10, part2)]
pub fn part2(input: &Vec<u8>) -> u128 {
    let mut sorted = input.clone();

    sorted.push(0); // starting point for the jolt socket
    sorted.push(*input.iter().max().unwrap() + 3);

    sorted.sort_unstable();

    *sorted
        .iter()
        .skip(1)
        .fold(
            vec![0u128; (*sorted.iter().max().unwrap() + 1) as usize],
            |mut acc, &n| {
                acc[0] = 1; // don't know how to initialize it better

                let idx = n as usize;

                if n >= 1 {
                    acc[idx] += acc[idx - 1];
                }
                if n >= 2 {
                    acc[idx] += acc[idx - 2];
                }
                if n >= 3 {
                    acc[idx] += acc[idx - 3];
                }

                acc
            },
        )
        .last()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "16
10
15
5
1
11
7
19
6
12
4";

    const INPUT2: &str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    #[test]
    pub fn test_generator() {
        assert_eq!(
            input_generator(INPUT1),
            vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4]
        );
    }

    #[test]
    pub fn test_part1_input1() {
        assert_eq!(part1(&input_generator(INPUT1)), 35);
    }

    #[test]
    pub fn test_part1_input2() {
        assert_eq!(part1(&input_generator(INPUT2)), 220);
    }

    #[test]
    pub fn test_part2_input1() {
        assert_eq!(part2(&input_generator(INPUT1)), 8);
    }

    #[test]
    pub fn test_part2_input2() {
        assert_eq!(part2(&input_generator(INPUT2)), 19208);
    }
}
