use std::mem::swap;

use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[derive(Debug)]
pub struct Vessel {
    x: i32,
    y: i32,
    dir: i32,
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|l| String::from(l))
        .collect::<Vec<String>>()
}

#[aoc(day12, part1)]
pub fn part1(input: &Vec<String>) -> i32 {
    let mut ves = Vessel { x: 0, y: 0, dir: 0 };

    let matcher = Regex::new(r"^([NSEWLRF]{1})(\d+)$").unwrap();

    input.iter().for_each(|s| {
        let caps = matcher.captures(s);
        if let Some(cap) = caps {
            let command = cap.get(1).unwrap().as_str();
            let value = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
            match command {
                "N" => {
                    ves.y += value;
                }
                "S" => {
                    ves.y -= value;
                }
                "E" => {
                    ves.x += value;
                }
                "W" => {
                    ves.x -= value;
                }
                "L" => {
                    ves.dir += value;
                }
                "R" => {
                    ves.dir -= value;
                    if ves.dir < 0 {
                        ves.dir = 360 + ves.dir;
                    }
                }
                "F" => {
                    ves.x += ((value as f64) * (ves.dir as f64).to_radians().cos()) as i32;
                    ves.y += ((value as f64) * (ves.dir as f64).to_radians().sin()) as i32;
                }
                _ => panic!("Isn't supposed to happen."),
            }
        }
    });

    ves.x.abs() + ves.y.abs()
}

#[aoc(day12, part2)]
pub fn part2(input: &Vec<String>) -> i32 {
    let mut ves = Vessel { x: 0, y: 0, dir: 0 };
    let mut wp = Vessel {
        x: 10,
        y: 1,
        dir: 0,
    };

    let matcher = Regex::new(r"^([NSEWLRF]{1})(\d+)$").unwrap();

    input.iter().for_each(|s| {
        let caps = matcher.captures(s);
        if let Some(cap) = caps {
            let command = cap.get(1).unwrap().as_str();
            let value = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
            match command {
                "N" => {
                    wp.y += value;
                }
                "S" => {
                    wp.y -= value;
                }
                "E" => {
                    wp.x += value;
                }
                "W" => {
                    wp.x -= value;
                }
                "L" => {
                    for _ in 0..value / 90 {
                        swap(&mut wp.x, &mut wp.y);
                        wp.x = -wp.x;
                    }
                }
                "R" => {
                    for _ in 0..value / 90 {
                        swap(&mut wp.x, &mut wp.y);
                        wp.y = -wp.y;
                    }
                }
                "F" => {
                    ves.x += value * wp.x;
                    ves.y += value * wp.y;
                }
                _ => panic!("Isn't supposed to happen."),
            }
        }
    });

    ves.x.abs() + ves.y.abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "F10
N3
F7
R90
F11";

    #[test]
    pub fn test_generator() {
        assert_eq!(
            input_generator(INPUT),
            vec![
                String::from("F10"),
                String::from("N3"),
                String::from("F7"),
                String::from("R90"),
                String::from("F11")
            ]
        );
    }

    #[test]
    pub fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 25);
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 286);
    }
}
