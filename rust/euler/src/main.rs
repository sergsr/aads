extern crate euler;

#[macro_use] extern crate indoc;

use euler::inputs;
use euler::solutions;

fn main() {
  println!("{}",
    solutions::largest_product_in_a_series(13, inputs::INPUT_8).unwrap());
}
