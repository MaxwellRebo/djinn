#![feature(box_syntax)]

extern crate djinn;
extern crate cpython;
use cpython::Python;
use cpython::ObjectProtocol;

fn main() {
    // First test Python bindings
    let gil = Python::acquire_gil();
    let py = gil.python();

    let sys = py.import("sys").unwrap();
    let version: String = sys.get(py, "version").unwrap().extract(py).unwrap();

    let os = py.import("os").unwrap();
    let getenv = os.get(py, "getenv").unwrap();
    let user: String = getenv.call(py, ("USER",), None).unwrap().extract(py).unwrap();

    println!("Hello {}, I'm Python {}", user, version);
}

