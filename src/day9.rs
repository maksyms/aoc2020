use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|l| l.parse::<u64>().unwrap_or(0))
        .collect::<Vec<u64>>()
}

fn check_element(el: u64, v: &[u64; 25]) -> u64 {
    v.iter().fold(0, |acc, n| {
        //lala
        acc
    })
}

// A C-way of doing things
pub fn solver(input: &Vec<u64>, preamble: usize) -> u64 {
    // start from the 6th element
    // check whether the element can be the sum of any two of the previous 25 ones
    // move on if it can, and stop and return if it cannot
    let mut start: usize = 0; // start of slice to check
    let mut end: usize = preamble - 1; // end of slice to check
    let mut el: usize = preamble - 1; // the index of a value being checked for the sum; should start with end, as it is pre-incremented
    let mut first: usize = start; // the first element of the sum
    let mut second: usize = first + 1; // the second element of the sum
    let mut found = true;

    while el < input.len() && found {
        el += 1;
        found = false;
        while first < end && !found {
            while input[first] < input[el] && second <= end && !found {
                if input[first] + input[second] == input[el] {
                    found = true;
                }
                second += 1;
            }
            first += 1;
            second = first + 1;
        }
        start += 1;
        end += 1;
        first = start;
        second = first + 1;
    }

    if el < input.len() {
        input[el]
    } else {
        0
    }
}

#[aoc(day9, part1)]
pub fn part1(input: &Vec<u64>) -> u64 {
    solver(input, 25)
}

pub fn solver2(input: &Vec<u64>, preamble: usize) -> u64 {
    let num = solver(input, preamble);
    let mut left: usize = 0;
    let mut right: usize = left + 1;
    let mut found = false;

    while left < right && right < input.len() && !found {
        let sum: u64 = input[left..=right].iter().sum();

        if sum < num {
            right += 1;
        } else if sum > num {
            left += 1;
        } else {
            found = true;
        }
    }

    input[left..=right].iter().min().unwrap() + input[left..=right].iter().max().unwrap()
}

#[aoc(day9, part2)]
pub fn part2(input: &Vec<u64>) -> u64 {
    solver2(input, 25)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1\n2\n3";
    const INPUT2: &str =
        "35\n20\n15\n25\n47\n40\n62\n55\n65\n95\n102\n117\n150\n182\n127\n219\n299\n277\n309\n576";

    #[test]
    pub fn test_generator() {
        assert_eq!(input_generator(INPUT), vec![1, 2, 3]);
        assert_eq!(
            input_generator(INPUT2),
            vec![
                35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277,
                309, 576
            ]
        );
    }

    #[test]
    pub fn test_part1() {
        assert_eq!(solver(&input_generator(INPUT2), 5), 127);
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(solver2(&input_generator(INPUT2), 5), 62);
    }
}
