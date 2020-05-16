// This module lays out the 3D projective geometric algebra (PGA)
// that we will be using. The metric is R(3,0,1).


// The following conventions are used for coefficients of the basis:
// a0
// + a1 * e0 + a2 * e1 + a3 * e2 + a4 * e3
// + a5 * e01 + a6 * e02 + a7 * e03 + a8 * e12 + a9 * e31 + a10 * e23
// + a11 * e021 + a12 * e013 + a13 * e032 + a14 * e123
// + a15 * e0123

use crate::global::Float;

const BASIS_COUNT: usize = 16;

pub trait Multivector {
    
}

// Scalars are just elements of Float_t

impl Multivector for Float {
}

// Vectors represent planes

pub struct Vector {
    a1: Float,
    a2: Float,
    a3: Float,
    a4: Float,
}

impl Multivector for Vector {
}

// Bivectors represent lines

pub struct Bivector {
    a5: Float, // e01
    a6: Float, // e02
    a7: Float, // e03
    a8: Float, // e04
    a9: Float, // e05
    a10: Float, // e06
}

impl Multivector for Bivector {
}

// Trivectors represent points

pub struct Trivector {
    a11: Float, // e021
    a12: Float, // e013
    a13: Float, // e032
    a14: Float, // e123
}

impl Multivector for Trivector {
}

// Pseudoscalars are not particularly useful so we don't specify them
// (they can still be represented through FullMultivectors)

// Full multivector

pub struct FullMultivector {
    a: [Float; BASIS_COUNT]
}

impl Multivector for FullMultivector {
}
