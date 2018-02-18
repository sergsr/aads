pub fn digits_from_str(digits: &str) -> Vec<u64> {
  return digits.chars()
    .filter(|c| *c != '\n')
    .map(|c| c.to_digit(10).unwrap() as u64)
    .collect::<Vec<_>>();
}

pub fn grid_from_str(grid: &str) -> Vec<Vec<u32>> {
  return grid.split('\n')
    .map(|line| line.split(' '))
    .map(|line| line
      .map(|word| word.parse::<u32>().unwrap())
      .collect())
    .collect();
}

// TODO: io_wrapper: closures str -> str for raw input -> answer
