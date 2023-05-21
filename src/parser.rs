use crate::{
    atom::{Atom, AtomType},
    chain::Chain,
    residue::Residue,
    structure::Structure,
    unit_cell::UnitCell,
};

use pyo3::{exceptions::PyException, pyclass, pymethods, Py, PyResult, Python};

use std::{fs::File, io::Read, str::FromStr};

/// Parser - a class for parsing structures in PDB format.
#[pyclass(module = "nanoPDB", frozen)]
pub struct Parser;

#[pymethods]
impl Parser {
    // ----------------------------------------------------------------------------------------
    // Special methods
    // ----------------------------------------------------------------------------------------

    #[new]
    pub fn __new__() -> Self {
        Parser {}
    }

    // ----------------------------------------------------------------------------------------
    // Methods
    // ----------------------------------------------------------------------------------------

    /// Fetches structure from RCSB PDB database, parses it and returns Structure object.
    ///
    ///
    /// Parameters
    /// ----------
    /// pdbid : str
    ///     PDB ID of structure from RCSB PDB.
    ///
    ///
    /// Returns
    /// -------
    /// Structure
    ///     Parsed structure.
    ///
    ///
    /// Examples
    /// --------
    /// Fetching structure from RCSB PDB database.
    ///
    /// >>> parser = nanoPDB.Parser()
    /// >>> structure = parser.fetch("1zhy")
    /// ...
    /// >>> structure
    ///
    /// Structure {
    ///     pdbid: "1ZHY",
    ///     classification: "LIPID BINDING PROTEIN",
    ///     date: "26-APR-05",
    /// }
    #[pyo3(signature = (pdbid, /))]
    pub fn fetch(&self, python: Python, pdbid: String) -> PyResult<Structure> {
        let response = match reqwest::blocking::get(format!(
            "https://files.rcsb.org/download/{}.pdb",
            pdbid.to_lowercase()
        )) {
            Ok(response) => response,
            Err(error) => return Err(PyException::new_err(format!("{}", error))),
        };

        let response_status = response.status();

        if response_status != 200 {
            return Err(PyException::new_err(format!(
                "connection error, status: {}",
                response_status
            )));
        }

        let content = match response.text() {
            Ok(content) => content,
            Err(error) => return Err(PyException::new_err(format!("{}", error))),
        };

        parse_pdb(python, &content)
    }

    /// Parses PDB file and returns the Structure object.
    ///
    ///
    /// Parameters
    /// ----------
    /// path : str
    ///     The path to the PDB file.
    ///
    ///
    /// Returns
    /// -------
    /// Structure
    ///     Parsed structure.
    ///
    ///
    /// Examples
    /// --------
    /// Loading structure from file.
    ///
    /// >>> parser = nanoPDB.Parser()
    /// >>> structure = parser.parse("tests/1zhy.pdb")
    /// >>> structure
    ///
    /// Structure {
    ///     pdbid: "1ZHY",
    ///     classification: "LIPID BINDING PROTEIN",
    ///     date: "26-APR-05",
    /// }
    #[pyo3(signature = (path, /))]
    pub fn parse(&self, python: Python, path: String) -> PyResult<Structure> {
        let mut content = String::with_capacity(1024 * 1024 * 4);
        File::open(path)?.read_to_string(&mut content)?;

        parse_pdb(python, &content)
    }
}

#[inline(always)]
fn parse_numeric<T: FromStr>(
    line: &str,
    line_number: usize,
    from: usize,
    to: usize,
) -> PyResult<T> {
    line[from..to].trim().parse::<T>().map_err(|_| {
        PyException::new_err(format!(
            "error in line: {}, cannot parse numeric",
            line_number + 1
        ))
    })
}

#[inline(always)]
fn parse_header_into(line: &str, line_number: usize, structure: &mut Structure) -> PyResult<()> {
    if line.len() < 66 {
        return Err(PyException::new_err(format!(
            "error in line: {}, HEADER line to short",
            line_number + 1
        )));
    }

    let pdbid = line[62..66].trim();
    let classification = line[10..50].trim();
    let date = line[50..59].trim();

    structure.set_header(pdbid, classification, date);

    Ok(())
}

#[inline(always)]
fn parse_cryst1_into(
    python: Python,
    line: &str,
    line_number: usize,
    structure: &mut Structure,
) -> PyResult<()> {
    if line.len() < 54 {
        return Err(PyException::new_err(format!(
            "error in line: {}, CRYST1 line to short",
            line_number + 1
        )));
    }

    let a = parse_numeric::<f64>(line, line_number, 6, 15)?;
    let b = parse_numeric::<f64>(line, line_number, 15, 24)?;
    let c = parse_numeric::<f64>(line, line_number, 24, 33)?;
    let alpha = parse_numeric::<f64>(line, line_number, 33, 40)?;
    let beta = parse_numeric::<f64>(line, line_number, 40, 47)?;
    let gamma = parse_numeric::<f64>(line, line_number, 47, 54)?;

    let unit_cell = UnitCell::new(a, b, c, alpha, beta, gamma);
    structure.set_unit_cell(python, unit_cell)?;

    Ok(())
}

#[inline(always)]
fn parse_atom_into(
    python: Python,
    line: &str,
    line_number: usize,
    label: AtomType,
    structure: &mut Structure,
) -> PyResult<()> {
    if line.len() < 78 {
        return Err(PyException::new_err(format!(
            "error in line: {}, ATOM/HETATM line to short",
            line_number + 1
        )));
    }

    let atom_number = parse_numeric::<i32>(line, line_number, 6, 11)?;
    let atom_name = line[12..16].trim();
    let residue_name = line[17..20].trim();
    let chain_name = line.chars().nth(21).unwrap();
    let residue_number = parse_numeric::<i32>(line, line_number, 22, 26)?;
    let atom_pos_x = parse_numeric::<f64>(line, line_number, 30, 38)?;
    let atom_pos_y = parse_numeric::<f64>(line, line_number, 38, 46)?;
    let atom_pos_z = parse_numeric::<f64>(line, line_number, 46, 54)?;
    let atom_occupancy = parse_numeric::<f64>(line, line_number, 54, 60)?;
    let atom_element = line[76..78].trim();

    let full_add = match structure.chains.get(&chain_name) {
        // Chain exists:
        Some(chain) => {
            match chain
                .as_ref()
                .expect(concat!("memory error in: ", file!(), ", line: ", line!()))
                .borrow(python)
                .residues
                .get(&(residue_name.into(), residue_number))
            {
                // Residue exists (adds atom):
                Some(residue) => {
                    let atom = Atom::new(
                        label.clone(),
                        atom_number,
                        atom_name,
                        atom_element,
                        (atom_pos_x, atom_pos_y, atom_pos_z),
                        atom_occupancy,
                    );

                    residue
                        .as_ref()
                        .expect(concat!("memory error in: ", file!(), ", line: ", line!()))
                        .borrow_mut(python)
                        .atoms
                        .push(Some(Py::new(python, atom)?));

                    true
                }
                // Residue does not exist:
                None => false,
            }
        }
        // Chain does not exist (adds chain, adds residue, adds atom):
        None => {
            let atom = Atom::new(
                label.clone(),
                atom_number,
                atom_name,
                atom_element,
                (atom_pos_x, atom_pos_y, atom_pos_z),
                atom_occupancy,
            );
            let mut residue = Residue::new(residue_number, residue_name);
            let mut chain = Chain::new(chain_name);

            residue.atoms.push(Some(Py::new(python, atom)?));
            chain.residues.insert(
                (residue_name.into(), residue_number),
                Some(Py::new(python, residue)?),
            );
            structure
                .chains
                .insert(chain_name, Some(Py::new(python, Chain::new(chain_name))?));

            true
        }
    };

    if !full_add {
        let atom = Atom::new(
            label,
            atom_number,
            atom_name,
            atom_element,
            (atom_pos_x, atom_pos_y, atom_pos_z),
            atom_occupancy,
        );
        let mut residue = Residue::new(residue_number, residue_name);

        residue.atoms.push(Some(Py::new(python, atom)?));
        structure
            .chains
            .get(&chain_name)
            .unwrap()
            .as_ref()
            .expect(concat!("memory error in: ", file!(), ", line: ", line!()))
            .borrow_mut(python)
            .residues
            .insert(
                (residue_name.into(), residue_number),
                Some(Py::new(python, residue)?),
            );
    }

    Ok(())
}

#[inline(always)]
fn parse_pdb(python: Python, content: &str) -> PyResult<Structure> {
    let mut structure = Structure::new(python)?;

    for (line_number, line) in content.lines().enumerate() {
        if line.len() < 6 {
            // return Err(PyException::new_err(format!(
            //     "error in line: {}, label field error",
            //     line_number + 1
            // )));
            continue;
        }

        if &line[0..4] == "ATOM" {
            parse_atom_into(python, line, line_number, AtomType::ATOM, &mut structure)?;
        } else if &line[0..6] == "HETATM" {
            parse_atom_into(python, line, line_number, AtomType::HETATM, &mut structure)?;
        } else if &line[0..6] == "HEADER" {
            parse_header_into(line, line_number, &mut structure)?;
        } else if &line[0..6] == "CRYST1" {
            parse_cryst1_into(python, line, line_number, &mut structure)?;
        }
    }

    Ok(structure)
}
