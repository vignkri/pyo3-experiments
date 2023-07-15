//! Experiments with Arrow Interoperabilty
//!
//! Testing the interoperability of PyO3 with messages sent to Rust
//! and passed from python using bytes that are parsed back.

use polars::{
    io::{SerReader, SerWriter},
    prelude::{IpcStreamWriter, IpcWriter},
};
use pyo3::prelude::*;
use std::{fs::File, io::Read, marker::PhantomData};

/// Initialiser for the python handle
struct PythonHandler<'a> {
    entry_file: String,
    phantom: PhantomData<&'a u8>,
}

impl<'a> PythonHandler<'a> {
    /// Generate the new handle for handling operations
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

    /// Run the python handle as long as <T> can be converted to a python object
    fn run(&self, data: Box<&[u8]>) -> Result<Vec<u8>, PyErr> {
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
    let mut roblox_data = polars::io::csv::CsvReader::from_path("./data/RBLX.csv")
        .expect("unable to open data")
        .has_header(true)
        // .with_schema (can be used for verifying schema of the values)
        .finish()
        .expect("unable to parse to csv dataframe");

    // issue: only writes 39723 bytes and the buffer also is only 39727 bytes. However, the expected
    // metadata bytes are approximately 1330795073 bytes. This would mean that the reader is probably not
    // expecting compression. The ipcWriter actually writes to a file and not a stream.
    let mut buffer = Vec::new();
    let mut writer = IpcStreamWriter::new(&mut buffer).with_compression(None);
    writer.finish(&mut roblox_data).expect("Writing to buffer");

    // generate the buffer for storage
    let dataframe = Box::new(buffer.as_slice());

    // send forward to the buffer and thereby to the system
    let result_from_python = snake.run(dataframe);
    // // run from python
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
