import pytest
from tihrc.migine import parse


@pytest.mark.parametrize("expr", [
    "a = x - a",
    "score = 10 + 5 * 2",
    "x = y = 1",
    "f(x) = x - 5",
    "add(x, y, z) = x + y + z",
    "g(x) = 2x^2 + 3x - 1",
    "1..10",
    "0..10,2",
    "[1, 2, 3]",
    "[1; 2; 3]",
    "[]",
])
def test_parse_expressions(expr):
    ast = parse(expr)
    assert ast is not None


@pytest.mark.parametrize("expr", [
    "5 m + 4 s",
    "x != y",
    "y'' - 4*y' + 3*y == 0",
])
def test_parse_advanced(expr):
    ast = parse(expr)
    assert ast is not None
