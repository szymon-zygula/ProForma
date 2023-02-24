use num_traits::Float;
use std;

#[derive(Clone)]
pub struct Matrix<T: Float, const M: usize, const N: usize> {
    data: [[T; M]; N],
}

impl<T: Float, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn at(&self, row: usize, col: usize) -> T {
        self.data[col][row]
    }

    pub fn at_mut(&mut self, row: usize, col: usize) -> &mut T {
        &mut self.data[col][row]
    }

    pub fn transpose(&self) -> Matrix<T, M, N> {
        let mut data: [[T; M]; N] = unsafe { std::mem::MaybeUninit::uninit().assume_init() };

        for i in 0..M {
            for j in 0..N {
                data[i][j] = self.data[j][i];
            }
        }

        Matrix::<T, M, N> { data }
    }
}

impl<T: Float> Matrix<T, 1, 1> {
    pub fn num(&self) -> T {
        self.data[0][0]
    }
}

impl<T: Float, const M: usize, const N: usize> std::ops::Add<&Matrix<T, M, N>>
    for &Matrix<T, M, N>
{
    type Output = Matrix<T, M, N>;

    fn add(self, rhs: &Matrix<T, M, N>) -> Self::Output {
        let mut data: [[T; M]; N] = unsafe { std::mem::MaybeUninit::uninit().assume_init() };

        for i in 0..M {
            for j in 0..N {
                data[i][j] = self.data[i][j] + rhs.data[i][j];
            }
        }

        Self::Output { data }
    }
}

impl<T: Float + std::ops::AddAssign<T>, const M: usize, const N: usize, const L: usize>
    std::ops::Mul<&Matrix<T, N, L>> for &Matrix<T, M, N>
{
    type Output = Matrix<T, M, L>;

    fn mul(self, rhs: &Matrix<T, N, L>) -> Matrix<T, M, L> {
        let mut data: [[T; M]; L] = [[T::from(0.0).unwrap(); M]; L];

        for i in 0..M {
            for j in 0..N {
                for k in 0..L {
                    data[i][j] += self.data[k][j] * rhs.data[i][k];
                }
            }
        }

        Matrix { data }
    }
}
