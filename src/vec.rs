use std::fmt;
use std::ops;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {
            x: x,
            y: y,
            z: z
        }
    }

    pub fn is_finite(&self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }

    pub fn is_within(&self, epsilon: f64) -> bool {
        self.x.abs() <= epsilon && self.y.abs() <= epsilon && self.z.abs() <= epsilon
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalized(&self) -> Vec3 {
       *self / self.length()
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x
        }
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y, 
            z: self.z + other.z
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y, 
            z: self.z - other.z
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        Vec3 {
            x: self.x * other,
            y: self.y * other, 
            z: self.z * other
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self * other.x,
            y: self * other.y, 
            z: self * other.z
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        Vec3 {
            x: self.x / other,
            y: self.y / other, 
            z: self.z / other
        }
    }
}

impl ops::Div<Vec3> for f64 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self / other.x,
            y: self / other.y, 
            z: self / other.z
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Vec2 {
        Vec2 {
            x: x,
            y: y
        }
    }

    pub fn is_finite(&self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }

    pub fn is_within(&self, epsilon: f64) -> bool {
        self.x.abs() <= epsilon && self.y.abs() <= epsilon
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalized(&self) -> Vec2 {
        *self / self.length()
    }

    pub fn cross(&self, other: &Vec2) -> f64 {
        self.x * other.y - self.y * other.x
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl ops::Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y, 
        }
    }
}

impl ops::Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y, 
        }
    }
}

impl ops::Mul<f64> for Vec2 {
    type Output = Vec2;

    fn mul(self, other: f64) -> Vec2 {
        Vec2 {
            x: self.x * other,
            y: self.y * other, 
        }
    }
}

impl ops::Mul<Vec2> for f64 {
    type Output = Vec2;

    fn mul(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self * other.x,
            y: self * other.y, 
        }
    }
}

impl ops::Div<f64> for Vec2 {
    type Output = Vec2;

    fn div(self, other: f64) -> Vec2 {
        Vec2 {
            x: self.x / other,
            y: self.y / other, 
        }
    }
}

impl ops::Div<Vec2> for f64 {
    type Output = Vec2;

    fn div(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self / other.x,
            y: self / other.y, 
        }
    }
}

// Implements a matrix:
// [ a b ]
// [ c d ]

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Mat2 {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
}

impl Mat2 {
    pub fn new(a: f64, b: f64, c: f64, d: f64) -> Mat2 {
        Mat2 {
            a: a,
            b: b,
            c: c,
            d: d
        }
    }

    pub fn is_finite(&self) -> bool {
        self.a.is_finite() && self.b.is_finite() && self.c.is_finite() && self.d.is_finite()
    }

    pub fn det(&self) -> f64 {
        return self.a * self.d - self.b * self.c;
    }

    pub fn inv(&self) -> Mat2 {
        let det = self.det();

        Mat2 {
            a: self.d / det,  b: -self.b / det,
            c: -self.c / det, d: self.a / det,
        }
    }
}

impl fmt::Display for Mat2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[[{}, {}], [{}, {}]]", self.a, self.b, self.c, self.d)
    }
}

impl ops::Add for Mat2 {
    type Output = Mat2;

    fn add(self, other: Mat2) -> Mat2 {
        Mat2 {
            a: self.a + other.a,
            b: self.b + other.b, 
            c: self.c + other.c,
            d: self.d + other.d
        }
    }
}

impl ops::Sub for Mat2 {
    type Output = Mat2;

    fn sub(self, other: Mat2) -> Mat2 {
        Mat2 {
            a: self.a - other.a,
            b: self.b - other.b,
            c: self.c - other.c,
            d: self.d - other.d
        }
    }
}

impl ops::Mul<f64> for Mat2 {
    type Output = Mat2;

    fn mul(self, other: f64) -> Mat2 {
        Mat2 {
            a: self.a * other,
            b: self.b * other, 
            c: self.c * other,
            d: self.d * other
        }
    }
}

impl ops::Mul<Mat2> for f64 {
    type Output = Mat2;

    fn mul(self, other: Mat2) -> Mat2 {
        Mat2 {
            a: self * other.a,
            b: self * other.b, 
            c: self * other.c,
            d: self * other.d
        }
    }
}

impl ops::Div<f64> for Mat2 {
    type Output = Mat2;

    fn div(self, other: f64) -> Mat2 {
        Mat2 {
            a: self.a / other,
            b: self.b / other, 
            c: self.c / other,
            d: self.d / other
        }
    }
}

impl ops::Div<Mat2> for f64 {
    type Output = Mat2;

    fn div(self, other: Mat2) -> Mat2 {
        Mat2 {
            a: self / other.a,
            b: self / other.b, 
            c: self / other.c,
            d: self / other.d
        }
    }
}

impl ops::Mul<Mat2> for Mat2 {
    type Output = Mat2;

    fn mul(self, other: Mat2) -> Mat2 {
        Mat2 {
            a: self.a * other.a + self.b * other.c,
            b: self.a * other.b + self.b * other.d, 
            c: self.c * other.a + self.d * other.c,
            d: self.c * other.b + self.d * other.d
        }
    }
}

impl ops::Mul<Vec2> for Mat2 {
    type Output = Vec2;

    fn mul(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.a * other.x + self.b * other.y,
            y: self.c * other.x + self.d * other.y 
        }
    }
}

impl ops::Mul<Mat2> for Vec2 {
    type Output = Vec2;

    fn mul(self, other: Mat2) -> Vec2 {
        Vec2 {
            x: self.x * other.a + self.y * other.c,
            y: self.x * other.b + self.y * other.d
        }
    }
}

// TESTS

#[test]
fn vec_arithmetic() {
    // Basic arithmetic
    assert_eq!(Vec3::new(2., 3., 4.) + Vec3::new(9., -3., 0.), Vec3::new(11., 0., 4.));
    assert_eq!(Vec3::new(2., 3., 4.) - Vec3::new(9., -3., 0.), Vec3::new(-7., 6., 4.));
    assert_eq!(Vec3::new(2., 3., 4.) * 10., Vec3::new(20., 30., 40.));
    assert_eq!(-1. * Vec3::new(9., -3., 0.), Vec3::new(-9., 3., 0.));
    assert_eq!(Vec3::new(9., -3., 0.) / 3., Vec3::new(3., -1., 0.));
    assert_eq!(27. / Vec3::new(9., -3., 1.), Vec3::new(3., -9., 27.));
    assert_eq!(Vec3::new(1., 2., 2.).length(), 3.);
    assert_eq!(Vec3::new(0., 0., 2.).normalized(), Vec3::new(0., 0., 1.));

    assert_eq!(Vec2::new(2., 3.) + Vec2::new(9., -3.), Vec2::new(11., 0.));
    assert_eq!(Vec2::new(2., 3.) - Vec2::new(9., -3.), Vec2::new(-7., 6.));
    assert_eq!(Vec2::new(2., 3.) * 10., Vec2::new(20., 30.));
    assert_eq!(-1. * Vec2::new(9., -3.), Vec2::new(-9., 3.));
    assert_eq!(Vec2::new(9., -3.) / 3., Vec2::new(3., -1.));
    assert_eq!(27. / Vec2::new(9., -3.), Vec2::new(3., -9.));
    assert_eq!(Vec2::new(3., 4.).length(), 5.);
    assert_eq!(Vec2::new(0., 2.).normalized(), Vec2::new(0., 1.));

    // is_nan
    assert!(Vec3::new(5., 6., 7.).is_finite());
    assert!(!Vec3::new(5., 6., std::f64::NAN).is_finite());
    assert!(!Vec3::new(0., 0., 0.).normalized().is_finite());
    assert!(Vec2::new(5., 6.).is_finite());
    assert!(!Vec2::new(5., std::f64::NAN).is_finite());
    assert!(!Vec2::new(0., 0.).normalized().is_finite());

    // is_within
    assert!(!Vec3::new(-1., 2., -3.).is_within(2.5));
    assert!(Vec3::new(-1., 2., -3.).is_within(3.5));
    assert!(!Vec2::new(-1., -3.).is_within(2.5));
    assert!(Vec2::new(-1., -3.).is_within(3.5));

    // Matrix arithmetic
    assert_eq!(Mat2::new(1., 2., 3., 4.) * Vec2::new(5., 6.), Vec2::new(17., 39.));
    assert_eq!(Vec2::new(5., 6.) * Mat2::new(1., 2., 3., 4.), Vec2::new(23., 34.));
    assert_eq!(Mat2::new(10., 20., 30., 40.) * Mat2::new(1., 2., 3., 4.), Mat2::new(70., 100., 150., 220.));

    assert_eq!(Mat2::new(1., 2., 3., 4.).inv(), Mat2::new(-2., 1., 1.5, -0.5));
    assert_eq!(Mat2::new(1., 2., 2., 4.).det(), 0.);
    assert!(!Mat2::new(1., 2., 2., 4.).inv().is_finite());
}

