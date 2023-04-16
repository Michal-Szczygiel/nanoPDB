import nanoPDB

parser = nanoPDB.Parser()
structure = parser.fetch("1zhy")

print(structure)
print(structure.unit_cell)
