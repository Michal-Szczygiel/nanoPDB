from typing import Tuple


class UnitCell:
    a: float
    b: float
    c: float
    alpha: float
    beta: float
    gamma: float

    def __repr__(self) -> str: ...


class Atom:
    label: str
    number: int
    name: str
    element: str
    position: Tuple[float, float, float]
    occupancy: float

    def __repr__(self) -> str: ...


class Residue:
    number: int
    name: str

    def __getitem__(self, index: int) -> Atom: ...

    def __iter__(self) -> 'Residue': ...

    def __len__(self) -> int: ...

    def __next__(self) -> Atom: ...

    def __repr__(self) -> str: ...


class Chain:
    name: str

    def __getitem__(self, index: int) -> Residue: ...

    def __iter__(self) -> 'Chain': ...

    def __len__(self) -> int: ...

    def __next__(self) -> Residue: ...

    def __repr__(self) -> str: ...


class Structure:
    pdbid: str
    classification: str
    date: str
    unit_cell: UnitCell

    def __getitem__(self, index: int) -> Chain: ...

    def __iter__(self) -> 'Structure': ...

    def __len__(self) -> int: ...

    def __next__(self) -> Chain: ...

    def __repr__(self) -> str: ...


class Parser:
    def __init__(self) -> None: ...

    def fetch(self, pdbid: str) -> Structure: ...

    def parse(self, path: str) -> Structure: ...
