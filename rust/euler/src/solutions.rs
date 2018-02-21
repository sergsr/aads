use std;

pub fn largest_product_in_a_series(x: usize, digits: Vec<u64>) -> Option<u64> {
  (&digits[..])
    .windows(x)
    .map(|w| w.iter().product::<u64>())
    .max()
}

pub fn largest_product_in_a_grid(grid: Vec<Vec<u32>>) -> u32 {
  /* the following would be great once i figure out type issues
   * and something elegant for diagonals
  let horizontals = grid.iter()
    .map(|row| row.windows(4))
  let verticals = (&grid[..])
    .windows(4)
    .map(|rows| rows.iter()
      .map(|row| row.iter()));
  horizontals//.chain(verticals)
    .product::<u32>()
    .max::<u32>()

  and return an Option<u32>
  */
  let mut result = 0;
  let m = grid.len();
  let n = grid[0].len();
  for r in 0..m {
    for c in 0..n {
      let bottom_margin = r < m - 3;
      let right_margin = c < n - 3;
      let left_margin = 3 < c;
      if right_margin {
        result = std::cmp::max(result,
          grid[r][c] * grid[r][c + 1] * grid[r][c + 2] * grid[r][c + 3]);
      }
      if bottom_margin {
        result = std::cmp::max(result,
          grid[r][c] * grid[r + 1][c] * grid[r + 2][c] * grid[r + 3][c]);
      }
      if right_margin && bottom_margin {
        result = std::cmp::max(result,
          grid[r][c] * grid[r + 1][c + 1] * grid[r + 2][c + 2] * grid[r + 3][c + 3]);
      }
      if left_margin && bottom_margin {
        result = std::cmp::max(result,
          grid[r][c] * grid[r + 1][c - 1] * grid[r + 2][c - 2] * grid[r + 3][c - 3]);
      }
    }
  }
  result
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
}
