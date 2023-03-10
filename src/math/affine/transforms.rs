use crate::math::{
    affine::primitives::{Point, Vector},
    matrix::Matrix,
};

pub type AffineTransform = Matrix<f64, 4, 4>;

impl std::ops::Mul<Point> for AffineTransform {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        Point::from_affine(self * rhs.as_matrix())
    }
}

impl std::ops::Mul<Vector> for AffineTransform {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector::from_affine(self * rhs.as_matrix())
    }
}

pub fn rotate_x(angle: f64) -> AffineTransform {
    let mut rot_x = Matrix::zero();

    *rot_x.at_mut(0, 0) = 1.0;
    *rot_x.at_mut(3, 3) = 1.0;

    *rot_x.at_mut(1, 1) = angle.cos();
    *rot_x.at_mut(1, 2) = -angle.sin();
    *rot_x.at_mut(2, 1) = angle.sin();
    *rot_x.at_mut(2, 2) = angle.cos();

    rot_x
}

pub fn rotate_y(angle: f64) -> AffineTransform {
    let mut rot_y = Matrix::zero();

    *rot_y.at_mut(1, 1) = 1.0;
    *rot_y.at_mut(3, 3) = 1.0;

    *rot_y.at_mut(0, 0) = angle.cos();
    *rot_y.at_mut(0, 2) = angle.sin();
    *rot_y.at_mut(2, 0) = -angle.sin();
    *rot_y.at_mut(2, 2) = angle.cos();

    rot_y
}

pub fn rotate_z(angle: f64) -> AffineTransform {
    let mut rot_z = Matrix::zero();

    *rot_z.at_mut(2, 2) = 1.0;
    *rot_z.at_mut(3, 3) = 1.0;

    *rot_z.at_mut(0, 0) = angle.cos();
    *rot_z.at_mut(0, 1) = -angle.sin();
    *rot_z.at_mut(1, 0) = angle.sin();
    *rot_z.at_mut(1, 1) = angle.cos();

    rot_z
}

pub fn translate(vector: Vector) -> AffineTransform {
    let mut translation = Matrix::identity();

    *translation.at_mut(0, 3) = vector.at(0);
    *translation.at_mut(1, 3) = vector.at(1);
    *translation.at_mut(2, 3) = vector.at(2);

    translation
}

pub fn scale(sx: f64, sy: f64, sz: f64) -> AffineTransform {
    let mut scaling = Matrix::zero();

    *scaling.at_mut(0, 0) = sx;
    *scaling.at_mut(1, 1) = sy;
    *scaling.at_mut(2, 2) = sz;
    *scaling.at_mut(3, 3) = 1.0;

    scaling
}
