#![allow(non_snake_case)]

mod atom;
mod chain;
mod parser;
mod residue;
mod structure;
mod unit_cell;

use pyo3::{pymodule, types::PyModule, PyResult, Python};

#[pymodule]
fn nanoPDB(_python: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<atom::Atom>()?;
    module.add_class::<chain::Chain>()?;
    module.add_class::<parser::Parser>()?;
    module.add_class::<residue::Residue>()?;
    module.add_class::<structure::Structure>()?;
    module.add_class::<unit_cell::UnitCell>()?;

    Ok(())
}
