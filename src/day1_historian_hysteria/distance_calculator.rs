use crate::Result;
use super::{LocationLists, read_input};


pub fn total_distance() -> Result<i32>{
    let mut location_lists: LocationLists = read_input()?;

    // Sort left and right lists separately
    location_lists.left.sort();
    location_lists.right.sort();

    // Sum all the distances in the sorted lists
    let mut sum: i32 = 0;
    for (left, right) in location_lists {
        let distance = (left - right).abs();
        sum += distance;
    }

    Ok(sum)
}
