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

    // Get whether the curve is closed, i.e. d0(t_min) == d0(t_max)
    pub fn closed(&self) -> bool {
        match &self {
            Curve::Line(x) => x.closed(),
            Curve::Circle(x) => x.closed(),
            Curve::TrimmedCurve(x) => x.closed(),
        }
    }

    // Get the smallest valid parameter value of the curve,
    // or None if the curve extends without bound in the negative parameter direction
    pub fn t_min(&self) -> Option<Float> {
        match &self {
            Curve::Line(x) => x.t_min(),
            Curve::Circle(x) => x.t_min(),
            Curve::TrimmedCurve(x) => x.t_min(),
        }
    }

    // Get the largest valid parameter value of the curve,
    // or None if the curve extends without bound in the positive parameter direction
    pub fn t_max(&self) -> Option<Float> {
        match &self {
            Curve::Line(x) => x.t_max(),
            Curve::Circle(x) => x.t_max(),
            Curve::TrimmedCurve(x) => x.t_max(),
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

    pub fn closed(&self) -> bool {
        false
    }

    pub fn t_min(&self) -> Option<Float> {
        None
    }

    pub fn t_max(&self) -> Option<Float> {
        None
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

    pub fn closed(&self) -> bool {
        true
    }

    pub fn t_min(&self) -> Option<Float> {
        Some(0.)
    }

    pub fn t_max(&self) -> Option<Float> {
        Some(2. * PI)
    }
}

#[derive(Debug)]
pub struct TrimmedCurve {
    pub curve: Arc<Curve>,
    pub t_start: Float,
    pub t_end: Float,
}

impl TrimmedCurve {
    pub fn d0(&self, t: Float) -> Trivector {
        self.curve.d0(t)
    }

    pub fn closed(&self) -> bool {
        false
    }

    pub fn t_min(&self) -> Option<Float> {
        Some(self.t_start)
    }

    pub fn t_max(&self) -> Option<Float> {
        Some(self.t_end)
    }
}
