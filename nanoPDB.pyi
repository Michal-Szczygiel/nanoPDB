from typing import Tuple, List


class Periodic:
    def __init__(self) -> None: ...

    def get_radius(self, atom: str) -> float: ...


class UnitCell:
    """
    UnitCell - a class that represents a unit cell of a PDB structure.
    """

    a: float
    """[`float`] Length of side 'a' of unit cell."""

    b: float
    """[`float`] Length of side 'b' of unit cell."""

    c: float
    """[`float`] Length of side 'c' of unit cell."""

    alpha: float
    """[`float`] Alpha angle ('b' -> 'c') of unit cell (in degrees)."""

    beta: float
    """[`float`] Beta angle ('c' -> 'a') of unit cell (in degrees)."""

    gamma: float
    """[`float`] amma angle ('a' -> 'b') of unit cell (in degrees)."""

    # -----------------------------------------------------------------------------------------
    # Special methods
    # -----------------------------------------------------------------------------------------

    def __repr__(self) -> str: ...


class Atom:
    """
    Atom - a class that represents an atom of a PDB structure.
    """

    label: str
    """[`str`] Indicates the type of atom."""

    number: int
    """[`int`] Atom number."""

    name: str
    """[`str`] Atom name."""

    element: str
    """[`str`] Chemical element name."""

    position: Tuple[float, float, float]
    """[`(float, float, float)`] Position of an atom in 3D space."""

    occupancy: float
    """[`float`] Atom occupancy."""

    # -----------------------------------------------------------------------------------------
    # Special methods
    # -----------------------------------------------------------------------------------------

    def __repr__(self) -> str: ...


class Residue:
    """
    Residue - a class that represents a residue of a PDB structure.
    """

    number: int
    """[`int`] Residue number."""

    name: str
    """[`str`] Residue name."""

    # -----------------------------------------------------------------------------------------
    # Special methods
    # -----------------------------------------------------------------------------------------

    def __getitem__(self, index: int) -> Atom: ...

    def __iter__(self) -> 'Residue': ...

    def __len__(self) -> int: ...

    def __next__(self) -> Atom: ...

    def __repr__(self) -> str: ...

    # -----------------------------------------------------------------------------------------
    # Methods
    # -----------------------------------------------------------------------------------------

    def get_atoms(self) -> List[Atom]:
        """
        Returns a list of atoms that builds the residue.


        # Returns
        `list[Atom]`:
            The list of atoms that builds the residue.


        # Examples
        ### Retrieving the list of atoms that builds the residue.

        >>> parser = nanoPDB.Parser()
        >>> structure = parser.fetch("1zhy")
        >>> residue = structure[0][0]
        ...
        >>> residue.get_atoms()

        ``` raw
        [Atom {
            label: "ATOM",
            number: 1,
            name: "N",
            element: "N",
            position: (
                42.854,
                36.56,
                10.394,
            ),
            occupancy: 1.0,
        }, Atom {
            label: "ATOM",
            number: 2,
            name: "CA",
            element: "C",
            position: (
                42.25,
                35.232,
                10.096,
            ),
            occupancy: 1.0,
        }, Atom {
            label: "ATOM",
            number: 3,
            name: "C",
            element: "C",
            position: (
                41.642,
                34.623,
                11.355,
            ),
            occupancy: 1.0,
        }
        ...
        ```
        """


class Chain:
    """
    Chain - a class that represents a chain of a PDB structure.
    """

    name: str
    """[`str`] Chain name."""

    # -----------------------------------------------------------------------------------------
    # Special methods
    # -----------------------------------------------------------------------------------------

    def __getitem__(self, index: int) -> Residue: ...

    def __iter__(self) -> 'Chain': ...

    def __len__(self) -> int: ...

    def __next__(self) -> Residue: ...

    def __repr__(self) -> str: ...

    # -----------------------------------------------------------------------------------------
    # Methods
    # -----------------------------------------------------------------------------------------

    def get_atoms(self) -> List[Atom]:
        """
        Returns a list of atoms that builds the chain.


        # Returns
        `list[Atom]`
            The list of atoms that build the chain.


        # Examples
        ### Retrieving the list of atoms that builds the chain.

        >>> parser = nanoPDB.Parser()
        >>> structure = parser.fetch("1zhy")
        >>> chain = structure[0]
        ...
        >>> chain.get_atoms()

        ``` raw
        [Atom {
            label: "ATOM",
            number: 1,
            name: "N",
            element: "N",
            position: (
                42.854,
                36.56,
                10.394,
            ),
            occupancy: 1.0,
        }, Atom {
            label: "ATOM",
            number: 2,
            name: "CA",
            element: "C",
            position: (
                42.25,
                35.232,
                10.096,
            ),
            occupancy: 1.0,
        }, Atom {
            label: "ATOM",
            number: 3,
            name: "C",
            element: "C",
            position: (
                41.642,
                34.623,
                11.355,
            ),
            occupancy: 1.0,
        }
        ...
        ```
        """

    def get_residues(self) -> List[Residue]:
        """
        Returns the list of residues that builds the chain.


        # Returns
        `list[Residue]`
            The list of residues that builds the chain.


        # Examples
        ### Retrieving the list of residues that builds the residue.

        >>> parser = nanoPDB.Parser()
        >>> structure = parser.fetch("1zhy")
        >>> chain = structure[0]
        ...
        >>> chain.get_residues()

        ``` raw
        [Residue {
            number: -1,
            name: "MET",
        }, Residue {
            number: 0,
            name: "ASP",
        }, Residue {
            number: 1,
            name: "PRO",
        }
        ...
        ```
        """


class Structure:
    """
    Structure - a class that represents a PDB structure.
    """

    pdbid: str
    """[`str`] PDB ID of a structure."""

    classification: str
    """[`str`] Classifications of a macromolecule."""

    date: str
    """[`str`] Deposition date."""

    unit_cell: UnitCell
    """[`UnitCell`] The unit cell of the structure."""

    # -----------------------------------------------------------------------------------------
    # Special methods
    # -----------------------------------------------------------------------------------------

    def __getitem__(self, index: int) -> Chain: ...

    def __iter__(self) -> 'Structure': ...

    def __len__(self) -> int: ...

    def __next__(self) -> Chain: ...

    def __repr__(self) -> str: ...

    # -----------------------------------------------------------------------------------------
    # Methods
    # -----------------------------------------------------------------------------------------

    def get_atoms(self) -> List[Atom]:
        """
        Returns a list of atoms that builds the structure.


        # Returns
        `list[Atom]`
            The list of atoms that build the structure.


        # Examples
        ### Retrieving the list of atoms that builds the structure.

        >>> parser = nanoPDB.Parser()
        >>> structure = parser.fetch("1zhy")
        ...
        >>> structure.get_atoms()

        ``` raw
        [Atom {
            label: "ATOM",
            number: 1,
            name: "N",
            element: "N",
            position: (
                42.854,
                36.56,
                10.394,
            ),
            occupancy: 1.0,
        }, Atom {
            label: "ATOM",
            number: 2,
            name: "CA",
            element: "C",
            position: (
                42.25,
                35.232,
                10.096,
            ),
            occupancy: 1.0,
        }, Atom {
            label: "ATOM",
            number: 3,
            name: "C",
            element: "C",
            position: (
                41.642,
                34.623,
                11.355,
            ),
            occupancy: 1.0,
        }
        ...
        ```
        """

    def get_chains(self) -> List[Chain]:
        """
        Returns the list of chains that builds the chain.


        # Returns
        `list[Chain]`
            The list of chains that builds the structure.


        # Examples
        ### Retrieving the list of chains that builds the structure.

        >>> parser = nanoPDB.Parser()
        >>> structure = parser.fetch("1zhy")
        ...
        >>> structure.get_chains()

        ``` raw
        [Chain {
            name: 'A',
        }]
        ```
        """

    def get_residues(self) -> List[Residue]:
        """
        Returns the list of residues that builds the structure.


        # Returns
        `list[Residue]`
            The list of residues that builds the structure.


        # Examples
        ### Retrieving the list of residues that builds the structure.

        >>> parser = nanoPDB.Parser()
        >>> structure = parser.fetch("1zhy")
        ...
        >>> structure.get_residues()

        ``` raw
        [Residue {
            number: -1,
            name: "MET",
        }, Residue {
            number: 0,
            name: "ASP",
        }, Residue {
            number: 1,
            name: "PRO",
        }
        ...
        ```
        """


class Parser:
    """
    Parser - a class for parsing structures in PDB format.
    """

    # -----------------------------------------------------------------------------------------
    # Special methods
    # -----------------------------------------------------------------------------------------

    def __init__(self) -> None: ...

    # -----------------------------------------------------------------------------------------
    # Methods
    # -----------------------------------------------------------------------------------------

    def fetch(self, pdbid: str) -> Structure:
        """
        Fetches structure from RCSB PDB database, parses it and returns Structure object.


        # Parameters
        `pdbid` : str
            PDB ID of structure from RCSB PDB.


        # Returns
        `Structure`
            Parsed structure.


        # Examples
        ### Fetching structure from RCSB PDB database.

        >>> parser = nanoPDB.Parser()
        >>> structure = parser.fetch("1zhy")
        ...
        >>> structure

        ``` raw
        Structure {
            pdbid: "1ZHY",
            classification: "LIPID BINDING PROTEIN",
            date: "26-APR-05",
        }
        ```
        """

    def parse(self, path: str) -> Structure:
        """
        Parses PDB file and returns the Structure object.


        # Parameters
        `path` : str
            The path to the PDB file.


        # Returns
        `Structure`
            Parsed structure.


        # Examples
        ### Loading structure from file.

        >>> parser = nanoPDB.Parser()
        >>> structure = parser.parse("tests/1zhy.pdb")
        ...
        >>> structure

        ``` raw
        Structure {
            pdbid: "1ZHY",
            classification: "LIPID BINDING PROTEIN",
            date: "26-APR-05",
        }
        ```
        """
