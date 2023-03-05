/// A curve defines a path through space.
/// Curves may be bounded, infinite or periodic.
/// Typically, a subset of a curve will be used
/// when building up topology inside a Topo.
///
/// Curves as defined by this module may exist outside the context of a Topo
/// and are at a lower level of abstraction.

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

    /// Get the smallest parameter value corresponding to the given point
    /// This will return the same as t_last if the given point is not a self-intersection
    pub fn t_first(&self, p: Trivector) -> Float {
        match &self {
            Curve::Line(x) => x.t_first(p),
            Curve::Circle(x) => x.t_first(p),
        }
    }

    /// Get the largest parameter value corresponding to the given point
    /// This will return the same as t_first if the given point is not a self-intersection
    pub fn t_last(&self, p: Trivector) -> Float {
        match &self {
            Curve::Line(x) => x.t_last(p),
            Curve::Circle(x) => x.t_last(p),
        }
    }

    /// Get a set of points, the convex hull of which bounds the curve on the given parameter range
    pub fn hull(&self, start_t: Float, end_t: Float) -> Vec<Trivector> {
        match &self {
            Curve::Line(x) => x.hull(start_t, end_t),
            Curve::Circle(x) => x.hull(start_t, end_t),
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

    pub fn t_first(&self, p: Trivector) -> Float {
        // Compute a plane through p0 perpendicular to the line
        let plane = self.d & self.p0;

        // Measure signed distance from plane to projected point
        p.hat() & plane
    }

    pub fn t_last(&self, p: Trivector) -> Float {
        self.t_first(p)
    }

    pub fn hull(&self, start_t: Float, end_t: Float) -> Vec<Trivector> {
        // The bounding hull of a line segment is just its two endpoints
        vec![self.d0(start_t), self.d0(end_t)]
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

    pub fn t_first(&self, p: Trivector) -> Float {
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

    pub fn t_last(&self, p: Trivector) -> Float {
        self.t_first(p)
    }

    pub fn hull(&self, start_t: Float, end_t: Float) -> Vec<Trivector> {
        // The bounding hull of a circular arc is the circumscribed polygon consisting of:
        // The start point, the end point, and additional points formed by the intersection of the circle's tangent lines.
        // We add a new tangent line every 90 degrees:
        // For an arc of 0 - 90 degrees, the 2 tangents formed by the start and end points are sufficient.
        // This yields 1 additional intersection points, in addition to the endpoints, making a triangle.
        // For an arc of 90 - 180 degrees, we add one additional tangent at the halfway mark.
        // This yields 2 additional intersection points, in addition to the endpoints, making a trapezoid.

        let mut hull = Vec::<Trivector>::new();

        let p0 = self.d0(start_t);
        let p1 = self.d0(end_t);

        // If the two endpoints are not coincident, they are part of the hull.
        // If the two endpoints are coincident, they are unnecessary as the curve is a complete circle
        // and the four additional intersection points are sufficient
        if (p0 | p1).norm() > EPSILON_COINCIDENT_DISTANCE {
            hull.push(p0);
            hull.push(p1);
        }

        let n_tangents = (2. + (end_t - start_t) / (0.5 * PI)).floor().min(5.) as usize;

        let tangent_planes: Vec<Vector> = (0..n_tangents).map(|i| {
            let alpha = i as Float / (n_tangents - 1) as Float;
            let t = start_t * alpha + end_t * (1. - alpha);
            let pt = self.d0(t);
            // Convert this point to a tangent line by working 1 dimension higher,
            // eventually projecting everything down to the circle's plane
            // radius (as a plane) = join line a with point pt = a & pt
            // pt (as a line) = a.project(pt)
            // tangent (as a plane) = plane perpendicular to radius through pt = (a & pt) | a.project(pt)
            (self.a & pt) | self.a.project(pt)
        }).collect();

        // The reference plane
        let circle_plane = self.a | self.p0;

        // Compute intersections & project down to the circle's plane
        for pair in tangent_planes.windows(2) {
            if let &[t1, t2] = pair {
                // Push the intersection point to the hull
                hull.push(t1 ^ t2 ^ circle_plane);
            } else {
                panic!();
            }
        }

        hull
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
