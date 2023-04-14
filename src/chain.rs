use crate::{atom::Atom, residue::Residue};

use pyo3::{
    exceptions::PyIndexError, pyclass, pymethods, Py, PyRefMut, PyResult, PyTraverseError, PyVisit,
    Python,
};

#[pyclass]
pub struct Chain {
    #[pyo3(get)]
    name: char,

    residues: Vec<Option<Py<Residue>>>,
    current_index: usize,
}

#[pymethods]
impl Chain {
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
