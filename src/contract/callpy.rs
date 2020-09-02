use crate::common::myerror::{MyError, StrError};
use crate::common::tools::*;
use pyo3::{
    prelude::*,
    types::{IntoPyDict, PyModule},
};

fn read_code(file: &str) -> String {
    let dir = get_dot_env("CONTRACTDIR");
    let f = format!("{}/{}", dir, file);
    let code = read_file_as_txt(&f);
    code
}

pub fn call_terms(name: &str, arg: &str) -> Result<String, MyError> {
    let res = _call_terms(name, arg.to_owned());
    match res {
        Ok(s) => return Ok(s),
        Err(e) => {
            dbg!(e);
            return Err(MyError::OtherError(StrError::new("call terms fail")));
        }
    }
}

pub fn _call_terms(name: &str, arg: String) -> PyResult<String> {
    let file = format!("{}.py", name);
    let code = read_code(&file);
    let gil = Python::acquire_gil();
    let py = gil.python();
    let activators = PyModule::from_code(py, &code, &file, &name)?;
    let terms: String = activators.call1("terms", (arg,))?.extract()?;
    Ok(terms)
}

pub fn call_after(name: &str) -> Result<(), MyError> {
    let after = _call_after(name);
    if let Ok(_) = after {
        return Ok(());
    }
    Err(MyError::OtherError(StrError::new("call after fail")))
}

pub fn _call_after(name: &str) -> PyResult<()> {
    let file = format!("{}.py", name);
    let code = read_code(&file);
    let gil = Python::acquire_gil();
    let py = gil.python();
    let activators = PyModule::from_code(py, &code, &file, &name)?;
    let _res: String = activators.call1("after", ())?.extract()?;
    Ok(())
}
pub fn call_rollback(name: &str) -> Result<(), MyError> {
    let a = _call_rollback(name);
    if let Ok(_) = a {
        return Ok(());
    }
    Err(MyError::OtherError(StrError::new("call roll back fail")))
}

pub fn _call_rollback(name: &str) -> PyResult<()> {
    let file = format!("{}.py", name);
    let code = read_code(&file);
    let gil = Python::acquire_gil();
    let py = gil.python();
    let activators = PyModule::from_code(py, &code, &file, &name)?;
    let _res: String = activators.call1("rollback", ())?.extract()?;
    Ok(())
}
pub fn test() {
    let res = call_after("test");
    dbg!(res);
}
