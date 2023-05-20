import nanoPDB



parser = nanoPDB.Parser()
structure = parser.fetch("1zhy")
residue = structure[0][0]

print(residue.get_atoms())
