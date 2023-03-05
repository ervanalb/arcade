use crate::pga::*;
use crate::global::*;
use std::ops::BitXor;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Forward,
    Reverse,
}

impl BitXor for Direction {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::Forward, Self::Forward) => Self::Forward,
            (Self::Forward, Self::Reverse) => Self::Reverse,
            (Self::Reverse, Self::Forward) => Self::Reverse,
            (Self::Reverse, Self::Reverse) => Self::Forward,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Curve {
    Line(Line),
    Circle(Circle),
    //TrimmedCurve(TrimmedCurve),
}

impl Curve {
    // Evaluate the point on the curve at parameter value t
    pub fn d0(&self, t: Float) -> Trivector {
        match &self {
            Curve::Line(x) => x.d0(t),
            Curve::Circle(x) => x.d0(t),
            //Curve::TrimmedCurve(x) => x.d0(t),
        }
    }

    // Get whether the curve is closed, i.e. d0(t_min) == d0(t_max)
    pub fn closed(&self) -> bool {
        match &self {
            Curve::Line(x) => x.closed(),
            Curve::Circle(x) => x.closed(),
            //Curve::TrimmedCurve(x) => x.closed(),
        }
    }

    // Get the smallest valid parameter value of the curve,
    // or None if the curve extends without bound in the negative parameter direction
    pub fn t_min(&self) -> Option<Float> {
        match &self {
            Curve::Line(x) => x.t_min(),
            Curve::Circle(x) => x.t_min(),
            //Curve::TrimmedCurve(x) => x.t_min(),
        }
    }

    // Get the largest valid parameter value of the curve,
    // or None if the curve extends without bound in the positive parameter direction
    pub fn t_max(&self) -> Option<Float> {
        match &self {
            Curve::Line(x) => x.t_max(),
            Curve::Circle(x) => x.t_max(),
            //Curve::TrimmedCurve(x) => x.t_max(),
        }
    }

    // Reflect the curve about the given entity (point, line, plane)
    pub fn reflect<T>(&self, entity: T) -> Curve
    where Vector: Reflect<T>, Bivector: Reflect<T>, Trivector: Reflect<T>, FullMultivector: Reflect<T>, T: Copy {
        match &self {
            Curve::Line(x) => Curve::Line(x.reflect(entity)),
            Curve::Circle(x) => Curve::Circle(x.reflect(entity)),
            //Curve::TrimmedCurve(x) => Curve::TrimmedCurve(x.reflect(entity)),
        }
    }

    // Transform the curve with the given motor
    pub fn transform<T>(&self, entity: T) -> Curve
    where Vector: Transform<T>, Bivector: Transform<T>, Trivector: Transform<T>, FullMultivector: Transform<T>, T: Copy {
        match &self {
            Curve::Line(x) => Curve::Line(x.transform(entity)),
            Curve::Circle(x) => Curve::Circle(x.transform(entity)),
            //Curve::TrimmedCurve(x) => Curve::TrimmedCurve(x.transform(entity)),
        }
    }

    // Get the parameter value closest to the given point
    pub fn t(&self, p: Trivector) -> Float {
        match &self {
            Curve::Line(x) => x.t(p),
            Curve::Circle(x) => x.t(p),
        }
    }
}

// A line is parameterized by a (euclidean) point p0 and an infinite line d.
// The line starts at p0 and extends orthogonal to d. The parameter t is the signed distance from p0.
// The parametric equation for the point r at position t on the line is: r(t) = m(t) * p0 * ~m(t)
// where m(t) = exp(t / 2 * d)
#[derive(Debug, Clone)]
pub struct Line {
    pub p0: Trivector,
    pub d: Bivector,
}

impl Line {
    pub fn d0(&self, t: Float) -> Trivector {
        self.p0.transform((0.5 * t * self.d).exp())
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

    pub fn reflect<T>(&self, entity: T) -> Line
    where Bivector: Reflect<T>, Trivector: Reflect<T>, T: Copy {
        Line {p0: self.p0.reflect(entity), d: self.d.reflect(entity)}
    }

    pub fn transform<T>(&self, entity: T) -> Line
    where Bivector: Transform<T>, Trivector: Transform<T>, T: Copy {
        Line {p0: self.p0.transform(entity), d: self.d.transform(entity)}
    }

    pub fn t(&self, p: Trivector) -> Float {
        // Compute a plane through p0 perpendicular to the line
        let plane = self.d & self.p0;

        // Measure signed distance from plane to projected point
        p.hat() & plane
    }
}

// A circle is parameterized by a (euclidean) point p0 and a euclidean line a.
// The circle starts at p0 and rotates around a. The parameter t is the angle of rotation from p0.
// The parametric equation for the point r at position t on the line is: r(t) = m(t) * p0 * ~m(t)
// where m(t) = exp(t / 2 * d)
#[derive(Debug, Clone)]
pub struct Circle {
    pub p0: Trivector,
    pub a: Bivector,
}

impl Circle {
    pub fn d0(&self, t: Float) -> Trivector {
        self.p0.transform((0.5 * t * self.a).exp())
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

    pub fn reflect<T>(&self, entity: T) -> Circle
    where Bivector: Reflect<T>, Trivector: Reflect<T>, T: Copy {
        Circle {p0: self.p0.reflect(entity), a: self.a.reflect(entity)}
    }

    pub fn transform<T>(&self, entity: T) -> Circle
    where Bivector: Transform<T>, Trivector: Transform<T>, T: Copy {
        Circle {p0: self.p0.transform(entity), a: self.a.transform(entity)}
    }

    pub fn t(&self, p: Trivector) -> Float {
        // Compute plane through l and p0
        let plane0 = (self.a & self.p0).hat();

        // Compute plane through l and p
        let plane1 = (self.a & p).hat();

        // Get angle between planes
        let angle = (plane0 | plane1).acos();
        // See if angle > 180 by comparing sign of pt0 wrt plane1
        let lower_quadrants = plane1 & self.p0 < 0.;

        match lower_quadrants {
            true => 2. * PI - angle,
            false => angle,
        }
    }
}

//#[derive(Debug, Clone)]
//pub struct TrimmedCurve {
//    pub curve: Box<Curve>,
//    pub t_start: Float,
//    pub t_end: Float,
//}

//impl Clone for TrimmedCurve {
//    fn clone(&self) -> TrimmedCurve {
//        TrimmedCurve {
//            curve: Box::new(self.curve.clone()),
//            t_start: self.t_start,
//            t_end: self.t_end,
//        }
//    }
//}

//impl TrimmedCurve {
//    pub fn d0(&self, t: Float) -> Trivector {
//        self.curve.d0(t)
//    }
//
//    pub fn closed(&self) -> bool {
//        false
//    }
//
//    pub fn t_min(&self) -> Option<Float> {
//        Some(self.t_start)
//    }
//
//    pub fn t_max(&self) -> Option<Float> {
//        Some(self.t_end)
//    }
//
//    pub fn reflect<T>(&self, entity: T) -> TrimmedCurve
//    where Vector: Reflect<T>, Bivector: Reflect<T>, Trivector: Reflect<T>, FullMultivector: Reflect<T>, T: Copy {
//        TrimmedCurve {curve: Box::new(self.curve.reflect(entity)), t_start: self.t_start, t_end: self.t_end}
//    }
//
//    pub fn transform<T>(&self, entity: T) -> TrimmedCurve
//    where Vector: Transform<T>, Bivector: Transform<T>, Trivector: Transform<T>, FullMultivector: Transform<T>, T: Copy {
//        TrimmedCurve {curve: Box::new(self.curve.transform(entity)), t_start: self.t_start, t_end: self.t_end}
//    }
//}

pub fn curves_coincident(c0: &Curve, c1: &Curve) -> Option<Direction> {
    // Some(Forward) => Curves are coincident and have the same travel direction
    // Some(Reverse) => Curves are coincident but have opposing travel directions
    // None => Curves are not coincident

    // TODO implement this
    None
}
