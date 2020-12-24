use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::VecDeque;

// Tbh, using modular here is an overkill: I could have just manually "wrapped" the values
use modular::*;

#[aoc_generator(day23, part1, PoorMansLinkedList)]
pub fn input_generator(input: &str) -> Vec<usize> {
    let max_cup = input.len();
    let mut res = vec![0; max_cup + 1];

    let input = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();

    // Create a "single-linked-list" (-ish)
    // I couldn't find a decent implementation of single linked list in Rust!
    // Hence, "poor men's" single-linked list
    // The idea is that the index is the cup number, and the value is what the next cup is after that one
    for cup in 1..input.len() {
        res[input[cup - 1]] = input[cup];
    }
    res[input[max_cup - 1]] = input[0];

    res[0] = input[0];

    res
}

#[aoc_generator(day23, part2, PoorMansLinkedList)]
pub fn input_generator_p2(input: &str) -> Vec<usize> {
    let mut res = input_generator(input);
    let max_cup = res.len();

    // Find the last position, which is the position that points to the first digit of input
    // note that as I'm skipping res[0], then I need to add 1 to resulting position
    let pos = res
        .iter()
        .skip(1) // skip the res[0], which is the head of "linked list"
        .position(|&el| el == res[0])
        .unwrap()
        + 1; // This "+1" costed me many hours of debugging!

    res.resize(1000001, 0);
    res[pos] = max_cup;

    for cup in max_cup + 1..=1000000 {
        res[cup - 1] = cup;
    }

    res[1000000] = res[0];

    res
}

pub fn solver(input: &Vec<usize>, steps: usize) -> Vec<usize> {
    let mut cups = input.clone();
    let modulo: u32 = cups.len() as u32;
    let mut current_cup = cups[0];

    for _ in 1..=steps {
        let three_cups: [usize; 3] = [
            cups[current_cup],
            cups[cups[current_cup]],
            cups[cups[cups[current_cup]]],
        ];
        let next_cup = cups[cups[cups[cups[current_cup]]]];

        // find the element to look for
        let mut destination = modulo!(current_cup as i32 - 1, modulo);
        while three_cups.contains(&(destination.remainder() as usize))
            || destination == 0.to_modulo(modulo)
        {
            destination = destination - 1.to_modulo(modulo);
        }

        let destination = destination.remainder() as usize;

        // Update cup ordering
        cups[three_cups[2]] = cups[destination];
        cups[destination] = cups[current_cup];
        cups[current_cup] = next_cup;
        current_cup = next_cup;
    }

    cups
}

#[aoc(day23, part1, PoorMansLinkedList)]
pub fn part1(input: &Vec<usize>) -> usize {
    let cups = solver(input, 100);

    let mut res: usize = 0;
    let mut cup: usize = cups[1];
    while cup != 1 {
        res = res * 10 + cup;
        cup = cups[cup];
    }
    res
}

#[aoc(day23, part2, PoorMansLinkedList)]
pub fn part2(input: &Vec<usize>) -> usize {
    let cups = solver(input, 10000000);

    let cup1 = cups[1];
    let cup2 = cups[cup1];
    cup1 * cup2
}

#[aoc_generator(day23, part1, VecDeque)]
pub fn input_generator_vd(input: &str) -> VecDeque<usize> {
    input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<VecDeque<usize>>()
}

pub fn solver_vd(input: &VecDeque<usize>, steps: usize) -> VecDeque<usize> {
    let mut cups = input.iter().copied().collect::<VecDeque<usize>>();

    let modulo: u32 = (cups.iter().max().unwrap() + 1) as u32;

    for _ in 1..=steps {
        // let cups_before = cups.clone();
        let current_cup = cups.pop_front().unwrap();
        let three_cups: [usize; 3] = [
            cups.pop_front().unwrap(),
            cups.pop_front().unwrap(),
            cups.pop_front().unwrap(),
        ];

        // find the element to look for
        let mut candidate = modulo!((current_cup - 1) as i32, modulo);
        while three_cups.contains(&(candidate.remainder() as usize))
            || candidate == 0.to_modulo(modulo)
        {
            candidate = candidate - 1.to_modulo(modulo);
        }
        let pos = cups
            .iter()
            .position(|&el| (el as i32).to_modulo(modulo) == candidate)
            .unwrap();

        cups.insert(pos + 1, three_cups[2]);
        cups.insert(pos + 1, three_cups[1]);
        cups.insert(pos + 1, three_cups[0]);
        cups.push_back(current_cup);
    }

    cups
}

#[aoc(day23, part1, VecDeque)]
pub fn part1_vd(input: &VecDeque<usize>) -> usize {
    let mut cups = solver_vd(&input, 100);

    // Here cups contains the right stuff
    // Now I need to rotate it to get "1" to the start
    let one_pos = cups.iter().enumerate().find(|&(_, &el)| el == 1).unwrap().0;
    cups.rotate_left(one_pos);
    // And pop it off the front
    cups.pop_front();
    // Then concatenate the numbers
    cups.iter().enumerate().fold(0, |acc, (idx, el)| {
        acc + el * 10usize.pow((cups.len() - idx - 1) as u32)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "389125467";
    #[test]
    pub fn test_generator() {
        assert_eq!(input_generator(INPUT), [3, 2, 5, 8, 6, 4, 7, 3, 9, 1]);
    }

    #[test]
    pub fn test_generator_p2() {
        assert_eq!(input_generator_p2(INPUT).len(), 1000001);
    }

    #[test]
    pub fn test_solver10() {
        let cups = solver(&input_generator(INPUT), 10);
        let mut res: usize = 0;
        let mut cup: usize = cups[1];
        while cup != 1 {
            res = res * 10 + cup;
            cup = cups[cup];
        }

        assert_eq!(res, 92658374);
    }

    #[test]
    pub fn test_solver10_vd() {
        let mut cups = solver_vd(&input_generator_vd(INPUT), 10);
        let one_pos = cups.iter().enumerate().find(|&(_, &el)| el == 1).unwrap().0;
        cups.rotate_left(one_pos);
        // And pop it off the front
        cups.pop_front();
        // Then concatenate the numbers
        let res = cups.iter().enumerate().fold(0, |acc, (idx, el)| {
            acc + el * 10usize.pow((cups.len() - idx - 1) as u32)
        });

        assert_eq!(res, 92658374);
    }

    #[test]
    pub fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 67384529);
    }

    #[test]
    pub fn test_part1_vd() {
        assert_eq!(part1_vd(&input_generator_vd(INPUT)), 67384529);
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(part2(&input_generator_p2(INPUT)), 149245887792);
    }
}
