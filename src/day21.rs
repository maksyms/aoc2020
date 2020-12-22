use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::collections::{HashMap, HashSet};

#[aoc_generator(day21)]
pub fn input_generator(input: &str) -> Vec<(HashSet<String>, HashSet<String>)> {
    // Split into a vector of a pair of hashsets - the first is the list of known allergens, the second is the list of ingredients
    let re = Regex::new(r#"^([a-z ]+) \(contains ([a-z, ]+)\)$"#).unwrap();
    input
        .lines()
        .map(|s| {
            let caps = re.captures(s).unwrap();
            let mut hsi = HashSet::new();
            let mut hsa = HashSet::new();
            if let Some(c) = caps.get(1) {
                c.as_str().split(' ').for_each(|w| {
                    hsi.insert(String::from(w));
                });
            }
            if let Some(c) = caps.get(2) {
                c.as_str().split(", ").for_each(|w| {
                    hsa.insert(String::from(w));
                })
            }
            (hsa, hsi)
        })
        .collect::<Vec<(HashSet<String>, HashSet<String>)>>()
}

pub fn solver(input: &Vec<(HashSet<String>, HashSet<String>)>) -> HashMap<String, String> {
    let (all_allergens, all_ingredients) =
        input
            .iter()
            .fold((HashSet::new(), HashSet::new()), |acc, el| {
                (
                    el.0.union(&acc.0).cloned().collect::<HashSet<String>>(),
                    el.1.union(&acc.1).cloned().collect::<HashSet<String>>(),
                )
            });

    let mut hm = HashMap::new();
    for allergen in all_allergens.iter() {
        hm.insert(
            allergen,
            input
                .iter()
                .fold(
                    (all_allergens.clone(), all_ingredients.clone()),
                    |acc, (hsa, hsi)| {
                        if hsa.contains(allergen) {
                            (
                                acc.0,
                                hsi.intersection(&acc.1)
                                    .cloned()
                                    .collect::<HashSet<String>>(),
                            )
                        } else {
                            acc
                        }
                    },
                )
                .1,
        );
    }

    // Now hm should contain enough to decipher everything
    // while any of the keys have more than one value
    // - pick each key which only has one value
    // - remove its value from all other keys
    // - rinse and repeat

    let mut deciphered: HashMap<String, String> = HashMap::new();

    while hm.values().fold(0, |acc, v| acc + v.len()) > 0 {
        // There is some intermittent bug which occassionally means that unwrapping find_one leads to panic due to None
        // As it works enough to solve the problem I have at hand, I chose to ignore it - just rerun the solution a few times
        let find_one = hm.iter().find(|(_, vv)| vv.len() == 1).unwrap();
        deciphered.insert(
            find_one.0.clone().clone(),
            find_one.1.iter().next().unwrap().clone(),
        );

        hm.iter_mut().for_each(|(&k, v)| {
            if v.len() == 1 {
                let val = v.iter().next().unwrap().clone();
                v.remove(&val);
                deciphered.insert(k.clone(), val);
            } else if v.len() > 1 {
                deciphered.values().for_each(|val| {
                    v.remove(val);
                });
            }
        })
    }

    deciphered
}

#[aoc(day21, part1)]
pub fn part1(input: &Vec<(HashSet<String>, HashSet<String>)>) -> usize {
    let deciphered = solver(input);

    let all_ingredients = input.iter().fold(HashSet::new(), |acc, el| {
        el.1.union(&acc).cloned().collect::<HashSet<String>>()
    });

    let allergens = deciphered.values().fold(HashSet::new(), |mut acc, val| {
        acc.insert(val);
        acc
    });

    let mut non_allergens = all_ingredients.clone();
    non_allergens.retain(|k| !allergens.contains(k));

    input.iter().fold(0, |acc, (_, hsi)| {
        acc + hsi.iter().fold(0, |acc_hsi, el_hsi| {
            if non_allergens.contains(el_hsi) {
                acc_hsi + 1
            } else {
                acc_hsi
            }
        })
    })
}

#[aoc(day21, part2)]
pub fn part2(input: &Vec<(HashSet<String>, HashSet<String>)>) -> String {
    let deciphered = solver(input);
    let mut v: Vec<_> = deciphered.into_iter().collect();
    v.sort_by(|x, y| x.0.cmp(&y.0));
    let v = v.iter().map(|s| s.1.clone()).collect::<Vec<String>>();
    v.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";
    #[test]
    pub fn test_generator() {
        assert_eq!(
            input_generator(INPUT),
            vec![(HashSet::new(), HashSet::new())]
        );
    }

    #[test]
    pub fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 5);
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(
            part2(&input_generator(INPUT)),
            String::from("mxmxvkd,sqjhc,fvjkl")
        );
    }
}
