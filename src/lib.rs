use pyo3::prelude::*;
use pyo3::types::{PyList, PyTuple, PyAny};
// use pyo3::wrap_pyfunction;
use rusqlite::types::{ToSqlOutput, ValueRef, Value};
use rusqlite::{Error as RusqliteError, ToSql, Result, Connection, Params};

#[pyclass(name = "Connection")]
pub struct PyConnection {
    conn: Connection,
}


fn replace_question_marks<T: ToString + ?Sized>(input: &str, to_replace: Vec<Box<T>>) -> PyResult<String> {
    let mut result = String::new();
    let mut iter = to_replace.into_iter();
    
    for c in input.chars() {
        if c == '?' {
            match iter.next() {
                Some(val) => result.push_str(&val.to_string()),
                None => return Err(PyErr::new::<pyo3::exceptions::PyException, _>("Parameters vector has less elements than the number of question marks in the query"))
            }
        } else {
            result.push(c);
        }
    }

    if iter.next().is_some() {
        return Err(PyErr::new::<pyo3::exceptions::PyException, _>("Parameters vector has more elements than the number of question marks in the query"));
    }

    Ok(result)

}

// 将 Python 中的 PyObject 转换为 rusqlite 可处理的类型
fn py_object_to_rusqlite_value(py: &PyAny) -> Box<dyn ToString> {
    if let Ok(val) = py.extract::<i64>() {
        Box::new(val)
    } else if let Ok(val) = py.extract::<f64>() {
        Box::new(val)
    } else if let Ok(val) = py.extract::<String>() {
        Box::new(format!("'{}'", val))
    } else {
        panic!("Unsupported dtype");
    }
}

// 从 PyList 构建 rusqlite 的参数
fn params_from_py_list(py_list: &Vec<&PyAny>) -> Vec<Box<dyn ToString>> {
    let mut params = Vec::new();
    for item in py_list {
        params.push(py_object_to_rusqlite_value(item));
    }
    params
}

enum PyIter<'source> {
    List(&'source PyList),
    Tuple(&'source PyTuple)
}

impl<'source> FromPyObject<'source> for PyIter<'source> {
    fn extract(ob: &'source PyAny) -> PyResult<Self> {
        if let Ok(list) = ob.extract::<&'source PyList>() {
            return Ok(PyIter::List(list))
        } else if let Ok(tuple) = ob.extract::<&'source PyTuple>() {
            return Ok(PyIter::Tuple(tuple))
        } else {
            return Err(PyErr::new::<pyo3::exceptions::PyException, _>("Unsupported type"));
        }
    }
}


#[pymethods]
impl PyConnection {
    pub fn execute(&self, sql: &str, params: Vec<&PyAny>) -> PyResult<()> {
        let params = params_from_py_list(&params);
        let sql = replace_question_marks(sql, params)?;
        let exec = self.conn.execute(sql.as_str(), ());
        match exec {
            Ok(_) => Ok(()),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyException, _>(e.to_string())),
        }
    }
}

#[pyfunction]
fn connect(db_name: &str) -> PyResult<PyConnection> {
    match Connection::open(db_name) {
        Ok(conn) => Ok(PyConnection { conn }),
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyException, _>(e.to_string())),
    }
}

#[pymodule]
fn rusql2py(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(connect))?;

    Ok(())
}