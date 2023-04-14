use rand::Rng;

#[cfg(feature = "python")]
use pyo3::{prelude::*, wrap_pyfunction};

#[derive(Clone, Debug)]
#[repr(C)]
#[cfg_attr(feature = "python", pyclass)]
pub enum Material {
    Plastic,
    Rubber,
}

#[derive(Clone, Debug)]
#[repr(C)]
#[cfg_attr(feature = "python", pyclass)]
pub struct Size2D {
    #[cfg_attr(feature = "python", pyo3(get))]
    pub width: f64,
    #[cfg_attr(feature = "python", pyo3(get))]
    pub height: f64,
}

#[derive(Clone, Debug)]
#[repr(C)]
#[cfg_attr(feature = "python", pyclass)]
pub struct Tire {
    #[cfg_attr(feature = "python", pyo3(get))]
    pub material: Material,
    #[cfg_attr(feature = "python", pyo3(get))]
    pub pressure: f64,
    #[cfg_attr(feature = "python", pyo3(get))]
    pub size: Size2D,
}

#[no_mangle]
#[cfg_attr(feature = "python", pyfunction)]
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

#[cfg(feature = "python")]
fn bind_test(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Tire>()?;
    m.add_class::<Size2D>()?;
    m.add_class::<Material>()?;
    m.add_wrapped(wrap_pyfunction!(create_random_tire))?;

    Ok(())
}
