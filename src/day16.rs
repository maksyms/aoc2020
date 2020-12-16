use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::collections::{HashMap, HashSet};

// The HashMap solution and the bitmask solution are both possible
// This is the HashMap solution

#[derive(Debug, Clone)]
pub struct Rule {
    name: String,
    min1: usize,
    max1: usize,
    min2: usize,
    max2: usize,
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> (Vec<Rule>, Vec<usize>, Vec<Vec<usize>>) {
    let firstsplit = &input
        .split("\n\n")
        .map(|s| String::from(s))
        .collect::<Vec<String>>();

    let rules = firstsplit[0].clone();
    let myticket = firstsplit[1].clone();
    let neartickets = firstsplit[2].clone();

    let rules_matcher =
        Regex::new(r"^([a-z ]+): (\d{1,3})-(\d{1,3}) or (\d{1,3})-(\d{1,3})$").unwrap();
    let rules = rules
        .lines()
        .map(|l| {
            if let Some(caps) = rules_matcher.captures(l) {
                Rule {
                    name: String::from(caps.get(1).unwrap().as_str()),
                    min1: caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                    max1: caps.get(3).unwrap().as_str().parse::<usize>().unwrap(),
                    min2: caps.get(4).unwrap().as_str().parse::<usize>().unwrap(),
                    max2: caps.get(5).unwrap().as_str().parse::<usize>().unwrap(),
                }
            } else {
                panic!("Isn't supposed to happen.")
            }
        })
        .collect();

    let myticket = myticket
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let neartickets = neartickets
        .lines()
        .skip(1)
        .map(|l| {
            l.split(',')
                .map(|li| li.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();

    (rules, myticket, neartickets)
}

#[aoc(day16, part1)]
pub fn part1((rules, _myticket, neartickets): &(Vec<Rule>, Vec<usize>, Vec<Vec<usize>>)) -> usize {
    // Build hashmap with all the values
    let mut set: HashSet<usize> = HashSet::new();
    rules.iter().for_each(|r| {
        (r.min1..=r.max1).for_each(|n| {
            set.insert(n);
        });
        (r.min2..=r.max2).for_each(|n| {
            set.insert(n);
        });
    });

    neartickets.iter().fold(0, |te, tv| {
        te + tv.iter().fold(
            0,
            |tei, num| {
                if !set.contains(num) {
                    tei + num
                } else {
                    tei
                }
            },
        )
    })
}

pub fn get_invalid_tickets(
    rules: &Vec<Rule>,
    neartickets: &Vec<Vec<usize>>,
) -> HashSet<Vec<usize>> {
    // Build hashmap with all the values
    let mut set: HashSet<usize> = HashSet::new();
    rules.iter().for_each(|r| {
        (r.min1..=r.max1).for_each(|n| {
            set.insert(n);
        });
        (r.min2..=r.max2).for_each(|n| {
            set.insert(n);
        });
    });

    neartickets.iter().fold(HashSet::new(), |mut acc, tv| {
        tv.iter().for_each(|t| {
            if !set.contains(t) {
                acc.insert(tv.clone());
            }
        });
        acc
    })
}

// Copied from here: https://stackoverflow.com/questions/64498617/how-to-transpose-a-vector-of-vectors-in-rust
pub fn transpose<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

#[aoc(day16, part2)]
pub fn part2((rules, myticket, neartickets): &(Vec<Rule>, Vec<usize>, Vec<Vec<usize>>)) -> usize {
    // Get invalid tickets
    let invalid_tickets = get_invalid_tickets(rules, neartickets);

    // Transpose the vectors to put the values to check in the same vector
    let cols = transpose(
        &neartickets
            .iter()
            .filter(|&t| !invalid_tickets.contains(t))
            .cloned()
            .collect::<Vec<Vec<usize>>>(),
    );

    // Find all the rules that match particular column
    let mut fields_mapping = cols
        .iter()
        .enumerate()
        .fold(HashMap::new(), |mut hm, (col, v)| {
            let rulematch: Vec<String> = rules
                .iter()
                .filter(|&r| {
                    v.iter().fold(true, |acc, &n| {
                        acc && (((n >= r.min1) && (n <= r.max1))
                            || ((n >= r.min2) && (n <= r.max2)))
                    })
                })
                .map(|r| r.name.clone())
                .collect();

            hm.insert(col, rulematch);
            hm
        });

    // Now reduce fields_mapping like this:
    // 1. find the field mapping where only one match exists
    // 2. remove that match from all other field mappings
    // 3. repeat the above until no more field mappings are left
    let mut final_mapping: HashMap<usize, String> = HashMap::new();
    while fields_mapping.len() > 0 {
        let sfm = fields_mapping.iter().find(|&(_, mv)| mv.len() == 1);
        if let Some((col, vs)) = sfm {
            final_mapping.insert(*col, vs[0].clone());
        }
        final_mapping.keys().for_each(|k| {
            fields_mapping.remove(k);
            // Now remove from vectors inside
            fields_mapping.iter_mut().for_each(|(_, s)| {
                let oidx = s.iter().position(|x| x == &final_mapping[k]);
                if let Some(idx) = oidx {
                    s.remove(idx);
                }
            });
        });
    }

    myticket
        .iter()
        .enumerate()
        .filter(|(i, _)| final_mapping[i].starts_with("departure"))
        .map(|x| x.1)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

    const INPUT2: &str = "departure class: 0-1 or 4-19
departure row: 0-5 or 8-19
departure seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";

    /*     #[test]
       pub fn test_generator() {
           assert_eq!(input_generator(INPUT), (vec![], vec![], vec![]));
       }
    */

    #[test]
    pub fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 71);
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT2)), 1716);
    }
}
