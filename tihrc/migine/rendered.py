import numpy as np
from attrs import define, field
from sympy.printing.latex import LatexPrinter

from .protocols import MigineProtocol
from .units import BaseUnit


class MiginePrinter(LatexPrinter):
    _default_settings = {
                            "max_dim_row": 5,
                            "max_dim_col": 5
                        } | LatexPrinter._default_settings
    
    def _print_matrix_contents(self, expr):
        total_rows = expr.rows
        total_cols = expr.cols
        
        truncate_rows = total_rows > self._settings.get('max_dim_row')
        truncate_cols = total_cols > self._settings.get('max_dim_col')
        
        if truncate_rows:
            row_indices = [0, 1, total_rows - 2, total_rows - 1]
        else:
            row_indices = list(range(total_rows))
        
        if truncate_cols:
            col_indices = [0, 1, total_cols - 2, total_cols - 1]
        else:
            col_indices = list(range(total_cols))
        
        lines = []
        
        for r_idx, r in enumerate(row_indices):
            if truncate_rows and r == total_rows - 2:
                if truncate_cols:
                    separator_row = " & ".join(["-"] * 2) + " & | & " + " & ".join(["-"] * 2)
                else:
                    separator_row = " & ".join(["-"] * total_cols)
                lines.append(separator_row)
            
            current_row_elements = []
            
            for c_idx, c in enumerate(col_indices):
                if truncate_cols and c == total_cols - 2:
                    current_row_elements.append("|")
                val = expr[r, c]
                current_row_elements.append(self._print(val))
            lines.append(" & ".join(current_row_elements))
        
        mat_str = self._settings.get('mat_str', None)
        if mat_str is None:
            if self._settings.get('mode') == 'inline':
                mat_str = 'smallmatrix'
            else:
                if total_cols <= 10:
                    mat_str = 'matrix'
                else:
                    mat_str = 'array'
        
        out_str = r'\begin{%MATSTR%}%s\end{%MATSTR%}'
        out_str = out_str.replace('%MATSTR%', mat_str)
        
        if mat_str == 'array':
            if truncate_cols:
                col_align = 'c' * 2 + '|' + 'c' * 2
            else:
                col_align = 'c' * total_cols
            out_str = out_str.replace('%s', '{' + col_align + '}%s')
        return out_str % r"\\".join(lines)


@define
class MigineRender:
    _engine: MigineProtocol
    _printer: MiginePrinter = field(factory=MiginePrinter)
    
    def render(self, unit: BaseUnit):
        return self._printer.doprint(unit)
    
    def prepare_format(self, value):
        
        if isinstance(value, np.ndarray):
            if value.size > 5:
                items = [f"{x:.2f}" for x in value[:2]]
                return rf"\left[ {', '.join(items)}, \dots, {value[-1]:.2f} \right]"
            return rf"\left[ {', '.join([f'{x:.2f}' for x in value])} \right]"
        
        if hasattr(value, 'evalf'):
            return self._printer.doprint(value.evalf(5))
        
        return str(value)
