use arcade::construct::*;
use arcade::pga::*;
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

    let o = point_from_xyz(0., 0., 0.);
    let x = point_from_xyz(1., 0., 0.);
    let y = point_from_xyz(0., 1., 0.);
    let z = point_from_xyz(0., 0., 1.);

    let pl = plane_from_three_points(o, y, z).hat();

    println!("pl: {:?}", pl);

    let po = o.project(pl);
    let px = x.project(pl);
    let py = y.project(pl);
    let pz = z.project(pl);

    println!("proj x: {:?}", (po & px).norm());
    println!("proj y: {:?}", (po & py).norm());
    println!("proj z: {:?}", (po & pz).norm());

    let primary = py;
    let secondary = pz;

    let lu = (po & primary).hat();
    println!("lu: {:?}", lu);

    // Redefine plane to get consistent ordering
    let pl = (lu & secondary).hat();

    let lv = pl ^ (lu | po);
    println!("lv: {:?}", lv);

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
