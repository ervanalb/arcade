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
