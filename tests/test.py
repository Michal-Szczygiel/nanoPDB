import nanoPDB

periodic = nanoPDB.Periodic()

parser = nanoPDB.Parser()
structure = parser.fetch("1zhy")
residue = structure[0][0]
atom = residue[0]

print(periodic.get_radius(atom.element))
