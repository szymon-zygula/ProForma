use num_traits::Float;
use std;

#[derive(Clone, Copy, Debug)]
pub struct Matrix<T: Float, const M: usize, const N: usize> {
    data: [[T; N]; M],
}

impl<T: Float, const M: usize, const N: usize> Matrix<T, M, N> {
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
