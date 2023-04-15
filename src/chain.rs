use crate::{atom::Atom, residue::Residue};

use pyo3::{
    exceptions::PyIndexError, pyclass, pymethods, types::PyList, Py, PyRefMut, PyResult,
    PyTraverseError, PyVisit, Python,
};

/// Chain - a class that represents a chain of a PDB structure.
#[pyclass(module = "nanoPDB")]
pub struct Chain {
    /// [str] Chain name.
    #[pyo3(get)]
    pub name: char,

    pub residues: Vec<Option<Py<Residue>>>,
    pub current_index: usize,
}

#[pymethods]
impl Chain {
    // ----------------------------------------------------------------------------------------
    // Special methods
    // ----------------------------------------------------------------------------------------

    pub fn __clear__(&mut self) {
        for residue in self.residues.iter_mut() {
            *residue = None;
        }
    }

    pub fn __getitem__(&self, python: Python, index: usize) -> PyResult<Py<Residue>> {
        if index < self.residues.len() {
            Ok(self.residues[index]
                .as_ref()
                .expect("")
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
        self.residues.len()
    }

    pub fn __next__(&mut self, python: Python) -> Option<Py<Residue>> {
        if self.current_index < self.residues.len() {
            self.current_index += 1;

            Some(
                self.residues[self.current_index - 1]
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
        for residue in self.residues.iter() {
            if let Some(residue) = residue {
                visit.call(residue)?;
            }
        }

        Ok(())
    }

    // ----------------------------------------------------------------------------------------
    // Methods
    // ----------------------------------------------------------------------------------------

    /// Returns a list of atoms that builds the chain.
    ///
    ///
    /// Returns
    /// -------
    /// list[Atom]
    ///     The list of atoms that build the chain.
    ///
    ///
    /// Examples
    /// --------
    /// Retrieving the list of atoms that builds the chain.
    ///
    /// >>> parser = nanoPDB.Parser()
    /// >>> structure = parser.fetch("1zhy")
    /// >>> chain = structure[0]
    /// >>> chain.get_atoms()
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
    pub fn get_atoms(&self, python: Python) -> PyResult<Py<PyList>> {
        let atoms = PyList::empty(python);

        for residue in self.residues.iter().map(|residue| {
            residue
                .as_ref()
                .expect(concat!("memory error in: ", file!(), ", line: ", line!()))
                .borrow(python)
        }) {
            for atom in residue.atoms.iter().map(|atom| {
                atom.as_ref()
                    .expect(concat!("memory error in: ", file!(), ", line: ", line!()))
            }) {
                atoms.append(atom)?;
            }
        }

        Ok(atoms.into())
    }

    /// Returns the list of residues that builds the chain.
    ///
    ///
    /// Returns
    /// -------
    /// list[Residue]
    ///     The list of residues that builds the chain.
    ///
    ///
    /// Examples
    /// --------
    /// Retrieving the list of residues that builds the residue.
    ///
    /// >>> parser = nanoPDB.Parser()
    /// >>> structure = parser.fetch("1zhy")
    /// >>> chain = structure[0]
    /// >>> chain.get_residues()
    ///
    /// [Residue {
    ///     number: -1,
    ///     name: "MET",
    /// }, Residue {
    ///     number: 0,
    ///     name: "ASP",
    /// }, Residue {
    ///     number: 1,
    ///     name: "PRO",
    /// }
    /// ...
    #[pyo3(signature = (/))]
    pub fn get_residues(&self, python: Python) -> Py<PyList> {
        PyList::new(
            python,
            self.residues.iter().map(|residue| {
                residue
                    .as_ref()
                    .expect(concat!("memory error in: ", file!(), ", line: ", line!()))
            }),
        )
        .into()
    }
}

impl Chain {
    #[inline(always)]
    pub fn new(name: char) -> Self {
        Chain {
            name,
            residues: Vec::default(),
            current_index: 0,
        }
    }

    #[inline(always)]
    pub fn add_atom(&mut self, python: Python, atom: Atom) -> PyResult<()> {
        self.residues
            .last()
            .unwrap()
            .as_ref()
            .expect(concat!("memory error in: ", file!(), ", line: ", line!()))
            .borrow_mut(python)
            .add_atom(python, atom)?;

        Ok(())
    }

    #[inline(always)]
    pub fn add_residue(&mut self, python: Python, residue: Residue) -> PyResult<()> {
        self.residues.push(Some(Py::new(python, residue)?));

        Ok(())
    }
}

impl std::fmt::Display for Chain {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Chain")
            .field("name", &self.name)
            .finish()
    }
}
