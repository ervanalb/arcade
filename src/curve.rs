use crate::pga::*;
use crate::global::*;
use std::sync::Arc;

#[derive(Debug)]
pub enum Curve {
    Line(Line),
    Circle(Circle),
    TrimmedCurve(TrimmedCurve),
}

impl Curve {
    // Evaluate the point on the curve at parameter value t
    pub fn d0(&self, t: Float) -> Trivector {
        match &self {
            Curve::Line(x) => x.d0(t),
            Curve::Circle(x) => x.d0(t),
            Curve::TrimmedCurve(x) => x.d0(t),
        }
    }
}

// A line is parameterized by a (euclidean) point p0 and an infinite line d.
// The line starts at p0 and extends orthogonal to d. The parameter t is the signed distance from p0.
// The parametric equation for the point r at position t on the line is: r(t) = m(t) * p0 * ~m(t)
// where m(t) = exp(t / 2 * d)
#[derive(Debug)]
pub struct Line {
    pub p0: Trivector,
    pub d: Bivector,
}

impl Line {
    pub fn d0(&self, t: Float) -> Trivector {
        (0.5 * t * self.d).exp().transform(self.p0)
    }
}

// A circle is parameterized by a (euclidean) point p0 and a euclidean line a.
// The circle starts at p0 and rotates around a. The parameter t is the angle of rotation from p0.
// The parametric equation for the point r at position t on the line is: r(t) = m(t) * p0 * ~m(t)
// where m(t) = exp(t / 2 * d)
#[derive(Debug)]
pub struct Circle {
    pub p0: Trivector,
    pub a: Bivector,
}

impl Circle {
    pub fn d0(&self, t: Float) -> Trivector {
        (0.5 * t * self.a).exp().transform(self.p0)
    }
}

#[derive(Debug)]
pub struct TrimmedCurve {
    pub curve: Arc<Curve>,
    pub p_start: Trivector,
    pub p_end: Trivector,
    pub t_start: Float,
    pub t_end: Float,
}

impl TrimmedCurve {
    pub fn d0(&self, t: Float) -> Trivector {
        self.curve.d0(t)
    }
}
