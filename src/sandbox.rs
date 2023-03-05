use arcade::construct::*;
//use arcade::interpolate::interpolate_curve_fixed;

fn main() {
    //let pt = point_from_xyz(1., 1., 1.);
    //let axis = point_from_xyz(2., 2., 0.) & point_from_xyz(2., 2., 1.);
    //let alpha = PI / 4.;
    //
    //let motor = ((alpha / 2.) * axis).exp();
    //println!("pt: {:?}", pt);
    //println!("motor: {:?}", motor);
    //println!("newpt: {:?}", motor.transform(pt));

    let pt0 = point_from_xyz(1., 1., 1.);
    let pt1 = point_from_xyz(2., 2., 1.);
    let pt2 = point_from_xyz(1., 3., 1.);

    let c = plane_from_three_points(pt0, pt1, pt2);
    println!("good plane: {:?}", c);

    let pt0 = point_from_xyz(1., 1., 1.);
    let pt1 = point_from_xyz(2., 2., 1.);
    let pt2 = point_from_xyz(3., 3.001, 1.);

    let c = plane_from_three_points(pt0, pt1, pt2);
    if c.norm() < EPSILON
    println!("bad plane: {:?}", c);

    //let c = circle_from_three_points(pt0, pt1, pt2);
    //println!("c: {:?}", c);
    //println!("t at pt0: {:?}", c.t(pt0));
    //println!("t at pt1: {:?}", c.t(pt1));
    //println!("t at pt2: {:?}", c.t(pt2));
    //println!("t at 0,2,1: {:?}", c.t(point_from_xyz(0., 2., 1.)));

    //let c = line_from_two_points(pt0, pt2);
    //println!("c: {:?}", c);
    //println!("t at pt0: {:?}", c.t(pt0));
    //println!("t at pt1: {:?}", c.t(pt1));
    //println!("t at pt2: {:?}", c.t(pt2));
    //let pts = interpolate_curve_fixed(&c, 10);
    //for pt in pts {
    //    println!("interp: {:?}", pt);
    //}
}
