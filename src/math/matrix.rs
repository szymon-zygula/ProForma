use num_traits::Float;
use std;

#[derive(Clone)]
pub struct Matrix<T: Float, const M: usize, const N: usize> {
    data: [[T; M]; N],
}

impl<T: Float, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn zero() -> Matrix<T, M, N> {
        let data: [[T; M]; N] = [[T::from(0.0).unwrap(); M]; N];
        Matrix::<T, M, N> { data }
    }

    pub unsafe fn uninit() -> Matrix<T, M, N> {
        let data: [[T; M]; N] = std::mem::MaybeUninit::uninit().assume_init();
        Matrix::<T, M, N> { data }
    }

    pub fn at(&self, row: usize, col: usize) -> T {
        self.data[col][row]
    }

    pub fn at_mut(&mut self, row: usize, col: usize) -> &mut T {
        &mut self.data[col][row]
    }

    pub fn transpose(&self) -> Matrix<T, N, M> {
        let mut result = unsafe { Matrix::<T, N, M>::uninit() };

        for i in 0..M {
            for j in 0..N {
                result.data[i][j] = self.data[j][i];
            }
        }

        result
    }
}

impl<T: Float, const M: usize> Matrix<T, M, M> {
    pub fn diagonal(diagonal_values: &[T; M]) -> Matrix<T, M, M> {
        let mut result = Matrix::<T, M, M>::zero();

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

        for i in 0..M {
            for j in 0..N {
                result.data[i][j] = self.data[i][j] + rhs.data[i][j];
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
            for j in 0..N {
                for k in 0..L {
                    result.data[i][j] += self.data[k][j] * rhs.data[i][k];
                }
            }
        }

        result
    }
}
