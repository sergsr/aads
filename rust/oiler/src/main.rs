extern crate oiler;

use oiler::io;
use oiler::solutions;

fn main() {
    let input = include_str!("inputs/008.txt");
    println!(
        "Solution to 8: {}",
        solutions::largest_product_in_a_series(13, io::digits_from_str(input)).unwrap()
    );
    let input = include_str!("inputs/011.txt");
    println!(
        "Solution to 11: {}",
        solutions::largest_product_in_a_grid(io::grid_from_str(input)).unwrap()
    );
    println!("Solution to 15: {}", solutions::lattice_paths(20));
}
