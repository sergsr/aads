use num::integer;

pub fn largest_product_in_a_series(x: usize, digits: Vec<u64>) -> Option<u64> {
  (&digits[..])
    .windows(x)
    .map(|w| w.iter().product::<u64>())
    .max()
}

pub fn largest_product_in_a_grid(grid: Vec<Vec<u32>>) -> Option<u32> {
  if grid.is_empty() || grid.len() < 4 && grid[0].len() < 4 {
    return None;
  }

  let bottom = grid.len() as isize;
  let right = grid[0].len() as isize;
  let directions = [(1, 0), (0, 1), (1, 1), (-1, 1)];

  iproduct!((0..bottom), (0..right), directions.iter())
    .filter(|&(row, col, &(row_step, col_step))|
      0 <= row + 3*row_step && row + 3*row_step < bottom &&
      0 <= col + 3*col_step && col + 3*col_step < right)
    .map(|(row, col, &(row_step, col_step))|
      (0..4)
        .map(|steps| grid[(row + steps*row_step) as usize][(col + steps*col_step) as usize])
        .product())
    .max()
}

pub fn lattice_paths(size: u64) -> u64 {
  integer::binomial(2*size, size)
}

#[cfg(test)]
mod tests {
  use super::*;
  use super::super::raw_inputs;
  use super::super::io;

  #[test]
  fn test_largest_product_in_a_series() {
    assert_eq!(
      largest_product_in_a_series(4, io::digits_from_str(raw_inputs::INPUT_8)),
      Some(5832));
  }

  #[test]
  fn test_lattice_paths() {
    assert_eq!(1, lattice_paths(0));
    assert_eq!(6, lattice_paths(2));
  }
}
