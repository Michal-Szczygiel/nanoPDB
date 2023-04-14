use pyo3::{pyclass, pymethods};

#[pyclass]
#[derive(Default)]
pub struct UnitCell {
    #[pyo3(get)]
    pub a: f64,

    #[pyo3(get)]
    pub b: f64,

    #[pyo3(get)]
    pub c: f64,

    #[pyo3(get)]
    pub alpha: f64,

    #[pyo3(get)]
    pub beta: f64,

    #[pyo3(get)]
    pub gamma: f64,
}

#[pymethods]
impl UnitCell {
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
