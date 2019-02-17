use docopt::{Docopt, Error};
use serde_derive::Deserialize;

const USAGE: &'static str = include_str!("usage.txt");

#[derive(Debug, Deserialize)]
pub struct Args {
    pub cmd_quotes: bool,
    pub arg_symbol: Vec<String>,
}

pub fn get_args() -> Result<Args, Error> {
    Docopt::new(USAGE).and_then(|d| d.deserialize())
}
