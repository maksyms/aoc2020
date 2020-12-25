use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet, VecDeque};

#[aoc_generator(day24)]
pub fn input_generator(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|s| String::from(s))
        .collect::<Vec<String>>()
}

// Lets use cube coordinates for hexagonal grid
// More details are here: https://www.redblobgames.com/grids/hexagons/#coordinates
// Then we map each line of input to a coordinate
// And once arrive there, increment the counter to indicate the number of flips

trait BlackOrWhite {
    fn is_black(&self) -> bool;
    fn is_white(&self) -> bool;
}

impl BlackOrWhite for usize {
    fn is_black(&self) -> bool {
        self % 2 == 1
    }

    fn is_white(&self) -> bool {
        self % 2 == 0
    }
}

pub fn move_coords((x, y, z): (i32, i32, i32), dir: &str) -> (i32, i32, i32) {
    let mut x = x;
    let mut y = y;
    let mut z = z;
    match dir {
        "e" => {
            x += 1;
            y -= 1;
        }
        "w" => {
            x -= 1;
            y += 1;
        }
        "ne" => {
            x += 1;
            z -= 1;
        }
        "nw" => {
            y += 1;
            z -= 1;
        }
        "se" => {
            y -= 1;
            z += 1;
        }
        "sw" => {
            x -= 1;
            z += 1;
        }
        _ => unreachable!("match direction: unknown direction"),
    };
    (x, y, z)
}

pub fn path_to_coords(input: &str) -> (i32, i32, i32) {
    let mut idq = input.chars().collect::<VecDeque<char>>();
    let mut x = 0;
    let mut y = 0;
    let mut z = 0;

    while idq.len() > 0 {
        if let Some(c) = idq.front() {
            // If the char is e or w, then it is single char
            // otherwise, it is double
            let direction = match c {
                'e' | 'w' => idq.pop_front().unwrap().to_string(),
                's' | 'n' => format!(
                    "{}{}",
                    idq.pop_front().unwrap().to_string(),
                    idq.pop_front().unwrap().to_string()
                ),
                _ => unreachable!("direction: unknown direction when parsing input"),
            };
            let t = move_coords((x, y, z), direction.as_str());
            x = t.0;
            y = t.1;
            z = t.2;
        }
    }

    (x, y, z)
}

pub fn figure_tile_count(
    (x, y, z): (i32, i32, i32),
    tiles: &HashMap<(i32, i32, i32), usize>,
) -> (usize, HashSet<(i32, i32, i32)>) {
    let mut new_tiles = HashSet::<(i32, i32, i32)>::new();
    let mut color = 0;
    if let Some(orig_tile) = tiles.get(&(x, y, z)) {
        let orig_tile_is_black = orig_tile.is_black();
        let adjacent_black_count =
            ["e", "w", "ne", "nw", "se", "sw"]
                .iter()
                .fold(0usize, |acc, &dir| {
                    let (xn, yn, zn) = move_coords((x, y, z), dir);
                    if let Some(tile) = tiles.get(&(xn, yn, zn)) {
                        if tile.is_black() {
                            acc + 1
                        } else {
                            acc
                        }
                    } else {
                        new_tiles.insert((xn, yn, zn));
                        acc
                    }
                });
        color = if orig_tile_is_black {
            if adjacent_black_count == 0 || adjacent_black_count > 2 {
                tiles[&(x, y, z)] + 1
            } else {
                tiles[&(x, y, z)]
            }
        } else {
            if adjacent_black_count == 2 {
                tiles[&(x, y, z)] + 1
            } else {
                tiles[&(x, y, z)]
            }
        };
        if !orig_tile_is_black && adjacent_black_count == 0 {
            new_tiles.clear();
        }
    }
    (color, new_tiles)
}

pub fn hexagonal_conway_step(
    input: &HashMap<(i32, i32, i32), usize>,
) -> HashMap<(i32, i32, i32), usize> {
    let mut res = input.clone();
    input.iter().for_each(|(&(x, y, z), _)| {
        let tc = figure_tile_count((x, y, z), input);
        let (new_count, new_tiles) = tc;
        res.insert((x, y, z), new_count);
        // check newly inserted tiles
        let mut new_tiles = new_tiles.clone();
        while new_tiles.len() > 0 {
            new_tiles.clone().iter().for_each(|&(xn, yn, zn)| {
                let tv = figure_tile_count((xn, yn, zn), input);
                new_tiles = tv.1;
                res.insert((xn, yn, zn), tv.0);
            });
        }
    });

    res
}

pub fn count_black_tiles(input: &HashMap<(i32, i32, i32), usize>) -> usize {
    input.values().filter(|&v| v.is_black()).count()
}

#[aoc(day24, part1)]
pub fn part1(input: &Vec<String>) -> usize {
    let tiles = input.iter().map(|l| path_to_coords(l.as_str())).fold(
        HashMap::<(i32, i32, i32), usize>::new(),
        |mut t, (x, y, z)| {
            let counter = t.entry((x, y, z)).or_insert(0);
            *counter += 1;
            t
        },
    );
    count_black_tiles(&tiles)
}

#[aoc(day24, part2)]
pub fn part2(input: &Vec<String>) -> usize {
    let mut tiles = input.iter().map(|l| path_to_coords(l.as_str())).fold(
        HashMap::<(i32, i32, i32), usize>::new(),
        |mut t, (x, y, z)| {
            let counter = t.entry((x, y, z)).or_insert(0);
            *counter += 1;
            t
        },
    );

    // Now I need to insert all the missing tiles, surrounding the existing ones
    let mut tiles_new = tiles.clone();
    tiles.iter().for_each(|(&(x, y, z), _)| {
        ["e", "w", "ne", "nw", "se", "sw"].iter().for_each(|&dir| {
            let (xn, yn, zn) = move_coords((x, y, z), dir);
            tiles_new.entry((xn, yn, zn)).or_insert(0);
        });
    });

    tiles = tiles_new;

    for _ in 0..100 {
        tiles = hexagonal_conway_step(&tiles);
    }

    count_black_tiles(&tiles)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

    #[test]
    pub fn test_generator() {
        assert_eq!(input_generator(INPUT).len(), 20);
    }

    #[test]
    pub fn test_path_function() {
        assert_eq!(path_to_coords("nwwswee"), (0, 0, 0));
    }

    #[test]
    pub fn test_conway_step() {
        let input = input_generator(INPUT);
        let mut tiles = input.iter().map(|l| path_to_coords(l.as_str())).fold(
            HashMap::<(i32, i32, i32), usize>::new(),
            |mut t, (x, y, z)| {
                let counter = t.entry((x, y, z)).or_insert(0);
                *counter += 1;
                t
            },
        );

        let mut tiles_new = tiles.clone();
        tiles.iter().for_each(|(&(x, y, z), _)| {
            ["e", "w", "ne", "nw", "se", "sw"].iter().for_each(|&dir| {
                let (xn, yn, zn) = move_coords((x, y, z), dir);
                tiles_new.entry((xn, yn, zn)).or_insert(0);
            });
        });

        tiles = tiles_new;

        let tiles = hexagonal_conway_step(&tiles);
        assert_eq!(count_black_tiles(&tiles), 15);
        let tiles = hexagonal_conway_step(&tiles);
        assert_eq!(count_black_tiles(&tiles), 12);
        let tiles = hexagonal_conway_step(&tiles);
        assert_eq!(count_black_tiles(&tiles), 25);
    }

    #[test]
    pub fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 10);
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 2208);
    }
}
