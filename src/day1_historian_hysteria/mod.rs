mod input_file_reader;
mod location_lists;
pub mod distance_calculator;

use input_file_reader::read_input;
use location_lists::LocationLists;
pub use distance_calculator::total_distance;

const FILENAME: &str = "data/day1_input.txt";
