use super::{FILENAME, XMASLetter, Matrix};
use crate::{Error, Result};


pub(super) fn read_file() -> Result<Matrix> {
    let file_str = std::fs::read_to_string(FILENAME)
        .map_err(|e| Error::CannotReadFile { msg: e.to_string() })?;

    // Parse input into a matrix where we only store the XMAS letters
    // the input is given in the form of matrix already so we utilize this

    let file_str = &file_str;

    Ok(Matrix::from(file_str))

    // Initialize matrix
    //let rows_str: Vec<&str> = file_str.trim().split('\n').collect();
    //let len_rows: usize = rows_str.len();
    //let len_cols: usize = rows_str[0].len();
    //let mut matrix = Matrix::new(len_rows, len_cols);
    //for (idx_x, line) in file_str.lines().enumerate() {
        //for char in line.chars() {
            //let val = XMASLetter::from(char);
            //matrix.push_to_row(idx_x, val);
        //}
    //}

    // Check correctness on the num. of cols
    //for row in &matrix.data {
        //assert_eq!(row.len(), len_cols);
    //}
    //Ok(matrix)
}
