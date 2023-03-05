use arcade::pga::{Trivector, Normalize};
use arcade::construct::{point_from_xyz, plane_from_standard_form};
use arcade::topo::{Topo, reflect, combine};
use arcade::interpolate::{interpolate_curve_subset_fixed, interpolate_closed_curve_fixed};

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

fn draw_vertices(window: &mut Window, topo: &Topo) {
    for vertex in &topo.vertices {
        let pt3 = pga_to_point3(*vertex);
        window.set_point_size(6.);
        window.draw_point(&pt3, &Point3::new(1., 1., 1.));
    }
}

fn draw_edges(window: &mut Window, topo: &Topo) {
    for edge in &topo.edges {
        let curve = &topo.curves[edge.curve];
        let pts = match &edge.bounds {
            Some(bounds) => {
                let start_pt = topo.vertices[bounds.start];
                let end_pt = topo.vertices[bounds.end];
                let start_t = curve.t(start_pt);
                let end_t = curve.t(end_pt);
                let mut pts = interpolate_curve_subset_fixed(&curve, start_t, end_t, 50);
                // Replace the start and end points with the vertices
                pts[0] = start_pt;
                let ix = pts.len() - 1;
                pts[ix] = end_pt;
                pts
            },
            None => {
                let mut pts = interpolate_closed_curve_fixed(&curve, 50);
                // Add a point at the end of the list, equal to the first point,
                // so that the rendered segments join up and draw a closed curve
                pts.push(pts[0]);
                pts
            }
        };

        let mut pts_iter = pts.iter();
        let mut prev = pga_to_point3(*pts_iter.next().unwrap());
        for pga_pt in pts_iter {
            let pt = pga_to_point3(*pga_pt);
            window.draw_line(&prev, &pt, &Point3::new(0.5, 0.5, 0.5));
            prev = pt;
        }
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

    let pt1 = point_from_xyz(-width / 2., 0., 0.);
    let pt2 = point_from_xyz(-width / 2., -thickness / 4., 0.);
    let pt3 = point_from_xyz(0., -thickness / 2., 0.);
    let pt4 = point_from_xyz(width / 2., -thickness / 4., 0.);
    let pt5 = point_from_xyz(width / 2., 0., 0.);

    let e1 = Topo::line_segment_from_two_points(pt1, pt2).unwrap();
    let e2 = Topo::circular_arc_from_three_points(pt2, pt3, pt4).unwrap();
    let e3 = Topo::line_segment_from_two_points(pt4, pt5).unwrap();

    let topo = combine(&[e1, e2, e3]).unwrap();

    //let v1 = topo.add_vertex(pt1);
    //let v2 = topo.add_vertex(pt2);
    //let _v3 = topo.add_vertex(pt3);
    //let v4 = topo.add_vertex(pt4);
    //let v5 = topo.add_vertex(pt5);

    //let c1 = topo.add_curve(circle_from_three_points(pt2, pt3, pt4));
    //let c2 = topo.add_curve(line_from_two_points(pt1, pt2));
    //let c3 = topo.add_curve(line_from_two_points(pt4, pt5));

    //let _e1 = topo.add_edge_with_endpoints(c1, v2, v4);
    //let _e2 = topo.add_edge_with_endpoints(c2, v1, v2);
    //let _e3 = topo.add_edge_with_endpoints(c3, v4, v5);

    // Reflect the geometry

    let mirror = plane_from_standard_form(0., 1., 0., 0.).hat(); // Y = 0 plane
    ////let motor = ((point_from_xyz(0., 0., 0.) & point_from_xyz(0., 0., 1.)) * I).ihat().exp();

    let mirrored = reflect(topo.clone(), mirror);
    let topo = combine(&[topo, mirrored]).unwrap();

    //let f1 = topo.add_face(planar_face_from_edges(&topo.edges));

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

        draw_vertices(&mut window, &topo);
        draw_edges(&mut window, &topo);
        //draw_vertices(&mut window, &topo2);
        //draw_edges(&mut window, &topo2);
    }
}
