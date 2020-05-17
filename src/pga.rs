// This module lays out the 3D projective geometric algebra (PGA)
// that we will be using. The metric is R(3,0,1).


// The following conventions are used for coefficients of the basis:
// a0
// + a1 * e0 + a2 * e1 + a3 * e2 + a4 * e3
// + a5 * e01 + a6 * e02 + a7 * e03 + a8 * e12 + a9 * e31 + a10 * e23
// + a11 * e021 + a12 * e013 + a13 * e032 + a14 * e123
// + a15 * e0123

// Operators:
// * geometric product
// ^ wedge (meet)
// & vee (join)
// | dot

// This file heavily inspired by https://bivector.net/tools.html and their generated rust code.
// Hopefully the conventions here are the same as ganja.js and company,
// so there is good interoperability.
// One exception is that using ! to take the dual has been removed (use .dual())

use crate::global::Float;
use std::fmt;
use std::ops::{Index, IndexMut, Neg, Mul, BitXor, BitAnd, BitOr};

const BASIS_COUNT: usize = 16;

pub trait Multivector: fmt::Debug + Clone + Copy + PartialEq
    + Neg {

    type Dual: Multivector;

    fn reverse(self) -> Self;
    fn dual(self) -> Self::Dual;
    fn conjugate(self) -> Self;
    fn to_full_multivector(self) -> FullMultivector;
}

// ===========================================================================
// Scalars
// ===========================================================================

// Unary operations

impl Multivector for Float {
    type Dual = FullMultivector;

    fn reverse(self) -> Float {
        self
    }

    fn dual(self) -> FullMultivector {
        FullMultivector {a: [
            0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., self
        ]}
    }

    fn conjugate(self) -> Float {
        self
    }

    fn to_full_multivector(self) -> FullMultivector {
        FullMultivector {a: [
            self, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0. 
        ]}
    }
}

// Geometric Product Routines for LHS = Float

impl Mul<Vector> for Float {
    type Output = Vector;

    fn mul(self, b: Vector) -> Vector {
        Vector {
            a1: self * b.a1,
            a2: self * b.a2,
            a3: self * b.a3,
            a4: self * b.a4,
        }
    }
}

impl Mul<Bivector> for Float {
    type Output = Bivector;

    fn mul(self, b: Bivector) -> Bivector {
        Bivector {
            a5: self * b.a5,
            a6: self * b.a6,
            a7: self * b.a7,
            a8: self * b.a8,
            a9: self * b.a9,
            a10: self * b.a10,
        }
    }
}

impl Mul<Trivector> for Float {
    type Output = Trivector;

    fn mul(self, b: Trivector) -> Trivector {
        Trivector {
            a11: self * b.a11,
            a12: self * b.a12,
            a13: self * b.a13,
            a14: self * b.a14,
        }
    }
}

impl Mul<FullMultivector> for Float {
    type Output = FullMultivector;

    fn mul(self, b: FullMultivector) -> FullMultivector {
        let a = &self;
        FullMultivector {a: [
            a*b[0], a*b[1], a*b[2], a*b[3], a*b[4], a*b[5], a*b[6], a*b[7], a*b[8], a*b[9], a*b[10], a*b[11], a*b[12], a*b[13], a*b[14], a*b[15]
        ]}
    }
}

// Wedge Product Routines for LHS = Float

impl BitXor<Vector> for Float {
    type Output = FullMultivector;

    fn bitxor(self, b: Vector) -> FullMultivector {
        // Placeholder routine
        self.to_full_multivector() ^ b.to_full_multivector()
    }
}

impl BitXor<Bivector> for Float {
    type Output = FullMultivector;

    fn bitxor(self, b: Bivector) -> FullMultivector {
        // Placeholder routine
        self.to_full_multivector() ^ b.to_full_multivector()
    }
}

impl BitXor<Trivector> for Float {
    type Output = FullMultivector;

    fn bitxor(self, b: Trivector) -> FullMultivector {
        // Placeholder routine
        self.to_full_multivector() ^ b.to_full_multivector()
    }
}

impl BitXor<FullMultivector> for Float {
    type Output = FullMultivector;

    fn bitxor(self, b: FullMultivector) -> FullMultivector {
        // Placeholder routine
        self.to_full_multivector() ^ b
    }
}

// ===========================================================================
// Vectors
// ===========================================================================

#[derive(Default,Debug,Clone,Copy,PartialEq)]
pub struct Vector {
    a1: Float,
    a2: Float,
    a3: Float,
    a4: Float,
}

// Unary operations

impl Multivector for Vector {
    type Dual = Trivector;

    fn reverse(self) -> Vector {
        self
    }

    fn dual(self) -> Trivector {
        Trivector {
            a11: self.a4,
            a12: self.a3,
            a13: self.a2,
            a14: self.a1,
        }
    }

    fn conjugate(self) -> Vector {
        -self
    }

    fn to_full_multivector(self) -> FullMultivector {
        FullMultivector {a: [
            0., self.a1, self.a2, self.a3, self.a4, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0. 
        ]}
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector {
            a1: -self.a1,
            a2: -self.a2,
            a3: -self.a3,
            a4: -self.a4,
        }
    }
}

// ===========================================================================
// Bivectors
// ===========================================================================

#[derive(Default,Debug,Clone,Copy,PartialEq)]
pub struct Bivector {
    a5: Float, // e01
    a6: Float, // e02
    a7: Float, // e03
    a8: Float, // e04
    a9: Float, // e05
    a10: Float, // e06
}

// Unary operations

impl Multivector for Bivector {
    type Dual = Bivector;

    fn reverse(self) -> Bivector {
        -self
    }

    fn dual(self) -> Bivector {
        Bivector {
            a5: self.a10,
            a6: self.a9,
            a7: self.a8,
            a8: self.a7,
            a9: self.a6,
            a10: self.a5,
        }
    }

    fn conjugate(self) -> Bivector {
        -self
    }

    fn to_full_multivector(self) -> FullMultivector {
        FullMultivector {a: [
            0., 0., 0., 0., 0., self.a5, self.a6, self.a7, self.a8, self.a9, self.a10, 0., 0., 0., 0., 0. 
        ]}
    }
}

impl Neg for Bivector {
    type Output = Bivector;

    fn neg(self) -> Bivector {
        Bivector {
            a5: -self.a5,
            a6: -self.a6,
            a7: -self.a7,
            a8: -self.a8,
            a9: -self.a9,
            a10: -self.a10,
        }
    }
}

// ===========================================================================
// Trivectors
// ===========================================================================

#[derive(Default,Debug,Clone,Copy,PartialEq)]
pub struct Trivector {
    a11: Float, // e021
    a12: Float, // e013
    a13: Float, // e032
    a14: Float, // e123
}

// Unary operations

impl Multivector for Trivector {
    type Dual = Vector;

    fn reverse(self) -> Trivector {
        -self
    }

    fn dual(self) -> Vector {
        Vector {
            a1: self.a14,
            a2: self.a13,
            a3: self.a12,
            a4: self.a11,
        }
    }

    fn conjugate(self) -> Trivector {
        self
    }

    fn to_full_multivector(self) -> FullMultivector {
        FullMultivector {a: [
            0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., self.a11, self.a12, self.a13, self.a14, 0. 
        ]}
    }
}

impl Neg for Trivector {
    type Output = Trivector;

    fn neg(self) -> Trivector {
        Trivector {
            a11: -self.a11,
            a12: -self.a12,
            a13: -self.a13,
            a14: -self.a14,
        }
    }
}

// ===========================================================================
// Full multivectors
// ===========================================================================

// Pseudoscalars are not particularly useful so we don't specify them
// (they can still be represented through FullMultivectors)

#[derive(Default,Debug,Clone,Copy,PartialEq)]
pub struct FullMultivector {
    a: [Float; BASIS_COUNT]
}

impl Index<usize> for FullMultivector {
    type Output = Float;

    fn index<'a>(&'a self, index: usize) -> &'a Self::Output {
        &self.a[index]
    }
}

impl IndexMut<usize> for FullMultivector {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut Self::Output {
        &mut self.a[index]
    }
}

// Unary operations

impl Multivector for FullMultivector {
    type Dual = FullMultivector;

    fn reverse(self) -> FullMultivector {
        FullMultivector {a: [
            self[0],
            self[1],
            self[2],
            self[3],
            self[4],
            -self[5],
            -self[6],
            -self[7],
            -self[8],
            -self[9],
            -self[10],
            -self[11],
            -self[12],
            -self[13],
            -self[14],
            self[15],
        ]}
    }

    fn dual(self) -> FullMultivector {
        FullMultivector {a: [
            self[15],
            self[14],
            self[13],
            self[12],
            self[11],
            self[10],
            self[9],
            self[8],
            self[7],
            self[6],
            self[5],
            self[4],
            self[3],
            self[2],
            self[1],
            self[0],
        ]}
    }

    fn conjugate(self) -> FullMultivector {
        FullMultivector {a: [
            self[0],
            -self[1],
            -self[2],
            -self[3],
            -self[4],
            -self[5],
            -self[6],
            -self[7],
            -self[8],
            -self[9],
            -self[10],
            self[11],
            self[12],
            self[13],
            self[14],
            self[15],
        ]}
    }

    fn to_full_multivector(self) -> FullMultivector {
        self
    }
}

impl Neg for FullMultivector {
    type Output = FullMultivector;

    fn neg(self) -> FullMultivector {
        FullMultivector {a: [
            -self[0],
            -self[1],
            -self[2],
            -self[3],
            -self[4],
            -self[5],
            -self[6],
            -self[7],
            -self[8],
            -self[9],
            -self[10],
            -self[11],
            -self[12],
            -self[13],
            -self[14],
            -self[15],
        ]}
    }
}

// Geometric Product Routines for LHS = FullMultivector

impl Mul for FullMultivector {
    type Output = FullMultivector;

    // These taken straight from enki's generated code:
    fn mul(self, b: FullMultivector) -> FullMultivector {
        let a = &self;
        FullMultivector {a: [
            b[0]*a[0]+b[2]*a[2]+b[3]*a[3]+b[4]*a[4]-b[8]*a[8]-b[9]*a[9]-b[10]*a[10]-b[14]*a[14],
            b[1]*a[0]+b[0]*a[1]-b[5]*a[2]-b[6]*a[3]-b[7]*a[4]+b[2]*a[5]+b[3]*a[6]+b[4]*a[7]+b[11]*a[8]+b[12]*a[9]+b[13]*a[10]+b[8]*a[11]+b[9]*a[12]+b[10]*a[13]+b[15]*a[14]-b[14]*a[15],
            b[2]*a[0]+b[0]*a[2]-b[8]*a[3]+b[9]*a[4]+b[3]*a[8]-b[4]*a[9]-b[14]*a[10]-b[10]*a[14],
            b[3]*a[0]+b[8]*a[2]+b[0]*a[3]-b[10]*a[4]-b[2]*a[8]-b[14]*a[9]+b[4]*a[10]-b[9]*a[14],
            b[4]*a[0]-b[9]*a[2]+b[10]*a[3]+b[0]*a[4]-b[14]*a[8]+b[2]*a[9]-b[3]*a[10]-b[8]*a[14],
            b[5]*a[0]+b[2]*a[1]-b[1]*a[2]-b[11]*a[3]+b[12]*a[4]+b[0]*a[5]-b[8]*a[6]+b[9]*a[7]+b[6]*a[8]-b[7]*a[9]-b[15]*a[10]-b[3]*a[11]+b[4]*a[12]+b[14]*a[13]-b[13]*a[14]-b[10]*a[15],
            b[6]*a[0]+b[3]*a[1]+b[11]*a[2]-b[1]*a[3]-b[13]*a[4]+b[8]*a[5]+b[0]*a[6]-b[10]*a[7]-b[5]*a[8]-b[15]*a[9]+b[7]*a[10]+b[2]*a[11]+b[14]*a[12]-b[4]*a[13]-b[12]*a[14]-b[9]*a[15],
            b[7]*a[0]+b[4]*a[1]-b[12]*a[2]+b[13]*a[3]-b[1]*a[4]-b[9]*a[5]+b[10]*a[6]+b[0]*a[7]-b[15]*a[8]+b[5]*a[9]-b[6]*a[10]+b[14]*a[11]-b[2]*a[12]+b[3]*a[13]-b[11]*a[14]-b[8]*a[15],
            b[8]*a[0]+b[3]*a[2]-b[2]*a[3]+b[14]*a[4]+b[0]*a[8]+b[10]*a[9]-b[9]*a[10]+b[4]*a[14],
            b[9]*a[0]-b[4]*a[2]+b[14]*a[3]+b[2]*a[4]-b[10]*a[8]+b[0]*a[9]+b[8]*a[10]+b[3]*a[14],
            b[10]*a[0]+b[14]*a[2]+b[4]*a[3]-b[3]*a[4]+b[9]*a[8]-b[8]*a[9]+b[0]*a[10]+b[2]*a[14],
            b[11]*a[0]-b[8]*a[1]+b[6]*a[2]-b[5]*a[3]+b[15]*a[4]-b[3]*a[5]+b[2]*a[6]-b[14]*a[7]-b[1]*a[8]+b[13]*a[9]-b[12]*a[10]+b[0]*a[11]+b[10]*a[12]-b[9]*a[13]+b[7]*a[14]-b[4]*a[15],
            b[12]*a[0]-b[9]*a[1]-b[7]*a[2]+b[15]*a[3]+b[5]*a[4]+b[4]*a[5]-b[14]*a[6]-b[2]*a[7]-b[13]*a[8]-b[1]*a[9]+b[11]*a[10]-b[10]*a[11]+b[0]*a[12]+b[8]*a[13]+b[6]*a[14]-b[3]*a[15],
            b[13]*a[0]-b[10]*a[1]+b[15]*a[2]+b[7]*a[3]-b[6]*a[4]-b[14]*a[5]-b[4]*a[6]+b[3]*a[7]+b[12]*a[8]-b[11]*a[9]-b[1]*a[10]+b[9]*a[11]-b[8]*a[12]+b[0]*a[13]+b[5]*a[14]-b[2]*a[15],
            b[14]*a[0]+b[10]*a[2]+b[9]*a[3]+b[8]*a[4]+b[4]*a[8]+b[3]*a[9]+b[2]*a[10]+b[0]*a[14],
            b[15]*a[0]+b[14]*a[1]+b[13]*a[2]+b[12]*a[3]+b[11]*a[4]+b[10]*a[5]+b[9]*a[6]+b[8]*a[7]+b[7]*a[8]+b[6]*a[9]+b[5]*a[10]-b[4]*a[11]-b[3]*a[12]-b[2]*a[13]-b[1]*a[14]+b[0]*a[15],
        ]}
    }
}

// Wedge Product Routines for LHS = FullMultivector

impl BitXor<FullMultivector> for FullMultivector {
    type Output = FullMultivector;

    // These taken straight from enki's generated code:
    fn bitxor(self, b: FullMultivector) -> FullMultivector {
        let a = &self;
        FullMultivector {a: [
            b[0]*a[0],
            b[1]*a[0]+b[0]*a[1],
            b[2]*a[0]+b[0]*a[2],
            b[3]*a[0]+b[0]*a[3],
            b[4]*a[0]+b[0]*a[4],
            b[5]*a[0]+b[2]*a[1]-b[1]*a[2]+b[0]*a[5],
            b[6]*a[0]+b[3]*a[1]-b[1]*a[3]+b[0]*a[6],
            b[7]*a[0]+b[4]*a[1]-b[1]*a[4]+b[0]*a[7],
            b[8]*a[0]+b[3]*a[2]-b[2]*a[3]+b[0]*a[8],
            b[9]*a[0]-b[4]*a[2]+b[2]*a[4]+b[0]*a[9],
            b[10]*a[0]+b[4]*a[3]-b[3]*a[4]+b[0]*a[10],
            b[11]*a[0]-b[8]*a[1]+b[6]*a[2]-b[5]*a[3]-b[3]*a[5]+b[2]*a[6]-b[1]*a[8]+b[0]*a[11],
            b[12]*a[0]-b[9]*a[1]-b[7]*a[2]+b[5]*a[4]+b[4]*a[5]-b[2]*a[7]-b[1]*a[9]+b[0]*a[12],
            b[13]*a[0]-b[10]*a[1]+b[7]*a[3]-b[6]*a[4]-b[4]*a[6]+b[3]*a[7]-b[1]*a[10]+b[0]*a[13],
            b[14]*a[0]+b[10]*a[2]+b[9]*a[3]+b[8]*a[4]+b[4]*a[8]+b[3]*a[9]+b[2]*a[10]+b[0]*a[14],
            b[15]*a[0]+b[14]*a[1]+b[13]*a[2]+b[12]*a[3]+b[11]*a[4]+b[10]*a[5]+b[9]*a[6]+b[8]*a[7]+b[7]*a[8]+b[6]*a[9]+b[5]*a[10]-b[4]*a[11]-b[3]*a[12]-b[2]*a[13]-b[1]*a[14]+b[0]*a[15],
        ]}
    }
}

// Vee Product Routines for LHS = FullMultivector

impl BitAnd<FullMultivector> for FullMultivector {
    type Output = FullMultivector;

    // These taken straight from enki's generated code:
    fn bitand(self, b: FullMultivector) -> FullMultivector {
        let a = &self;
        FullMultivector {a: [
            b[0]*a[15]+b[1]*a[14]+b[2]*a[13]+b[3]*a[12]+b[4]*a[11]+b[5]*a[10]+b[6]*a[9]+b[7]*a[8]+b[8]*a[7]+b[9]*a[6]+b[10]*a[5]-b[11]*a[4]-b[12]*a[3]-b[13]*a[2]-b[14]*a[1]+b[15]*a[0],
            b[1]*a[15]+b[5]*a[13]+b[6]*a[12]+b[7]*a[11]+b[11]*a[7]+b[12]*a[6]+b[13]*a[5]+b[15]*a[1],
            b[2]*a[15]-b[5]*a[14]+b[8]*a[12]-b[9]*a[11]-b[11]*a[9]+b[12]*a[8]-b[14]*a[5]+b[15]*a[2],
            b[3]*a[15]-b[6]*a[14]-b[8]*a[13]+b[10]*a[11]+b[11]*a[10]-b[13]*a[8]-b[14]*a[6]+b[15]*a[3],
            b[4]*a[15]-b[7]*a[14]+b[9]*a[13]-b[10]*a[12]-b[12]*a[10]+b[13]*a[9]-b[14]*a[7]+b[15]*a[4],
            b[5]*a[15]+b[11]*a[12]-b[12]*a[11]+b[15]*a[5],
            b[6]*a[15]-b[11]*a[13]+b[13]*a[11]+b[15]*a[6],
            b[7]*a[15]+b[12]*a[13]-b[13]*a[12]+b[15]*a[7],
            b[8]*a[15]+b[11]*a[14]-b[14]*a[11]+b[15]*a[8],
            b[9]*a[15]+b[12]*a[14]-b[14]*a[12]+b[15]*a[9],
            b[10]*a[15]+b[13]*a[14]-b[14]*a[13]+b[15]*a[10],
            b[11]*a[15]+b[15]*a[11],
            b[12]*a[15]+b[15]*a[12],
            b[13]*a[15]+b[15]*a[13],
            b[14]*a[15]+b[15]*a[14],
            b[15]*a[15],
        ]}
    }
}

// Dot Product Routines for LHS = FullMultivector

impl BitOr<FullMultivector> for FullMultivector {
    type Output = FullMultivector;

    // These taken straight from enki's generated code:
    fn bitor(self, b: FullMultivector) -> FullMultivector {
        let a = &self;
        FullMultivector {a: [
            b[0]*a[0]+b[2]*a[2]+b[3]*a[3]+b[4]*a[4]-b[8]*a[8]-b[9]*a[9]-b[10]*a[10]-b[14]*a[14],
            b[1]*a[0]+b[0]*a[1]-b[5]*a[2]-b[6]*a[3]-b[7]*a[4]+b[2]*a[5]+b[3]*a[6]+b[4]*a[7]+b[11]*a[8]+b[12]*a[9]+b[13]*a[10]+b[8]*a[11]+b[9]*a[12]+b[10]*a[13]+b[15]*a[14]-b[14]*a[15],
            b[2]*a[0]+b[0]*a[2]-b[8]*a[3]+b[9]*a[4]+b[3]*a[8]-b[4]*a[9]-b[14]*a[10]-b[10]*a[14],
            b[3]*a[0]+b[8]*a[2]+b[0]*a[3]-b[10]*a[4]-b[2]*a[8]-b[14]*a[9]+b[4]*a[10]-b[9]*a[14],
            b[4]*a[0]-b[9]*a[2]+b[10]*a[3]+b[0]*a[4]-b[14]*a[8]+b[2]*a[9]-b[3]*a[10]-b[8]*a[14],
            b[5]*a[0]-b[11]*a[3]+b[12]*a[4]+b[0]*a[5]-b[15]*a[10]-b[3]*a[11]+b[4]*a[12]-b[10]*a[15],
            b[6]*a[0]+b[11]*a[2]-b[13]*a[4]+b[0]*a[6]-b[15]*a[9]+b[2]*a[11]-b[4]*a[13]-b[9]*a[15],
            b[7]*a[0]-b[12]*a[2]+b[13]*a[3]+b[0]*a[7]-b[15]*a[8]-b[2]*a[12]+b[3]*a[13]-b[8]*a[15],
            b[8]*a[0]+b[14]*a[4]+b[0]*a[8]+b[4]*a[14],
            b[9]*a[0]+b[14]*a[3]+b[0]*a[9]+b[3]*a[14],
            b[10]*a[0]+b[14]*a[2]+b[0]*a[10]+b[2]*a[14],
            b[11]*a[0]+b[15]*a[4]+b[0]*a[11]-b[4]*a[15],
            b[12]*a[0]+b[15]*a[3]+b[0]*a[12]-b[3]*a[15],
            b[13]*a[0]+b[15]*a[2]+b[0]*a[13]-b[2]*a[15],
            b[14]*a[0]+b[0]*a[14],
            b[15]*a[0]+b[0]*a[15],
        ]}
    }
}
