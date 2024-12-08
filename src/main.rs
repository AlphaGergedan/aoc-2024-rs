mod day1_historian_hysteria;
mod error;

use error::Error;

type Result<T> = std::result::Result<T, Error>;


fn main() -> Result<()> {
    let total_distance = day1_historian_hysteria::total_distance()?;
    println!("- Day 01: {}\t\t--Total distance", total_distance);

    Ok(())
}
