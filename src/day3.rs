use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|l| String::from(l))
        .collect::<Vec<String>>()
}

pub fn solve_generic(input: &Vec<String>, right: usize, down: usize) -> usize {
    input
        .iter()
        .enumerate()
        .filter(|(i, l)| {
            // Filter each string based on whether we hit the "#" or not
            l.len() > 0
                && i % down == 0
                && l.chars().nth((i / down * right) % l.len()).unwrap_or(' ') == '#'
        })
        .count()
}

#[aoc(day3, part1)]
pub fn part1(input: &Vec<String>) -> usize {
    solve_generic(input, 3, 1)
}

#[aoc(day3, part2)]
pub fn part2(input: &Vec<String>) -> usize {
    solve_generic(input, 1, 1)
        * solve_generic(input, 3, 1)
        * solve_generic(input, 5, 1)
        * solve_generic(input, 7, 1)
        * solve_generic(input, 1, 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> String {
        String::from(
            "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
",
        )
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(get_input().as_str())), 7);
    }

    #[test]
    fn test_generic1() {
        assert_eq!(
            solve_generic(&input_generator(get_input().as_str()), 1, 1),
            2
        );
    }

    #[test]
    fn test_generic2() {
        assert_eq!(
            solve_generic(&input_generator(get_input().as_str()), 3, 1),
            7
        );
    }

    #[test]
    fn test_generic3() {
        assert_eq!(
            solve_generic(&input_generator(get_input().as_str()), 5, 1),
            3
        );
    }

    #[test]
    fn test_generic4() {
        assert_eq!(
            solve_generic(&input_generator(get_input().as_str()), 7, 1),
            4
        );
    }

    #[test]
    fn test_generic5() {
        assert_eq!(
            solve_generic(&input_generator(get_input().as_str()), 1, 2),
            2
        );
    }
}
