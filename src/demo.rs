use arcade::pga::{Point};
use arcade::nurbs::{Curve};

extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::light::Light;
use kiss3d::window::Window;
use kiss3d::camera::ArcBall;
use na::{Point3, Vector3};

fn pga_to_point3(pt: Point) -> Point3<f32> {
    let (x, y, z) = pt.to_xyz();
    Point3::new(x as f32, y as f32, z as f32)
}

fn draw_axes(window: &mut Window) {
    window.draw_line(&Point3::origin(), &Point3::new(1., 0., 0.), &Point3::new(1., 0., 0.));
    window.draw_line(&Point3::origin(), &Point3::new(0., 1., 0.), &Point3::new(0., 1., 0.));
    window.draw_line(&Point3::origin(), &Point3::new(0., 0., 1.), &Point3::new(0., 0., 1.));
}

//fn draw_pts(window: &mut Window, pts: &Mat3xN) {
//    for col in pts.column_iter() {
//        let pt = vec_to_point(&col);
//        window.set_point_size(6.);
//        window.draw_point(&pt, &Point3::new(1., 1., 1.));
//    }
//}

fn draw(window: &mut Window, pts: &Vec<Point>) {
    let mut pts_iter = pts.iter();
    let mut prev = pga_to_point3(*pts_iter.next().unwrap());
    for pga_pt in pts_iter {
        let pt = pga_to_point3(*pga_pt);
        window.draw_line(&prev, &pt, &Point3::new(1., 1., 1.));
        prev = pt;
    }
}

fn main() {
    let mut window = Window::new("Arcade demo");

    window.set_light(Light::StickToCamera);

    let mut arc_ball = ArcBall::new(Point3::new(3., -10., 3.), Point3::origin());
    arc_ball.set_up_axis(Vector3::new(0., 0., 1.));

    let nurbs_points = vec![
        Point::from_xyz(0., 0., 0.),
        Point::from_xyz(1., 0., 0.),
        Point::from_xyz(0., 1., 1.) * 1.,
        Point::from_xyz(1., 1., 1.) * 1.,
        Point::from_xyz(1., 1., 1.),
        Point::from_xyz(2., 1., 1.),
        Point::from_xyz(0., 2., 2.) * 1.,
        Point::from_xyz(2., 2., 2.) * 1.,
    ];

    let nurbs_knots = vec![0., 0., 0., 0., 1., 2., 3., 4., 5., 5., 5., 5.];
    let nurbs1 = Curve {
        points: nurbs_points,
        knots: nurbs_knots
    };

    let nurbs1_rendered = nurbs1.interpolate_fixed(50);

    while window.render_with_camera(&mut arc_ball) {
        draw_axes(&mut window);
        draw(&mut window, &nurbs1_rendered);
        draw(&mut window, &nurbs1.points);
    }
}
