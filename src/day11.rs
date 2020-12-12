use std::mem::swap;

use aoc_runner_derive::{aoc, aoc_generator};
use array2d::Array2D;

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Array2D<char> {
    Array2D::from_rows(
        &input
            .lines()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>(),
    )
}

pub fn get_part1_value(a: &Array2D<char>, (row, col): (usize, usize)) -> char {
    let mut count: u32 = 0;
    let orig = a[(row, col)];
    for &row_temp in [row.wrapping_sub(1), row, row + 1].iter() {
        for &col_temp in [col.wrapping_sub(1), col, col + 1].iter() {
            if (row_temp, col_temp) != (row, col) {
                if let Some(el) = a.get(row_temp, col_temp) {
                    if el == &'X' {
                        count += 1;
                    }
                }
            }
        }
    }
    match orig {
        'L' => {
            if count == 0 {
                'X'
            } else {
                'L'
            }
        }
        'X' => {
            if count >= 4 {
                'L'
            } else {
                'X'
            }
        }
        _ => orig,
    }
}

#[aoc(day11, part1)]
pub fn part1(input: &Array2D<char>) -> usize {
    let mut source = input.clone();
    let mut dest = Array2D::filled_with('.', input.num_rows(), input.num_columns());

    while source != dest {
        for row_idx in 0..source.num_rows() {
            for col_idx in 0..source.num_columns() {
                dest[(row_idx, col_idx)] = get_part1_value(&source, (row_idx, col_idx));
            }
        }
        swap(&mut source, &mut dest);
    }
    dest.elements_row_major_iter()
        .filter(|&&c| c == 'X')
        .count()
}

pub fn get_part2_value(a: &Array2D<char>, (row, col): (usize, usize)) -> char {
    let mut count: u32 = 0;
    let orig = a[(row, col)];
    // We need to check 4 lines:
    // 1. Horizontal, going across the point
    // 2. Vertical, going across the point
    // 3. Down diagonal
    // 4. Up diagonal
    // All of them must be split in two parts
    for row_temp in (0..row).rev() {
        if let Some(el) = a.get(row_temp, col) {
            match el {
                &'X' => {
                    count += 1;
                    break;
                }
                &'L' => break,
                _ => continue,
            }
        }
    }
    for row_temp in row + 1..a.num_rows() {
        if let Some(el) = a.get(row_temp, col) {
            match el {
                &'X' => {
                    count += 1;
                    break;
                }
                &'L' => break,
                _ => continue,
            }
        }
    }
    for col_temp in (0..col).rev() {
        // Lala
        if let Some(el) = a.get(row, col_temp) {
            match el {
                &'X' => {
                    count += 1;
                    break;
                }
                &'L' => break,
                _ => continue,
            }
        }
    }
    for col_temp in col + 1..a.num_columns() {
        // Lala
        if let Some(el) = a.get(row, col_temp) {
            match el {
                &'X' => {
                    count += 1;
                    break;
                }
                &'L' => break,
                _ => continue,
            }
        }
    }

    let mut row_temp = row;
    let mut col_temp = col;

    while row_temp < a.num_rows() && col_temp < a.num_columns() {
        row_temp = row_temp.wrapping_sub(1);
        col_temp = col_temp.wrapping_sub(1);
        if let Some(el) = a.get(row_temp, col_temp) {
            match el {
                &'X' => {
                    count += 1;
                    break;
                }
                &'L' => break,
                _ => continue,
            }
        }
    }

    row_temp = row;
    col_temp = col;

    while row_temp < a.num_rows() && col_temp < a.num_columns() {
        row_temp = row_temp.wrapping_add(1);
        col_temp = col_temp.wrapping_add(1);
        if let Some(el) = a.get(row_temp, col_temp) {
            match el {
                &'X' => {
                    count += 1;
                    break;
                }
                &'L' => break,
                _ => continue,
            }
        }
    }

    row_temp = row;
    col_temp = col;

    while row_temp < a.num_rows() && col_temp < a.num_columns() {
        row_temp = row_temp.wrapping_sub(1);
        col_temp = col_temp.wrapping_add(1);
        if let Some(el) = a.get(row_temp, col_temp) {
            match el {
                &'X' => {
                    count += 1;
                    break;
                }
                &'L' => break,
                _ => continue,
            }
        }
    }

    row_temp = row;
    col_temp = col;

    while row_temp < a.num_rows() && col_temp < a.num_columns() {
        row_temp = row_temp.wrapping_add(1);
        col_temp = col_temp.wrapping_sub(1);
        if let Some(el) = a.get(row_temp, col_temp) {
            match el {
                &'X' => {
                    count += 1;
                    break;
                }
                &'L' => break,
                _ => continue,
            }
        }
    }

    match orig {
        'L' => {
            if count == 0 {
                'X'
            } else {
                'L'
            }
        }
        'X' => {
            if count >= 5 {
                'L'
            } else {
                'X'
            }
        }
        _ => orig,
    }
}

#[aoc(day11, part2)]
pub fn part2(input: &Array2D<char>) -> usize {
    let mut source = input.clone();
    let mut dest = Array2D::filled_with('.', input.num_rows(), input.num_columns());

    while source != dest {
        for row_idx in 0..source.num_rows() {
            for col_idx in 0..source.num_columns() {
                dest[(row_idx, col_idx)] = get_part2_value(&source, (row_idx, col_idx));
            }
        }
        swap(&mut source, &mut dest);
    }
    dest.elements_row_major_iter()
        .filter(|&&c| c == 'X')
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    pub fn test_generator() {
        assert_eq!(true, true);
    }

    #[test]
    pub fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 37);
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 26);
    }
}
