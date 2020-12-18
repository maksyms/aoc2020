use aoc_runner_derive::aoc;
use lazy_static::lazy_static;
use pest::iterators::{Pair, Pairs};
use pest::prec_climber::{Assoc, Operator, PrecClimber};
use pest::Parser;
use pest_derive::*;

// From here: https://pest.rs/book/intro.html
#[derive(Parser)]
#[grammar = "day18.pest"]
pub struct CalcParser;

lazy_static! {
    static ref PREC_CLIMBER_P1: PrecClimber<Rule> = {
        use Assoc::*;
        use Rule::*;

        PrecClimber::new(vec![
            Operator::new(add, Left)
                | Operator::new(subtract, Left)
                | Operator::new(multiply, Left)
                | Operator::new(divide, Left),
            Operator::new(power, Right),
        ])
    };
    static ref PREC_CLIMBER_P2: PrecClimber<Rule> = {
        use Assoc::*;
        use Rule::*;

        PrecClimber::new(vec![
            Operator::new(multiply, Left) | Operator::new(divide, Left),
            Operator::new(add, Left) | Operator::new(subtract, Left),
            Operator::new(power, Right),
        ])
    };
}

fn eval(pc: &PrecClimber<Rule>, expression: Pairs<Rule>) -> f64 {
    pc.climb(
        expression,
        |pair: Pair<Rule>| match pair.as_rule() {
            Rule::num => pair.as_str().parse::<f64>().unwrap(),
            Rule::expr => eval(pc, pair.into_inner()),
            _ => unreachable!(),
        },
        |lhs: f64, op: Pair<Rule>, rhs: f64| match op.as_rule() {
            Rule::add => lhs + rhs,
            Rule::subtract => lhs - rhs,
            Rule::multiply => lhs * rhs,
            Rule::divide => lhs / rhs,
            Rule::power => lhs.powf(rhs),
            _ => unreachable!(),
        },
    )
}

//#[aoc_generator(day18)]
// Commented this out because I didn't want to debug why "aoc" macro was requesting me to provide lifetime specifiers
pub fn input_generator(input: &str) -> Vec<Pairs<Rule>> {
    input
        .lines()
        .map(|s| CalcParser::parse(Rule::calculation, s).unwrap())
        .collect::<Vec<Pairs<Rule>>>()
}

#[aoc(day18, part1)]
pub fn part1(input: &str) -> f64 {
    let input = input
        .lines()
        .map(|s| CalcParser::parse(Rule::calculation, s).unwrap())
        .collect::<Vec<Pairs<Rule>>>();

    input
        .iter()
        .map(|p| eval(&*PREC_CLIMBER_P1, p.clone()))
        .sum()
}

#[aoc(day18, part2)]
pub fn part2(input: &str) -> f64 {
    let input = input
        .lines()
        .map(|s| CalcParser::parse(Rule::calculation, s).unwrap())
        .collect::<Vec<Pairs<Rule>>>();

    input
        .iter()
        .map(|p| eval(&*PREC_CLIMBER_P2, p.clone()))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1 + 2 * 3 + 4 * 5 + 6";
    #[test]
    pub fn test_generator() {}

    #[test]
    pub fn test_part1() {
        assert_eq!(part1(&INPUT), 71.0);
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(part2(&INPUT), 231.0);
    }
}
