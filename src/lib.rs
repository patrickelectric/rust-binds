use rand::Rng;

#[cfg(feature = "python")]
use pyo3::{prelude::*, wrap_pyfunction};

macro_rules! export_cpy {
    (enum $name:ident { $($variant:ident,)* }) => {
        #[derive(Clone, Debug)]
        #[repr(C)]
        #[cfg_attr(feature = "python", pyo3::prelude::pyclass)]
        pub enum $name {
            $(
                $variant,
            )*
        }
    };
    (struct $name:ident { $($field:ident : $ftype:ty,)* }) => {
        #[derive(Clone, Debug)]
        #[repr(C)]
        #[cfg_attr(feature = "python", pyo3::prelude::pyclass(get_all, set_all))]
        pub struct $name {
            $(
                pub $field: $ftype,
            )*
        }
    };
    ($(fn $name:ident() -> $ret:ty $body:block)+) => {
        $(
            #[no_mangle]
            #[cfg_attr(feature = "python", pyfunction)]
            pub extern "C" fn $name() -> $ret {
                $body
            }
        )+
    };
    (generate_bind_test { $($class:ident),* }, { $($fn:ident),* }) => {
        #[cfg(feature = "python")]
        #[pymodule]
        fn bind_test(_py: Python, m: &PyModule) -> PyResult<()> {
            $(
                m.add_class::<$class>()?;
            )*
            $(
                m.add_wrapped(wrap_pyfunction!($fn))?;
            )*

            Ok(())
        }
    };
}

export_cpy!(
    enum Material {
        Plastic,
        Rubber,
    }
);

export_cpy!(
    struct Size2D {
        width: f64,
        height: f64,
    }
);

export_cpy!(
    struct Tire {
        material: Material,
        pressure: f64,
        size: Size2D,
    }
);

export_cpy!(
    fn create_random_tire() -> Tire {
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
);

export_cpy!(
    generate_bind_test {
        // Classes
        Tire,
        Size2D,
        Material
    },
    {
        // Functions
        create_random_tire
    }
);
