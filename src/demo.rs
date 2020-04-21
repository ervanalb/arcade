use arcade::vertex::Vertex;
use arcade::edge::{Segment, CubicNURBSCurve};
use arcade::edge::Edge;
use arcade::interpolate::interpolate_edge_fixed;
use arcade::types::{VecN, Mat4xN};

extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::light::Light;
use kiss3d::window::Window;
use kiss3d::camera::ArcBall;
use na::{Point3, Vector3, MatrixSlice3x1};

fn vec_to_point(pt: &MatrixSlice3x1<f64>) -> Point3<f32> {
    Point3::new(pt[0] as f32, pt[1] as f32, pt[2] as f32)
}

fn draw_axes(window: &mut Window) {
    window.draw_line(&Point3::origin(), &Point3::new(1., 0., 0.), &Point3::new(1., 0., 0.));
    window.draw_line(&Point3::origin(), &Point3::new(0., 1., 0.), &Point3::new(0., 1., 0.));
    window.draw_line(&Point3::origin(), &Point3::new(0., 0., 1.), &Point3::new(0., 0., 1.));
}

fn draw_vertex(window: &mut Window, v: &Vertex) {
    let p = Point3::new(v.point().x as f32, v.point().y as f32, v.point().z as f32);
    window.set_point_size(6.);
    window.draw_point(&p, &Point3::new(1., 1., 1.));
}

/*
fn draw_segment(window: &mut Window, s: &Segment) {
    let v1 = s.d0(0.);
    let v2 = s.d0(1.);
    let p1 = Point3::new(v1.x as f32, v1.y as f32, v1.z as f32);
    let p2 = Point3::new(v2.x as f32, v2.y as f32, v2.z as f32);
    window.draw_line(&p1, &p2, &Point3::new(1., 1., 1.));
}
*/

fn draw_edge<T: Edge>(window: &mut Window, e: &T) {
    let pts = interpolate_edge_fixed(e, 50);

    let mut pts_iter = pts.column_iter();
    let mut prev = vec_to_point(&pts_iter.next().unwrap());
    for col in pts_iter {
        let pt = vec_to_point(&col);
        window.draw_line(&prev, &pt, &Point3::new(1., 1., 1.));
        prev = pt;
    }
}

fn main() {
    let mut window = Window::new("Arcade demo");

    window.set_light(Light::StickToCamera);

    let mut arc_ball = ArcBall::new(Point3::new(3., -10., 3.), Point3::origin());
    arc_ball.set_up_axis(Vector3::new(0., 0., 1.));

    let v1 = Vertex::new(Vector3::new(0., 0., 0.)).unwrap();
    let v2 = Vertex::new(Vector3::new(1., 1., 0.2)).unwrap();
    let v3 = Vertex::new(Vector3::new(-0.3, 2., 0.)).unwrap();

    let s1 = Segment::new(&v1, &v2).unwrap();
    let s2 = Segment::new(&v2, &v3).unwrap();
    let s3 = Segment::new(&v3, &v1).unwrap();

    let nurbs_points = Mat4xN::from_row_slice(&[
        0., 1.,  0., 1., 1., 2., 0., 2.,
        0., 0.,  1., 1., 1., 1., 2., 2.,
        0., 0.,  1., 1., 1., 1., 2., 2.,
        1., 1.,  3., 3., 1., 1., 3., 3.,
    ]);

    let nurbs_knots = VecN::from_row_slice(&[0., 0., 0., 0., 1., 2., 3., 4., 5., 5., 5., 5.]);
    let nurbs1 = CubicNURBSCurve::new(&nurbs_points, &nurbs_knots).unwrap();
    let nurbs2 = nurbs1.insert_knot(0.5, 3).unwrap();

    while window.render_with_camera(&mut arc_ball) {
        draw_axes(&mut window);
        draw_vertex(&mut window, &v1);
        draw_vertex(&mut window, &v2);
        draw_vertex(&mut window, &v3);
        draw_edge(&mut window, &s1);
        draw_edge(&mut window, &s2);
        draw_edge(&mut window, &s3);
        draw_edge(&mut window, &nurbs1);
        draw_edge(&mut window, &nurbs2);
    }

    //let pt1 = Vertex::new(Vec3::new(2., 3., 4.)).unwrap();
    //let pt2 = Vertex::new(Vec3::new(8., 7., 6.)).unwrap();
    //let seg = Segment::new(pt1, pt2);
    //println!("Segment is: {:?}", seg);

    //let v1 = Vertex::new(Vec3::new(0., 0., 0.)).unwrap();
    //let v2 = Vertex::new(Vec3::new(1., 1., 0.)).unwrap();
    //let v3 = Vertex::new(Vec3::new(-0.3, 2., 0.)).unwrap();

    //let arc = Arc::from_three_points(&v1, &v2, &v3).unwrap();
    //println!("Arc is: {:?}", arc);
}
