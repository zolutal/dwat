import dwat
import sys

if len(sys.argv) < 2:
    print("Usage: dump.py <path>")

path = sys.argv[1]
dw = dwat.load_dwarf_path(path);

vars = dw.get_named_types_dict(dwat.NamedType.Struct)

for name, struct in vars.items():
    print(str(struct))
