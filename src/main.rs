mod day1_historian_hysteria;
mod day2_red_nosed_reports;
mod error;

use day1_historian_hysteria::total_distance;
use day2_red_nosed_reports::get_safe_reports_len;
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

    Ok(())
}
