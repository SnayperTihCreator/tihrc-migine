use pyo3::prelude::*;
use pyo3_stub_gen::define_stub_info_gatherer;
use pyo3_stub_gen::derive::gen_stub_pyfunction;
use pyo3::exceptions::{PyValueError};

use crate::ast::Node;
use crate::pyast::*;
use crate::tokenized::Operators;

mod tokenized;
mod ast;
mod pyast;
mod parsed;

macro_rules! register_classes {
    ($module:expr, $( $class:path ),* $(,)?) => {
        $(
            $module.add_class::<$class>()?;
        )*
    };
}

fn rnode2pnode(py: Python, node: &Node) -> PyResult<Py<PyAny>> {
    match node {
        Node::Integer(n) => Ok(PyInteger::new(py, *n).into()),
        Node::Number(n) => Ok(PyNumber::new(py, *n).into()),
        Node::Complex(re, im) => Ok(PyComplex::new(py, *re, *im).into()),
        Node::Variable(name) => Ok(PyVariable::new(py, name.clone()).into()),
        Node::Constant(name) => Ok(PyConstant::new(py, name.clone()).into()),
        
        Node::UnaryOp { op, child } => {
            let py_child = rnode2pnode(py, child)?;
            Ok(PyUnaryOp::new(py, op.clone(), py_child).into())
        }
        
        Node::BinOp { op, left, right } => {
            let py_left = rnode2pnode(py, left)?;
            let py_right = rnode2pnode(py, right)?;
            Ok(PyBinOp::new(py, op.clone(), py_left, py_right).into())
        }
        
        Node::FunctionCall { name, args } => {
            let mut py_args = Vec::new();
            for arg in args.iter() {
                py_args.push(rnode2pnode(py, arg)?);
            }
            Ok(PyFunctionCall::new(py, name.clone(), py_args).into())
        }
        
        Node::FunctionDef { name, params, body } => {
            let py_body = rnode2pnode(py, body)?;
            Ok(PyFunctionDef::new(py, name.clone(), params.clone(), py_body).into())
        }
        
        Node::Range { start, end, step } => {
            let py_start = rnode2pnode(py, start)?;
            let py_end = rnode2pnode(py, end)?;
            let py_step = match step {
                Some(s) => Some(rnode2pnode(py, s)?),
                None => None,
            };
            Ok(PyRange::new(py, py_start, py_end, py_step).into())
        }

        Node::Matrix { rows } => {
            let mut py_rows = Vec::new();
            for row in rows {
                let mut py_row = Vec::new();
                for node in row {
                    py_row.push(rnode2pnode(py, node)?);
                }
                py_rows.push(py_row);
            }
            Ok(PyMatrix::new(py, py_rows).into())
        }

        // Конвертация новых узлов
        Node::Index { expr, indices } => {
            let py_expr = rnode2pnode(py, expr)?;
            let mut py_indices = Vec::new();
            for idx in indices {
                py_indices.push(rnode2pnode(py, idx)?);
            }
            Ok(PyIndex::new(py, py_expr, py_indices).into())
        }

        Node::EmptySlice => Ok(PyEmptySlice::new(py).into()),

        Node::Summation { var, start, end, body } => {
            let py_start = rnode2pnode(py, start)?;
            let py_end = rnode2pnode(py, end)?;
            let py_body = rnode2pnode(py, body)?;
            Ok(PySummation::new(py, var.clone(), py_start, py_end, py_body).into())
        }

        Node::Product { var, start, end, body } => {
            let py_start = rnode2pnode(py, start)?;
            let py_end = rnode2pnode(py, end)?;
            let py_body = rnode2pnode(py, body)?;
            Ok(PyProduct::new(py, var.clone(), py_start, py_end, py_body).into())
        }

        Node::DefIntegral { var, start, end, body } => {
            let py_start = rnode2pnode(py, start)?;
            let py_end = rnode2pnode(py, end)?;
            let py_body = rnode2pnode(py, body)?;
            Ok(PyDefIntegral::new(py, var.clone(), py_start, py_end, py_body).into())
        }
        Node::IndefIntegral { var, body } => {
            let py_body = rnode2pnode(py, body)?;
            Ok(PyIndefIntegral::new(py, var.clone(), py_body).into())
        }
        Node::Limit { var, target, body } => {
            let py_target = rnode2pnode(py, target)?;
            let py_body = rnode2pnode(py, body)?;
            Ok(PyLimit::new(py, var.clone(), py_target, py_body).into())
        }

        Node::DerivativeExpr { var, order, body } => {
            let py_order = rnode2pnode(py, order)?;
            let py_body = rnode2pnode(py, body)?;
            Ok(PyDerivativeExpr::new(py, var.clone(), py_order, py_body).into())
        }
    }
}

#[gen_stub_pyfunction]
#[pyfunction]
fn parse(py: Python, input: &str) -> PyResult<Py<PyAny>> {
    let tokens = tokenized::tokenizez(input).map_err(|e| PyValueError::new_err(e))?;
    let mut parser = parsed::Parser::new(tokens);
    let ast = parser.parse().map_err(|e| PyValueError::new_err(e))?;
    let py_ast = rnode2pnode(py, &ast)?;
    Ok(py_ast)
}

#[pymodule]
mod _core_migine {
    use pyo3::prelude::*;
    use super::Operators;
    use crate::pyast::*;

    #[pymodule_export]
    use crate::parse;

    #[pymodule_init]
    fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
        register_classes!(
            m,
            PyNode, PyInteger, PyNumber, PyComplex, PyVariable, PyConstant,
            PyUnaryOp, PyBinOp,
            PyFunctionCall,PyFunctionDef,
            PyRange, PyMatrix, PyIndex, PyEmptySlice,
            PySummation, PyProduct, PyDerivativeExpr, PyDefIntegral, PyIndefIntegral, PyLimit,

            Operators
        );
        Ok(())
    }
}

define_stub_info_gatherer!(stub_info);