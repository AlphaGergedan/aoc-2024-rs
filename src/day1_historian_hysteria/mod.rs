mod read_input;
mod location_lists;

pub mod total_distance;

use read_input::read_input;
use location_lists::LocationLists;
pub use total_distance::total_distance;


const FILENAME: &str = "data/day1_input.txt";
