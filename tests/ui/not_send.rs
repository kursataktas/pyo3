use pyo3::prelude::*;
use pyo3::types::PyString;

fn allow_thread_prevents_token() {
    Python::with_gil(|py| {
        py.allow_threads().with(|| { drop(py); });
    })
}

fn allow_thread_prevents_gil_ref() {
    Python::with_gil(|py| {
        let string = PyString::new(py, "foo");

        py.allow_threads().with(|| {
            println!("{:?}", string);
        });
    });
}

fn main() {
    allow_thread_prevents_token();
    allow_thread_prevents_gil_ref();
}
