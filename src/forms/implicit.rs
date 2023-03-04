use crate::math::affine::{
    primitives::{Point, Vector},
    transforms::AffineTransform,
};

pub trait ImplicitForm {
    fn implicit_form_value(&self, u: Point) -> f64;

    fn contains_point(&self, u: Point) -> bool {
        -std::f64::EPSILON <= self.implicit_form_value(u)
            || self.implicit_form_value(u) <= std::f64::EPSILON
    }
}

pub trait DifferentialForm {
    fn derivative_at(&self, u: Point) -> Vector;
}

pub trait QuadraticForm {
    fn quadratic_form_matrix(&self) -> AffineTransform;
}

impl<T: QuadraticForm> ImplicitForm for T {
    fn implicit_form_value(&self, u: Point) -> f64 {
        (u.as_transpose() * self.quadratic_form_matrix() * u.as_matrix()).num()
    }
}

impl<T: QuadraticForm> DifferentialForm for T {
    fn derivative_at(&self, u: Point) -> Vector {
        let vec = u.as_transpose()
            * (self.quadratic_form_matrix() + self.quadratic_form_matrix().transpose());
        Vector::new(vec.at(0, 0), vec.at(0, 1), vec.at(0, 2))
    }
}
