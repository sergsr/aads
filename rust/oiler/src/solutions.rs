use num::integer;
use rayon::prelude::*;

pub fn largest_product_in_a_series(x: usize, digits: &Vec<u64>) -> Option<u64> {
    (0..digits.len() - x)
        .into_par_iter()
        .map(|i| (&digits[i..i + x]).iter().product())
        .max()
}

pub fn largest_product_in_a_grid(grid: &Vec<Vec<u32>>) -> Option<u32> {
    if grid.is_empty() {
        return None;
    }
    let bottom = grid.len();
    let right = grid[0].len();

    (0..bottom)
        .into_par_iter()
        .filter_map(|r| {
            let go_down = r + 3 < bottom;
            (0..right)
                .into_par_iter()
                .filter_map(|c| {
                    let go_right = c + 3 < right;
                    let go_left = 2 < c;
                    let mut result: Option<u32> = None;
                    if go_right {
                        result = result.max(Some((0..4).map(|d| grid[r][c + d]).product()));
                    }
                    if go_down {
                        result = result.max(Some((0..4).map(|d| grid[r + d][c]).product()));
                    }
                    if go_right && go_down {
                        result = result.max(Some((0..4).map(|d| grid[r + d][c + d]).product()));
                    }
                    if go_left && go_down {
                        result = result.max(Some(
                            (0..4)
                                .map(|d| grid[r + d][(c as isize - d as isize) as usize])
                                .product(),
                        ));
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
