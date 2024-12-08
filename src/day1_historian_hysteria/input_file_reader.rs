use crate::{Error, Result};
use super::{LocationLists, FILENAME};


pub(super) fn read_input() -> Result<LocationLists> {

    let lists = std::fs::read_to_string(FILENAME)
        .map_err(|e| Error::CannotReadFile { msg: e.to_string() } )?;

    // Read line by line into these lists
    let mut left = Vec::<i32>::new();
    let mut right = Vec::<i32>::new();

    for line in lists.lines() {
        let pair: Vec<&str> = line.split("   ").collect();
        assert!(pair.len() == 2);

        let first_element = pair[0];
        let second_element = pair[1];

        let first_element: i32 = first_element.parse()
            .map_err(|_| Error::CannotParseLine)?;

        let second_element: i32 = second_element.parse()
            .map_err(|_| Error::CannotParseLine)?;

        left.push(first_element);
        right.push(second_element);
    }

    let both_lists_together = LocationLists { left, right };
    Ok(both_lists_together)
}
