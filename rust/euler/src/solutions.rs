use num::integer;

pub fn largest_product_in_a_series(x: usize, digits: Vec<u64>) -> Option<u64> {
  (&digits[..])
    .windows(x)
    .map(|w| w.iter().product::<u64>())
    .max()
}

fn contains_end(grid: &Vec<Vec<u32>>, r: i32, c: i32, dr: i32, dc: i32) -> bool {
  0 <= r + 3*dr && r + 3*dr < grid.len() as i32 && 0 <= c + 3*dc && c + 3*dc < grid[0].len() as i32
}

fn line_product(grid: &Vec<Vec<u32>>, r: i32, c: i32, dr: i32, dc: i32) -> u32 {
  (0..4).map(|d| grid[(r + d*dr) as usize][(c + d*dc) as usize]).product()
}

pub fn largest_product_in_a_grid(grid: Vec<Vec<u32>>) -> Option<u32> {
  if grid.is_empty() || grid.len() < 4 && grid[0].len() < 4 {
    return None;
  }
  let directions = [(1, 0), (0, 1), (1, 1), (-1, 1)];
  iproduct!(0..(grid.len() as i32), 0..(grid[0].len() as i32), directions.iter())
    .filter(|&(r, c, &(dr, dc))| contains_end(&grid, r, c, dr, dc))
    .map(|(r, c, &(dr, dc))| line_product(&grid, r, c, dr, dc))
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
