#[cfg(test)]

use super::*;

use vec::Vec3;
use vertex::Vertex;
use edge::Segment;
use error::Error;

#[test]
fn vec3_arithmetic() {
    // Basic arithmetic
    assert_eq!(Vec3::new(2., 3., 4.) + Vec3::new(9., -3., 0.), Vec3::new(11., 0., 4.));
    assert_eq!(Vec3::new(2., 3., 4.) - Vec3::new(9., -3., 0.), Vec3::new(-7., 6., 4.));
    assert_eq!(Vec3::new(2., 3., 4.) * 10., Vec3::new(20., 30., 40.));
    assert_eq!(-1. * Vec3::new(9., -3., 0.), Vec3::new(-9., 3., 0.));

    // is_nan
    assert!(Vec3::new(5., 6., 7.).is_finite());
    assert!(!Vec3::new(5., 6., std::f64::NAN).is_finite());

    // is_within
    assert!(!Vec3::new(-1., 2., -3.).is_within(2.5));
    assert!(Vec3::new(-1., 2., -3.).is_within(3.5));
}

#[test]
fn vertex_construction() {
    assert!(
        Vertex::new(limits::WORKSPACE_SIZE * Vec3::new(0.5, 0.9, -0.3))
        .is_ok());
    assert_eq!(
        Vertex::new(limits::WORKSPACE_SIZE * Vec3::new(0.5, 0.9, -1.3)).unwrap_err(),
        Error::OutOfBounds);
    assert_eq!(
        Vertex::new(Vec3::new(55., std::f64::NAN, -23.)).unwrap_err(),
        Error::NotANumber);

    let v = Vec3::new(0.5, 0.9, -0.3) * limits::WORKSPACE_SIZE;
    assert!(Vertex::new(v).unwrap().is_coincident(
        Vertex::new(v + Vec3::new(0.3, -0.9, -0.7) * limits::EPSILON_VERTEX_COINCIDENT).unwrap()));

    assert!(!Vertex::new(v).unwrap().is_coincident(
        Vertex::new(v + Vec3::new(1.3, -0.9, -1.7) * limits::EPSILON_VERTEX_COINCIDENT).unwrap()));
}

#[test]
fn segment_construction() {
    let v = limits::WORKSPACE_SIZE * Vec3::new(0.5, 0.9, -0.3);

    assert!(
        Segment::new(Vertex::new(v).unwrap(),
                     Vertex::new(v + limits::MINIMUM_VERTEX_SEPARATION * Vec3::new(1.5, 0.5, -2.)).unwrap())
        .is_ok());

    assert_eq!(
        Segment::new(Vertex::new(v).unwrap(),
                     Vertex::new(v + limits::MINIMUM_VERTEX_SEPARATION * Vec3::new(0.3, 0.5, -0.8)).unwrap()).unwrap_err(),
        Error::VerticesTooClose);
}
