use num::integer;

pub fn largest_product_in_a_series(x: usize, digits: Vec<u64>) -> Option<u64> {
    (&digits[..])
        .windows(x)
        .map(|w| w.iter().product::<u64>())
        .max()
}

pub fn largest_product_in_a_grid(grid: Vec<Vec<u32>>) -> Option<u32> {
    let line_length = 4;

    if grid.is_empty() || grid.len() < line_length && grid[0].len() < line_length {
        return None;
    }

    let step_count = (line_length as isize) - 1;
    let bottom = grid.len() as isize;
    let right = grid[0].len() as isize;
    let directions = [(1, 0), (0, 1), (1, 1), (-1, 1)];

    iproduct!((0..bottom), (0..right), directions.iter())
        .filter(|&(row, col, &(row_step, col_step))| {
            0 <= row + step_count * row_step && row + step_count * row_step < bottom
                && 0 <= col + step_count * col_step
                && col + step_count * col_step < right
        })
        .map(|(row, col, &(row_step, col_step))| {
            (0..step_count + 1)
                .map(|steps| {
                    grid[(row + steps * row_step) as usize][(col + steps * col_step) as usize]
                })
                .product()
        })
        .max()
}

pub fn lattice_paths(size: u64) -> u64 {
    integer::binomial(2 * size, size)
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::io;

    #[test]
    fn test_largest_product_in_a_series() {
        let input = include_str!("inputs/008.txt");
        assert_eq!(
            largest_product_in_a_series(4, io::digits_from_str(input)),
            Some(5832)
        );
    }

    #[test]
    fn test_lattice_paths() {
        assert_eq!(1, lattice_paths(0));
        assert_eq!(6, lattice_paths(2));
    }
}
