use pyo3::{pyclass, pymethods};

use heapless;

#[derive(Clone)]
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

#[pyclass(module = "nanoPDB", frozen)]
pub struct Atom {
    pub label: AtomType,

    /// [int] Atom number.
    #[pyo3(get)]
    pub number: i32,

    pub name: heapless::String<4>,
    pub element: heapless::String<4>,

    /// [(float, float, float)] Position of an atom in 3D space.
    #[pyo3(get)]
    pub position: (f64, f64, f64),

    /// [float] Atom occupancy.
    #[pyo3(get)]
    pub occupancy: f64,
}

/// Atom - a class that represents an atom of a PDB structure.
#[pymethods]
impl Atom {
    // ----------------------------------------------------------------------------------------
    // Getters
    // ----------------------------------------------------------------------------------------

    /// [str] Chemical element name.
    #[getter]
    pub fn element(&self) -> String {
        self.element.to_string()
    }

    /// [str] Indicates the type of atom.
    #[getter]
    pub fn label(&self) -> String {
        format!("{}", self.label)
    }

    /// [str] Atom name.
    #[getter]
    pub fn name(&self) -> String {
        self.name.to_string()
    }

    // ----------------------------------------------------------------------------------------
    // Special methods
    // ----------------------------------------------------------------------------------------

    pub fn __repr__(&self) -> String {
        format!("{:#}", self)
    }
}

impl Atom {
    #[inline(always)]
    pub fn new(
        label: AtomType,
        number: i32,
        name: &str,
        element: &str,
        position: (f64, f64, f64),
        occupancy: f64,
    ) -> Self {
        Atom {
            label,
            number,
            name: name.into(),
            element: element.into(),
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
