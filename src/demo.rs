use arcade::vec::Vec3;
use arcade::vertex::Vertex;
use arcade::edge::Segment;
use arcade::edge::GenericEdge;

extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::light::Light;
use kiss3d::window::Window;
use kiss3d::camera::ArcBall;
use na::{Point3, Vector3};

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

fn draw_segment(window: &mut Window, s: &Segment) {
    let v1 = s.d0(0.);
    let v2 = s.d0(1.);
    let p1 = Point3::new(v1.x as f32, v1.y as f32, v1.z as f32);
    let p2 = Point3::new(v2.x as f32, v2.y as f32, v2.z as f32);
    window.draw_line(&p1, &p2, &Point3::new(1., 1., 1.));
}

fn main() {
    let mut window = Window::new("Arcade demo");

    window.set_light(Light::StickToCamera);

    let mut arc_ball = ArcBall::new(Point3::new(3., -10., 3.), Point3::origin());
    arc_ball.set_up_axis(Vector3::new(0., 0., 1.));

    let v1 = Vertex::new(Vec3::new(0., 0., 0.)).unwrap();
    let v2 = Vertex::new(Vec3::new(1., 1., 0.2)).unwrap();
    let v3 = Vertex::new(Vec3::new(-0.3, 2., 0.)).unwrap();

    let s1 = Segment::new(&v1, &v2).unwrap();
    let s2 = Segment::new(&v2, &v3).unwrap();
    let s3 = Segment::new(&v3, &v1).unwrap();

    while window.render_with_camera(&mut arc_ball) {
        draw_axes(&mut window);
        draw_vertex(&mut window, &v1);
        draw_vertex(&mut window, &v2);
        draw_vertex(&mut window, &v3);
        draw_segment(&mut window, &s1);
        draw_segment(&mut window, &s2);
        draw_segment(&mut window, &s3);
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
