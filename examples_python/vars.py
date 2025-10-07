# Example of doing variable analysis to all variables that are unions
import dwat
import sys

if len(sys.argv) < 2:
    print("Usage: vars.py <path>")

path = sys.argv[1]
dw = dwat.load_dwarf_path(path);

vars = dw.get_named_types(dwat.NamedType.Variable)

for name, var in vars:
    try:
        if isinstance(var.type(), dwat.Union):
            print(name, repr(var.type()))
    except:
        pass

