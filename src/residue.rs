use crate::atom::Atom;

use pyo3::{
    exceptions::PyIndexError, pyclass, pymethods, types::PyList, Py, PyRefMut, PyResult,
    PyTraverseError, PyVisit, Python,
};

use heapless;

/// Residue - a class that represents a residue of a PDB structure.
#[pyclass(module = "nanoPDB")]
pub struct Residue {
    /// [int] Residue number.
    #[pyo3(get)]
    pub number: i32,

    pub name: heapless::String<4>,
    pub atoms: Vec<Option<Py<Atom>>>,
    pub current_index: usize,
}

#[pymethods]
impl Residue {
    // ----------------------------------------------------------------------------------------
    // Getters
    // ----------------------------------------------------------------------------------------

    /// [str] Residue name.
    #[getter]
    pub fn name(&self) -> String {
        self.name.to_string()
    }

    // ----------------------------------------------------------------------------------------
    // Special methods
    // ----------------------------------------------------------------------------------------

    pub fn __clear__(&mut self) {
        for atom in self.atoms.iter_mut() {
            *atom = None;
        }
    }

    pub fn __getitem__(&self, python: Python, index: usize) -> PyResult<Py<Atom>> {
        if index < self.atoms.len() {
            Ok(self.atoms[index]
                .as_ref()
                .expect(concat!("memory error in: ", file!(), ", line: ", line!()))
                .as_ref(python)
                .into())
        } else {
            Err(PyIndexError::new_err("index out of range"))
        }
    }

    pub fn __iter__(mut slf: PyRefMut<Self>) -> PyRefMut<Self> {
        slf.current_index = 0;

        slf
    }

    pub fn __len__(&self) -> usize {
        self.atoms.len()
    }

    pub fn __next__(&mut self, python: Python) -> Option<Py<Atom>> {
        if self.current_index < self.atoms.len() {
            self.current_index += 1;

            Some(
                self.atoms[self.current_index - 1]
                    .as_ref()
                    .expect(concat!("memory error in: ", file!(), ", line: ", line!()))
                    .as_ref(python)
                    .into(),
            )
        } else {
            None
        }
    }

    pub fn __repr__(&self) -> String {
        format!("{:#}", self)
    }

    pub fn __traverse__(&self, visit: PyVisit<'_>) -> Result<(), PyTraverseError> {
        for atom in self.atoms.iter() {
            if let Some(atom) = atom {
                visit.call(atom)?;
            }
        }

        Ok(())
    }

    // ----------------------------------------------------------------------------------------
    // Methods
    // ----------------------------------------------------------------------------------------

    /// Returns a list of atoms that builds the residue.
    ///
    ///
    /// Returns
    /// -------
    /// list[Atom]
    ///     The list of atoms that builds the residue.
    ///
    ///
    /// Examples
    /// --------
    /// Retrieving the list of atoms that builds the residue.
    ///
    /// >>> parser = nanoPDB.Parser()
    /// >>> structure = parser.fetch("1zhy")
    /// >>> residue = structure[0][0]
    /// ...
    /// >>> residue.get_atoms()
    ///
    /// [Atom {
    ///     label: "ATOM",
    ///     number: 1,
    ///     name: "N",
    ///     element: "N",
    ///     position: (
    ///         42.854,
    ///         36.56,
    ///         10.394,
    ///     ),
    ///     occupancy: 1.0,
    /// }, Atom {
    ///     label: "ATOM",
    ///     number: 2,
    ///     name: "CA",
    ///     element: "C",
    ///     position: (
    ///         42.25,
    ///         35.232,
    ///         10.096,
    ///     ),
    ///     occupancy: 1.0,
    /// }, Atom {
    ///     label: "ATOM",
    ///     number: 3,
    ///     name: "C",
    ///     element: "C",
    ///     position: (
    ///         41.642,
    ///         34.623,
    ///         11.355,
    ///     ),
    ///     occupancy: 1.0,
    /// }
    /// ...
    #[pyo3(signature = (/))]
    pub fn get_atoms(&self, python: Python) -> Py<PyList> {
        PyList::new(
            python,
            self.atoms.iter().map(|atom| {
                atom.as_ref()
                    .expect(concat!("memory error in: ", file!(), ", line: ", line!()))
            }),
        )
        .into()
    }
}

impl Residue {
    #[inline(always)]
    pub fn new(number: i32, name: &str) -> Self {
        Residue {
            number,
            name: name.into(),
            atoms: Vec::default(),
            current_index: 0,
        }
    }
}

impl std::fmt::Display for Residue {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Residue")
            .field("number", &self.number)
            .field("name", &self.name)
            .finish()
    }
}
