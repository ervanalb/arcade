use arcade::construct::{point_from_xyz, three_point_arc};

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

    let c = three_point_arc(pt0, pt1, pt2);
    println!("c: {:?}", c);
}
