use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::collections::HashMap;

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|s| String::from(s))
        .collect::<Vec<String>>()
}

#[aoc(day14, part1)]
pub fn part1(input: &Vec<String>) -> usize {
    let mut mem: HashMap<usize, usize> = HashMap::new();
    // mask.0 will be the mask to extract unchanged bits
    // mask.1 will be the bits to add
    let mut mask: (usize, usize) = (0, 0);
    let mask_regex = Regex::new(r"^mask = ([01X]{36})$").unwrap();
    let mem_regex = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
    input.iter().for_each(|l| {
        // 1. Parse the line
        // 2. Either update mask
        // 3. Or update hashmap with address/value pair after applying mask
        if let Some(caps) = mask_regex.captures(l) {
            let digits = caps.get(1).unwrap().as_str();
            mask.0 = usize::from_str_radix(digits.replace('1', "0").replace('X', "1").as_str(), 2)
                .unwrap();
            mask.1 = usize::from_str_radix(digits.replace('X', "0").as_str(), 2).unwrap();
        } else if let Some(caps) = mem_regex.captures(l) {
            let addr = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let val = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let val = (val & mask.0) + mask.1;
            mem.insert(addr, val);
        }
    });
    mem.values().sum()
}

#[aoc(day14, part2)]
pub fn part2(input: &Vec<String>) -> usize {
    let mut mem: HashMap<usize, usize> = HashMap::new();
    let mut mask: (usize, usize) = (0, 0);
    let mask_regex = Regex::new(r"^mask = ([01X]{36})$").unwrap();
    let mem_regex = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
    input.iter().for_each(|l| {
        if let Some(caps) = mask_regex.captures(l) {
            let extracted_mask = String::from(caps.get(1).unwrap().as_str());
            mask = (
                usize::from_str_radix(extracted_mask.replace('X', "0").as_str(), 2).unwrap(),
                usize::from_str_radix(extracted_mask.replace('X', "1").as_str(), 2).unwrap(),
            );
        } else if let Some(caps) = mem_regex.captures(l) {
            let addr = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let val = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let bitdiff = mask.0 ^ mask.1;
            // we will need to write to 2^(Xes in the mask) addresses, which is the number of 1s in bitdiff
            for num_of_bits in 0..2usize.pow(bitdiff.count_ones()) {
                // Calculate the address to write this time by flipping the relevant bits in the address, 2^bitdiff.count_ones() times
                let addr_to_write = (0..bitdiff.count_ones() as usize)
                    .fold(
                        (addr | mask.0, bitdiff),
                        |(temp_addr, temp_bitdiff), ones_counter| {
                            (
                                temp_addr
                                    ^ bitdiff
                                        & (temp_bitdiff
                                            ^ (temp_bitdiff - (num_of_bits >> ones_counter & 1))),
                                temp_bitdiff & (temp_bitdiff - 1),
                            )
                        },
                    )
                    .0;
                eprintln!("addr: {:b}", addr_to_write);
                mem.insert(addr_to_write, val);
            }
        } else {
            panic!("Unexpected line format.")
        }
    });
    mem.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1 2 3";
    const INPUT2: &str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    const INPUT3: &str = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

    #[test]
    pub fn test_generator() {
        assert_eq!(
            input_generator(INPUT),
            vec![String::from("1"), String::from("2"), String::from("3")]
        );
    }

    #[test]
    pub fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT2)), 165);
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT3)), 208);
    }
}
