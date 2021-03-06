use num::integer;
use num::Integer;
use rayon::prelude::*;
use std;

pub fn largest_product_in_a_series(x: usize, digits: &Vec<u64>) -> Option<u64> {
    (0..digits.len() - x)
        .into_par_iter()
        .map(|i| (&digits[i..i + x]).iter().product())
        .max()
}

pub fn special_pythagorean_triplet(sum: u32) -> Option<u32> {
    // WLOG assume a = min(a, b). (sum - 2a)^2 > c^2 > a^2 => sum > 3a
    (1..sum / 3 + 1)
        .filter_map(|a| {
            (a..sum - a)
                .map(|b| (b, sum - a - b))
                .find(|(b, c)| c * c == a * a + b * b)
                .map(|(b, c)| a * b * c)
        })
        .nth(0)
}

pub fn largest_product_in_a_grid(grid: &Vec<Vec<u32>>) -> Option<u32> {
    (0..grid.len())
        .into_par_iter()
        .filter_map(|r| {
            (0..grid[0].len())
                .into_par_iter()
                .filter_map(|c| {
                    let mut result: Option<u32> = None;
                    if c + 3 < grid[0].len() {
                        result = result.max(Some(
                            grid[r][c] * grid[r][c + 1] * grid[r][c + 2] * grid[r][c + 3],
                        ));
                    }
                    if r + 3 < grid.len() {
                        result = result.max(Some(
                            grid[r][c] * grid[r + 1][c] * grid[r + 2][c] * grid[r + 3][c],
                        ));
                    }
                    if c + 3 < grid[0].len() && r + 3 < grid.len() {
                        result = result.max(Some(
                            grid[r][c]
                                * grid[r + 1][c + 1]
                                * grid[r + 2][c + 2]
                                * grid[r + 3][c + 3],
                        ));
                    }
                    if 2 < c && r + 3 < grid.len() {
                        result = result.max(Some(
                            grid[r][c]
                                * grid[r + 1][c - 1]
                                * grid[r + 2][c - 2]
                                * grid[r + 3][c - 3],
                        ));
                    }
                    return result;
                })
                .max()
        })
        .max()
}

fn collatz_length(seed: u64) -> u64 {
    let mut curr = seed;
    let mut count = 0;
    while curr > 1 {
        if curr.is_even() {
            curr = curr / 2;
        } else {
            curr = 3 * curr + 1;
        }
        count = count + 1;
    }
    count
}

pub fn longest_collatz_sequence(bound: u64) -> Option<usize> {
    (1..bound)
        .map(|x| collatz_length(x))
        .enumerate()
        .max_by_key(|&x| x.1)
        .map(|x| x.0 + 1)
}

pub fn lattice_paths(size: u64) -> u64 {
    integer::binomial(2 * size, size)
}

pub fn maximum_path_sum(rows: &Vec<Vec<u32>>) -> Option<u32> {
    rows.last().and_then(|last| {
        let mut accum = last.to_owned();
        rows.iter().rev().skip(1).for_each(|row| {
            row.iter().enumerate().for_each(|(i, x)| {
                accum[i] = x + std::cmp::max(accum[i], accum[i + 1]);
            })
        });
        accum.first().map(|x| *x)
    })
}

#[cfg(test)]
mod tests {
    use super::super::io;
    use super::*;

    #[test]
    fn test_largest_product_in_a_series() {
        let input = include_str!("inputs/008.txt");
        assert_eq!(
            largest_product_in_a_series(4, &io::digits_from_str(input)),
            Some(5832)
        );
    }

    #[test]
    fn test_special_pythagorean_triplet() {
        assert_eq!(special_pythagorean_triplet(3 + 4 + 5), Some(3 * 4 * 5));
    }

    #[test]
    fn test_largest_product_in_a_grid() {
        let input = Vec::new();
        assert_eq!(largest_product_in_a_grid(&input), None);

        let input = vec![Vec::new()];
        assert_eq!(largest_product_in_a_grid(&input), None);

        let input = io::grid_from_str(
            "
            0 0 0 1
            0 0 1 0
            0 1 0 0
            1 0 0 0
        ",
        );
        assert_eq!(largest_product_in_a_grid(&input), Some(1));
    }

    #[test]
    fn test_longest_collatz_sequence() {
        assert_eq!(longest_collatz_sequence(0), None);
        assert_eq!(longest_collatz_sequence(1), None);
        assert_eq!(longest_collatz_sequence(2), Some(1));
    }

    #[test]
    fn test_lattice_paths() {
        assert_eq!(1, lattice_paths(0));
        assert_eq!(6, lattice_paths(2));
    }

    #[test]
    fn test_maximum_path_sum() {
        assert_eq!(maximum_path_sum(&Vec::new()), None)
    }
}
