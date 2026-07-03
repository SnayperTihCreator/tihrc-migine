use pyo3::prelude::*;
use pyo3_stub_gen::derive::gen_stub_pyclass;
use crate::tokenized::Operators;

macro_rules! impl_api {
    ($type:ident, $name:expr, $( $field:ident ),*) => {
        #[pymethods]
        impl $type {
            #[classattr]
            const __MATCH_ARGS__: &'static [&'static str] = &[
                $(stringify!($field),)*
            ];

            fn __repr__(&self, _py: Python<'_>) -> String {
                #[allow(unused_mut)]
                let mut fields_fmt = String::new();
                $(
                    if !fields_fmt.is_empty() { fields_fmt.push_str(", "); }
                    let repr_str = match self.$field.clone().into_pyobject(_py) {
                        Ok(py_obj) => {
                            match py_obj.as_any().repr() {
                                Ok(py_str) => py_str.to_string(),
                                Err(_) => "Error".to_string(),
                            }
                        }
                        Err(_) => "Error".to_string(),
                    };
                    
                    fields_fmt.push_str(&repr_str);
                )*
                format!("{}({})", $name, fields_fmt)
            }
        }
    };
}

macro_rules! impl_new {
    ($type:ident, $( $field:ident : $ty:ty ),*) => {
        impl $type {
            pub fn new(py: Python, $( $field: $ty ),*) -> Py<$type> {
                let init = PyClassInitializer::from(PyNode)
                    .add_subclass($type { $( $field ),* });
                Py::new(py, init).unwrap()
            }
        }
    };
}

#[gen_stub_pyclass]
#[pyclass(subclass, name = "Node", skip_from_py_object)]
#[derive(Clone, Debug)]
pub struct PyNode;

// --- Базовые Узлы ---

#[gen_stub_pyclass]
#[pyclass(extends=PyNode, name = "Integer", skip_from_py_object)]
#[derive(Clone, Debug)]
pub struct PyInteger { #[pyo3(get, set)] pub value: i64 }
impl_api!(PyInteger, "Integer", value);
impl_new!(PyInteger, value: i64);

#[gen_stub_pyclass]
#[pyclass(extends=PyNode, name = "Number", skip_from_py_object)]
#[derive(Clone, Debug)]
pub struct PyNumber { #[pyo3(get, set)] pub value: f64 }
impl_api!(PyNumber, "Number", value);
impl_new!(PyNumber, value: f64);

#[gen_stub_pyclass]
#[pyclass(extends=PyNode, name = "Complex", skip_from_py_object)]
#[derive(Clone, Debug)]
pub struct PyComplex { #[pyo3(get, set)] pub real: f64, #[pyo3(get, set)] pub imag: f64 }
impl_api!(PyComplex, "Complex", real, imag);
impl_new!(PyComplex, real: f64, imag: f64);

#[gen_stub_pyclass]
#[pyclass(extends=PyNode, name = "Variable", skip_from_py_object)]
#[derive(Clone, Debug)]
pub struct PyVariable { #[pyo3(get, set)] pub name: String }
impl_api!(PyVariable, "Variable", name);
impl_new!(PyVariable, name: String);

#[gen_stub_pyclass]
#[pyclass(extends=PyNode, name = "Constant", skip_from_py_object)]
#[derive(Clone, Debug)]
pub struct PyConstant { #[pyo3(get, set)] pub name: String }
impl_api!(PyConstant, "Constant", name);
impl_new!(PyConstant, name: String);

#[gen_stub_pyclass]
#[pyclass(extends=PyNode, name = "UnaryOp", skip_from_py_object)]
#[derive(Clone, Debug)]
pub struct PyUnaryOp { #[pyo3(get, set)] pub op: Operators, #[pyo3(get, set)] pub child: Py<PyAny> }
impl_api!(PyUnaryOp, "UnaryOp", op, child);
impl_new!(PyUnaryOp, op: Operators, child: Py<PyAny>);

#[gen_stub_pyclass]
#[pyclass(extends=PyNode, name = "BinOp", skip_from_py_object)]
#[derive(Clone, Debug)]
pub struct PyBinOp { #[pyo3(get, set)] pub op: Operators, #[pyo3(get, set)] pub left: Py<PyAny>, #[pyo3(get, set)] pub right: Py<PyAny> }
impl_api!(PyBinOp, "BinOp", op, left, right);
impl_new!(PyBinOp, op: Operators, left: Py<PyAny>, right: Py<PyAny>);

#[gen_stub_pyclass]
#[pyclass(extends=PyNode, name = "FunctionCall", skip_from_py_object)]
#[derive(Clone, Debug)]
pub struct PyFunctionCall { #[pyo3(get, set)] pub name: String, #[pyo3(get, set)] pub args: Vec<Py<PyAny>> }
impl_api!(PyFunctionCall, "FunctionCall", name, args);
impl_new!(PyFunctionCall, name: String, args: Vec<Py<PyAny>>);

#[gen_stub_pyclass]
#[pyclass(extends=PyNode, name = "FunctionDef", skip_from_py_object)]
#[derive(Clone, Debug)]
pub struct PyFunctionDef { #[pyo3(get, set)] pub name: String, #[pyo3(get, set)] pub params: Vec<String>, #[pyo3(get, set)] pub body: Py<PyAny> }
impl_api!(PyFunctionDef, "FunctionDef", name, params, body);
impl_new!(PyFunctionDef, name: String, params: Vec<String>, body: Py<PyAny>);

#[gen_stub_pyclass]
#[pyclass(extends=PyNode, name = "Range", skip_from_py_object)]
#[derive(Clone, Debug)]
pub struct PyRange { #[pyo3(get, set)] pub start: Py<PyAny>, #[pyo3(get, set)] pub end: Py<PyAny>, #[pyo3(get, set)] pub step: Option<Py<PyAny>> }
impl_api!(PyRange, "Range", start, end, step);
impl_new!(PyRange, start: Py<PyAny>, end: Py<PyAny>, step: Option<Py<PyAny>>);

#[gen_stub_pyclass]
#[pyclass(extends=PyNode, name = "Matrix", skip_from_py_object)]
#[derive(Clone, Debug)]
pub struct PyMatrix { #[pyo3(get, set)] pub rows: Vec<Vec<Py<PyAny>>> }
impl_api!(PyMatrix, "Matrix", rows);
impl_new!(PyMatrix, rows: Vec<Vec<Py<PyAny>>>);

// --- Новые Математические Узлы ---

#[gen_stub_pyclass]
#[pyclass(extends=PyNode, name = "Index", skip_from_py_object)]
#[derive(Clone, Debug)]
pub struct PyIndex { #[pyo3(get, set)] pub expr: Py<PyAny>, #[pyo3(get, set)] pub indices: Vec<Py<PyAny>> }
impl_api!(PyIndex, "Index", expr, indices);
impl_new!(PyIndex, expr: Py<PyAny>, indices: Vec<Py<PyAny>>);

#[gen_stub_pyclass]
#[pyclass(extends=PyNode, name = "EmptySlice", skip_from_py_object)]
#[derive(Clone, Debug)]
pub struct PyEmptySlice;
impl_api!(PyEmptySlice, "EmptySlice",);
impl_new!(PyEmptySlice,);

#[gen_stub_pyclass]
#[pyclass(extends=PyNode, name = "Summation", skip_from_py_object)]
#[derive(Clone, Debug)]
pub struct PySummation { #[pyo3(get, set)] pub var: String, #[pyo3(get, set)] pub start: Py<PyAny>, #[pyo3(get, set)] pub end: Py<PyAny>, #[pyo3(get, set)] pub body: Py<PyAny> }
impl_api!(PySummation, "Summation", var, start, end, body);
impl_new!(PySummation, var: String, start: Py<PyAny>, end: Py<PyAny>, body: Py<PyAny>);

#[gen_stub_pyclass]
#[pyclass(extends=PyNode, name = "Product", skip_from_py_object)]
#[derive(Clone, Debug)]
pub struct PyProduct { #[pyo3(get, set)] pub var: String, #[pyo3(get, set)] pub start: Py<PyAny>, #[pyo3(get, set)] pub end: Py<PyAny>, #[pyo3(get, set)] pub body: Py<PyAny> }
impl_api!(PyProduct, "Product", var, start, end, body);
impl_new!(PyProduct, var: String, start: Py<PyAny>, end: Py<PyAny>, body: Py<PyAny>);

#[gen_stub_pyclass]
#[pyclass(extends=PyNode, name = "DerivativeExpr", skip_from_py_object)]
#[derive(Clone, Debug)]
pub struct PyDerivativeExpr { #[pyo3(get, set)] pub var: String, #[pyo3(get, set)] pub order: Py<PyAny>, #[pyo3(get, set)] pub body: Py<PyAny> }
impl_api!(PyDerivativeExpr, "DerivativeExpr", var, order, body);
impl_new!(PyDerivativeExpr, var: String, order: Py<PyAny>, body: Py<PyAny>);

#[gen_stub_pyclass]
#[pyclass(extends=PyNode, name = "DefIntegral", skip_from_py_object)]
#[derive(Clone, Debug)]
pub struct PyDefIntegral { #[pyo3(get, set)] pub var: String, #[pyo3(get, set)] pub start: Py<PyAny>, #[pyo3(get, set)] pub end: Py<PyAny>, #[pyo3(get, set)] pub body: Py<PyAny> }
impl_api!(PyDefIntegral, "DefIntegral", var, start, end, body);
impl_new!(PyDefIntegral, var: String, start: Py<PyAny>, end: Py<PyAny>, body: Py<PyAny>);

#[gen_stub_pyclass]
#[pyclass(extends=PyNode, name = "IndefIntegral", skip_from_py_object)]
#[derive(Clone, Debug)]
pub struct PyIndefIntegral { #[pyo3(get, set)] pub var: String, #[pyo3(get, set)] pub body: Py<PyAny> }
impl_api!(PyIndefIntegral, "IndefIntegral", var, body);
impl_new!(PyIndefIntegral, var: String, body: Py<PyAny>);

#[gen_stub_pyclass]
#[pyclass(extends=PyNode, name = "Limit", skip_from_py_object)]
#[derive(Clone, Debug)]
pub struct PyLimit { #[pyo3(get, set)] pub var: String, #[pyo3(get, set)] pub target: Py<PyAny>, #[pyo3(get, set)] pub body: Py<PyAny> }
impl_api!(PyLimit, "Limit", var, target, body);
impl_new!(PyLimit, var: String, target: Py<PyAny>, body: Py<PyAny>);