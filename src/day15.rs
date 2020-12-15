use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> HashMap<usize, usize> {
    let mut res: HashMap<usize, usize> = HashMap::new();
    let mut counter = 1;
    input.split(',').for_each(|s| {
        let num = s.parse::<usize>().unwrap();
        res.insert(num, counter);
        counter += 1;
    });
    res
}

pub fn solver(limit: usize, input: &HashMap<usize, usize>) -> usize {
    let mut counter = input.len();
    let mut hm = input.clone();
    let mut lastspoken = 0;
    let mut nowspoken = 0;

    // we take lastspoken and update it, while updating

    while counter < limit {
        lastspoken = nowspoken;
        counter += 1;

        if let Some(&val) = hm.get(&lastspoken) {
            nowspoken = counter - val;
        } else {
            nowspoken = 0;
        }
        hm.insert(lastspoken, counter);
    }
    lastspoken
}

#[aoc(day15, part1)]
pub fn part1(input: &HashMap<usize, usize>) -> usize {
    solver(2020, input)
}

#[aoc(day15, part2)]
pub fn part2(input: &HashMap<usize, usize>) -> usize {
    solver(30000000, input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "1,2,3";
    const INPUT2: [&str; 7] = [
        "0,3,6", "1,3,2", "2,1,3", "1,2,3", "2,3,1", "3,2,1", "3,1,2",
    ];
    const RESULT2: [usize; 7] = [436, 1, 10, 27, 78, 438, 1836];
    const RESULT3: [usize; 7] = [175594, 2578, 3544142, 261214, 6895259, 18, 362];

    #[test]
    pub fn test_generator() {
        assert_eq!(
            input_generator(INPUT1),
            [(1, 1), (2, 2), (3, 3)]
                .iter()
                .cloned()
                .collect::<HashMap<usize, usize>>()
        );
    }

    #[test]
    pub fn test_part1() {
        INPUT2.iter().zip(RESULT2.iter()).for_each(|(&s, &res)| {
            assert_eq!(part1(&input_generator(s)), res);
        });
    }
    #[test]
    pub fn test_part2() {
        INPUT2.iter().zip(RESULT3.iter()).for_each(|(&s, &res)| {
            assert_eq!(part2(&input_generator(s)), res);
        });
    }

    #[test]
    pub fn test_part2_t1() {
        assert_eq!(part2(&input_generator("0,3,6")), 175594);
    }
}
