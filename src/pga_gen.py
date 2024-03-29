import sympy

# What objects we want to represent

BASIS_COUNT = 16

objects = {
    "Float": [1] + [0] * 15,
    "Vector": [0] + [1] * 4 + [0] * 11,
    "Bivector": [0] * 5 + [1] * 6 + [0] * 5,
    "Trivector": [0] * 11 + [1] * 4 + [0],
    "Pseudoscalar": [0] * 15 + [1],
    "ScalarAndBivector": [1] + [0] * 4 + [1] * 6 + [0] * 5,
    "FullMultivector": [1] * 16,
}

# What operations we want to generate

def elem_mul(a, b):
    return [x * y for x, y in zip(a, b)]

def reverse(a):
    return elem_mul(
        [1, 1, 1, 1, 1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 1],
        a
    )

def dual(a):
    return list(reversed(a))

def conjugate(a):
    return elem_mul(
        [1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 1, 1, 1, 1, 1],
        a
    )

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

def norm(a):
    r = geometric_product(a, reverse(a))
    return [sympy.sqrt(r[0])] + [0 for _ in range(BASIS_COUNT - 1)]

def inorm(a):
    return norm(dual(a))

def select_object(obj):
    def _select(a):
        return elem_mul(objects[obj], a)
    return _select

def project(a, b):
    return geometric_product(dot_product(a, b), b)

def reflect(a, b):
    return geometric_product(geometric_product(b, a), b)

def transform(a, b):
    return geometric_product(geometric_product(b, a), reverse(b))

unary_operations = {
    "neg": neg,
    "dual": dual,
    "conjugate": conjugate,
    "reverse": reverse,
    "norm": norm,
    "inorm": inorm,
    "scalar": select_object("Float"),
    "vector": select_object("Vector"),
    "bivector": select_object("Bivector"),
    "trivector": select_object("Trivector"),
    "pseudoscalar": select_object("Pseudoscalar"),
    "full_multivector": lambda x: x,
}

binary_operations = {
    "add": add,
    "sub": sub,
    "mul": geometric_product,
    "bitxor": wedge_product,
    "bitand": vee_product,
    "bitor": dot_product,
    "project": project,
    "reflect": reflect,
    "transform": transform,
}

# Code gen helper functions

def get_multivector(letter, components=None):
    """ Returns a multivector (list of length BASIS_COUNT) containing sympy symbols.
    components can be 0 or 1 to specify which terms to include.
    """

    mv = sympy.symbols(" ".join("{}{:02d}".format(letter, i) for i in range(BASIS_COUNT)), real=True)
    if components is None:
        return mv
    return [x * y for (x, y) in zip(mv, components)] 

def simplify_mv(mv):
    """ Takes a multivector (list of length BASIS_COUNT) and simplifies each component of it,
    returning the simplified multivector.
    """

    return [sympy.simplify(c) for c in mv]

def get_object(mv):
    """ Takes a multivector (list of length BASIS_COUNT) and returns the name of the
    first object that can represent it (is non-zero in all the same places)
    """

    for (name, components) in objects.items():
        if [x * y for (x, y) in zip(mv, components)] == mv:
            return name

def is_object(mv, obj):
    """ Takes a multivector (list of length BASIS_COUNT) and returns whether it can be expressed
    using the given object name.
    """

    components = objects[obj]
    return [x * y for (x, y) in zip(mv, components)] == mv

def indent(s, tabs=1):
    """ Takes a string of code and indents each line by four spaces
    for every "tab" specified in arguments.
    """

    return "\n".join("    " * tabs + line if line else "" for line in s.split("\n"))

def fix_float(s):
    """ Replaces bare "0" with "0." for rust. """

    s = s.replace("*", " * ")

    def fix(term):
        import sys
        out = ""
        i = 0
        needs_dot = False
        while i < len(term) and term[i] in "(0123456789.-":
            if term[i] in "0123456789":
                needs_dot = True
            if term[i] in ".":
                needs_dot = False
                break

            out += term[i]
            i += 1
        if needs_dot:
            out += "." + term[i:]
        else:
            out = term
        return out

    return " ".join(fix(term) for term in s.split(" "))

def gen_code_for_mv(mv, components=None, sub_a="a", sub_b="b", a_is_scalar=False, b_is_scalar=False, result_is_scalar=False):
    """ Generates code for the inner part of a multivector return value.
    See gen_result_code for full return value generation.

    mv: The sympy multivector to represent
    components: Array of 0/1 to select which components to output
    sub_a: What to call the sympy "a" variable (e.g. "self")
    sub_b: What to call the sympy "b" variable (e.g. "rhs")
    a_is_scalar: Flag to avoid generating property accesses on a ("self.a0" becomes just "self")
    b_is_scalar: Flag to avoid generating property accesses on b ("rhs.a0" becomes just "rhs")
    result_is_scalar: Flag to avoid generating property assignments on the reuslt ("a0: 0." becomes just "0.")
    """

    subs = [
        ("sym_a0", "sym_a"),
        ("sym_b0", "sym_b"),
        ("sym_a", sub_a + ".a"),
        ("sym_b", sub_b + ".a"),
    ]

    if a_is_scalar:
        subs.append((sub_a + ".a0", sub_a))
    if b_is_scalar:
        subs.append((sub_b + ".a0", sub_b))

    if result_is_scalar:
        # Special-case scalars
        code = fix_float(sympy.rust_code(mv[0]))
        for find, replace in subs:
            code = code.replace(find, replace)
        #code = code.replace(".a0", "")
        return code
    else:
        result = []
        for i, x in enumerate(mv):
            if components is not None and components[i] == 0:
                assert x == 0, "Tried to set an element not present in type!"
                continue
            code = fix_float(sympy.rust_code(x))
            for find, replace in subs:
                code = code.replace(find, replace)
            result.append(f"a{i}: {code},")

        return "\n".join(result)

def gen_result_code(result_type, result, *args, **kwargs):
    """ Generates code for a multivector return value.
    See gen_code_for_mv for the inner part of this return value generation.
    This function just wraps it with the type name (or leaves it bare in the case of Float)
    """

    result_components = objects[result_type]
    result_elems = gen_code_for_mv(result, result_components, *args, **kwargs)

    if result_type == "Float":
        return result_elems

    return f"""{result_type} {{
{indent(result_elems)}
}}"""

def gen_unary_operator(obj_name, op_name, impl=None, result_type=None, pub=False):
    """ Generates code for a unary operator.
    obj_name: which struct to implement this operator on (type of "self")
    op_name: what to name the function, and an index into unary_operations
    impl: what trait this operator is implementing (optional)
    full_result: whether the result should be a FullMultivector or a smaller type if available
    """

    a = get_multivector("sym_a", objects[obj_name])
    b = unary_operations[op_name](a)
    b = simplify_mv(b)
    if b is None:
        return
    if result_type == None:
        result_type = get_object(b)

    pub_str = "pub " if pub else ""

    result_code = gen_result_code(
        result_type, b, sub_a="self",
        a_is_scalar=obj_name == "Float",
        result_is_scalar=result_type == "Float"
    )
    rust_code = f"""{pub_str}fn {op_name}(self) -> {result_type} {{
{indent(result_code)}
}}"""
    if impl is not None:
        rust_code = wrap_impl(rust_code, obj_name, impl, types=[("Output", result_type)])

    return rust_code

def gen_binary_operator(obj_name, rhs_obj_name, op_name, impl=None, pub=False, require_output_type=None):
    """ Generates code for a binary operator.
    obj_name: which struct to implement this operator on (type of "self")
    rhs_obj_name: type of second argument (right-hand-side of operator)
    op_name: what to name the function, and an index into binary_operations
    impl: what trait this operator is implementing (optional)
    """

    a = get_multivector("sym_a", objects[obj_name])
    b = get_multivector("sym_b", objects[rhs_obj_name])
    c = binary_operations[op_name](a, b)
    c = simplify_mv(c)

    if require_output_type is None:
        result_type = get_object(c)
    else:
        if is_object(c, require_output_type):
            result_type = require_output_type
        else:
            # This operator does not apply
            return None

    result_code = gen_result_code(
        result_type, c, sub_a="self", sub_b="r",
        a_is_scalar=obj_name == "Float",
        b_is_scalar=rhs_obj_name == "Float",
        result_is_scalar=result_type == "Float",
    )
    underscore = "" if "r" in result_code.replace(result_type, "") else "_" # A bit of a hack, might not catch all cases of unused "r"

    pub_str = "pub " if pub else ""

    rust_code = f"""fn {op_name}(self, {underscore}r: {rhs_obj_name}) -> {result_type} {{
{indent(result_code)}
}}"""

    if impl is not None:
        types = [("Output", result_type)] if require_output_type is None else None
        rust_code = wrap_impl(rust_code, obj_name, impl, template=rhs_obj_name, types=types)
    return rust_code

def wrap_impl(rust_code, obj_name, impl_name=None, template=None, types=None):
    """ Wraps rust code in an "impl" block.
    rust_code: the code to wrap
    obj_name: which struct to implement this trait on (type of "self")
    impl_name: the trait to implement
    template: impl template arguments (string) (e.g. RHS type for binary operators)
    types: any type aliases to put at the top of the impl (array of pairs of strings, e.g. [("Output", "Float")])
    """

    nl = "\n"
    types_str = ""
    if types is not None:
        types_str = "\n".join(f"    type {l} = {r};" for (l, r) in types) + "\n\n"
    return f"""impl{" " + impl_name if impl_name is not None else ""}{"<" + template + ">" if template is not None else ""}{" for" if impl_name is not None else ""} {obj_name} {{
{types_str}{indent(rust_code)}
}}"""

def struct(obj_name):
    """ Generates the "struct" definition for the given type. """

    components = objects[obj_name]
    a = "".join("    a{}: Float,\n".format(i) for i in range(BASIS_COUNT) if components[i])
    return f"""
#[derive(Default,Debug,Clone,Copy,PartialEq)]
pub struct {obj_name} {{
{a}}}"""

def gen_reverse_ops(obj_name):
    """ Generates the "Reverse" trait implementation for the given type """

    ops = ["reverse"]

    rust_code = [
        gen_unary_operator(obj_name, op)
        for op in ops
    ]

    rust_code = "\n\n".join(x for x in rust_code if x is not None)

    return wrap_impl(rust_code, obj_name, "Reverse")


def gen_dual_ops(obj_name):
    """ Generates the "Dual" trait implementation for the given type """

    ops = ["dual"]

    rust_code = [
        gen_unary_operator(obj_name, op)
        for op in ops
    ]

    dual_type = get_object(dual(get_multivector("a", objects[obj_name])))

    rust_code = "\n\n".join(x for x in rust_code if x is not None)

    return wrap_impl(rust_code, obj_name, "Dual", types=[("Output", dual_type)])

def gen_conjugate_ops(obj_name):
    """ Generates the "Conjugate" trait implementation for the given type """

    ops = ["conjugate"]

    rust_code = [
        gen_unary_operator(obj_name, op)
        for op in ops
    ]

    rust_code = "\n\n".join(x for x in rust_code if x is not None)

    return wrap_impl(rust_code, obj_name, "Conjugate")

def gen_normalize_ops(obj_name):
    """ Generates the "Normalize" trait implementation for the given type """

    ops = ["norm"]

    rust_code = [
        gen_unary_operator(obj_name, op)
        for op in ops
    ]

    rust_code = "\n\n".join(x for x in rust_code if x is not None)

    return wrap_impl(rust_code, obj_name, "Normalize")

def gen_normalize_infinite_ops(obj_name):
    """ Generates the "NormalizeInfinite" trait implementation for the given type """

    ops = ["inorm"]

    rust_code = [
        gen_unary_operator(obj_name, op)
        for op in ops
    ]

    rust_code = "\n\n".join(x for x in rust_code if x is not None)

    return wrap_impl(rust_code, obj_name, "NormalizeInfinite")

def gen_impl_ops(obj_name):
    """ Generates the non-trait-based implementation for the given type. """

    rust_code = []

    # First generate a "new" constructor

    var_names = [f"a{i}" for i, e in enumerate(objects[obj_name]) if e]
    args = ", ".join([f"{v}: Float" for v in var_names])
    constructor_code = ", ".join(var_names)
    rust_code += [
        f"""pub fn new({args}) -> {obj_name} {{
    {obj_name} {{{constructor_code}}}
}}"""
    ]

    # And a "zero" constructor

    rust_code += [
        f"""pub fn zero() -> {obj_name} {{
    Default::default()
}}"""
    ]

    # And "as_tuple"

    var_names = [f"a{i}" for i, e in enumerate(objects[obj_name]) if e]
    return_type = ", ".join(["Float"] * len(var_names))
    return_tuple = ", ".join([f"self.{v}" for v in var_names])
    if len(var_names) > 1:
        return_type = f"({return_type})"
        return_tuple = f"({return_tuple})"
    rust_code += [
        f"""pub fn as_tuple(&self) -> {return_type} {{
    {return_tuple}
}}"""
    ]

    # Now generate some additional functions

    mv_ops = ["scalar", "vector", "bivector", "trivector", "full_multivector"]

    force_type = {
        "scalar": "Float",
        "vector": "Vector",
        "bivector": "Bivector",
        "trivector": "Trivector",
        "full_multivector": "FullMultivector",
    }

    def op_isnt_trivial(op):
        if op not in force_type:
            # Not a grade selector function
            return True

        elements_input = objects[obj_name]
        elements_output = objects[force_type[op]]

        if elements_input == elements_output:
            # Returning the same type that we are
            return False

        if not any([a and b for (a, b) in zip(elements_input, elements_output)]):
            # Returning zero
            return False

        return True

    mv_ops = [op for op in mv_ops if op_isnt_trivial(op)]

    rust_code += [
        gen_unary_operator(obj_name, op, result_type=force_type.get(op), pub=True)
        for op in mv_ops
    ]

    rust_code = "\n\n".join(x for x in rust_code if x is not None)

    return wrap_impl(rust_code, obj_name)

#def gen_impl_ops(obj_name):
#    """ Generates non-trait-based implementation for the given type. """
#
#    mv_ops = ["reverse", "dual", "conjugate", "norm", "inorm",
#        "scalar", "vector", "bivector", "trivector", "full_multivector"]
#
#    force_type = {
#        "scalar": "Float",
#        "vector": "Vector",
#        "bivector": "Bivector",
#        "trivector": "Trivector",
#        "full_multivector": "FullMultivector",
#    }
#
#    dual_type = get_object(dual(get_multivector("a", objects[obj_name])))
#
#    rust_code = [
#        gen_unary_operator(obj_name, op, result_type=force_type.get(op))
#        for op in mv_ops
#    ]
#
#    rust_code = "\n\n".join(x for x in rust_code if x is not None)
#
#    #return wrap_impl(rust_code, obj_name, "Multivector", types=[("Dual", dual_type)])
#    return wrap_impl(rust_code, obj_name)
#    ops = []
#
#    rust_code = [
#        gen_unary_operator(obj_name, op)
#        for op in ops
#    ]
#
#    rust_code = "\n\n".join(x for x in rust_code if x is not None)
#
#    return wrap_impl(rust_code, obj_name)

def gen_unary_arithmetic(obj_name):
    """ Generates overloaded unary arithemtic operators for the given type. """

    arith_ops = [("neg", "Neg")]

    return "\n".join(gen_unary_operator(obj_name, op, impl) for (op, impl) in arith_ops)

def gen_binary_arithmetic(obj_name):
    """ Generates overloaded binary arithemtic operators for the given type. """

    arith_ops = [
        ("add", "Add"),
        ("sub", "Sub"),
        ("mul", "Mul"),
        ("bitxor", "BitXor"),
        ("bitand", "BitAnd"),
        ("bitor", "BitOr"),
    ]

    blocks = []

    for (op, impl) in arith_ops:
        for rhs in objects:
            if obj_name == "Float" and rhs == "Float":
                # Do not re-implement Float+Float
                continue
            blocks.append(gen_binary_operator(
                obj_name,
                rhs,
                op,
                impl,
            ))

    return "\n\n".join(x for x in blocks if x is not None)

def gen_special_binary_operators(obj_name):
    """ Generates special binary operators for the given type. """

    ops = [
        ("project", "Project"),
        ("reflect", "Reflect"),
        ("transform", "Transform"),
    ]

    blocks = []

    for (op, impl) in ops:
        for rhs in objects:
            blocks.append(gen_binary_operator(
                obj_name,
                rhs,
                op,
                impl,
                require_output_type = obj_name,
            ))

    return "\n\n".join(x for x in blocks if x is not None)

def main():
    """ Generates the pga.rs file, containing:
      * Header shown below, including the Multivector trait
      * Struct definitions for each type specified in "objects" at the top of this file
      * Non-trait-based implementation for each object
      * Implementation of the Multivector trait for each object
      * Implementation of overloaded unary operators for each object
      * Implementation of overloaded binary operators for each pair of objects

    The result is printed to stdout.
    """

    blocks = []
    for obj in objects:
        header = f"""// ===========================================================================
// {obj}
// ==========================================================================="""
        blocks.append(header)
        if obj != "Float":
            blocks.append(struct(obj))
        blocks.append(gen_reverse_ops(obj))
        blocks.append(gen_dual_ops(obj))
        blocks.append(gen_conjugate_ops(obj))
        blocks.append(gen_normalize_ops(obj))
        blocks.append(gen_normalize_infinite_ops(obj))
        if obj != "Float":
            blocks.append(gen_impl_ops(obj))
            blocks.append(gen_unary_arithmetic(obj))
        blocks.append(gen_binary_arithmetic(obj))
        blocks.append(gen_special_binary_operators(obj))

    generated_code = "\n\n".join([b for b in blocks if b])

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
// *  geometric product
// ^  wedge (meet)
// &  vee (join)
// |  dot

// This file heavily inspired by https://bivector.net/tools.html and their generated rust code.
// Hopefully the conventions here are the same as ganja.js and company,
// so there is good interoperability.
// One exception is that using ! to take the dual has been removed (use .dual())

use crate::global::{{Float, FLOAT_DIVISION_EPSILON}};
use std::ops::{{Add, Sub, Mul, Neg, BitXor, BitAnd, BitOr}};

pub trait Reverse {{
    fn reverse(self) -> Self;
}}

pub trait Dual {{
    type Output;
    fn dual(self) -> Self::Output;
}}

pub trait Conjugate {{
    fn conjugate(self) -> Self;
}}

pub trait Normalize {{
    fn norm(self) -> Float;

    // Return a normalized copy
    fn hat(self) -> Self where Self: Copy + Mul<Float, Output=Self> {{
        assert!(self.is_finite(), "norm is zero");
        self * (1. / self.norm())
    }}

    fn is_finite(self) -> bool where Self: Copy {{
        return self.norm() > FLOAT_DIVISION_EPSILON;
    }}
}}

pub trait NormalizeInfinite {{
    fn inorm(self) -> Float;

    // Return a infinite-normalized copy
    fn ihat(self) -> Self where Self: Copy + Mul<Float, Output=Self> {{
        assert!(self.is_infinite(), "inorm is zero");
        self * (1. / self.inorm())
    }}

    fn is_infinite(self) -> bool where Self: Copy {{
        return self.inorm() > FLOAT_DIVISION_EPSILON;
    }}
}}

pub trait Project<Entity> {{
    fn project(self, r: Entity) -> Self;
}}

pub trait Reflect<Entity> {{
    fn reflect(self, r: Entity) -> Self;
}}

pub trait Transform<Entity> {{
    fn transform(self, r: Entity) -> Self;
}}

{generated_code}

impl Bivector {{
    pub fn exp(self) -> ScalarAndBivector {{
        let theta = self.norm();
        if theta.abs() < FLOAT_DIVISION_EPSILON {{
            1. + self
        }} else {{
            theta.cos() + (theta.sin() / theta) * self
        }}
    }}
}}

pub const I: Pseudoscalar = Pseudoscalar {{ a15: 1. }};

"""

    print(rust_template)

if __name__ == "__main__":
    main()
