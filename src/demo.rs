use arcade::pga::{Trivector, Normalize};
use arcade::construct::{point_from_xyz, circle_from_three_points, line_from_two_points, plane_from_standard_form};
use arcade::topo::{Arena3D};
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

    // Build a flask!
    let width = 5.;
    let thickness = 3.;

    let mut arena = Arena3D::new();

    let pt1 = point_from_xyz(-width / 2., 0., 0.);
    let pt2 = point_from_xyz(-width / 2., -thickness / 4., 0.);
    let pt3 = point_from_xyz(0., -thickness / 2., 0.);
    let pt4 = point_from_xyz(width / 2., -thickness / 4., 0.);
    let pt5 = point_from_xyz(width / 2., 0., 0.);

    let v1 = arena.add_vertex(pt1);
    let v2 = arena.add_vertex(pt2);
    let v3 = arena.add_vertex(pt3);
    let v4 = arena.add_vertex(pt4);
    let v5 = arena.add_vertex(pt5);

    let c1 = arena.add_curve(circle_from_three_points(pt2, pt3, pt4));
    let c2 = arena.add_curve(line_from_two_points(pt1, pt2));
    let c3 = arena.add_curve(line_from_two_points(pt4, pt5));

    let e1 = arena.add_edge_with_endpoints(c1, v2, v4);
    let e2 = arena.add_edge_with_endpoints(c2, v1, v2);
    let e3 = arena.add_edge_with_endpoints(c3, v4, v5);

    //let mirror = plane_from_standard_form(0., 1., 0., 0.).hat(); // Y = 0 plane
    ////let motor = ((point_from_xyz(0., 0., 0.) & point_from_xyz(0., 0., 1.)) * I).ihat().exp();

    //let arc2 = arc1.reflect(mirror);
    //let seg3 = seg1.reflect(mirror);
    //let seg4 = seg2.reflect(mirror);
    ////let arc2 = arc1.transform(motor);
    ////let seg3 = seg1.transform(motor);
    ////let seg4 = seg2.transform(motor);

    //let arc1_rendered = interpolate_curve_fixed(&arc1, 50);
    //let arc2_rendered = interpolate_curve_fixed(&arc2, 50);
    //let seg1_rendered = interpolate_curve_fixed(&seg1, 5);
    //let seg2_rendered = interpolate_curve_fixed(&seg2, 5);
    //let seg3_rendered = interpolate_curve_fixed(&seg3, 5);
    //let seg4_rendered = interpolate_curve_fixed(&seg4, 5);

    while window.render_with_camera(&mut arc_ball) {
        draw_axes(&mut window);

        draw_pts(&mut window, &arena.vertices);
        //draw(&mut window, &arc1_rendered);
        //draw(&mut window, &arc2_rendered);
        //draw(&mut window, &seg1_rendered);
        //draw(&mut window, &seg2_rendered);
        //draw(&mut window, &seg3_rendered);
        //draw(&mut window, &seg4_rendered);
        //draw_pts(&mut window, &vec![pt1, pt2, pt3, pt4, pt5]);
    }
}
