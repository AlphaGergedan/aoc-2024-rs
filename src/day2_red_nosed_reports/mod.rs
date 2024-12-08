mod report_list;
mod input_file_reader;
mod report_validator;

use report_list::ReportList;
use input_file_reader::read_input_file;
pub use report_validator::get_safe_reports_len;

const FILENAME: &str = "data/day2_input.txt";
