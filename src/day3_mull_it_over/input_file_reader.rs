use crate::{Error, Result};
use super::FILENAME;

/// For this task we just read the file into string
/// and return it to be processed by the parser.
pub(super) fn read_input_file() -> Result<String> {
    std::fs::read_to_string(FILENAME)
        .map_err(|e| Error::CannotReadFile { msg: e.to_string() } )
}
