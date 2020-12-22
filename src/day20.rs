use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;
use simple_error::*;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tile {
    id: usize,
    sides: [usize; 8],
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

lazy_static! {
    static ref TILEID: Regex = Regex::new(r"^Tile (\d{4}):$").unwrap();
}

fn to_bin_string(input: &Vec<String>) -> String {
    input
        .iter()
        .cloned()
        .collect::<String>()
        .replace('.', "0")
        .replace('#', "1")
}

impl FromStr for Tile {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Parse out tile ID
        // Convert sides to numbers
        let caps = TILEID.captures(s.lines().next().unwrap());

        if let Some(cap) = caps {
            let tile_id = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();

            let vlines = s
                .lines()
                .skip(1)
                .map(|l| l.chars().map(|c| c.to_string()).collect::<Vec<String>>())
                .collect::<Vec<Vec<String>>>();

            // fl - top line, as-is
            // ll - bottom line, as-is
            // flr - top line, flipped Y
            // llr - bottom line, flipped Y
            let fl = to_bin_string(&vlines[0]);
            let ll = to_bin_string(&vlines[vlines.len() - 1]);
            let flr = fl.chars().rev().collect::<String>();
            let llr = ll.chars().rev().collect::<String>();

            let fl = usize::from_str_radix(fl.as_str(), 2).unwrap();
            let ll = usize::from_str_radix(ll.as_str(), 2).unwrap();
            let flr = usize::from_str_radix(flr.as_str(), 2).unwrap();
            let llr = usize::from_str_radix(llr.as_str(), 2).unwrap();

            // transpose X and Y
            let vlines = transpose(&vlines);

            // flt - left vertical
            // llt - right vertical
            // fltr - left vertical, flipped X
            // lltr - right vertical, flipped Y

            let flt = to_bin_string(&vlines[0]);
            let llt = to_bin_string(&vlines[vlines.len() - 1]);
            let fltr = flt.chars().rev().collect::<String>();
            let lltr = llt.chars().rev().collect::<String>();

            let flt = usize::from_str_radix(flt.as_str(), 2).unwrap();
            let llt = usize::from_str_radix(llt.as_str(), 2).unwrap();
            let fltr = usize::from_str_radix(fltr.as_str(), 2).unwrap();
            let lltr = usize::from_str_radix(lltr.as_str(), 2).unwrap();

            // So the tile in "normal" orientation can be fl -> llt -> ll -> flt
            // Then rotating it 90 degrees clockwise gets to fltr -> fl -> lltr -> ll

            Ok(Tile {
                id: tile_id,
                sides: [fl, ll, flr, llr, flt, llt, fltr, lltr],
            })
        } else {
            bail!("no tile id")
        }
    }
}

#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> Vec<Tile> {
    // Each tile is to be represented by the following:
    // 1. Tile ID
    // 2. 4 numbers, representing 4 sides of the tile, converted from 10-bit binary representation
    // 3. all possible permitations for rotations and flips?
    input
        .split("\n\n")
        .map(|s| Tile::from_str(s).unwrap())
        .collect::<Vec<Tile>>()
}

#[aoc(day20, part1)]
pub fn part1(input: &Vec<Tile>) -> usize {
    // First, find out the size of the side; it is square root of the no of tiles
    // In the example, it is 3, in the input, it is 12

    let side_size = (input.len() as f32).sqrt() as usize;

    let mut hm: HashMap<usize, HashSet<usize>> = HashMap::with_capacity(side_size * side_size * 8);

    input.iter().for_each(|t| {
        for &i in t.sides.iter() {
            if let Some(v) = hm.get_mut(&i) {
                v.insert(t.id);
            } else {
                let mut hs: HashSet<usize> = HashSet::new();
                hs.insert(t.id);
                hm.insert(i, hs);
            }
        }
    });

    hm.iter()
        .filter(|&(_, hs)| hs.len() == 1)
        .fold(HashMap::<usize, usize>::new(), |mut acc, (_, hs)| {
            hs.iter().for_each(|&tile_id| {
                if let Some(counter) = acc.get_mut(&tile_id) {
                    *counter += 1;
                } else {
                    acc.insert(tile_id, 1);
                }
            });
            acc
        })
        .iter()
        .filter(|&(_, &count)| count == 4)
        .map(|(&tile_id, _)| tile_id)
        .product()
}

#[aoc(day20, part2)]
pub fn part2(input: &Vec<Tile>) -> usize {
    // Actually, figure out the sequence of tiles
    // Orient them correctly
    // Cut the borders on each tile (apply mask and shift right 1 bit)
    // Glue the tiles together (i.e, 8 bit numbers in square formation, contatenated - so some math to arrive at complete numbers)
    // Define the forward and reverse patterns for sea monsters
    // Check original and

    input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_GEN: &str = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###";

    const INPUT: &str = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";

    #[test]
    pub fn test_generator() {
        assert_eq!(
            input_generator(INPUT_GEN),
            vec![Tile {
                id: 2311,
                sides: [210, 231, 300, 924, 498, 89, 318, 616],
            }]
        );
    }

    #[test]
    pub fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 20899048083289);
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 273);
    }
}
