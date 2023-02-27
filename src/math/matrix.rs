use num_traits::Float;
use std;

#[derive(Clone, Copy, Debug)]
pub struct Matrix<T: Float, const M: usize, const N: usize> {
    data: [[T; N]; M],
}

impl<T: Float + Copy, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn zero() -> Matrix<T, M, N> {
        let data = [[T::from(0.0).unwrap(); N]; M];
        Matrix::<T, M, N> { data }
    }

    pub unsafe fn uninit() -> Matrix<T, M, N> {
        let data = std::mem::MaybeUninit::uninit().assume_init();
        Matrix::<T, M, N> { data }
    }

    pub fn from_data(data: [[T; N]; M]) -> Matrix<T, M, N> {
        Matrix::<T, M, N> { data }
    }

    pub fn raw(&self) -> &[T] {
        // TODO: use `slice::flatten` if it ever gets into stable
        unsafe { std::slice::from_raw_parts(self.data.as_ptr().cast(), N * M) }
    }

    pub fn at(&self, row: usize, col: usize) -> T {
        self.data[row][col]
    }

    pub fn at_mut(&mut self, row: usize, col: usize) -> &mut T {
        &mut self.data[row][col]
    }

    pub fn transpose(&self) -> Matrix<T, N, M> {
        let mut result = unsafe { Matrix::uninit() };

        for row in 0..M {
            for col in 0..N {
                result.data[col][row] = self.data[row][col];
            }
        }

        result
    }

    pub fn with_type<U: Float>(&self) -> Matrix<U, M, N> {
        let mut result = unsafe { Matrix::<U, M, N>::uninit() };

        for row in 0..M {
            for col in 0..N {
                result.data[row][col] = U::from(self.data[row][col]).unwrap();
            }
        }

        result
    }

    pub fn swap_rows(&mut self, row1: usize, row2: usize) {
        let copy = self.data[row1];
        self.data[row1] = self.data[row2];
        self.data[row2] = copy;
    }

    pub fn add_row1_to_row2(&mut self, row1: usize, row2: usize, multiplier: T) {
        for col in 0..N {
            self.data[row2][col] = self.data[row2][col] + multiplier * self.data[row1][col];
        }
    }

    pub fn mutiply_row(&mut self, row: usize, multiplier: T) {
        for col in 0..N {
            self.data[row][col] = multiplier * self.data[row][col];
        }
    }

    pub fn apply_row_ops(&mut self, row_ops: &[RowOp<T>]) {
        for op in row_ops {
            match op {
                RowOp::Swap(row1, row2) => self.swap_rows(*row1, *row2),
                RowOp::Add(row1, row2, mul) => self.add_row1_to_row2(*row1, *row2, *mul),
                RowOp::Mul(row, mul) => self.mutiply_row(*row, *mul),
            }
        }
    }

    pub fn gaussian_elimination_with_partial_pivoting(
        &self,
    ) -> Option<(Matrix<T, M, N>, Vec<RowOp<T>>)> {
        let mut gepp_matrix = *self;
        let mut operations = Vec::with_capacity(M * N);

        for i in 0..std::cmp::min(M, N) {
            let mut pivot = i;

            while gepp_matrix.data[i][pivot] == T::from(0.0).unwrap() {
                pivot += 1;
            }

            if pivot == N {
                return None;
            }

            if pivot != i {
                operations.push(RowOp::Swap(i, pivot));
                gepp_matrix.swap_rows(i, pivot);
            }

            let multiplier = T::from(1.0).unwrap() / gepp_matrix.data[i][i];
            operations.push(RowOp::Mul(i, multiplier));
            gepp_matrix.mutiply_row(i, multiplier);

            for row in (i + 1)..M {
                let multiplier = -gepp_matrix.data[row][i];
                operations.push(RowOp::Add(i, row, multiplier));
                gepp_matrix.add_row1_to_row2(i, row, multiplier);
            }
        }

        Some((gepp_matrix, operations))
    }
}

impl<T: Float, const M: usize> Matrix<T, M, M> {
    pub fn identity() -> Matrix<T, M, M> {
        Self::diagonal(&[T::from(1.0).unwrap(); M])
    }

    pub fn diagonal(diagonal_values: &[T; M]) -> Matrix<T, M, M> {
        let mut result = Matrix::zero();

        for i in 0..M {
            result.data[i][i] = diagonal_values[i];
        }

        result
    }

    pub fn diagonalization_of_gaussed(&self) -> Vec<RowOp<T>> {
        let mut diagonal = *self;
        let mut row_ops = Vec::new();

        for i in (0..M).rev() {
            for j in 0..i {
                let multiplier = -diagonal.data[j][i];
                row_ops.push(RowOp::Add(i, j, multiplier));
                diagonal.add_row1_to_row2(i, j, multiplier);
            }
        }

        row_ops
    }

    pub fn solve_linear_system<const L: usize>(
        &self,
        mut constant_terms: Matrix<T, M, L>,
    ) -> Option<Matrix<T, M, L>> {
        let (upper_triangular, gauss_ops) = self.gaussian_elimination_with_partial_pivoting()?;
        let solution_ops = upper_triangular.diagonalization_of_gaussed();

        constant_terms.apply_row_ops(&gauss_ops);
        constant_terms.apply_row_ops(&solution_ops);
        Some(constant_terms)
    }

    pub fn inverse(&self) -> Option<Matrix<T, M, M>> {
        self.solve_linear_system(Matrix::<T, M, M>::identity())
    }
}

impl<T: Float> Matrix<T, 1, 1> {
    pub fn num(&self) -> T {
        self.data[0][0]
    }
}

impl<T: Float, const M: usize, const N: usize> std::ops::Add<Matrix<T, M, N>> for Matrix<T, M, N> {
    type Output = Matrix<T, M, N>;

    fn add(self, rhs: Matrix<T, M, N>) -> Self::Output {
        let mut result = unsafe { Self::Output::uninit() };

        for row in 0..M {
            for col in 0..N {
                result.data[row][col] = self.data[row][col] + rhs.data[row][col];
            }
        }

        result
    }
}

impl<T: Float + std::ops::AddAssign<T>, const M: usize, const N: usize, const L: usize>
    std::ops::Mul<Matrix<T, N, L>> for Matrix<T, M, N>
{
    type Output = Matrix<T, M, L>;

    fn mul(self, rhs: Matrix<T, N, L>) -> Self::Output {
        let mut result = Self::Output::zero();

        for i in 0..M {
            for j in 0..L {
                for k in 0..N {
                    result.data[i][j] += self.data[i][k] * rhs.data[k][j];
                }
            }
        }

        result
    }
}

impl<T, const M: usize, const N: usize> std::fmt::Display for Matrix<T, M, N>
where
    T: Float + std::fmt::Display,
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        let write_row = |row: usize, formatter: &mut std::fmt::Formatter| -> std::fmt::Result {
            write!(formatter, "[")?;
            for col in 0..(N - 1) {
                write!(formatter, "{}, ", self.data[row][col])?;
            }

            write!(formatter, "{}]", self.data[row][N - 1])?;
            Ok(())
        };

        write!(formatter, "[")?;

        for row in 0..(M - 1) {
            write_row(row, formatter)?;
            write!(formatter, "\n ")?;
        }

        write_row(M - 1, formatter)?;
        write!(formatter, "]\n")?;

        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
pub enum RowOp<T: Float> {
    Swap(usize, usize),
    Add(usize, usize, T),
    Mul(usize, T),
}
