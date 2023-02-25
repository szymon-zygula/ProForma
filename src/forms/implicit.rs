use crate::math::affine::primitives::Point;

pub trait ImplicitForm {
    fn implicit_form(&self, u: Point) -> f64;

    fn contains(&self, u: Point) -> bool {
        -std::f64::EPSILON <= self.implicit_form(u) || self.implicit_form(u) <= std::f64::EPSILON
    }
}
