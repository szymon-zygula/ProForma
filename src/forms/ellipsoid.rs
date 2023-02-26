use crate::{forms::implicit::*, math::affine::transforms::*};

#[derive(Clone, Copy, Debug)]
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
        Self::with_curvatures(1.0 / (rx * rx), 1.0 / (ry * ry), 1.0 / (rz * rz))
    }
}

impl QuadraticForm for Ellipsoid {
    fn quadratic_form_matrix(&self) -> AffineTransform {
        AffineTransform::diagonal(&[self.a, self.b, self.c, -1.0])
    }
}
