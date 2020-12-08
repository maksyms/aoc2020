use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[derive(Clone)]
pub struct Instruction {
    op: String,
    arg: i32,
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| {
            // Split by space, parse out:
            // - operation
            // - signed operand
            // Map onto Instruction structure
            let mut lsp = l.split(" ");
            let op = String::from(lsp.next().unwrap());
            let arg = lsp.next().unwrap().parse::<i32>().unwrap();
            Instruction { op, arg }
        })
        .collect::<Vec<Instruction>>()
}

// This function simulates code execution and returns accumulator and pc at the end
pub fn simulate(input: &Vec<Instruction>) -> (i32, i32) {
    let mut pc: i32 = 0;
    let mut acc: i32 = 0;
    let mut hs = HashSet::new();
    let vl = input.len() as i32;
    while pc >= 0 && pc < vl && !hs.contains(&pc) {
        //eprintln!("PC: {} OP: {} ARG: {} ACC: {}", pc, instr.op, instr.arg, acc);
        let instr = &input[pc as usize];
        hs.insert(pc);
        match instr.op.as_str() {
            "acc" => {
                acc = acc + instr.arg;
                pc = pc + 1
            }
            "jmp" => pc = pc + instr.arg,
            "nop" => pc = pc + 1,
            _ => panic!("Not supposed to happen"),
        };
    }
    // Run a match on operation and act on operand
    (acc, pc)
}

#[aoc(day8, part1)]
pub fn part1(input: &Vec<Instruction>) -> i32 {
    simulate(input).0
}

#[aoc(day8, part2)]
pub fn part2(input: &Vec<Instruction>) -> i32 {
    // Idea: generate a vec of vecs of instructions where only one jmp or nop are altered
    // Then run simulation on a generated vector
    input
        .iter()
        .enumerate()
        .fold(0, |ac, (idx, instr)| match instr.op.as_str() {
            "jmp" => {
                let mut newvec = input.to_vec();
                newvec[idx].op = String::from("nop");
                let (acc, pc) = simulate(&newvec);
                if pc >= newvec.len() as i32 {
                    ac + acc
                } else {
                    ac
                }
            }
            "nop" => {
                let mut newvec = input.to_vec();
                newvec[idx].op = String::from("jmp");
                let (acc, pc) = simulate(&newvec);
                if pc >= newvec.len() as i32 {
                    ac + acc
                } else {
                    ac
                }
            }
            _ => ac,
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    /*     #[test]
       pub fn test_generator() {
           assert_eq!(
               input_generator(INPUT),
               vec![String::from("1"), String::from("2"), String::from("3")]
           );
       }
    */
    #[test]
    pub fn test_part1() {
        assert_eq!(part1(&input_generator(&INPUT)), 5);
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(part2(&input_generator(&INPUT)), 8);
    }
}
