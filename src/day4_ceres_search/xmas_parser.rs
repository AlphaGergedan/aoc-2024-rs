use super::{XMASLetter, read_file, Matrix};
use crate::Result;


pub fn xmas_search() -> Result<usize> {
    let matrix: Matrix = read_file()?;
    let mut count_xmas: usize = 0;
    for idx_row in 0..matrix.nrows {
        for idx_col in 0..matrix.ncols {
            if let Some(XMASLetter::X) = matrix.get(idx_row, idx_col) {
                count_xmas += matrix.count_xmas(idx_row, idx_col);
            }
        }
    }
    Ok(count_xmas)
}

pub fn cross_search() -> Result<usize> {
    let matrix: Matrix = read_file()?;
    let mut count_cross: usize = 0;
    for idx_row in 0..matrix.nrows {
        for idx_col in 0..matrix.ncols {
            if let Some(XMASLetter::A) = matrix.get(idx_row, idx_col) {
                if matrix.is_cross_xmas(idx_row, idx_col) {
                    count_cross += 1;
                }
            }
        }
    }
    Ok(count_cross)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_horizontal_forward() {
        let data = "
            XMAS
        ".to_string();
        let matrix = Matrix::from(&data);
        assert!(matrix.horizontal_forward(0, 0));
    }

    #[test]
    fn test_horizontal_backward() {
        let data = "
            SAMX
        ".to_string();
        let matrix = Matrix::from(&data);
        assert!(matrix.horizontal_backward(0, 3));
    }

    #[test]
    fn test_vertical_forward() {
        let data = "
            X
            M
            A
            S
        ".to_string();
        let matrix = Matrix::from(&data);
        assert!(matrix.vertical_forward(0, 0));
    }

    #[test]
    fn test_vertical_backward() {
        let data = "
            S
            A
            M
            X
        ".to_string();
        let matrix = Matrix::from(&data);
        assert!(matrix.vertical_backward(3, 0));
    }

    #[test]
    fn test_diagonal_forward_down() {
        let data = "
            X...
            .M..
            ..A.
            ...S
        ".to_string();
        let matrix = Matrix::from(&data);
        assert!(matrix.diagonal_forward_down(0, 0));
    }

    #[test]
    fn test_diagonal_forward_up() {
        let data = "
            ...S
            ..A.
            .M..
            X...
        ".to_string();
        let matrix = Matrix::from(&data);
        assert!(matrix.diagonal_forward_up(3, 0));
    }

    #[test]
    fn test_diagonal_backward_up() {
        let data = "
            S...
            .A..
            ..M.
            ...X
        ".to_string();
        let matrix = Matrix::from(&data);
        assert!(matrix.diagonal_backward_up(3, 3));
    }

    #[test]
    fn test_diagonal_backward_down() {
        let data = "
            ...X
            ..M.
            .A..
            S...
        ".to_string();
        let matrix = Matrix::from(&data);
        assert!(matrix.diagonal_backward_down(0, 3));
    }

    #[test]
    fn test_example_1() {
        let data = "
            MMMSXXMASM
            MSAMXMSMSA
            AMXSXMAAMM
            MSAMASMSMX
            XMASAMXAMM
            XXAMMXXAMA
            SMSMSASXSS
            SAXAMASAAA
            MAMMMXMMMM
            MXMXAXMASX
        ".to_string();
        let matrix = Matrix::from(&data);
        let mut count_xmas: usize = 0;
        for idx_row in 0..matrix.nrows {
            for idx_col in 0..matrix.ncols {
                if let Some(XMASLetter::X) = matrix.get(idx_row, idx_col) {
                    count_xmas += matrix.count_xmas(idx_row, idx_col);
                }
            }
        }
        assert_eq!(count_xmas, 18);
    }

    #[test]
    fn test_cross_xmas() {
        let data = "
            M.S
            .A.
            M.S
        ".to_string();
        let matrix = Matrix::from(&data);
        let mut count_cross: usize = 0;
        for idx_row in 0..matrix.nrows {
            for idx_col in 0..matrix.ncols {
                if let Some(XMASLetter::A) = matrix.get(idx_row, idx_col) {
                    if matrix.is_cross_xmas(idx_row, idx_col) {
                        count_cross += 1;
                    }
                }
            }
        }
        assert_eq!(count_cross, 1);
    }

    #[test]
    fn test_cross_xmas_bigger_input() {
        let data = "
            .M.S......
            ..A..MSMS.
            .M.S.MAA..
            ..A.ASMSM.
            .M.S.M....
            ..........
            S.S.S.S.S.
            .A.A.A.A..
            M.M.M.M.M.
            ..........
        ".to_string();
        let matrix = Matrix::from(&data);
        let mut count_cross: usize = 0;
        for idx_row in 0..matrix.nrows {
            for idx_col in 0..matrix.ncols {
                if let Some(XMASLetter::A) = matrix.get(idx_row, idx_col) {
                    if matrix.is_cross_xmas(idx_row, idx_col) {
                        count_cross += 1;
                    }
                }
            }
        }
        assert_eq!(count_cross, 9);
    }
}
