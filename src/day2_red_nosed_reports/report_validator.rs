use num::Signed;
use crate::Result;
use super::{read_input_file, ReportList};


/// Returns the number of safe reports
///
/// tolerance: For part 2, whether to tolerate a single element or not.
///
pub fn get_safe_reports_len(tolerate: bool) -> Result<u64> {
    let reports: ReportList = read_input_file()?;
    let mut num_safe_reports: u64 = 0;

    // If both holds:
    // 1. The levels are either all increasing or all decreasing.
    // 2. Any two adjacent levels differ by at least one and at most three.
    // Then the report is safe.

    for report in &reports {
        if is_report_safe(&report) {
            num_safe_reports += 1;
        } else {
            // Only check again if we are in the second part (tolerate)
            if tolerate {
                // Test report safety in all subsets of size N and N-1
                let subsets: Vec<Vec<i32>> = subsets_of_size_minus_one(report);
                for subset in subsets {
                    if is_report_safe(&subset) {
                        num_safe_reports += 1;
                        break // Found one safe subset
                    }
                }
            }
        }
    }

    Ok(num_safe_reports)
}

fn is_report_safe(report: &[i32]) -> bool {
    // Trivial case
    if report.len() <= 1 {
        return false;
    }

    // For keeping track of the 1. cond
    // - zero if nothing is compared yet
    // - positive if current element is ascending
    // - negative if current element is descending
    let mut ascending_len = 0;
    let mut previous_element = *report.first().unwrap();

    // Iterate elements starting from the first index
    for &element in &report[1..] {
        let difference = element - previous_element;
        let abs_difference = difference.abs();

        // 2. cond
        if abs_difference < 1 || abs_difference > 3 {
            return false;
        }

        // Ascending
        if difference.is_positive() {
            if ascending_len.is_negative() {
                return false; // 1. cond is violated
            }

            ascending_len += 1;
        } else {
            if ascending_len.is_positive() {
                return false; // 1. cond is violated
            }

            ascending_len -= 1;
        }

        previous_element = element;
    }

    true
}

/// Returns all subsets of lenght N-1 given array length equals N
fn subsets_of_size_minus_one(lst: &[i32]) -> Vec<Vec<i32>> {
    let mut subsets = Vec::new();

    for i in 0..lst.len() {
        let subset: Vec<i32> = lst
            .iter()
            .enumerate()
            .filter_map(|(j, &item)| if i != j { Some(item) } else { None })
            //.filter(|(j, _)| i != *j)
            //.map(|(_, &item)| item)
            .collect();

        subsets.push(subset);
    }

    subsets
}
