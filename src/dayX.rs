use aoc_runner_derive::{aoc, aoc_generator};

// TEMPLATE

#[aoc_generator(dayX)]
pub fn input_generator(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|s| String::from(s))
        .collect::<Vec<String>>()
}

#[aoc(dayX, part1)]
pub fn part1(input: &Vec<String>) -> usize {
    input.len()
}

#[aoc(dayX, part2)]
pub fn part2(input: &Vec<String>) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1 2 3";
    #[test]
    pub fn test_generator() {
        assert_eq!(
            input_generator(INPUT),
            vec![String::from("1"), String::from("2"), String::from("3")]
        );
    }
}
