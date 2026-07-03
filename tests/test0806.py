import numpy as np
from functools import wraps
from sympy import Number, Symbol, Expr
from sympy.matrices.matrixbase import MatrixBase
from sympy.matrices.expressions.matexpr import MatrixExpr


def convert_to_numpy(func):
    """
    Декоратор, который:
    1. Конвертирует матрицы SymPy на входе в numpy.ndarray.
    2. Оптимизирует тип (dtype) возвращаемого ndarray на выходе,
       избавляясь от dtype=object, если внутри только числа.
    """
    
    def _to_np(arg):
        if isinstance(arg, (MatrixBase, MatrixExpr)):
            return np.array(arg)
        return arg
    
    def _optimize_dtype(array):
        # Если это не ndarray или у него и так нормальный числовой тип, не трогаем
        if not isinstance(array, np.ndarray) or array.dtype != object:
            return array
        
        # Проверяем, есть ли внутри массива абстрактные символы SymPy
        has_symbols = any(
            isinstance(x, (Symbol, Expr)) and not isinstance(x, Number)
            for x in array.ravel()
        )
        
        if not has_symbols:
            try:
                # Проверяем, есть ли комплексные числа
                if any(complex(x).imag != 0 for x in array.ravel()):
                    return array.astype(np.complex128)
                else:
                    return array.astype(np.float64)
            except (TypeError, ValueError):
                # Если вдруг конвертация не удалась, возвращаем как есть
                return array
        
        return array
    
    @wraps(func)
    def wrapper(*args, **kwargs):
        # 1. Входная конвертация
        new_args = [_to_np(arg) for arg in args]
        new_kwargs = {k: _to_np(v) for k, v in kwargs.items()}
        
        # 2. Вызов функции
        result = func(*new_args, **new_kwargs)
        
        # 3. Выходная оптимизация типа данных
        return _optimize_dtype(result)
    
    return wrapper


# --- Проверка работы ---

if __name__ == "__main__":
    from sympy import Matrix, symbols
    
    
    @convert_to_numpy
    def double_matrix(data_matrix):
        return data_matrix * 2
    
    
    # Тест 1: Чисто числовая матрица SymPy
    mat_numeric = Matrix([[1, 5], [3, 4]])
    res_numeric = double_matrix(mat_numeric)
    
    print("--- Тест 1 (Только числа) ---")
    print(res_numeric)
    print(f"dtype: {res_numeric.dtype}")  # Выведет float64!
    
    # Тест 2: Матрица с символом 'x'
    x = symbols('x')
    mat_symbolic = Matrix([[1, x], [3, 4]])
    res_symbolic = double_matrix(mat_symbolic)
    
    print("\n--- Тест 2 (Символьная матрица) ---")
    print(res_symbolic)
    print(f"dtype: {res_symbolic.dtype}")  # Безопасно оставит object