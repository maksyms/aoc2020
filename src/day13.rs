use aoc_runner_derive::{aoc, aoc_generator};

// TEMPLATE

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> (u32, Vec<Option<u32>>) {
    let mut li = input.lines();
    let tts = li.next().unwrap().parse::<u32>().unwrap();
    let vb = li
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<u32>().ok())
        .collect::<Vec<Option<u32>>>();
    (tts, vb)
}

#[aoc(day13, part1)]
pub fn part1((tts, vb): &(u32, Vec<Option<u32>>)) -> u32 {
    let result = vb
        .iter()
        .filter(|&el| el.is_some())
        .map(|&el| el.unwrap())
        .map(|el| (el, (((tts / el) as f32).ceil() + 1.0) as u32 * el - *tts))
        .min_by(|(_, diff1), (_, diff2)| diff1.cmp(diff2))
        .unwrap();
    result.0 * result.1
}

#[aoc(day13, part2)]
pub fn part2((_, vb): &(u32, Vec<Option<u32>>)) -> isize {
    let buses = vb
        .iter()
        .enumerate()
        .map(|(i, &el)| (el, i))
        .filter(|&(el, _)| el.is_some())
        .map(|(el, i)| (el.unwrap() as isize, i as isize))
        .collect::<Vec<(isize, isize)>>();

    // buses are prime numbers!
    // our task is formulated so that the diff in time is the remainder in the modulus division
    // i.e., t + bus_diff = bus * k, hence t = -bus_diff mod bus
    // So for the example, t = 0 mod 7, t = -1 mod 13, t = -4 mod 59, etc, etc.
    // As we have the remainders of dividing t by several integers, we can determine t modulus
    // the product of all bus numbers - hence, the result we're looking for -
    // using Chinese remainder theorem: https://en.wikipedia.org/wiki/Chinese_remainder_theorem

    let prod = buses.iter().map(|&(bus, _)| bus).product();

    buses
        .iter()
        .map(|(bus, diff)| {
            -diff * (prod / bus) * (0..bus - 2).fold(1, |acc, _| (acc * prod / bus) % bus)
        })
        .sum::<isize>()
        .rem_euclid(prod)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "939\n7,13,x,x,59,x,31,19";
    const INPUT1: &str = "939\n17,x,13,19";

    #[test]
    pub fn test_generator() {
        assert_eq!(
            input_generator(INPUT),
            (
                939,
                vec![
                    Some(7),
                    Some(13),
                    None,
                    None,
                    Some(59),
                    None,
                    Some(31),
                    Some(19)
                ]
            )
        );
    }

    #[test]
    pub fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 295);
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 1068781);
    }

    #[test]
    pub fn test_part2_simple() {
        assert_eq!(part2(&input_generator(INPUT1)), 3417);
    }
}
