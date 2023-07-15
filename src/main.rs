//! Experiments with Arrow Interoperabilty
//!
//! Testing the interoperability of PyO3 with messages sent to Rust
//! and passed from python using bytes that are parsed back.

use pyo3::prelude::*;
use std::{fs::File, io::Read, marker::PhantomData};

struct PythonHandler<'a> {
    entry_file: String,
    phantom: PhantomData<&'a u8>,
}

impl<'a> PythonHandler<'a> {
    fn new(path: &str) -> Self {
        let mut locn = env!("CARGO_MANIFEST_DIR").to_owned();
        locn.push_str(path);

        // open file
        let mut py_entrypoint = File::open(locn).expect("Unable to open file");
        let mut file_contents = String::new();
        py_entrypoint.read_to_string(&mut file_contents).unwrap();

        Self {
            entry_file: file_contents,
            phantom: PhantomData,
        }
    }

    ///
    fn run<T: pyo3::IntoPy<Py<PyAny>>>(&self, data: Box<T>) -> Result<Vec<u8>, PyErr> {
        // generate a result from the python interface
        let res = Python::with_gil(|python| -> Result<Vec<u8>, PyErr> {
            // log the currently utilised version of python
            let version_info = python.version_info();
            println!("Python: {:?}", version_info);

            // load the python versioning of the code
            let app: &PyAny = PyModule::from_code(python, &self.entry_file, "", "")
                .unwrap()
                .getattr("handler")
                .unwrap();

            // run the code
            let result: &PyAny = app.call((data.into_py(python),), None)?;

            // handle data conversions to the internal rust objects
            let data: &[u8] = result.extract()?;
            let byte_vec = data.iter().map(|v| *v).collect::<Vec<u8>>();

            // return a valid byte vec
            Ok(byte_vec)
        });

        res
    }
}

fn main() {
    // startup the gil for operations
    pyo3::prepare_freethreaded_python();

    // read the file structures and handle the application layer
    let snake = PythonHandler::new("/cart/entry.py");

    // Store in box and retrieve from box
    let data = String::from("hello");
    let argument_data = data.as_bytes();
    let as_boxed = Box::new(argument_data);

    // generate result
    let result_from_python = snake.run(as_boxed);

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
