from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="bind_test",
    version="0.1.0",
    rust_extensions=[RustExtension("bind_test.rust_bindings", "Cargo.toml", binding=Binding.PyO3)],
    packages=["bind_test"],
    zip_safe=False,
)