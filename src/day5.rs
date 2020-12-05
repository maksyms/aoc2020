use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<u16> {
    let mut bz = [0; 2];
    let mut bo = [0; 2];
    let zero = '0'.encode_utf8(&mut bz);
    let one = '1'.encode_utf8(&mut bo);
    input
        .replace('F', zero)
        .replace('B', one)
        .replace('L', zero)
        .replace('R', one)
        .lines()
        .map(|l| u16::from_str_radix(l, 2).unwrap_or(0))
        .collect::<Vec<u16>>()
}

#[aoc(day5, part1)]
pub fn part1(input: &Vec<u16>) -> u16 {
    *input.iter().max().unwrap()
}

#[aoc(day5, part2)]
pub fn part2(input: &Vec<u16>) -> u16 {
    let mut sorted = input.clone();
    sorted.sort();
    for idx in 1..sorted.len() {
        if sorted[idx] - sorted[idx - 1] > 1 {
            return sorted[idx] - 1;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_generator() {
        assert_eq!(input_generator("BFFFBBFRRR"), vec![567]);
    }
}
