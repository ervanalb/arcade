use arcade::pga::*;
use arcade::primitives::{point_from_xyz};
use std::f64::consts::PI;

fn main() {
    let pt = point_from_xyz(1., 1., 1.);
    let axis = point_from_xyz(2., 2., 0.) & point_from_xyz(2., 2., 1.);
    let alpha = PI / 4.;
    
    let motor = ((alpha / 2.) * axis).exp();
    println!("pt: {:?}", pt);
    println!("motor: {:?}", motor);
    println!("newpt: {:?}", motor.transform(pt));
}
