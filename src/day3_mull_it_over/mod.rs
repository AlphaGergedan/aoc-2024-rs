mod input_file_reader;
mod mult_parser;

use input_file_reader::read_input_file;
pub use mult_parser::{get_sum_of_mults, get_sum_of_mults_regex, get_sum_of_mults_with_instructions};

const FILENAME: &str = "data/day3_input.txt";
