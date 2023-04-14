import nanoPDB

parser = nanoPDB.Parser()

structure = parser.fetch("1zhy")

for chain in structure:
    for residue in chain:
        print(residue)
