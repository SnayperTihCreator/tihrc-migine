import pytest
from tihrc.migine import Migine, parse


def test_migine_translator():
    engine = Migine()
    ast = parse("(a+b)*c")
    result = engine.translator.translate(ast)
    assert result is not None


@pytest.mark.parametrize("expr", [
    "(a+b)*c",
    "g(x,y)=x+y",
    "prod[k=1..n](k)",
    "a[1]",
])
def test_translate_expressions(expr):
    engine = Migine()
    ast = parse(expr)
    result = engine.translator.translate(ast)
    assert result is not None
