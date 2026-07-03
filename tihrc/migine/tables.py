import sympy as sp

TABLE_ALLOWED_FUNCTIONS = {
    "sin": sp.sin, "cos": sp.cos, "tan": sp.tan, "cot": sp.cot, "sec": sp.sec, "csc": sp.csc,
    "asin": sp.asin, "acos": sp.acos, "atan": sp.atan, "acot": sp.acot, "asec": sp.asec, "acsc": sp.acsc,
    "sinh": sp.sinh, "cosh": sp.cosh, "tanh": sp.tanh, "coth": sp.coth, "sech": sp.sech, "csch": sp.csch,
    "asinh": sp.asinh, "acosh": sp.acosh, "atanh": sp.atanh, "acoth": sp.acoth, "asech": sp.asech, "acsch": sp.acsch,
    "sqrt": sp.sqrt, "cbrt": sp.cbrt,
    "exp": sp.exp, "log": sp.log,
}