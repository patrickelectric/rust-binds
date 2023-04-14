use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use rand::Rng;

#[repr(C)]
#[pyclass]
pub struct Point {
    #[pyo3(get)]
    pub x: f64,
    #[pyo3(get)]
    pub y: f64,
}

#[pyfunction]
#[no_mangle]
pub extern "C" fn create_random_point() -> Point {
    let mut rng = rand::thread_rng();
    Point {
        x: rng.gen_range(-100.0..100.0),
        y: rng.gen_range(-100.0..100.0),
    }
}

#[pymodule]
fn rust_bindings(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Point>()?;
    m.add_wrapped(wrap_pyfunction!(create_random_point))?;

    Ok(())
}
