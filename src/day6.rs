use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

// TEMPLATE

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Vec<HashSet<char>>> {
    input
        .split("\n\n")
        .map(|s| {
            s.split('\n')
                .map(|g| {
                    g.chars().fold(HashSet::new(), |mut a, c| {
                        a.insert(c);
                        a
                    })
                })
                .collect::<Vec<HashSet<char>>>()
        })
        .collect::<Vec<Vec<HashSet<char>>>>()
}

/* pub fn input_generator(input: &str) -> Vec<String> {
    input
        .split("\n\n")
        .map(|s| String::from(s))
        .collect::<Vec<String>>()
}
 */

#[aoc(day6, part1)]
pub fn part1(input: &Vec<Vec<HashSet<char>>>) -> usize {
    input
        .iter()
        .map(|el| {
            el.iter()
                .fold(HashSet::new(), |a, hs| a.union(hs).copied().collect())
                .len()
        })
        .sum()
}

/* pub fn part1(input: &Vec<String>) -> usize {
    input
        .iter()
        .map(|s| s.split('\n').collect::<String>())
        .map(|l| {
            l.chars()
                .fold(HashSet::new(), |mut a, c| {
                    a.insert(c);
                    a
                })
                .len()
        })
        .sum()
}
 */

#[aoc(day6, part2)]
pub fn part2(input: &Vec<Vec<HashSet<char>>>) -> usize {
    input
        .iter()
        .map(|vhs| {
            let mut iter = vhs.iter();
            if let Some(hs) = iter
                .next()
                .cloned()
                .map(|i| iter.fold(i, |a, hs| a.intersection(hs).copied().collect()))
            {
                hs.len()
            } else {
                0
            }
        })
        .sum()
}

/* pub fn part2(input: &Vec<String>) -> usize {
    input
        .iter()
        .map(|s| {
            // in each group, replace each person with a HashSet of their "yes" answers
            let mut iter = s.split('\n').map(|g| {
                g.chars().fold(HashSet::new(), |mut a, c| {
                    a.insert(c);
                    a
                })
            });
            // now iter has the iterator over HashSets representing each person in the group
            // then the below trickery is to calculate the intersection of several HashSets
            // and identify how many questions were answered the same by everyone in the group
            if let Some(hs) = iter
                .next()
                .map(|i| iter.fold(i, |hs1, hs2| hs1.intersection(&hs2).copied().collect()))
            {
                hs.len()
            } else {
                0
            }
        })
        .sum()
}
 */

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    pub fn test_generator() {
        /*         assert_eq!(
                   input_generator(INPUT),
                   vec![
                       String::from("abc"),
                       String::from("a\nb\nc"),
                       String::from("ab\nac"),
                       String::from("a\na\na\na"),
                       String::from("b")
                   ]
               );
        */
    }

    #[test]
    pub fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 11);
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 6);
    }
}
