use crate::{
    atom::{Atom, AtomType},
    chain::Chain,
    residue::Residue,
    structure::Structure,
    unit_cell::UnitCell,
};

use pyo3::{exceptions::PyException, pyclass, pymethods, PyResult, Python};

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
        let mut content = String::with_capacity(1024);
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
    line[from..to]
        .trim()
        .parse::<T>()
        .map_err(|_| PyException::new_err(format!("error in line: {}", line_number + 1)))
}

#[inline(always)]
fn parse_atom(
    python: Python,
    line: &str,
    line_number: usize,
    label: AtomType,
    structure: &mut Structure,
    current_chain_name: &mut char,
    current_residue_number: &mut isize,
) -> PyResult<()> {
    if line.len() >= 78 {
        let atom_number = parse_numeric::<usize>(line, line_number, 6, 11)?;
        let atom_name = line[12..16].trim();
        let residue_name = &line[17..20];
        let chain_name = line.chars().nth(21).unwrap();
        let residue_number = parse_numeric::<isize>(line, line_number, 22, 26)?;
        let atom_pos_x = parse_numeric::<f64>(line, line_number, 30, 38)?;
        let atom_pos_y = parse_numeric::<f64>(line, line_number, 38, 46)?;
        let atom_pos_z = parse_numeric::<f64>(line, line_number, 46, 54)?;
        let atom_occupancy = parse_numeric::<f64>(line, line_number, 54, 60)?;
        let atom_element = line[76..78].trim();

        let atom = Atom::new(
            label,
            atom_number,
            atom_name,
            atom_element,
            (atom_pos_x, atom_pos_y, atom_pos_z),
            atom_occupancy,
        );

        if *current_residue_number == residue_number {
            structure.add_atom(python, atom)?;
        } else if *current_chain_name == chain_name {
            let residue = Residue::new(residue_number, residue_name);

            structure.add_residue(python, residue)?;
            structure.add_atom(python, atom)?;

            *current_residue_number = residue_number;
        } else {
            let chain = Chain::new(chain_name);
            let residue = Residue::new(residue_number, residue_name);

            structure.add_chain(python, chain)?;
            structure.add_residue(python, residue)?;
            structure.add_atom(python, atom)?;

            *current_chain_name = chain_name;
            *current_residue_number = residue_number;
        }

        Ok(())
    } else {
        Err(PyException::new_err(format!(
            "error in line: {}",
            line_number + 1
        )))
    }
}

#[inline(always)]
fn parse_header(line: &str, line_number: usize, structure: &mut Structure) -> PyResult<()> {
    if line.len() >= 66 {
        let pdbid = line[62..66].trim();
        let classification = line[10..50].trim();
        let date = line[50..59].trim();

        structure.set_header(pdbid, classification, date);

        Ok(())
    } else {
        Err(PyException::new_err(format!(
            "error in line: {}",
            line_number + 1
        )))
    }
}

#[inline(always)]
fn parse_cryst1(
    python: Python,
    line: &str,
    line_number: usize,
    structure: &mut Structure,
) -> PyResult<()> {
    if line.len() >= 54 {
        let a = parse_numeric::<f64>(line, line_number, 6, 15)?;
        let b = parse_numeric::<f64>(line, line_number, 15, 24)?;
        let c = parse_numeric::<f64>(line, line_number, 24, 33)?;
        let alpha = parse_numeric::<f64>(line, line_number, 33, 40)?;
        let beta = parse_numeric::<f64>(line, line_number, 40, 47)?;
        let gamma = parse_numeric::<f64>(line, line_number, 47, 54)?;

        let unit_cell = UnitCell::new(a, b, c, alpha, beta, gamma);
        structure.set_unit_cell(python, unit_cell)?;

        Ok(())
    } else {
        Err(PyException::new_err(format!(
            "error in line: {}",
            line_number + 1
        )))
    }
}

#[inline(always)]
fn parse_pdb(python: Python, content: &str) -> PyResult<Structure> {
    let mut current_chain_name: char = ' ';
    let mut current_residue_number: isize = isize::MIN;

    let mut structure = Structure::default();

    for (line_number, line) in content.lines().enumerate() {
        if &line[0..4] == "ATOM" {
            parse_atom(
                python,
                line,
                line_number,
                AtomType::ATOM,
                &mut structure,
                &mut current_chain_name,
                &mut current_residue_number,
            )?;
        } else if &line[0..6] == "HETATM" {
            parse_atom(
                python,
                line,
                line_number,
                AtomType::HETATM,
                &mut structure,
                &mut current_chain_name,
                &mut current_residue_number,
            )?;
        } else if &line[0..6] == "HEADER" {
            parse_header(line, line_number, &mut structure)?;
        } else if &line[0..6] == "CRYST1" {
            parse_cryst1(python, line, line_number, &mut structure)?;
        }
    }

    Ok(structure)
}
