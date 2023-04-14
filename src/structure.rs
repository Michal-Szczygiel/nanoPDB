use crate::{atom::Atom, chain::Chain, residue::Residue, unit_cell::UnitCell};

use pyo3::{
    exceptions::PyIndexError, pyclass, pymethods, Py, PyRefMut, PyResult, PyTraverseError, PyVisit,
    Python,
};

#[pyclass]
#[derive(Default)]
pub struct Structure {
    #[pyo3(get)]
    pdbid: String,

    #[pyo3(get)]
    classification: String,

    #[pyo3(get)]
    date: String,

    unit_cell: Option<Py<UnitCell>>,
    chains: Vec<Option<Py<Chain>>>,
    current_index: usize,
}

#[pymethods]
impl Structure {
    #[getter]
    pub fn unit_cell(&self, python: Python) -> Py<UnitCell> {
        self.unit_cell
            .as_ref()
            .expect(concat!("memory error in: ", file!(), ", line: ", line!()))
            .as_ref(python)
            .into()
    }

    pub fn __clear__(&mut self) {
        self.unit_cell = None;

        for chain in self.chains.iter_mut() {
            *chain = None;
        }
    }

    pub fn __getitem__(&self, python: Python, index: usize) -> PyResult<Py<Chain>> {
        if index < self.chains.len() {
            Ok(self.chains[index].as_ref().expect("").as_ref(python).into())
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

        for chain in self.chains.iter() {
            if let Some(chain) = chain {
                visit.call(chain)?;
            }
        }

        Ok(())
    }
}

impl Structure {
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

    #[inline(always)]
    pub fn add_atom(&mut self, python: Python, atom: Atom) -> PyResult<()> {
        self.chains
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
        self.chains
            .last()
            .unwrap()
            .as_ref()
            .expect(concat!("memory error in: ", file!(), ", line: ", line!()))
            .borrow_mut(python)
            .add_residue(python, residue)?;

        Ok(())
    }

    #[inline(always)]
    pub fn add_chain(&mut self, python: Python, chain: Chain) -> PyResult<()> {
        self.chains.push(Some(Py::new(python, chain)?));

        Ok(())
    }
}

impl std::fmt::Display for Structure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Structure")
            .field("name", &self.pdbid)
            .finish()
    }
}
