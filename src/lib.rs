#[macro_use]
extern crate cpython;

use cpython::{Python, PyResult};

fn rustfib(terms: u64) -> u64 {
    if terms <= 1 {
        terms
    }
    else {
        rustfib(terms - 1) + rustfib(terms - 2)
    }
}


fn pyrustfib(_py: Python, terms: u64) -> PyResult<u64> {
    Ok(rustfib(terms))
}


fn count_doubles(_py: Python, val: &str) -> PyResult<u64> {
    let mut total = 0u64;

    for (char1, char2) in val.chars().zip(val.chars().skip(1)) {
        if char1 == char2 {
            total += 1;
        }
    }

    Ok(total)

}

py_module_initializer!(librusttest, initlibtestlib, PyInit_librusttest, |py, m | {
    try!(m.add(py, "__doc__", "This module is written in Rust!"));
    try!(m.add(py, "count_doubles", py_fn!(py, count_doubles(val: &str))));
    try!(m.add(py, "rustfib", py_fn!(py, pyrustfib(terms: u64))));
    Ok(())

});
