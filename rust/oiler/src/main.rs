extern crate oiler;
extern crate rayon;

use oiler::io::*;
use oiler::solutions::*;

fn print_solution<T>(number: usize, result: T)
where
    T: std::fmt::Display,
{
    println!("Solution to {}: {}", number, result);
}

fn main() {
    let input8 = include_str!("inputs/008.txt");
    let input8 = digits_from_str(input8);

    let input11 = include_str!("inputs/011.txt");
    let input11 = grid_from_str(input11);

    let input18 = include_str!("inputs/018.txt");
    let input18 = grid_from_str(input18);

    let input67 = include_str!("inputs/067.txt");
    let input67 = grid_from_str(input67);

    print_solution(8, largest_product_in_a_series(13, &input8).unwrap());
    print_solution(9, special_pythagorean_triplet(1000).unwrap());
    print_solution(11, largest_product_in_a_grid(&input11).unwrap());
    print_solution(14, longest_collatz_sequence(1_000_000).unwrap());
    print_solution(15, lattice_paths(20));
    print_solution(18, maximum_path_sum(&input18).unwrap());
    print_solution(67, maximum_path_sum(&input67).unwrap());
}
