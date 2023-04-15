import nanoPDB

parser = nanoPDB.Parser()
structure = parser.parse("tests/1zhy.pdb")

print(structure)
