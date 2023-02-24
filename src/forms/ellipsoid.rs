use crate::{forms::implicit::ImplicitForm, math::matrix::Matrix};

pub struct Ellipsoid {
    a: f64,
    b: f64,
    c: f64,
}

impl ImplicitForm for Ellipsoid {
    fn implicit_fun(&self, u: Matrix<f64, 4, 1>) -> f64 {
        let ellipsoid_matrix = Matrix::<f64, 4, 4>::diagonal(&[self.a, self.b, self.c, -1.0]);

        (u.transpose() * ellipsoid_matrix * u).num()
    }
}
