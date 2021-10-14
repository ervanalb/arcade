use arcade::pga::{Trivector, Normalize};
use arcade::construct::{point_from_xyz, three_point_arc};
use arcade::interpolate::interpolate_curve_fixed;

extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::light::Light;
use kiss3d::window::Window;
use kiss3d::camera::ArcBall;
use na::{Point3, Vector3};

fn pga_to_point3(pt: Trivector) -> Point3<f32> {
    let npt = pt.hat();
    let (x, y, z, _) = npt.as_tuple();
    Point3::new(x as f32, y as f32, z as f32)
}

fn draw_axes(window: &mut Window) {
    window.draw_line(&Point3::origin(), &Point3::new(1., 0., 0.), &Point3::new(1., 0., 0.));
    window.draw_line(&Point3::origin(), &Point3::new(0., 1., 0.), &Point3::new(0., 1., 0.));
    window.draw_line(&Point3::origin(), &Point3::new(0., 0., 1.), &Point3::new(0., 0., 1.));
}

fn draw_pts(window: &mut Window, pts: &Vec<Trivector>) {
    for pt in pts {
        let pt3 = pga_to_point3(*pt);
        window.set_point_size(6.);
        window.draw_point(&pt3, &Point3::new(1., 1., 1.));
    }
}

fn draw(window: &mut Window, pts: &Vec<Trivector>) {
    let mut pts_iter = pts.iter();
    let mut prev = pga_to_point3(*pts_iter.next().unwrap());
    for pga_pt in pts_iter {
        let pt = pga_to_point3(*pga_pt);
        window.draw_line(&prev, &pt, &Point3::new(0.5, 0.5, 0.5));
        prev = pt;
    }
}

fn main() {
    let mut window = Window::new("Arcade demo");

    window.set_light(Light::StickToCamera);

    let mut arc_ball = ArcBall::new(Point3::new(3., -10., 3.), Point3::origin());
    arc_ball.set_up_axis(Vector3::new(0., 0., 1.));

    
    //let nurbs_points = vec![
    //    Point::from_xyz(0., 0., 0.),
    //    Point::from_xyz(1., 0., 0.),
    //    Point::from_xyz(0., 1., 1.) * 1.,
    //    Point::from_xyz(1., 1., 1.) * 1.,
    //    Point::from_xyz(1., 1., 1.),
    //    Point::from_xyz(2., 1., 1.),
    //    Point::from_xyz(0., 2., 2.) * 1.,
    //    Point::from_xyz(2., 2., 2.) * 1.,
    //];

    //let nurbs_knots = vec![0., 0., 0., 0., 1., 2., 3., 4., 5., 5., 5., 5.];
    //let nurbs1 = Curve {
    //    points: nurbs_points,
    //    knots: nurbs_knots
    //};

    let pt0 = point_from_xyz(1., 1., 1.);
    let pt1 = point_from_xyz(2., 2., 1.);
    let pt2 = point_from_xyz(1., 2.5, 1.);

    let arc0 = three_point_arc(pt0, pt1, pt2);
    let arc0_rendered = interpolate_curve_fixed(&arc0, 50);

    while window.render_with_camera(&mut arc_ball) {
        draw_axes(&mut window);
        draw(&mut window, &arc0_rendered);
        draw_pts(&mut window, &vec![pt0, pt1, pt2]);
    }
}
