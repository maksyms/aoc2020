use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct Pass {
    ch: char,
    min_freq: usize,
    max_freq: usize,
    pass: String,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Pass> {
    let re = Regex::new(r"^(\d+)-(\d+) (\S): (\S+)$").unwrap();
    input
        .lines()
        .map(|l| {
            // Parse the line now
            // The format is:
            // "x-y ch: ppppppp"
            let caps = re.captures(l).unwrap();
            Pass {
                ch: caps
                    .get(3)
                    .map_or(' ', |m| m.as_str().chars().next().unwrap()),
                min_freq: caps
                    .get(1)
                    .map_or(0, |m| m.as_str().parse::<usize>().unwrap()),
                max_freq: caps
                    .get(2)
                    .map_or(0, |m| m.as_str().parse::<usize>().unwrap()),
                pass: caps
                    .get(4)
                    .map_or(String::new(), |m| String::from(m.as_str())),
            }
        })
        .collect::<Vec<Pass>>()
}

#[aoc(day2, part1)]
pub fn solve_day2_part1(input: &Vec<Pass>) -> usize {
    input
        .iter()
        .filter(|p| {
            let count = p
                .pass
                .char_indices()
                .fold(0, |acc, c| if c.1 == p.ch { acc + 1 } else { acc });
            count >= p.min_freq && count <= p.max_freq
        })
        .count()
}

#[aoc(day2, part2)]
pub fn solve_day2_part2(input: &Vec<Pass>) -> usize {
    input
        .iter()
        .filter(|p| {
            let firstmatch = p.pass.chars().nth(p.min_freq - 1).unwrap_or(' ') == p.ch;
            let secondmatch = p.pass.chars().nth(p.max_freq - 1).unwrap_or(' ') == p.ch;
            firstmatch ^ secondmatch
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_generator() {
        assert_eq!(
            input_generator("5-6 c: cbccxc\n8-9 c: cccccccnp"),
            vec![
                Pass {
                    ch: 'c',
                    min_freq: 5,
                    max_freq: 6,
                    pass: String::from("cbccxc")
                },
                Pass {
                    ch: 'c',
                    min_freq: 8,
                    max_freq: 9,
                    pass: String::from("cccccccnp")
                }
            ]
        );
    }
}
