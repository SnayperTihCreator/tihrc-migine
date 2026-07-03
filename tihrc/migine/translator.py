import logging
from typing import Any

from attrs import define, field
import sympy as sp
import operator as _oper

from . import _core_migine as _core
from .protocols import MigineProtocol
from .units import *
from .tables import TABLE_ALLOWED_FUNCTIONS

logger = logging.getLogger(__name__)

_operators = {
    _core.Operators.Plus: _oper.add,
    _core.Operators.Minus: _oper.sub,
    _core.Operators.Slash: _oper.truediv,
    _core.Operators.Star: _oper.mul,
    _core.Operators.Caret: _oper.pow,
    
    _core.Operators.NotEqual: sp.Ne,
    _core.Operators.DoubleEqual: sp.Eq,
    _core.Operators.LessEqual: sp.Le,
    _core.Operators.GreaterEqual: sp.Ge,
    _core.Operators.LessThan: sp.Lt,
    _core.Operators.GreaterThan: sp.Gt,
}


@define
class MigineTranslator:
    engine: MigineProtocol
    _symbols: dict[str, sp.Symbol] = field(factory=dict)
    _constants: dict[str, Any] = field(factory=dict)
    _table: dict[str, Any] = field(factory=dict)
    
    def __attrs_post_init__(self):
        self._constants.update({
            "PI": sp.pi,
            "E": sp.E,
            "INF": sp.oo
        })
        
        self._table.update(TABLE_ALLOWED_FUNCTIONS)
    
    def translate(self, node: _core.Node):
        result = self._trans(node)
        if result is None:
            return None
        if not isinstance(result, FinalizeUnit):
            return Result(result)
        return result
    
    def _trans(self, node: _core.Node):
        return self._do_find_impl(node)
    
    def _tl_Number(self, node: _core.Number):
        return sp.Float(node.value)
    
    def _tl_Integer(self, node: _core.Integer):
        return sp.Integer(node.value)
    
    def _tl_Complex(self, node: _core.Complex):
        return sp.Float(node.real) + sp.Float(node.imag) * sp.I
    
    def _tl_Variable(self, node: _core.Variable):
        if node.name not in self._symbols:
            self._symbols[node.name] = sp.Symbol(node.name)
        return self._symbols[node.name]
    
    def _tl_Constant(self, node: _core.Constant):
        if node.name not in self._constants:
            raise ValueError(f"Constant {node.name} not found")
        return self._constants[node.name]
    
    def _tl_BinOp(self, node: _core.BinOp):
        match node.op:
            case _core.Operators.Equal:
                return self._tl_BinOp_Equal(node)
            case _:
                return self._tl_BinOp_Common(node)
    
    def _tl_BinOp_Equal(self, node: _core.BinOp):
        left = self._trans(node.left)
        right = self._trans(node.right)
        if isinstance(right, Define):
            raise ValueError(f"BinOp node {right} is not equal to right")
        if isinstance(left, sp.Symbol):
            return Define(left, right)
        if isinstance(left, sp.Derivative):
            return DefineODE(left, right)
        return None
    
    def _tl_BinOp_Common(self, node: _core.BinOp):
        left = self._trans(node.left)
        if left is None: return None
        right = self._trans(node.right)
        if right is None: return None
        impl = _operators.get(node.op, None)
        if impl is not None:
            return impl(left, right)
        logger.warning(f"No translation binary operator for node {node}")
        return None
    
    def _tl_UnaryOp(self, node: _core.UnaryOp):
        child = self._trans(node.child)
        if node.op == _core.Operators.Minus:
            return -child
        return child
    
    def _tl_Summation(self, node: _core.Summation):
        a = self._trans(node.start)
        b = self._trans(node.end)
        var = sp.Symbol(node.var)
        body = self._trans(node.body)
        return sp.Sum(body, (var, a, b))
    
    def _tl_Product(self, node: _core.Product):
        var = sp.Symbol(node.var)
        start = self._trans(node.start)
        end = self._trans(node.end)
        body = self._trans(node.body)
        return sp.Product(body, (var, start, end))
    
    def _tl_Range(self, node: _core.Range):
        a = self._trans(node.start)
        b = self._trans(node.end)
        k = sp.Integer(1) if node.step is None else self._trans(node.step)
        return FloatRange(a, b, k)
    
    def _tl_EmptySlice(self, _: _core.EmptySlice):
        return SymPyColon()
    
    def _tl_Index(self, node: _core.Index):
        expr = self._trans(node.expr)
        if expr is None:
            return None
        
        indices = [self._trans(idx) for idx in node.indices]
        if any(idx is None for idx in indices):
            return None

        if isinstance(expr, sp.Matrix):
            py_indices = [slice(None) if isinstance(idx, SymPyColon) else idx for idx in indices]
            try:
                if len(py_indices) == 1:
                    return expr[py_indices[0]]
                return expr[tuple(py_indices)]
            except Exception as e:
                logger.warning(f"Failed to slice concrete matrix: {e}")
                return None
        return sp.Indexed(sp.IndexedBase(expr), *indices)
    
    def _tl_IndefIntegral(self, node: _core.IndefIntegral):
        var = sp.Symbol(node.var)
        body = self._trans(node.body)
        if body is None: return None
        return sp.Integral(body, var)
    
    def _tl_DefIntegral(self, node: _core.DefIntegral):
        var = sp.Symbol(node.var)
        start = self._trans(node.start)
        if start is None: return None
        end = self._trans(node.end)
        if end is None: return None
        body = self._trans(node.body)
        if body is None: return None
        return sp.Integral(body, (var, start, end))
    
    def _tl_Limit(self, node: _core.Limit):
        var = sp.Symbol(node.var)
        body = self._trans(node.body)
        if body is None: return None
        target = self._trans(node.target)
        if target is None: return None
        return sp.Limit(body, var, target)
    
    def _tl_DerivativeExpr(self, node: _core.DerivativeExpr):
        var = sp.Symbol(node.var)
        body = self._trans(node.body)
        if body is None: return None
        power = self._trans(node.order)
        if power is None: return None
        return sp.Derivative(body, var, power)
    
    def _tl_FunctionCall(self, node: _core.FunctionCall):
        args = [self._trans(n) for n in node.args]
        impl = self._table.get(node.name, None)
        if impl is not None: impl(*args)
        args = [self._trans(el) for el in node.args]
        func = sp.Function(node.name)
        return func(*args)
    
    def _tl_Matrix(self, node: _core.Matrix):
        if not node.rows:
            return sp.Matrix()
        rows = [[self._trans(el) for el in row] for row in node.rows]
        return sp.Matrix(rows)
    
    def _tl_FunctionDef(self, node: _core.FunctionDef):
        name = node.name
        body = self._trans(node.body)
        if body is None: return None
        args = node.params
        if args is None: return None
        self._table[name] = PFunction(args, body)
        return self._table[name]
    
    def _do_find_impl(self, node: _core.Node):
        impl = getattr(self, f"_tl_{node.__class__.__name__}", None)
        if impl is None:
            logger.warning(f"No translation method found for node {node}")
            return None
        return impl(node)
