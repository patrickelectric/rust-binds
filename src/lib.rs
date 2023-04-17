#![recursion_limit = "1024"]

use rand::Rng;

#[cfg(feature = "python")]
use pyo3::{prelude::*, wrap_pyfunction};

macro_rules! export_cpy {
    ($(enum $enum_name:ident { $($enum_variant:ident,)* })+
     $(struct $struct_name:ident { $($struct_field:ident : $struct_field_type:ty,)* })+
     $(fn $func_name:ident() -> $func_ret:ty $func_body:block)+) => {

        $(
            export_cpy!(enum $enum_name { $($enum_variant,)* });
        )+

        $(
            export_cpy!(struct $struct_name { $($struct_field : $struct_field_type,)* });
        )+

        $(
            export_cpy!(fn $func_name() -> $func_ret $func_body);
        )+

        #[cfg(feature = "python")]
        #[pymodule]
        fn bind_test(_py: Python, m: &PyModule) -> PyResult<()> {
            $(
                m.add_class::<$struct_name>()?;
            )+
            $(
                m.add_class::<$enum_name>()?;
            )+
            $(
                m.add_wrapped(wrap_pyfunction!($func_name))?;
            )+
            Ok(())
        }
    };
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
            fn $name() -> $ret {
                $body
            }
        )+
    };
}

export_cpy!(
    enum Material {
        Plastic,
        Rubber,
    }

    struct Size2D {
        width: f64,
        height: f64,
    }

    struct Tire {
        material: Material,
        pressure: f64,
        size: Size2D,
    }

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