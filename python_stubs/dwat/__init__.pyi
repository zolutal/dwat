import typing
from dwat import *

def load_dwarf(
    file: typing.IO,
) -> Dwarf: ...

def load_dwarf_path(
    path: str,
) -> Dwarf: ...

class Dwarf:
    def lookup_type(self, named_type: NamedType, name: str) -> Struct | Array | Enum | Pointer | Subroutine | Typedef | Union | Base | Const | Volatile | Restrict | Variable: ...

    def get_named_types_dict(self, named_type: NamedType) -> dict[str, Struct | Array | Enum | Pointer | Subroutine | Typedef | Union | Base | Const | Volatile | Restrict | Variable]: ...

    def get_named_types(self, named_type: NamedType) -> list[tuple[str, Struct | Array | Enum | Pointer | Subroutine | Typedef | Union | Base | Const | Volatile | Restrict | Variable]]: ...

class Struct:
    def members(self) -> list[Member]: ...
    byte_size: int | None
    name: str | None

class Array:
    def type(self) -> Struct | Array | Enum | Pointer | Subroutine | Typedef | Union | Base | Const | Volatile | Restrict: ...
    byte_size: int | None
    bounds: int

class Enum:
    def type(self) -> Struct | Array | Enum | Pointer | Subroutine | Typedef | Union | Base | Const | Volatile | Restrict: ...
    byte_size: int | None
    name: str | None

class Pointer:
    def type(self) -> Struct | Array | Enum | Pointer | Subroutine | Typedef | Union | Base | Const | Volatile | Restrict: ...
    def deref(self) -> Struct | Array | Enum | Pointer | Subroutine | Typedef | Union | Base | Const | Volatile | Restrict: ...
    byte_size: int | None

class Subroutine:
    def return_type(self) -> Struct | Array | Enum | Pointer | Subroutine | Typedef | Union | Base | Const | Volatile | Restrict: ...
    def params(self) -> list[Parameter]: ...

class Typedef:
    def type(self) -> Struct | Array | Enum | Pointer | Subroutine | Typedef | Union | Base | Const | Volatile | Restrict: ...
    byte_size: int | None
    name: str | None

class Union:
    def members(self) -> list[Member]: ...
    byte_size: int | None
    name: str | None

class Base:
    byte_size: int | None
    name: str | None

class Const:
    def type(self) -> Struct | Array | Enum | Pointer | Subroutine | Typedef | Union | Base | Const | Volatile | Restrict: ...
    byte_size: int | None
    name: str | None

class Volatile:
    def type(self) -> Struct | Array | Enum | Pointer | Subroutine | Typedef | Union | Base | Const | Volatile | Restrict: ...
    byte_size: int | None
    name: str | None

class Restrict:
    def type(self) -> Struct | Array | Enum | Pointer | Subroutine | Typedef | Union | Base | Const | Volatile | Restrict: ...
    byte_size: int | None
    name: str | None

class Member:
    def type(self) -> Struct | Array | Enum | Pointer | Subroutine | Typedef | Union | Base | Const | Volatile | Restrict: ...
    byte_size: int | None
    bit_size: int | None
    offset: str | None
    name: str | None

class Parameter:
    def type(self) -> Struct | Array | Enum | Pointer | Subroutine | Typedef | Union | Base | Const | Volatile | Restrict: ...

class Variable:
    def type(self) -> Struct | Array | Enum | Pointer | Subroutine | Typedef | Union | Base | Const | Volatile | Restrict: ...
    byte_size: int | None
    name: str | None
