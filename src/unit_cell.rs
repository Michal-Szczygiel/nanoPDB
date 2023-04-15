use pyo3::{pyclass, pymethods};

/// UnitCell - a class that represents a unit cell of a PDB structure.
#[pyclass(module = "nanoPDB", frozen)]
#[derive(Default)]
pub struct UnitCell {
    /// [float] Length of side 'a' of unit cell.
    #[pyo3(get)]
    pub a: f64,

    /// [float] Length of side 'b' of unit cell.
    #[pyo3(get)]
    pub b: f64,

    /// [float] Length of side 'c' of unit cell.
    #[pyo3(get)]
    pub c: f64,

    /// [float] Alpha angle ('b' -> 'c') of unit cell (in radians).
    #[pyo3(get)]
    pub alpha: f64,

    /// [float] Beta angle ('c' -> 'a') of unit cell (in radians).
    #[pyo3(get)]
    pub beta: f64,

    /// [float] amma angle ('a' -> 'b') of unit cell (in radians).
    #[pyo3(get)]
    pub gamma: f64,
}

#[pymethods]
impl UnitCell {
    // ----------------------------------------------------------------------------------------
    // Special methods
    // ----------------------------------------------------------------------------------------

    pub fn __repr__(&self) -> String {
        format!("{:#}", self)
    }
}

impl UnitCell {
    #[inline(always)]
    pub fn new(a: f64, b: f64, c: f64, alpha: f64, beta: f64, gamma: f64) -> Self {
        UnitCell {
            a,
            b,
            c,
            alpha,
            beta,
            gamma,
        }
    }
}

impl std::fmt::Display for UnitCell {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("UnitCell")
            .field("a", &self.a)
            .field("b", &self.b)
            .field("c", &self.c)
            .field("alpha", &self.alpha)
            .field("beta", &self.beta)
            .field("gamma", &self.gamma)
            .finish()
    }
}
