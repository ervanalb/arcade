use crate::vec::*;
use std::f64::consts::*;
use crate::vertex::Vertex;
use crate::error::Error;
use crate::error::Result;
use crate::limits;

trait Edge {
    // Evaluate the edge at the given parameter value
    fn eval(&self, t: f64) -> Vec3;
}

/*
#[derive(Debug)]
pub struct Segment {
    // Implements a segment parameterized as A + B * t where t ranges from 0 to 1
    a: Vec3, // First point
    b: Vec3, // Second point - First point
}

impl Segment {
    pub fn new(pt1: Vertex, pt2: Vertex) -> Result<Segment> {
        pt1.check_vertex_separation(&pt2)?;

        Ok(Segment {
            a: pt1.point(),
            b: pt2.point() - pt1.point()
        })
    }
}

impl Edge for Segment {
    fn eval(&self, t: f64) -> Vec3 {
        self.a + t * self.b
    }
}

#[derive(Debug)]
pub struct Arc {
    // Implements a circular arc parameterized as C + A cos(theta) + B sin(theta)
    // a and b are radii of the circle, and are perpendicular to each other. a points to the start.
    // theta = t * angle
    c: Vec3, // Center point
    a: Vec3, // A radius of the circle pointing to the first point
    b: Vec3, // A radius of the circle perpendicular to a
    angle: f64, // How much of an arc to sweep out
}

impl Arc {
    pub fn from_three_points(pt1: &Vertex, pt2: &Vertex, pt3: &Vertex) -> Result<Arc> {
        pt1.check_colinear(pt2, pt3)?;

        // This from https://math.stackexchange.com/a/1743505

        // Compute a new coordinate frame uvw where the three points lie in the uv plane.
        // The first point is the origin of the coordinate frame
        // and the second point lies on the Y axis.
        let u1 = pt2.point() - pt1.point();
        let o1 = pt3.point() - pt1.point();
        let w1 = o1.cross(u1);
        let u = u1.normalized();
        let w = w1.normalized();
        let v = w.cross(u);

        // Compute two vectors b and c that go from the origin of the new coordinate system (pt1)
        // to pt2 (b) and pt3 (c)
        let b = Vec2::new(u1.length(), 0.);
        let c = Vec2::new(o1 * u, o1 * v);

        // The center of the circle lies on the line x = bx / 2
        // and is equidistant from the origin and C,
        // let us call this point (bx / 2, h)
        // Solving for h gives us:

        let h = ((c.x - 0.5 * b.x).sq() + c.y.sq() - (0.5 * b.x).sq()) / (2. * c.y);
        let center_2d = Vec2::new(0.5 * b.x, h);

        // Get two radii in 2D, the first pointing from center to pt1
        // and the second orthogonal to it in the direction that the arc goes.
        let radius1 = -center_2d;
        let radius2 = Vec2::new(-center_2d.y, center_2d.x);

        // Use atan2 in this coordinate frame to get the angular sweep of the arc.
        let angle = ((c - center_2d) * radius2).atan2((c - center_2d) * radius1);
        let angle = angle + (2. * PI) * (angle < 0.) as i32 as f64;

        // Reproject into 3D
        let center_3d = pt1.point() + center_2d.x * u + center_2d.y * v;
        let radius1_3d = pt1.point() + radius1.x * u + radius1.y * v;
        let radius2_3d = pt1.point() + radius2.x * u + radius2.y * v;

        Ok(Arc {
            c: center_3d,
            a: radius1_3d,
            b: radius2_3d,
            angle: angle
        })
    }
}

impl Edge for Arc {
    fn eval(&self, t: f64) -> Vec3 {
        let theta = t * self.angle;
        self.c + self.a * theta.cos() + self.b * theta.sin()
    }
}

// TESTS
#[test]
fn segment_construction() {
    let v = limits::WORKSPACE_SIZE * Vec3::new(0.5, 0.9, -0.3);

    assert!(
        Segment::new(Vertex::new(v).unwrap(),
                     Vertex::new(v + limits::MINIMUM_VERTEX_SEPARATION * Vec3::new(1.5, 0.5, -2.)).unwrap())
        .is_ok());

    assert_eq!(
        Segment::new(Vertex::new(v).unwrap(),
                     Vertex::new(v + limits::MINIMUM_VERTEX_SEPARATION * Vec3::new(0.3, 0.5, -0.8)).unwrap()).unwrap_err(),
        Error::VerticesTooClose);
}
*/
