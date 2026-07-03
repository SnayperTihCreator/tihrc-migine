from tihrc.migine import Migine, parse

examples = [
    # "x = 1",
    # "a = x - a",
    # "score = 10 + 5 * 2",
    # "x = y = 1", # ошибка пройдена
    # "1..10",
    # "0..10,2",
    # "a..b,k",
    # "1 + 1 .. 10 * 2, 2^2",
    # "[1, 2, 3]",
    # "[1; 2; 3]",
    # "[1, 2; 3, 4]",
    # "[x, y + 1; sin(x), 4^2]",
    # "[]",
    # "MatrixFunc(x) = [1, 5; x, 2x]",
    # "5 m + 4 s",
    # "sum[i=1..10](i+5)",
    # "sum[i=1..10](i^2) + int[x](x^2) + diff[x, 2](x^3)",
    # "x != y",
    # 'int[x](3*x^2 + sin(x))',
    # 'int[x, 0, PI](cos(x))',
    # 'int[x, 0, INF](E^(-x))',
    # 'lim[x->0](sin(x) / x)',
    # 'lim[x->INF]((1 + 1/x)^x)',
    # 'lim[t->0](int[x, 0, t](x^2))',
    # "y'' - 4*y' + 3*y == 0",
    # "y(t)' = 0",
    # "y''' + 9*y' == 0",
    # "f(x) = x - 5",
    # "add(x, y, z) = x + y + z",
    # "g(x) = 2x^2 + 3x - 1",
    # "y(t)'' = 10",
    "(a+b)*c",
    "g(x,y)=x+y",
    "prod[k=1..n](k)",
    "a[1]",
    ":",
    "a[5, :]",
    "a[(0..1), 0]"

]
engine = Migine()

for row in examples:
    try:
        print(row)
        ast = parse(row)
        print(engine.translator.translate(ast))
    except Exception as e:
        print(f"Error: {e}")
