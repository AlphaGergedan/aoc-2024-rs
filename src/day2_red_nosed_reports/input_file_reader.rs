use crate::{Result, Error};
use super::{ReportList, FILENAME};

pub(super) fn read_input_file() -> Result<ReportList> {

    let input_str = std::fs::read_to_string(FILENAME)
        .map_err(|e| Error::CannotReadFile { msg: e.to_string() })?;

    let mut reports: ReportList = Vec::new();
    for line in input_str.lines() {
        let report: Vec<i32> = line
            .split(' ')
            .map(|elem| {
                elem.parse::<i32>()
                    .map_err(|_| Error::CannotParseLine)
            })
            .collect::<Result<Vec<i32>>>()?;

        reports.push(report);
    }

    Ok(reports)
}
