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

#[pyclass(module = "nanoPDB", frozen)]
pub struct Atom {
    pub label: AtomType,

    /// [int] Atom number.
    #[pyo3(get)]
    pub number: usize,

    /// [str] Atom name.
    #[pyo3(get)]
    pub name: String,

    /// [str] Chemical element name.
    #[pyo3(get)]
    pub element: String,

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

    /// [str] Indicates the type of atom.
    #[getter]
    pub fn label(&self) -> String {
        format!("{}", self.label)
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
