#[cfg(test)]

use super::*;
use vec::*;

use vertex::Vertex;
use edge::Segment;
use error::Error;

#[test]
fn vec_arithmetic() {
    // Basic arithmetic
    assert_eq!(Vec3::new(2., 3., 4.) + Vec3::new(9., -3., 0.), Vec3::new(11., 0., 4.));
    assert_eq!(Vec3::new(2., 3., 4.) - Vec3::new(9., -3., 0.), Vec3::new(-7., 6., 4.));
    assert_eq!(Vec3::new(2., 3., 4.) * 10., Vec3::new(20., 30., 40.));
    assert_eq!(-1. * Vec3::new(9., -3., 0.), Vec3::new(-9., 3., 0.));
    assert_eq!(Vec3::new(9., -3., 0.) / 3., Vec3::new(3., -1., 0.));
    assert_eq!(27. / Vec3::new(9., -3., 1.), Vec3::new(3., -9., 27.));

    assert_eq!(Vec2::new(2., 3.) + Vec2::new(9., -3.), Vec2::new(11., 0.));
    assert_eq!(Vec2::new(2., 3.) - Vec2::new(9., -3.), Vec2::new(-7., 6.));
    assert_eq!(Vec2::new(2., 3.) * 10., Vec2::new(20., 30.));
    assert_eq!(-1. * Vec2::new(9., -3.), Vec2::new(-9., 3.));
    assert_eq!(Vec2::new(9., -3.) / 3., Vec2::new(3., -1.));
    assert_eq!(27. / Vec2::new(9., -3.), Vec2::new(3., -9.));

    // is_nan
    assert!(Vec3::new(5., 6., 7.).is_finite());
    assert!(!Vec3::new(5., 6., std::f64::NAN).is_finite());
    assert!(Vec2::new(5., 6.).is_finite());
    assert!(!Vec2::new(5., std::f64::NAN).is_finite());

    // is_within
    assert!(!Vec3::new(-1., 2., -3.).is_within(2.5));
    assert!(Vec3::new(-1., 2., -3.).is_within(3.5));
    assert!(!Vec2::new(-1., -3.).is_within(2.5));
    assert!(Vec2::new(-1., -3.).is_within(3.5));

    // Matrix arithmetic
    assert_eq!(Mat2::new(1., 2., 3., 4.) * Vec2::new(5., 6.), Vec2::new(17., 39.));
    assert_eq!(Vec2::new(5., 6.) * Mat2::new(1., 2., 3., 4.), Vec2::new(23., 34.));
    assert_eq!(Mat2::new(10., 20., 30., 40.) * Mat2::new(1., 2., 3., 4.), Mat2::new(70., 100., 150., 220.));

    assert_eq!(Mat2::new(1., 2., 3., 4.).inv(), Mat2::new(-2., 1., 1.5, -0.5));
    assert_eq!(Mat2::new(1., 2., 2., 4.).det(), 0.);
    assert!(!Mat2::new(1., 2., 2., 4.).inv().is_finite());
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
