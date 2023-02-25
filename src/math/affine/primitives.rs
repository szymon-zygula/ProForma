use crate::math::matrix::Matrix;

type AffineElement = Matrix<f64, 4, 1>;

#[derive(Clone, Copy, Debug)]
pub struct Point {
    affine: AffineElement,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point {
            affine: AffineElement::from_data([[x], [y], [z], [1.0]]),
        }
    }

    pub fn from_affine(affine: AffineElement) -> Point {
        assert!(
            affine.at(3, 0) == 1.0,
            "creating point from an affine element {:?}",
            affine
        );
        Point { affine }
    }

    pub fn at(&self, i: usize) -> f64 {
        self.affine.at(i, 0)
    }

    pub fn at_mut(&mut self, i: usize) -> &mut f64 {
        self.affine.at_mut(i, 0)
    }

    pub fn as_matrix(&self) -> AffineElement {
        self.affine
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Vector {
    affine: AffineElement,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector {
            affine: AffineElement::from_data([[x], [y], [z], [0.0]]),
        }
    }

    pub fn from_affine(affine: AffineElement) -> Vector {
        assert!(
            affine.at(3, 0) == 0.0,
            "creating vector from an affine element {:?}",
            affine
        );

        Vector { affine }
    }

    pub fn at(&self, i: usize) -> f64 {
        self.affine.at(i, 0)
    }

    pub fn at_mut(&mut self, i: usize) -> &mut f64 {
        self.affine.at_mut(i, 0)
    }

    pub fn as_matrix(&self) -> AffineElement {
        self.affine
    }
}

impl std::ops::Mul<Vector> for Vector {
    type Output = f64;

    fn mul(self, rhs: Vector) -> Self::Output {
        (self.affine.transpose() * rhs.affine).num()
    }
}

macro_rules! impl_affine_add {
    ($type1:ident + $type2:ident -> $type_out:ident) => {
        impl std::ops::Add<$type2> for $type1 {
            type Output = $type_out;

            fn add(self, rhs: $type2) -> Self::Output {
                Self::Output {
                    affine: self.affine + rhs.affine,
                }
            }
        }
    };
}

impl_affine_add!(Vector + Vector -> Vector);
impl_affine_add!(Vector + Point -> Point);
impl_affine_add!(Point + Vector -> Point);