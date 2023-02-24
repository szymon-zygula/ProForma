use std;

#[derive(Clone)]
pub struct Matrix<const M: usize, const N: usize> {
    data: [[f32; M]; N],
}

impl<const M: usize, const N: usize> Matrix<M, N> {
    pub fn at(&self, row: usize, col: usize) -> f32 {
        self.data[col][row]
    }

    pub fn at_mut(&mut self, row: usize, col: usize) -> &mut f32 {
        &mut self.data[col][row]
    }

    pub fn transpose(&self) -> Matrix<M, N> {
        let mut data: [[f32; M]; N] = unsafe { std::mem::MaybeUninit::uninit().assume_init() };

        for i in 0..M {
            for j in 0..N {
                data[i][j] = self.data[j][i];
            }
        }

        Matrix::<M, N> { data }
    }
}

impl Matrix<1, 1> {
    pub fn num(&self) -> f32 {
        self.data[0][0]
    }
}

impl<const M: usize, const N: usize> std::ops::Add<&Matrix<M, N>> for &Matrix<M, N> {
    type Output = Matrix<M, N>;

    fn add(self, rhs: &Matrix<M, N>) -> Self::Output {
        let mut data: [[f32; M]; N] = unsafe { std::mem::MaybeUninit::uninit().assume_init() };

        for i in 0..M {
            for j in 0..N {
                data[i][j] = self.data[i][j] + rhs.data[i][j];
            }
        }

        Self::Output { data }
    }
}

impl<const M: usize, const N: usize, const L: usize> std::ops::Mul<&Matrix<N, L>>
    for &Matrix<M, N>
{
    type Output = Matrix<M, L>;

    fn mul(self, rhs: &Matrix<N, L>) -> Matrix<M, L> {
        let mut data: [[f32; M]; L] = [[0.0; M]; L];

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
