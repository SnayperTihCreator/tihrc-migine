import math
import re
from abc import ABC, abstractmethod
from numbers import Number as PyNumber
from typing import Self, Optional, TYPE_CHECKING

import numpy as np
import sympy as sp
from sympy import Expr, Number as SymNumber, Symbol, Basic, Matrix, And, Function, cacheit, Derivative
from sympy.core.kind import NumberKind
from sympy.core.singleton import S
from sympy.core.sympify import converter
from sympy.core.sympify import sympify
from sympy.functions.elementary.integers import ceiling
from sympy.sets.sets import Set, SetKind

from .core import RuntimeContext

if TYPE_CHECKING:
    from .rendered import MiginePrinter

converter[np.ndarray] = Matrix


class BaseUnit(Expr, ABC):
    @abstractmethod
    def execute(self, context: RuntimeContext) -> Self: ...


class Result(BaseUnit):
    def __new__(cls, expr: Expr, value=None):
        expr = sympify(expr)
        
        value = expr.doit() if value is None else sympify(value)
        return super().__new__(cls, expr, value)
    
    @property
    def result(self) -> Expr:
        return self.args[0]
    
    @property
    def value(self) -> Expr:
        return self.args[1]
    
    @property
    def rvalue(self) -> Optional[SymNumber | PyNumber]:
        value = self.value
        if value.is_number:
            if value.is_Integer:
                return int(value)
            if value.is_Float:
                return float(value)
            if value.is_complex and not value.is_real:
                return complex(value)
            return value.evalf()
        return None
    
    def _latex(self, printer: "MiginePrinter") -> str:
        r_value = printer.doprint(self.value.evalf(5) if self.value.is_number else self.value)
        
        if self.expr == self.value:
            return r_value
        
        try:
            r_expr = printer.doprint(self.expr)
        except Exception:
            r_expr = f"{self.expr}"
        
        if r_expr == r_value:
            return r_expr
        
        return f"{r_expr} = {r_value}"
    
    def execute(self, context: RuntimeContext) -> Self:
        if isinstance(self.expr, Symbol) and self.expr.name in context:
            value = context.get(self.expr.name)
        else:
            value = self.expr.subs(context).doit()
        return Result(self.expr, value)
    
    @property
    def free_symbols(self) -> set[Basic]:
        return self.expr.free_symbols | self.value.free_symbols
    
    def __repr__(self):
        return f"{self.__class__.__name__}({self.expr}; {self.value})"


class FinalizeUnit(BaseUnit, ABC): ...


class Define(FinalizeUnit):
    def __new__(cls, name, content):
        name = sympify(name)
        if not isinstance(content, Result):
            content = Result(sympify(content))
        return super().__new__(cls, name, content)
    
    @property
    def sname(self) -> Symbol:
        return self.args[0]
    
    @property
    def name(self) -> str:
        return self.sname.name
    
    @property
    def content(self) -> Result:
        return self.args[1]
    
    @property
    def rvalue(self) -> Optional[SymNumber | PyNumber]:
        return self.content.rvalue
    
    def _latex(self, printer: "MiginePrinter") -> str:
        return f"{printer.doprint(self.sname)} = {printer.doprint(self.content)}"
    
    def execute(self, context: RuntimeContext) -> Self:
        executed_content = self.content.execute(context)
        return Define(self.name, executed_content)
    
    @property
    def free_symbols(self) -> set[Basic]:
        return self.content.free_symbols


class DefineODE(FinalizeUnit):
    def __new__(cls, lhs, content):
        lhs = cls._parse_lhs(lhs)
        if not isinstance(content, Result):
            content = Result(sympify(content))
        return super().__new__(cls, lhs, content)
    
    @classmethod
    def _parse_lhs(cls, lhs):
        """Преобразует строку с производной в объект Derivative."""
        if isinstance(lhs, (Symbol, Derivative)):
            return lhs
        lhs_str = str(lhs)
        match = re.fullmatch(r'([a-zA-Z_]\w*)\(([^)]+)\)(\'{1,2})', lhs_str)
        if match:
            func_name, args, primes = match.groups()
            order = len(primes)
            vars_list = [sympify(arg.strip()) for arg in args.split(',')]
            func = Function(func_name)(*vars_list)
            return Derivative(func, *vars_list, order)
        return sympify(lhs_str)
    
    @property
    def sname(self) -> Symbol | Derivative:
        return self.args[0]
    
    @property
    def name(self) -> str:
        lhs = self.sname
        if isinstance(lhs, Derivative):
            func = lhs.expr
            order = len(lhs.variables)
            return f"{func}{order * chr(39)}"
        return lhs.name
    
    @property
    def content(self) -> Result:
        return self.args[1]
    
    @property
    def rvalue(self) -> Optional[SymNumber | PyNumber]:
        return self.content.rvalue
    
    def _latex(self, printer: "MiginePrinter") -> str:
        return f"{printer.doprint(self.sname)} = {printer.doprint(self.content)}"
    
    def execute(self, context: RuntimeContext) -> Self:
        executed_content = self.content.execute(context)
        return DefineODE(self.sname, executed_content)
    
    @property
    def free_symbols(self) -> set[Basic]:
        return self.content.free_symbols.union(self.sname.free_symbols)


class FloatRange(Set):
    
    def __new__(cls, *args):
        if len(args) == 1:
            start, stop, step = S.Zero, sympify(args[0]), S.One
        elif len(args) == 2:
            start, stop, step = sympify(args[0]), sympify(args[1]), S.One
        elif len(args) == 3:
            start, stop, step = sympify(args[0]), sympify(args[1]), sympify(args[2])
        else:
            raise TypeError("FloatRange принимает от 1 до 3 аргументов")
        
        if step == 0:
            raise ValueError("step не может быть равен 0")
        
        return Basic.__new__(cls, start, stop, step)
    
    @property
    def start(self):
        return self.args[0]
    
    @property
    def stop(self):
        return self.args[1]
    
    @property
    def step(self):
        return self.args[2]
    
    @property
    def size(self):
        return ceiling((self.stop - self.start) / self.step)
    
    @property
    def is_finite_set(self):
        if self.size.is_finite:
            return True
        if self.size.is_infinite:
            return False
        return None
    
    @property
    def is_empty(self):
        if self.size.is_nonpositive:
            return True
        if self.size.is_positive:
            return False
        return None
    
    def _kind(self):
        return SetKind(NumberKind)
    
    def __iter__(self):
        sz = self.size
        if not sz.is_Integer:
            raise TypeError("Невозможно итерировать символьный FloatRange")
        n = int(sz)
        if n <= 0:
            return
        
        start_f = float(self.start)
        step_f = float(self.step)
        for i in range(n):
            yield start_f + i * step_f
    
    def _contains(self, other):
        other = sympify(other)
        if not other.is_number:
            return None
        
        if self.step.is_positive:
            in_bounds = And(other >= self.start, other < self.stop)
        elif self.step.is_negative:
            in_bounds = And(other <= self.start, other > self.stop)
        else:
            in_bounds = None
        
        if in_bounds is S.false:
            return S.false
        
        ratio = (other - self.start) / self.step
        if ratio.is_number:
            try:
                val = float(ratio)
                if math.isclose(val, round(val), rel_tol=1e-9, abs_tol=1e-12):
                    return S.true
                else:
                    return S.false
            except TypeError:
                pass
        return None
    
    def __getitem__(self, i):
        from sympy.core.numbers import Integer
        if not isinstance(i, (int, Integer)):
            raise TypeError("Индекс должен быть целым числом")
        n = self.size
        if not n.is_Integer:
            raise ValueError("Невозможно получить элемент из символьного FloatRange")
        n_int = int(n)
        if i < 0:
            i = n_int + i
        if i < 0 or i >= n_int:
            raise IndexError("Индекс FloatRange вне диапазона")
        res = self.start + i * self.step
        if res.is_number:
            return float(res)
        return res
    
    @property
    def _inf(self):
        if self.is_empty:
            return S.EmptySet.inf
        n = self.size
        if n.is_positive is False:
            return S.NaN
        if self.step.is_positive:
            return self.start
        else:
            return self.start + (n - 1) * self.step
    
    @property
    def _sup(self):
        if self.is_empty:
            return S.EmptySet.sup
        n = self.size
        if n.is_positive is False:
            return S.NaN
        if self.step.is_positive:
            return self.start + (n - 1) * self.step
        else:
            return self.start
    
    def __repr__(self):
        return f"FloatRange({self.start}, {self.stop}, {self.step})"


class PFunction(Function):
    
    @cacheit
    def __new__(cls, parameters, expr, **options):
        params_tuple = tuple(parameters)
        obj = super().__new__(cls, params_tuple, expr, **options)
        obj._params = params_tuple
        obj._expr = expr
        return obj
    
    def __call__(self, arg):
        if not self._params:
            return self._expr
        param = self._params[0]
        new_expr = self._expr.subs(param, arg)
        new_params = self._params[1:]
        if new_params:
            return self.func(new_params, new_expr)
        else:
            return new_expr
    
    def _eval_subs(self, old, new):
        if old in self._params:
            new_params = list(self._params)
            new_params.remove(old)
            new_expr = self._expr.subs(old, new)
        else:
            new_params = self._params
            new_expr = self._expr.subs(old, new)
            if new_expr == self._expr:
                return self
        
        if not new_params:
            return new_expr
        return self.func(new_params, new_expr)
    
    def _latex(self, printer):
        params = ", ".join(printer.doprint(p) for p in self._params)
        return rf"\operatorname{{PF}}_{{{params}}}\!\left({printer.doprint(self._expr)}\right)"
    
    def __repr__(self):
        return f"PFunction({self._params}, {self._expr!r})"


class SymPyColon(sp.AtomicExpr):
    def _sympystr(self, printer):
        return ":"
    
    def _latex(self, printer):
        return r"\mathbf{:}"
    
    def _pretty(self, printer):
        from sympy.printing.pretty.stringpict import prettyForm
        return prettyForm(":")


__all__ = ["FinalizeUnit", "Define", "DefineODE", "Result", "FloatRange", "PFunction", "SymPyColon"]
