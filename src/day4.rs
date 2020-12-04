use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref VALIDATORS: HashMap<&'static str, Regex> = {
        let mut m = HashMap::new();
        m.insert("byr", Regex::new(r"byr:(\d{4})").unwrap());
        m.insert("iyr", Regex::new(r"iyr:(\d{4})").unwrap());
        m.insert("eyr", Regex::new(r"eyr:(\d{4})").unwrap());
        m.insert("hgt", Regex::new(r"hgt:(\d{2,3})(cm|in)").unwrap());
        m.insert("hcl", Regex::new(r"hcl:#[0-9a-f]{6}").unwrap());
        m.insert(
            "ecl",
            Regex::new(r"ecl:(amb|blu|brn|gry|grn|hzl|oth)").unwrap(),
        );
        m.insert("pid", Regex::new(r"pid:(\d{9}\s|\d{9}$)").unwrap());
        m
    };
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<String> {
    input
        .split("\n\n")
        // Ugly - but I don't know how to do conversion of char to &str better
        .map(|l| String::from(l).replace('\n', ' '.to_string().as_str()))
        .collect::<Vec<String>>()
}

#[aoc(day4, part1)]
pub fn part1(input: &Vec<String>) -> usize {
    input
        .iter()
        .filter(|l| VALIDATORS.keys().filter(|m| l.contains(*m)).count() >= 7)
        .count()
}

#[aoc(day4, part2)]
pub fn part2(input: &Vec<String>) -> usize {
    input
        .iter()
        .filter(|l| {
            //validator logic
            VALIDATORS
                .iter()
                .filter(|(&s, re)| {
                    // validating this specific field
                    let mut ret = false;
                    match s {
                        "byr" => {
                            if let Some(caps) = re.captures(l) {
                                if let Some(year) = caps.get(1) {
                                    if let Ok(year) = year.as_str().parse::<u16>() {
                                        ret = year >= 1920 && year <= 2002;
                                    }
                                }
                            }
                        }
                        "iyr" => {
                            if let Some(caps) = re.captures(l) {
                                if let Some(year) = caps.get(1) {
                                    if let Ok(year) = year.as_str().parse::<u16>() {
                                        ret = year >= 2010 && year <= 2020;
                                    }
                                }
                            }
                        }
                        "eyr" => {
                            if let Some(caps) = re.captures(l) {
                                if let Some(year) = caps.get(1) {
                                    if let Ok(year) = year.as_str().parse::<u16>() {
                                        ret = year >= 2020 && year <= 2030;
                                    }
                                }
                            }
                        }
                        "hgt" => {
                            if let Some(caps) = re.captures(l) {
                                if let Some(units) = caps.get(2) {
                                    let unit = units.as_str();
                                    if let Some(num) = caps.get(1) {
                                        if let Ok(num) = num.as_str().parse::<u8>() {
                                            ret = (unit == "cm" && num >= 150 && num <= 193)
                                                || (unit == "in" && num >= 59 && num <= 76);
                                        }
                                    }
                                }
                            }
                        }
                        "hcl" => ret = re.is_match(l),
                        "ecl" => ret = re.is_match(l),
                        "pid" => ret = re.is_match(l),
                        _ => ret = false,
                    }
                    ret
                })
                .count()
                == 7
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    const INVALID: &str = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

    const VALID: &str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

    #[test]
    pub fn test_generator() {
        assert_eq!(
            input_generator(INPUT),
            vec![
                "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm",
                "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884 hcl:#cfa07d byr:1929",
                "hcl:#ae17e1 iyr:2013 eyr:2024 ecl:brn pid:760753108 byr:1931 hgt:179cm",
                "hcl:#cfa07d eyr:2025 pid:166559648 iyr:2011 ecl:brn hgt:59in"
            ]
        );
    }

    #[test]
    pub fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 2);
    }

    #[test]
    pub fn test_part2_invalid() {
        assert_eq!(part2(&input_generator(INVALID)), 0);
    }

    #[test]
    pub fn test_part2_valid() {
        assert_eq!(part2(&input_generator(VALID)), 4);
    }
}
