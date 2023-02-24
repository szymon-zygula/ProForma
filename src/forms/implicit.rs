use crate::math::matrix::Matrix;

pub trait ImplicitForm {
    fn implicit_fun(&self, u: Matrix<f64, 4, 1>) -> f64;
}
