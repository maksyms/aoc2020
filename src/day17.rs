use aoc_runner_derive::{aoc, aoc_generator};
use ndarray::{s, Array2, Array3, Array4};

// Bitmap solution is possible
// But I want to play with ndarrays - never had the chance to do so in Rust
// Perhaps, even rayon and parallel iterators

#[aoc_generator(day17, part1)]
pub fn input_generator(input: &str) -> Array3<usize> {
    let input = input
        .lines()
        .flat_map(|s| {
            s.chars().map(|c| match c {
                '.' => 0,
                '#' => 1,
                _ => panic!("Shouldn't happen."),
            })
        })
        .collect::<Vec<usize>>();

    let arr_size = (input.len() as f32).sqrt() as usize;

    let mut res = Array3::<usize>::zeros((1, arr_size, arr_size));
    let mut slice = res.slice_mut(s![0, 0..arr_size, 0..arr_size]);

    let input = Array2::<usize>::from_shape_vec((arr_size, arr_size), input).unwrap();

    slice.assign(&input);
    res
}

#[aoc_generator(day17, part2)]
pub fn input_generator2(input: &str) -> Array4<usize> {
    let input = input
        .lines()
        .flat_map(|s| {
            s.chars().map(|c| match c {
                '.' => 0,
                '#' => 1,
                _ => panic!("Shouldn't happen."),
            })
        })
        .collect::<Vec<usize>>();

    let arr_size = (input.len() as f32).sqrt() as usize;

    let mut res = Array4::<usize>::zeros((1, 1, arr_size, arr_size));
    let mut slice = res.slice_mut(s![0, 0, 0..arr_size, 0..arr_size]);

    let input = Array2::<usize>::from_shape_vec((arr_size, arr_size), input).unwrap();

    slice.assign(&input);
    res
}

pub fn get_cell_value(input: &Array3<usize>, (z, y, x): (usize, usize, usize)) -> usize {
    // Idea:
    // 1. sum the values of all cells in a cube (x-1, x, x+1; y-1, y, y+1; z-1, z, z+1) and deduct the value of cell at (x, y, z)
    //   - assume that if out of bounds, then the value is 0
    let zmin = if z <= 1 { 0 } else { z - 1 };
    let ymin = if y <= 1 { 0 } else { y - 1 };
    let xmin = if x <= 1 { 0 } else { x - 1 };
    let zmax = if z + 1 >= input.dim().0 {
        input.dim().0 - 1
    } else {
        z + 1
    };
    let ymax = if y + 1 >= input.dim().1 {
        input.dim().1 - 1
    } else {
        y + 1
    };
    let xmax = if x + 1 >= input.dim().2 {
        input.dim().2 - 1
    } else {
        x + 1
    };

    let isl = input.slice(s![zmin..=zmax, ymin..=ymax, xmin..=xmax]);
    let mut active =
        if (zmin..=zmax).contains(&z) && (ymin..=ymax).contains(&y) && (xmin..=xmax).contains(&x) {
            input[(z, y, x)]
        } else {
            0
        };
    let sum = isl.sum() - active;

    // 2. apply the game rules
    if active == 1 {
        if sum < 2 || sum > 3 {
            active = 0;
        }
    } else {
        if sum == 3 {
            active = 1;
        }
    }
    // 3. return the value
    active
}

pub fn step(orig: &Array3<usize>) -> Array3<usize> {
    // Idea:
    // 1. clone orig array into 1 larger one, by 2 on each dimension, 1 on each side, called dest
    // 2. for each cell in dest, get the cell value using orig
    // 3. write the cell value in dest
    // 4. return dest
    let d = orig.dim();
    //eprintln!("dim: {:?}", d);
    let mut ocl = Array3::<usize>::zeros((d.0 + 2, d.1 + 2, d.2 + 2));
    let mut oclsl = ocl.slice_mut(s![1..=d.0, 1..=d.1, 1..=d.2]);
    oclsl.assign(orig);
    let orig = ocl.clone();
    //eprintln!("ocl: {:?}", ocl);
    for ((z, y, x), val) in ocl.indexed_iter_mut() {
        *val = get_cell_value(&orig, (z, y, x));
    }

    ocl
}

#[aoc(day17, part1)]
pub fn part1(input: &Array3<usize>) -> usize {
    let mut t = input.clone();

    for _i in 0..6 {
        //eprintln!("iteration: {} array: {:?}", i, t);
        t = step(&t);
    }

    //eprintln!("FINAL array: {:?}", t);

    t.sum()
}

pub fn get_cell_value2(input: &Array4<usize>, (w, z, y, x): (usize, usize, usize, usize)) -> usize {
    // Idea:
    // 1. sum the values of all cells in a cube (x-1, x, x+1; y-1, y, y+1; z-1, z, z+1) and deduct the value of cell at (x, y, z)
    //   - assume that if out of bounds, then the value is 0
    let wmin = if w <= 1 { 0 } else { w - 1 };
    let zmin = if z <= 1 { 0 } else { z - 1 };
    let ymin = if y <= 1 { 0 } else { y - 1 };
    let xmin = if x <= 1 { 0 } else { x - 1 };
    let wmax = if w + 1 >= input.dim().0 {
        input.dim().0 - 1
    } else {
        w + 1
    };
    let zmax = if z + 1 >= input.dim().1 {
        input.dim().1 - 1
    } else {
        z + 1
    };
    let ymax = if y + 1 >= input.dim().2 {
        input.dim().2 - 1
    } else {
        y + 1
    };
    let xmax = if x + 1 >= input.dim().3 {
        input.dim().3 - 1
    } else {
        x + 1
    };

    let isl = input.slice(s![wmin..=wmax, zmin..=zmax, ymin..=ymax, xmin..=xmax]);
    let mut active = if (wmin..=wmax).contains(&w)
        && (zmin..=zmax).contains(&z)
        && (ymin..=ymax).contains(&y)
        && (xmin..=xmax).contains(&x)
    {
        input[(w, z, y, x)]
    } else {
        0
    };
    let sum = isl.sum() - active;

    // 2. apply the game rules
    if active == 1 {
        if sum < 2 || sum > 3 {
            active = 0;
        }
    } else {
        if sum == 3 {
            active = 1;
        }
    }
    // 3. return the value
    active
}

pub fn step2(orig: &Array4<usize>) -> Array4<usize> {
    // Idea:
    // 1. clone orig array into 1 larger one, by 2 on each dimension, 1 on each side, called dest
    // 2. for each cell in dest, get the cell value using orig
    // 3. write the cell value in dest
    // 4. return dest
    let d = orig.dim();
    //eprintln!("dim: {:?}", d);
    let mut ocl = Array4::<usize>::zeros((d.0 + 2, d.1 + 2, d.2 + 2, d.3 + 2));
    let mut oclsl = ocl.slice_mut(s![1..=d.0, 1..=d.1, 1..=d.2, 1..=d.3]);
    oclsl.assign(orig);
    let orig = ocl.clone();
    //eprintln!("ocl: {:?}", ocl);
    for ((w, z, y, x), val) in ocl.indexed_iter_mut() {
        *val = get_cell_value2(&orig, (w, z, y, x));
    }

    ocl
}

#[aoc(day17, part2)]
pub fn part2(input: &Array4<usize>) -> usize {
    let mut t = input.clone();

    for _i in 0..6 {
        //eprintln!("iteration: {} array: {:?}", i, t);
        t = step2(&t);
    }

    //eprintln!("FINAL array: {:?}", t);

    t.sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::arr3;

    const INPUT: &str = ".#.
..#
###";

    #[test]
    pub fn test_generator() {
        assert_eq!(
            input_generator(INPUT),
            arr3(&[
                [[0, 0, 0], [0, 0, 0], [0, 0, 0]],
                [[0, 1, 0], [0, 0, 1], [1, 1, 1]],
                [[0, 0, 0], [0, 0, 0], [0, 0, 0]]
            ])
        );
    }

    #[test]
    pub fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 112);
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(part2(&input_generator2(INPUT)), 848);
    }
}
