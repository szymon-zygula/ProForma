use crate::{
    forms::implicit::ImplicitForm,
    math::affine::{primitives::Point, transforms::*},
};

pub struct Ellipsoid {
    a: f64,
    b: f64,
    c: f64,
}

impl Ellipsoid {
    pub fn with_curvatures(a: f64, b: f64, c: f64) -> Ellipsoid {
        Ellipsoid { a, b, c }
    }

    pub fn with_radii(rx: f64, ry: f64, rz: f64) -> Ellipsoid {
        Self::with_curvatures(1.0 / rx * rx, 1.0 / ry * ry, 1.0 / rz * rz)
    }
}

impl ImplicitForm for Ellipsoid {
    fn implicit_form(&self, u: Point) -> f64 {
        let ellipsoid_matrix = AffineTransform::diagonal(&[self.a, self.b, self.c, -1.0]);

        (u.as_transpose() * ellipsoid_matrix * u.as_matrix()).num()
    }
}
