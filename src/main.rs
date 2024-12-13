mod day1_historian_hysteria;
mod day2_red_nosed_reports;
mod day3_mull_it_over;

mod error;

use day1_historian_hysteria::total_distance;
use day2_red_nosed_reports::get_safe_reports_len;
use day3_mull_it_over::{get_sum_of_mults, get_sum_of_mults_with_instructions};
use error::Error;

type Result<T> = std::result::Result<T, Error>;


fn main() -> Result<()> {

    // DAY 1
    let total_distance = total_distance()?;
    println!("-- Day 01: Total distance --");
    println!("{total_distance}");

    // DAY 2
    let num_safe_reports_without_tolerance = get_safe_reports_len(false)?;
    let num_safe_reports_with_tolerance = get_safe_reports_len(true)?;
    println!("-- Day 02: Num safe reports --");
    println!("part 1: {num_safe_reports_without_tolerance}");
    println!("part 2: {num_safe_reports_with_tolerance}");

    // DAY 3
    let sum_of_multiplications = get_sum_of_mults()?;
    let sum_mults_with_instructions = get_sum_of_mults_with_instructions()?;
    println!("-- Day 03: Sum of parsed mult. instruction results --");
    println!("part 1: {sum_of_multiplications}");
    println!("part 2: {sum_mults_with_instructions}");

    Ok(())
}
