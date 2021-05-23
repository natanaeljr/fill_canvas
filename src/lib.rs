use std::fmt::{Display, Formatter};

#[cfg(test)]
mod tests;

/// FillCanvas is a wrap around a matrix with special methods to fill the cells.
///
/// FillCanvas provides methods to fill individual cells with a value,
/// or multiple cells from a row/column or just all cells within an area at once.
///
/// The filled cells have a T value != the Default::default().
///
/// PS: Initially developed to have EntityIDs (from ECS) filled into a matrix
/// based on the space they occupy on a canvas grid.
pub struct FillCanvas<T> {
    rows: usize,
    cols: usize,
    matrix: Vec<T>, // [ rows * cols ]
}

impl<T> FillCanvas<T> where T: Default + Clone + PartialEq {
    fn new(rows: usize, cols: usize) -> Self {
        let mut matrix = Vec::new();
        matrix.resize(rows * cols, T::default());
        Self { rows, cols, matrix }
    }

    pub fn resize(&mut self, rows: usize, cols: usize) {
        self.rows = rows;
        self.cols = cols;
        self.matrix.resize(rows * cols, T::default());
    }

    pub fn clear(&mut self) {
        self.matrix.fill(T::default())
    }

    pub fn get(&self, r: usize, c: usize) -> Option<&T> {
        let index = self.index(r, c)?;
        Some(&self.matrix[index])
    }

    pub fn get_filled(&self, r: usize, c: usize) -> Option<&T> {
        self.get(r, c).and_then(|v| (v != &T::default()).then(|| v))
    }

    pub fn fill_cell(&mut self, v: T, r: usize, c: usize) -> Option<()> {
        let index = self.index(r, c)?;
        self.matrix[index] = v;
        Some(())
    }

    pub fn fill_row(&mut self, v: T, r: usize) -> Option<()> {
        let begin = self.index(r, 0)?;
        let end = self.index(r, self.cols - 1)?;
        for index in begin..=end {
            self.matrix[index] = v.clone();
        }
        Some(())
    }

    pub fn fill_col(&mut self, v: T, c: usize) -> Option<()> {
        self.index(0, c)?; // valvate constrains
        for r in 0..self.rows {
            let index = self.index_unchecked(r, c);
            self.matrix[index] = v.clone();
        }
        Some(())
    }

    pub fn fill_area(&mut self, v: T, r0: usize, c0: usize, r1: usize, c1: usize) -> Option<()> {
        if r1 < r0 || c1 < c0 { return None; } // valvate
        self.index(r0, c0)?; // valvate
        self.index(r1, c1)?; // valvate
        for r in r0..=r1 {
            let begin = self.index_unchecked(r, c0);
            let end = self.index_unchecked(r, c1);
            for index in begin..=end {
                self.matrix[index] = v.clone();
            }
        }
        Some(())
    }

    fn index(&self, r: usize, c: usize) -> Option<usize> {
        let index = self.index_unchecked(r, c);
        if c < self.cols && index < self.matrix.len() { Some(index) } else { None }
    }

    fn index_unchecked(&self, r: usize, c: usize) -> usize {
        self.cols * r + c
    }
}

impl<T> Display for FillCanvas<T> where T: Clone + Default + Display + PartialEq {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("FillCanvas {\n")?;
        for index in 0..self.matrix.len() {
            if index % self.cols == 0 && index != 0 {
                f.write_str("\n")?;
            }
            write!(f, "{} ", self.matrix[index])?;
        }
        f.write_str("\n}")
    }
}