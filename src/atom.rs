use pyo3::{pyclass, pymethods};

pub enum AtomType {
    ATOM,
    HETATM,
}

impl std::fmt::Display for AtomType {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AtomType::ATOM => {
                write!(formatter, "ATOM")
            }
            AtomType::HETATM => {
                write!(formatter, "HETATM")
            }
        }
    }
}

#[pyclass]
pub struct Atom {
    label: AtomType,

    #[pyo3(get)]
    number: usize,

    #[pyo3(get)]
    name: String,

    #[pyo3(get)]
    element: String,

    #[pyo3(get)]
    position: (f64, f64, f64),

    #[pyo3(get)]
    occupancy: f64,
}

#[pymethods]
impl Atom {
    #[getter]
    pub fn label(&self) -> String {
        format!("{}", self.label)
    }

    pub fn __repr__(&self) -> String {
        format!("{:#}", self)
    }
}

impl Atom {
    #[inline(always)]
    pub fn new(
        label: AtomType,
        number: usize,
        name: &str,
        element: &str,
        position: (f64, f64, f64),
        occupancy: f64,
    ) -> Self {
        Atom {
            label,
            number,
            name: name.to_string(),
            element: element.to_string(),
            position,
            occupancy,
        }
    }
}

impl std::fmt::Display for Atom {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Atom")
            .field("label", &self.label())
            .field("number", &self.number)
            .field("name", &self.name)
            .field("element", &self.element)
            .field("position", &self.position)
            .field("occupancy", &self.occupancy)
            .finish()
    }
}
