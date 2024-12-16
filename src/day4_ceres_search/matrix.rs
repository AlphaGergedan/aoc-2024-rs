use super::XMASLetter;

pub(super) type MatrixEntity = Option<XMASLetter>;
pub(super) type Data = Vec<Vec<MatrixEntity>>;

#[derive(Debug)]
pub(super) struct Matrix {
    pub nrows: usize,
    pub ncols: usize,
    pub data: Data,
}

/// Implements matrix access and altering functions
impl Matrix {
    pub fn new(nrows: usize, ncols: usize) -> Self {
        let mut data = Vec::with_capacity(nrows);
        for _ in 0..nrows {
            data.push(Vec::with_capacity(ncols));
        }
        Self { nrows, ncols, data }
    }

    pub fn get(&self, idx_row: usize, idx_col: usize) -> &MatrixEntity {
        if (idx_row + 1) > self.nrows || (idx_col + 1) > self.ncols { return &None }
        self.data.get(idx_row).unwrap().get(idx_col).unwrap()
    }

    pub fn push_to_row(&mut self, idx_row: usize, val: Option<XMASLetter>) {
        if idx_row >= self.nrows { panic!("out of bounds on Matrix::set") }
        self.data[idx_row].push(val);
    }

}

/// Implements functions for checking the matching "XMAS" in the matrix
impl Matrix {
    /// Check if "SAMX" exists in the matrix at the given 'X' index
    pub fn horizontal_backward(&self, idx_row: usize, idx_col: usize) -> bool {
        if idx_col < 3 {
            false
        } else {
            let predicate_letter_m = self.get(idx_row, idx_col - 1).is_some_and(|c| c == XMASLetter::M);
            let predicate_letter_a = self.get(idx_row, idx_col - 2).is_some_and(|c| c == XMASLetter::A);
            let predicate_letter_s = self.get(idx_row, idx_col - 3).is_some_and(|c| c == XMASLetter::S);
            predicate_letter_m && predicate_letter_a && predicate_letter_s
        }
    }

    /// Check if "XMAS" exists in the matrix at the given 'X' index
    pub fn horizontal_forward(&self, idx_row: usize, idx_col: usize) -> bool {
        if idx_col > self.ncols - 4 {
            false
        } else {
            let predicate_letter_m = self.get(idx_row, idx_col + 1).is_some_and(|c| c == XMASLetter::M);
            let predicate_letter_a = self.get(idx_row, idx_col + 2).is_some_and(|c| c == XMASLetter::A);
            let predicate_letter_s = self.get(idx_row, idx_col + 3).is_some_and(|c| c == XMASLetter::S);
            predicate_letter_m && predicate_letter_a && predicate_letter_s
        }
    }

    /// Check if
    /// """
    /// X
    /// M
    /// A
    /// S
    /// """
    /// exists in the matrix at the given 'X' index
    pub fn vertical_forward(&self, idx_row: usize, idx_col: usize) -> bool {
        if idx_row > self.nrows - 4 {
            false
        } else {
            let predicate_letter_m = self.get(idx_row + 1, idx_col).is_some_and(|c| c == XMASLetter::M);
            let predicate_letter_a = self.get(idx_row + 2, idx_col).is_some_and(|c| c == XMASLetter::A);
            let predicate_letter_s = self.get(idx_row + 3, idx_col).is_some_and(|c| c == XMASLetter::S);
            predicate_letter_m && predicate_letter_a && predicate_letter_s
        }
    }

    /// Check if
    /// """
    /// S
    /// A
    /// M
    /// X
    /// """
    /// exists in the matrix at the given 'X' index
    pub fn vertical_backward(&self, idx_row: usize, idx_col: usize) -> bool {
        if idx_row < 3 {
            false
        } else {
            let predicate_letter_m = self.get(idx_row - 1, idx_col).is_some_and(|c| c == XMASLetter::M);
            let predicate_letter_a = self.get(idx_row - 2, idx_col).is_some_and(|c| c == XMASLetter::A);
            let predicate_letter_s = self.get(idx_row - 3, idx_col).is_some_and(|c| c == XMASLetter::S);
            predicate_letter_m && predicate_letter_a && predicate_letter_s
        }
    }

    /// Check if
    /// """
    /// X
    ///  M
    ///   A
    ///    S
    /// """
    /// exists in the matrix at the given 'X' index
    pub fn diagonal_forward_down(&self, idx_row: usize, idx_col: usize) -> bool {
        if idx_row > self.nrows - 4 || idx_col > self.ncols - 4 {
            false
        } else {
            let predicate_letter_m = self.get(idx_row + 1, idx_col + 1).is_some_and(|c| c == XMASLetter::M);
            let predicate_letter_a = self.get(idx_row + 2, idx_col + 2).is_some_and(|c| c == XMASLetter::A);
            let predicate_letter_s = self.get(idx_row + 3, idx_col + 3).is_some_and(|c| c == XMASLetter::S);
            predicate_letter_m && predicate_letter_a && predicate_letter_s
        }
    }

    /// Check if
    /// """
    /// S
    ///  A
    ///   M
    ///    X
    /// """
    /// exists in the matrix at the given 'X' index
    pub fn diagonal_backward_up(&self, idx_row: usize, idx_col: usize) -> bool {
        if idx_row < 3 || idx_col < 3 {
            false
        } else {
            let predicate_letter_m = self.get(idx_row - 1, idx_col - 1).is_some_and(|c| c == XMASLetter::M);
            let predicate_letter_a = self.get(idx_row - 2, idx_col - 2).is_some_and(|c| c == XMASLetter::A);
            let predicate_letter_s = self.get(idx_row - 3, idx_col - 3).is_some_and(|c| c == XMASLetter::S);
            predicate_letter_m && predicate_letter_a && predicate_letter_s
        }
    }

    /// Check if
    /// """
    ///    X
    ///   M
    ///  A
    /// S
    /// """
    /// exists in the matrix at the given 'X' index
    pub fn diagonal_backward_down(&self, idx_row: usize, idx_col: usize) -> bool {
        if idx_row > self.nrows - 4 || idx_col < 3 {
            false
        } else {
            let predicate_letter_m = self.get(idx_row + 1, idx_col - 1).is_some_and(|c| c == XMASLetter::M);
            let predicate_letter_a = self.get(idx_row + 2, idx_col - 2).is_some_and(|c| c == XMASLetter::A);
            let predicate_letter_s = self.get(idx_row + 3, idx_col - 3).is_some_and(|c| c == XMASLetter::S);
            predicate_letter_m && predicate_letter_a && predicate_letter_s
        }
    }

    /// Check if
    /// """
    ///    S
    ///   A
    ///  M
    /// X
    /// """
    /// exists in the matrix at the given 'X' index
    pub fn diagonal_forward_up(&self, idx_row: usize, idx_col: usize) -> bool {
        if idx_row < 3 || idx_col > self.ncols - 4 {
            false
        } else {
            let predicate_letter_m = self.get(idx_row - 1, idx_col + 1).is_some_and(|c| c == XMASLetter::M);
            let predicate_letter_a = self.get(idx_row - 2, idx_col + 2).is_some_and(|c| c == XMASLetter::A);
            let predicate_letter_s = self.get(idx_row - 3, idx_col + 3).is_some_and(|c| c == XMASLetter::S);
            predicate_letter_m && predicate_letter_a && predicate_letter_s
        }
    }

    /// Returns the number of total "xmas" matches at the given index of 'X'
    pub fn count_xmas(&self, idx_row: usize, idx_col: usize) -> usize {
        // Assume matrix is greater than 3x3 for simplicity, so we do not check that condition
        let xmases = [
            self.horizontal_forward(idx_row, idx_col), self.horizontal_backward(idx_row, idx_col),
            self.vertical_forward(idx_row, idx_col), self.vertical_backward(idx_row, idx_col),
            self.diagonal_forward_up(idx_row, idx_col), self.diagonal_forward_down(idx_row, idx_col),
            self.diagonal_backward_up(idx_row, idx_col), self.diagonal_backward_down(idx_row, idx_col),
        ];
        let count_xmas_entries = xmases.iter().filter(|&&e| e).count();
        count_xmas_entries
    }
}

/// Implements cross XMAS functions
impl Matrix {
    fn upper_right_m_bottom_left_s(&self, idx_row: usize, idx_col: usize) -> bool {
        if let Some(XMASLetter::M) = self.get(idx_row + 1, idx_col + 1) {
            if let Some(XMASLetter::S) = self.get(idx_row - 1, idx_col - 1) {
                true
            } else { false }
        } else { false }
    }

    fn upper_left_m_bottom_right_s(&self, idx_row: usize, idx_col: usize) -> bool {
        if let Some(XMASLetter::M) = self.get(idx_row + 1, idx_col - 1) {
            if let Some(XMASLetter::S) = self.get(idx_row - 1, idx_col + 1) {
                true
            } else { false }
        } else { false }
    }

    fn upper_right_s_bottom_left_m(&self, idx_row: usize, idx_col: usize) -> bool {
        if let Some(XMASLetter::M) = self.get(idx_row - 1, idx_col - 1) {
            if let Some(XMASLetter::S) = self.get(idx_row + 1, idx_col + 1) {
                true
            } else { false }
        } else { false }
    }

    fn upper_left_s_bottom_right_m(&self, idx_row: usize, idx_col: usize) -> bool {
        if let Some(XMASLetter::M) = self.get(idx_row - 1, idx_col + 1) {
            if let Some(XMASLetter::S) = self.get(idx_row + 1, idx_col - 1) {
                true
            } else { false }
        } else { false }
    }

    /// Whether a cross xmas pattern exists in the matrix such that e.g.
    ///
    /// M.S
    /// .A.
    /// M.S
    ///
    /// is found in the matrix where A has the given indices idx_row and idx_col.
    pub fn is_cross_xmas(&self, idx_row: usize, idx_col: usize) -> bool {
        if idx_row == 0 || idx_row == self.nrows - 1 || idx_col == 0 || idx_col == self.ncols - 1 {
            false
        } else {
            if (self.upper_right_m_bottom_left_s(idx_row, idx_col) && self.upper_left_m_bottom_right_s(idx_row, idx_col)) ||
               (self.upper_right_m_bottom_left_s(idx_row, idx_col) && self.upper_left_s_bottom_right_m(idx_row, idx_col)) ||
               (self.upper_right_s_bottom_left_m(idx_row, idx_col) && self.upper_left_s_bottom_right_m(idx_row, idx_col)) ||
               (self.upper_right_s_bottom_left_m(idx_row, idx_col) && self.upper_left_m_bottom_right_s(idx_row, idx_col)) {
                true
            } else {
                false
            }
        }
    }
}

impl From<&String> for Matrix {
    fn from(input: &String) -> Self {
        let data: Data = input
            .trim()
            .lines()
            .map(|line| {
                line.trim().chars()
                    .map(|c| XMASLetter::from(c))
                    .collect()
            })
            .collect();
        let nrows = data.len();
        let ncols = data.first().map_or(0, |row| row.len());

        // We naively assert here, TODO: proper error handling
        for row in &data {
            assert_eq!(row.len(), ncols);
        }
        Self { nrows, ncols, data }
    }
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.data {
            for entry in row {
                match entry {
                    Some(letter) => write!(f, "{} ", letter)?,
                    None => write!(f, ". ")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
