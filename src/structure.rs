use crate::{chain::Chain, unit_cell::UnitCell};

use pyo3::{
    exceptions::PyIndexError, pyclass, pymethods, types::PyList, Py, PyRefMut, PyResult,
    PyTraverseError, PyVisit, Python,
};

use indexmap::IndexMap;

/// Structure - a class that represents a PDB structure.
#[pyclass(module = "nanoPDB")]
pub struct Structure {
    /// [str] PDB ID of a structure.
    #[pyo3(get)]
    pub pdbid: String,

    /// [str] Classifications of a macromolecule.
    #[pyo3(get)]
    pub classification: String,

    /// [str] Deposition date.
    #[pyo3(get)]
    pub date: String,

    pub unit_cell: Option<Py<UnitCell>>,
    pub chains: IndexMap<char, Option<Py<Chain>>>,
    pub current_index: usize,
}

#[pymethods]
impl Structure {
    // ----------------------------------------------------------------------------------------
    // Getters
    // ----------------------------------------------------------------------------------------

    /// [UnitCell] The unit cell of the structure.
    #[getter]
    pub fn unit_cell(&self, python: Python) -> Py<UnitCell> {
        self.unit_cell
            .as_ref()
            .expect(concat!("memory error in: ", file!(), ", line: ", line!()))
            .as_ref(python)
            .into()
    }

    // ----------------------------------------------------------------------------------------
    // Special methods
    // ----------------------------------------------------------------------------------------

    pub fn __clear__(&mut self) {
        self.unit_cell = None;

        for chain in self.chains.values_mut() {
            *chain = None;
        }
    }

    pub fn __getitem__(&self, python: Python, index: usize) -> PyResult<Py<Chain>> {
        if index < self.chains.len() {
            Ok(self.chains[index]
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
        self.chains.len()
    }

    pub fn __next__(&mut self, python: Python) -> Option<Py<Chain>> {
        if self.current_index < self.chains.len() {
            self.current_index += 1;

            Some(
                self.chains[self.current_index - 1]
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
        if let Some(unit_cell) = &self.unit_cell {
            visit.call(unit_cell)?;
        }

        for chain in self.chains.values() {
            if let Some(chain) = chain {
                visit.call(chain)?;
            }
        }

        Ok(())
    }

    // ----------------------------------------------------------------------------------------
    // Methods
    // ----------------------------------------------------------------------------------------

    /// Returns a list of atoms that builds the structure.
    ///
    ///
    /// Returns
    /// -------
    /// list[Atom]
    ///     The list of atoms that build the structure.
    ///
    ///
    /// Examples
    /// --------
    /// Retrieving the list of atoms that builds the structure.
    ///
    /// >>> parser = nanoPDB.Parser()
    /// >>> structure = parser.fetch("1zhy")
    /// ...
    /// >>> structure.get_atoms()
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

        for chain in self.chains.values().map(|chain| {
            chain
                .as_ref()
                .expect(concat!("memory error in: ", file!(), ", line: ", line!()))
                .borrow(python)
        }) {
            for residue in chain.residues.values().map(|residue| {
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
        }

        Ok(atoms.into())
    }

    /// Returns the list of chains that builds the chain.
    ///
    ///
    /// Returns
    /// -------
    /// list[Chain]
    ///     The list of chains that builds the structure.
    ///
    ///
    /// Examples
    /// --------
    /// Retrieving the list of chains that builds the structure.
    ///
    /// >>> parser = nanoPDB.Parser()
    /// >>> structure = parser.fetch("1zhy")
    /// ...
    /// >>> structure.get_chains()
    ///
    /// [Chain {
    ///     name: 'A',
    /// }]
    #[pyo3(signature = (/))]
    pub fn get_chains(&self, python: Python) -> Py<PyList> {
        PyList::new(
            python,
            self.chains.values().map(|chain| {
                chain
                    .as_ref()
                    .expect(concat!("memory error in: ", file!(), ", line: ", line!()))
            }),
        )
        .into()
    }

    /// Returns the list of residues that builds the structure.
    ///
    ///
    /// Returns
    /// -------
    /// list[Residue]
    ///     The list of residues that builds the structure.
    ///
    ///
    /// Examples
    /// --------
    /// Retrieving the list of residues that builds the structure.
    ///
    /// >>> parser = nanoPDB.Parser()
    /// >>> structure = parser.fetch("1zhy")
    /// ...
    /// >>> structure.get_residues()
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
    pub fn get_residues(&self, python: Python) -> PyResult<Py<PyList>> {
        let residues = PyList::empty(python);

        for chain in self.chains.values().map(|chain| {
            chain
                .as_ref()
                .expect(concat!("memory error in: ", file!(), ", line: ", line!()))
                .borrow(python)
        }) {
            for residue in chain.residues.values().map(|residue| {
                residue
                    .as_ref()
                    .expect(concat!("memory error in: ", file!(), ", line: ", line!()))
            }) {
                residues.append(residue)?;
            }
        }

        Ok(residues.into())
    }
}

impl Structure {
    pub fn new(python: Python) -> PyResult<Self> {
        Ok(Structure {
            pdbid: String::default(),
            classification: String::default(),
            date: String::default(),
            unit_cell: Some(Py::new(python, UnitCell::default())?),
            chains: IndexMap::default(),
            current_index: 0,
        })
    }

    #[inline(always)]
    pub fn set_header(&mut self, pdbid: &str, classification: &str, date: &str) {
        self.pdbid = pdbid.to_string();
        self.classification = classification.to_string();
        self.date = date.to_string();
    }

    #[inline(always)]
    pub fn set_unit_cell(&mut self, python: Python, unit_cell: UnitCell) -> PyResult<()> {
        self.unit_cell = Some(Py::new(python, unit_cell)?);

        Ok(())
    }
}

impl std::fmt::Display for Structure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Structure")
            .field("pdbid", &self.pdbid)
            .field("classification", &self.classification)
            .field("date", &self.date)
            .finish()
    }
}
