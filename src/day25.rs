use aoc_runner_derive::{aoc, aoc_generator};
use mod_exp::mod_exp;

const SUBJECTNUM: usize = 7;
const MODULAR: usize = 20201227;

#[aoc_generator(day25)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

pub fn naive_discrete_log(input: usize) -> usize {
    let mut res = 2;
    let mut value = SUBJECTNUM.pow(res) % MODULAR;
    while value != input {
        value = value * SUBJECTNUM % MODULAR;
        res += 1;
    }
    res as usize
}

#[aoc(day25, part1)]
pub fn part1(input: &Vec<usize>) -> usize {
    let num1 = input[0];
    let num2 = input[1];
    eprintln!("num1: {} num2: {}", num1, num2);

    let loop1 = naive_discrete_log(num1);
    mod_exp(num2, loop1, MODULAR)
}

#[aoc(day25, part2)]
pub fn part2(input: &Vec<usize>) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "5764801
17807724";
    #[test]
    pub fn test_generator() {
        assert_eq!(input_generator(INPUT), vec![5764801, 17807724]);
    }

    #[test]
    pub fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 14897079);
    }
}
