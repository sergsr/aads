use std;

fn digits_string_to_vec(digits: &str) -> Vec<u64> {
  return digits.chars()
    .filter(|c| *c != '\n')
    .map(|c| c.to_digit(10).unwrap() as u64)
    .collect::<Vec<_>>();
}

pub fn largest_product_in_a_series(x: usize, digits: &str) -> Option<u64> {
  return (&digits_string_to_vec(digits)[..])
    .windows(x)
    .map(|w| w.iter().product::<u64>())
    .max()
}

fn str_grid_to_vec(grid: &str) -> Vec<Vec<u32>> {
  return grid.split('\n')
    .map(|line| line.split(' '))
    .map(|line| line
      .map(|word| word.parse::<u32>().unwrap())
      .collect())
    .collect();
}

pub fn largest_product_in_a_grid(raw_grid: &str) -> u32 {
  // have this be the or to the max iter..
  let max = std::u32::MIN;
  let grid = str_grid_to_vec(raw_grid);
  //let horizontals = grid
    //.flat_map(|&row| row.windows(4));
  // this one does windows then product / multizip?
  //let verticals =
  // .windows(4).flat_map multizip ?
  for rows in grid.windows(4) {
  }
  return max;
}

#[cfg(test)]
mod tests {
  use super::*;
  use super::super::inputs;

  #[test]
  fn test_largest_product_in_a_series() {
    assert_eq!(
      largest_product_in_a_series(4, inputs::INPUT_8).unwrap(),
      5832
    );
  }
}
