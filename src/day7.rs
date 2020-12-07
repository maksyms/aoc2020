use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
//use petgraph::dot::Dot;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::Direction;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

// The regex to extract from input
lazy_static! {
    static ref LINEPARSER: Regex = Regex::new(r"^([a-z ]+) bags contain ([0-9a-z ,]+)\.$").unwrap();
    static ref BAGPARSER: Regex = Regex::new(r"(\d) ([a-z ]+) bag[s]?").unwrap();
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> (Graph<String, usize>, HashMap<String, NodeIndex>) {
    // The input is most representative of a DAG.
    // So it makes sense to parse out input line-by-line and build a representative DAG out of it
    // I have a sneaky suspicion that the number of bags will be important, so it is worth assigning
    // the number as the weight to the edge in a DAG
    // It may be useful to output a HashMap, mapping each node to NodeIndex as well
    let mut res1 = Graph::<String, usize>::new();
    let mut res2 = HashMap::<String, NodeIndex>::new();
    // Parsing input line-by-line, parsing each line into >= 2 nodes and adding edges with the number as the weight
    input.lines().for_each(|l| {
        if let Some(caps) = LINEPARSER.captures(l) {
            // Process each capture, extracting:
            // - b: &str - destination vortex
            // - Vec<(a: &str, usize)> - source vortexes with corresponding edge weights to b
            // Then check res2 for b and each of a and, if not there, add a new node to res1 and store index in res2
            // Then add edges from each of a into b with respective weights
            let b = String::from(caps.get(1).unwrap().as_str());

            let mut b_node_id = res2.get(b.as_str()).copied();

            if b_node_id.is_none() {
                let new_node_id = res1.add_node(b.clone());
                res2.insert(b.clone(), new_node_id);
                b_node_id = Some(new_node_id);
            }

            caps.get(2).unwrap().as_str().split(", ").for_each(|s| {
                if let Some(bags) = BAGPARSER.captures(s) {
                    let num = bags.get(1).unwrap().as_str().parse::<usize>().unwrap();
                    let a = String::from(bags.get(2).unwrap().as_str());
                    let mut a_node_id = res2.get(a.as_str()).copied();
                    if a_node_id.is_none() {
                        let new_node_id = res1.add_node(a.clone());
                        res2.insert(a, new_node_id);
                        a_node_id = Some(new_node_id);
                    }
                    res1.add_edge(a_node_id.unwrap(), b_node_id.unwrap(), num);
                }
            });
        }
    });
    // Useful to visualise the resulting DAGraph
    //eprintln!("{:?}", Dot::with_config(&res1, &[]));
    (res1, res2)
}

fn count_all_containing(g: &Graph<String, usize>, idx: NodeIndex) -> HashSet<NodeIndex> {
    let mut res: HashSet<NodeIndex> = HashSet::new();
    g.neighbors(idx).for_each(|n| {
        res.insert(n);
        res = res.union(&count_all_containing(g, n)).copied().collect();
    });
    res
}

#[aoc(day7, part1)]
pub fn part1(input: &(Graph<String, usize>, HashMap<String, NodeIndex>)) -> usize {
    let (g, hm) = input;
    let bagidx = hm["shiny gold"];
    let res = count_all_containing(g, bagidx);
    res.len()
}

fn count_all_contained(g: &Graph<String, usize>, idx: NodeIndex) -> usize {
    // idea: bags_in_idx = sum_for_neighbours(edge*(1 + count_all_contained(g, neighbour))))
    g.neighbors_directed(idx, Direction::Incoming)
        .fold(0, |res, i| {
            res + g.edges_connecting(i, idx).next().unwrap().weight()
                * (1 + count_all_contained(&g, i))
        })
}

#[aoc(day7, part2)]
pub fn part2(input: &(Graph<String, usize>, HashMap<String, NodeIndex>)) -> usize {
    let (g, hm) = input;
    let bagidx = hm["shiny gold"];
    count_all_contained(&g, bagidx)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vibrant purple bags contain 3 plaid indigo bags, 3 dark gold bags, 3 striped yellow bags, 3 light tomato bags.\ndrab olive bags contain 5 dull blue bags.";
    const INPUT2: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    const INPUT3: &str = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    #[test]
    pub fn test_generator() {
        assert_eq!(true, true);
        let (_graph, _hm) = input_generator(&INPUT);
    }

    #[test]
    pub fn test_part1() {
        assert_eq!(part1(&input_generator(&INPUT2)), 4);
    }

    #[test]
    pub fn test_part2_1() {
        assert_eq!(part2(&input_generator(&INPUT3)), 126);
    }

    #[test]
    pub fn test_part2_2() {
        assert_eq!(part2(&input_generator(&INPUT2)), 32);
    }
}
