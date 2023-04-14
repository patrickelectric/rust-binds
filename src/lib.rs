use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use rand::Rng;

#[derive(Clone, Debug)]
#[repr(C)]
#[pyclass]
pub enum Material {
    Plastic,
    Rubber,
}

#[derive(Clone, Debug)]
#[repr(C)]
#[pyclass]
pub struct Size2D {
    #[pyo3(get)]
    pub width: f64,
    #[pyo3(get)]
    pub height: f64,
}

#[derive(Clone, Debug)]
#[repr(C)]
#[pyclass]
pub struct Tire {
    #[pyo3(get)]
    pub material: Material,
    #[pyo3(get)]
    pub pressure: f64,
    #[pyo3(get)]
    pub size: Size2D,
}

#[pyfunction]
#[no_mangle]
pub extern "C" fn create_random_tire() -> Tire {
    let mut rng = rand::thread_rng();
    Tire {
        material: Material::Plastic,
        pressure: rng.gen_range(30.0..60.0),
        size: Size2D {
            width: rng.gen_range(5.0..10.0),
            height: rng.gen_range(10.0..20.0),
        },
    }
}

#[pymodule]
fn bind_test(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Tire>()?;
    m.add_class::<Size2D>()?;
    m.add_class::<Material>()?;
    m.add_wrapped(wrap_pyfunction!(create_random_tire))?;

    Ok(())
}
