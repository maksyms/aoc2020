use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashSet, VecDeque};

#[aoc_generator(day22)]
pub fn input_generator(input: &str) -> (VecDeque<usize>, VecDeque<usize>) {
    let players = input.split("\n\n").collect::<Vec<&str>>();
    let (player1, player2) = (players[0], players[1]);

    let player1: VecDeque<usize> = player1
        .lines()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let player2: VecDeque<usize> = player2
        .lines()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    (player1, player2)
}

#[aoc(day22, part1)]
pub fn part1((p1, p2): &(VecDeque<usize>, VecDeque<usize>)) -> usize {
    let mut p1 = p1.clone();
    let mut p2 = p2.clone();

    while p1.len() > 0 && p2.len() > 0 {
        // Lala
        let p1_card = p1.pop_front().unwrap();
        let p2_card = p2.pop_front().unwrap();
        if p1_card > p2_card {
            p1.push_back(p1_card);
            p1.push_back(p2_card);
        } else {
            p2.push_back(p2_card);
            p2.push_back(p1_card);
        }
    }

    if p1.len() > p2.len() {
        p1.iter()
            .enumerate()
            .fold(0, |acc, (id, el)| acc + (p1.len() - id) * el)
    } else {
        p2.iter()
            .enumerate()
            .fold(0, |acc, (id, el)| acc + (p2.len() - id) * el)
    }
}

pub fn play_game(
    (p1, p2): &(VecDeque<usize>, VecDeque<usize>),
) -> (VecDeque<usize>, VecDeque<usize>) {
    let mut p1 = p1.clone();
    let mut p2 = p2.clone();

    // First, lets create a structure to keep all card sequences per player
    let mut p1_previous_hands: HashSet<VecDeque<usize>> = HashSet::new();
    let mut p2_previous_hands: HashSet<VecDeque<usize>> = HashSet::new();

    while p1.len() > 0 && p2.len() > 0 {
        // Check whether the previous hands existed here
        if p1_previous_hands.contains(&p1) || p2_previous_hands.contains(&p2) {
            // Player 1 won
            p1.append(&mut p2);
        } else {
            // And then insert into previous hands
            p1_previous_hands.insert(p1.clone());
            p2_previous_hands.insert(p2.clone());

            // Draw cards
            let p1_card = p1.pop_front().unwrap();
            let p2_card = p2.pop_front().unwrap();

            if p1_card <= p1.len() && p2_card <= p2.len() {
                // The winner is the winner of the recursive game
                let p1_copy = p1.range(0..p1_card).copied().collect::<VecDeque<usize>>();
                let p2_copy = p2.range(0..p2_card).copied().collect::<VecDeque<usize>>();

                let (p1_copy, p2_copy) = play_game(&(p1_copy, p2_copy));

                if p1_copy.len() > p2_copy.len() {
                    p1.push_back(p1_card);
                    p1.push_back(p2_card);
                } else {
                    p2.push_back(p2_card);
                    p2.push_back(p1_card);
                }
            } else {
                // The winner is the higher value card
                if p1_card > p2_card {
                    p1.push_back(p1_card);
                    p1.push_back(p2_card);
                } else {
                    p2.push_back(p2_card);
                    p2.push_back(p1_card);
                }
            }
        }
    }

    (p1, p2)
}

#[aoc(day22, part2)]
pub fn part2((p1, p2): &(VecDeque<usize>, VecDeque<usize>)) -> usize {
    let (p1, p2) = play_game(&(p1.clone(), p2.clone()));

    if p1.len() > p2.len() {
        p1.iter()
            .enumerate()
            .fold(0, |acc, (id, el)| acc + (p1.len() - id) * el)
    } else {
        p2.iter()
            .enumerate()
            .fold(0, |acc, (id, el)| acc + (p2.len() - id) * el)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";

    #[test]
    pub fn test_generator() {
        assert_eq!(input_generator(INPUT).0, [9, 2, 6, 3, 1]);
        assert_eq!(input_generator(INPUT).1, [5, 8, 4, 7, 10]);
    }

    #[test]
    pub fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 306);
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 291);
    }
}
