//! Experiments with Arrow Interoperabilty
//!
//! Testing the interoperability of PyO3 with messages sent to Rust
//! and passed from python using bytes that are parsed back.

use pyo3::prelude::*;

fn main() {
    // read the file structures and handle the application layer
    let python_entry = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/cart/arrow.py"));

    // startup the gil for operations
    pyo3::prepare_freethreaded_python();

    // generate a result from the python interface
    let result_from_python = Python::with_gil(|python| -> Result<Vec<u8>, PyErr> {
        // log the currently utilised version of python
        let version_info = python.version_info();
        println!("Python: {:?}", version_info);

        // load the python versioning of the code
        let app: &PyAny = PyModule::from_code(python, python_entry, "", "")
            .unwrap()
            .getattr("handler")
            .unwrap();

        // run the code
        let data = String::from("hello");
        let argument_data = data.as_bytes();

        let result: &PyAny = app.call((argument_data.into_py(python),), None)?;

        // handle data conversions to the internal rust objects
        let data: &[u8] = result.extract()?;
        let byte_vec = data.iter().map(|v| *v).collect::<Vec<u8>>();

        // return a valid byte vec
        Ok(byte_vec)
    });

    // run from python
    match result_from_python {
        Ok(inner) => {
            let byte_as_str = String::from_utf8(inner).unwrap();
            println!("'{:?}', from Python", byte_as_str);
        }
        Err(errmsg) => {
            println!("Python Error: {:?}", errmsg);
        }
    }
}
