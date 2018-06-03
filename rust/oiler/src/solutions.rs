use num::integer;
use rayon::prelude::*;

pub fn largest_product_in_a_series(x: usize, digits: &Vec<u64>) -> Option<u64> {
    (0..digits.len() - x)
        .into_par_iter()
        .map(|i| (&digits[i..i + x]).iter().product())
        .max()
}

fn line_product(old: Option<u32>, step: &Fn(usize) -> u32) -> Option<u32> {
    old.max(Some((0..4).map(step).product()))
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
    if grid.is_empty() {
        return None;
    }

    (0..grid.len())
        .into_par_iter()
        .filter_map(|r| {
            (0..grid[0].len())
                .into_par_iter()
                .filter_map(|c| {
                    let mut result: Option<u32> = None;
                    if c + 3 < grid[0].len() {
                        result = line_product(result, &|d| grid[r][c + d]);
                    }
                    if r + 3 < grid.len() {
                        result = line_product(result, &|d| grid[r + d][c]);
                    }
                    if c + 3 < grid[0].len() && r + 3 < grid.len() {
                        result = line_product(result, &|d| grid[r + d][c + d]);
                    }
                    if 2 < c && r + 3 < grid.len() {
                        result = line_product(result, &|d| grid[r + d][c - d]);
                    }
                    return result;
                })
                .max()
        })
        .max()
}

pub fn lattice_paths(size: u64) -> u64 {
    integer::binomial(2 * size, size)
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
    fn test_largest_product_in_a_grid_anti_diag() {
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
    fn test_largest_product_in_a_grid_empty_rows() {
        let input = Vec::new();
        assert_eq!(largest_product_in_a_grid(&input), None);
    }

    #[test]
    fn test_largest_product_in_a_grid_empty_cols() {
        let input = vec![Vec::new()];
        assert_eq!(largest_product_in_a_grid(&input), None);
    }

    #[test]
    fn test_lattice_paths() {
        assert_eq!(1, lattice_paths(0));
        assert_eq!(6, lattice_paths(2));
    }
}
