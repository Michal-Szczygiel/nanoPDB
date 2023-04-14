use crate::atom::Atom;

use pyo3::{
    exceptions::PyIndexError, pyclass, pymethods, Py, PyRefMut, PyResult, PyTraverseError, PyVisit,
    Python,
};

#[pyclass]
pub struct Residue {
    #[pyo3(get)]
    number: isize,

    #[pyo3(get)]
    name: String,

    atoms: Vec<Option<Py<Atom>>>,
    current_index: usize,
}

#[pymethods]
impl Residue {
    pub fn __clear__(&mut self) {
        for atom in self.atoms.iter_mut() {
            *atom = None;
        }
    }

    pub fn __getitem__(&self, python: Python, index: usize) -> PyResult<Py<Atom>> {
        if index < self.atoms.len() {
            Ok(self.atoms[index].as_ref().expect("").as_ref(python).into())
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
}

impl Residue {
    #[inline(always)]
    pub fn new(number: isize, name: &str) -> Self {
        Residue {
            number,
            name: name.to_string(),
            atoms: Vec::default(),
            current_index: 0,
        }
    }

    #[inline(always)]
    pub fn add_atom(&mut self, python: Python, atom: Atom) -> PyResult<()> {
        self.atoms.push(Some(Py::new(python, atom)?));

        Ok(())
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
