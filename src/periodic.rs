use pyo3::{exceptions::PyValueError, pyclass, pymethods, PyResult};

use std::collections::HashMap;

static TABLE: &[(&str, f64)] = &[
    ("H", 1.20),
    ("HE", 1.40),
    ("LI", 1.82),
    ("BE", 1.53),
    ("B", 1.92),
    ("C", 1.70),
    ("N", 1.55),
    ("O", 1.52),
    ("F", 1.47),
    ("NE", 1.54),
    ("NA", 2.27),
    ("MG", 1.73),
    ("AL", 1.84),
    ("SI", 2.10),
    ("P", 1.80),
    ("S", 1.80),
    ("CL", 1.75),
    ("AR", 1.88),
    ("K", 2.75),
    ("CA", 2.31),
    ("SC", 2.11),
    ("NI", 1.63),
    ("CU", 1.40),
    ("ZN", 1.39),
    ("GA", 1.87),
    ("GE", 2.11),
    ("AS", 1.85),
    ("SE", 1.90),
    ("BR", 1.85),
    ("KR", 2.02),
    ("RB", 3.03),
    ("SR", 2.49),
    ("PD", 1.63),
    ("AG", 1.72),
    ("CD", 1.58),
    ("IN", 1.93),
    ("SN", 2.17),
    ("SB", 2.06),
    ("TE", 2.06),
    ("I", 1.98),
    ("XE", 2.16),
    ("CS", 3.43),
    ("BA", 2.68),
    ("PT", 1.75),
    ("AU", 1.66),
    ("HG", 1.55),
    ("TL", 1.96),
    ("PB", 2.02),
    ("BI", 2.07),
    ("PO", 1.97),
    ("AT", 2.02),
    ("RN", 2.20),
    ("FR", 3.48),
    ("RA", 2.83),
    ("U", 1.86),
];

#[pyclass(module = "nanoPDB", frozen)]
pub struct Periodic {
    table: HashMap<&'static str, f64>,
}

#[pymethods]
impl Periodic {
    #[new]
    pub fn __new__() -> Self {
        Periodic {
            table: TABLE
                .iter()
                .cloned()
                .map(|(atom, radius)| (atom.into(), radius))
                .collect(),
        }
    }

    #[pyo3(signature = (atom, /))]
    pub fn get_radius(&self, atom: String) -> PyResult<f64> {
        match self.table.get(&atom.to_uppercase().trim()) {
            Some(radius) => Ok(*radius),
            None => Err(PyValueError::new_err(format!(
                "atom: {} not supported",
                atom.to_uppercase().trim()
            ))),
        }
    }
}
