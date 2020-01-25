use crate::vec::Vec3;
use crate::vertex::Vertex;
use crate::error::Error;
use crate::error::Result;
use crate::limits;

trait Edge {
    // Evaluate the edge at the given parameter value
    fn eval(&self, t: f64) -> Vec3;
}

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
    pub fn from_three_points(pt1: Vertex, pt2: Vertex, pt3: Vertex) -> Result<Arc> {
        pt1.check_vertex_separation(&pt2)?;
        pt1.check_vertex_separation(&pt3)?;
        pt2.check_vertex_separation(&pt3)?;

        // Compute a new coordinate frame where the three points lie in the uv plane
        let u1 = pt2.point() - pt1.point();
        let w1 = (pt3.point() - pt1.point()).cross(u1);
        let u = u1.normalized();
        let w = w1.normalized();
        let v = w.cross(u);

        // Calculate center from the following equation of a circle
        // (x - a)^2 + (y - b)^2 = r^2
        // Substitute in pt1, pt2, pt3 for (x, y) and solve for (a, b)
        // Subtracting the equations gets rid of r^2, a^2, and b^2 unknowns,
        // leaving only a and b in the following equations:
        // 2 * (X1 - X2) * a - 2 * (Y1 - Y2) * b = X1^2 - X2^2 + Y1^2 - Y2^2
        // 2 * (X1 - X3) * a - 2 * (Y1 - Y3) * b = X1^2 - X3^2 + Y1^2 - Y3^2
        // Solve this 2x2 linear system equations using matrices.

        //let x1 = pt1.point;

        //let a = Mat2::new(2 * pt1.point().x);
        Err(Error::NotImplemented)
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
