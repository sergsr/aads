use std;

pub fn largest_product_in_a_series(x: usize, digits: Vec<u64>) -> Option<u64> {
  return (&digits[..])
    .windows(x)
    .map(|w| w.iter().product::<u64>())
    .max()
}

pub fn largest_product_in_a_grid(grid: Vec<Vec<u32>>) -> Option<u32> {
  //let grid = str_grid_to_vec(raw_grid);
  //let horizontals = grid
    //.flat_map(|&row| row.windows(4));
  // this one does windows then product / multizip?
  //let verticals =
  // .windows(4).flat_map multizip ?
  /*
    return horizontals
      .chain(verticals)
      .chain(diagonals)
      .chain(other_diagonals)
      .product() //
      .max()
  */
  return Some(std::u32::MIN);
}

#[cfg(test)]
mod tests {
  use super::*;
  use super::super::raw_inputs;
  use super::super::io;

  #[test]
  fn test_largest_product_in_a_series() {
    assert_eq!(
      largest_product_in_a_series(0, io::digits_from_str(raw_inputs::INPUT_8)),
      None);
    assert_eq!(
      largest_product_in_a_series(4, io::digits_from_str(raw_inputs::INPUT_8)),
      Some(5832));
  }
}
