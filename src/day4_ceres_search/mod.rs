mod input_reader;
mod xmas_parser;
mod matrix;
mod xmas_letter;

use input_reader::read_file;
use matrix::Matrix;
use xmas_letter::XMASLetter;
pub use xmas_parser::{xmas_search, cross_search};

const FILENAME: &str = "data/day4_input.txt";
