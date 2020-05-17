import sympy

# What objects we want to represent

BASIS_COUNT = 16

objects = {
    "Scalar": [1] + [0] * 16,
    "Vector": [0] + [1] * 4 + [0] * 11,
    "Bivector": [0] * 5 + [1] * 6 + [0] * 5,
    "Trivector": [0] * 11 + [1] * 4 + [0],
    "FullMultivector": [1] * 16,
}

# What operations we want to generate

def reverse(a):
    return [
        a[0],
        a[1],
        a[2],
        a[3],
        a[4],
        -a[5],
        -a[6],
        -a[7],
        -a[8],
        -a[9],
        -a[10],
        -a[11],
        -a[12],
        -a[13],
        -a[14],
        a[15],
    ]

def dual(a):
    return list(reversed(a))

def conjugate(a):
    return [
        a[0],
        -a[1],
        -a[2],
        -a[3],
        -a[4],
        -a[5],
        -a[6],
        -a[7],
        -a[8],
        -a[9],
        -a[10],
        a[11],
        a[12],
        a[13],
        a[14],
        a[15],
    ]

def neg(a):
    return [-x for x in a]

def add(a, b):
    return [x + y for x, y in zip(a, b)]

def sub(a, b):
    return [x - y for x, y in zip(a, b)]

def geometric_product(a, b):
    return [
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
    ]

def wedge_product(a, b):
    return [
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
    ]

def dot_product(a, b):
    return [
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
    ]

def vee_product(a, b):
    return dual(wedge_product(dual(a), dual(b)))

unary_operations = {
    "negate": neg,
    "dual": dual,
    "conjugate": conjugate,
    "reverse": reverse,
}

binary_operations = {
    "add": add,
    "sub": sub,
    "geometric product": geometric_product,
    "wedge product": wedge_product,
    "vee product": vee_product,
    "dot product": dot_product,
}

# Helper functions

def get_multivector(letter, components=None):
    """ Returns a multivector (list of length BASIS_COUNT) containing sympy symbols.
    components can be 0 or 1 to specify which terms to include.
    """

    mv = sympy.symbols(" ".join("{}{:02d}".format(letter, i) for i in range(BASIS_COUNT)))
    if components is None:
        return mv
    return [x * y for (x, y) in zip(mv, components)] 

def get_object(mv):
    """ Takes a multivector (list of length BASIS_COUNT) and returns the name of the
    first object that can represent it (is non-zero in all the same places)
    """

    for (name, components) in objects.items():
        if [x * y for (x, y) in zip(mv, components)] == mv:
            return name

def indent(s, tabs=1):
    return "\n".join("    " * tabs + line for line in s.split("\n"))

def gen_code_for_mv(mv, components=None, sub_a=None, sub_b=None):
    subs = [
        ("a0", "a"),
        ("b0", "b"),
    ]

    if sub_a is not None:
        subs.append(("a", sub_a))
    
    if sub_b is not None:
        subs.append(("b", sub_b))

    result = []
    for i, x in enumerate(mv):
        if components is not None and components[i] == 0:
            assert x == 0, "Tried to set an element not present in type!"
            continue
        code = sympy.rust_code(x)
        for find, replace in subs:
            code = code.replace(find, replace)
        result.append(f"a{i}: {code},")

    return "\n".join(result)

def gen_unary_operator(obj_name, op_name):
    a = get_multivector("a", objects[obj_name])
    b = unary_operations[op_name](a)
    result_type = get_object(b)
    result_components = objects[result_type]
    result = indent(gen_code_for_mv(b, result_components), 2)
    rust_code = f"""fn {op_name}(self) -> {result_type} {{
    {result_type} {{
{result}
    }}
}}
"""
    return rust_code

def gen_binary_operator(op_name, function):
    for (a_name, a_components) in objects.items():
        for (b_name, b_components) in objects.items():
            a = get_multivector("a", a_components)
            b = get_multivector("b", b_components)
            c = function(a, b)
            print(a_name, op_name, b_name, "=", get_object(c))
            print("\n".join(str(x) for x in c))

#gen_unary_operator("Dual", dual)
#gen_binary_operator(".", dot_product)

generated_code = gen_unary_operator("Vector", "negate")

rust_template = f"""// ===========================================================================
// ======= THIS FILE IS AUTOGENERATED. PLEASE EDIT pga_gen.py INSTEAD ========
// ===========================================================================

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
use std::ops::{{Index, IndexMut, Neg, Mul, BitXor, BitAnd, BitOr}};

const BASIS_COUNT: usize = 16;

pub trait Multivector: fmt::Debug + Clone + Copy + PartialEq
    + Neg {{

    type Dual: Multivector;

    fn reverse(self) -> Self;
    fn dual(self) -> Self::Dual;
    fn conjugate(self) -> Self;
    fn to_full_multivector(self) -> FullMultivector;
}}

{generated_code}
"""

print(rust_template)
